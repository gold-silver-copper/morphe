use morphe_train::kaikki::Entry;
use morphe_train::spec::*;
use std::sync::OnceLock;

pub struct French;

impl Language for French {
    fn spec(&self) -> Spec {
        let mut s = Spec::new("french", "French", "fr");
        // Reflexive tables (se fier, me fie…) and compound tenses (avoir +
        // past participle) are phrases; alternative spellings are other words.
        s.drop_tags.extend(tags("reflexive multiword-construction alternative abbreviation dated"));
        s.soft_tags.extend(tags("poetic Louisiana"));
        s.features = vec![
            feature("Number", &[("Singular", "!plural"), ("Plural", "plural")]),
            feature("Gender", &[("Masculine", "masculine"), ("Feminine", "feminine")]),
            feature("Person", &[("First", "first-person"), ("Second", "second-person"), ("Third", "third-person")]),
            feature(
                "Tense",
                &[("Present", "present"), ("Imperfect", "imperfect"), ("PastHistoric", "historic past"), ("Future", "future")],
            ),
        ];
        let nominal = "!emphatic !possessive !negative";
        s.pos = vec![
            pos("noun", "noun", vec![group("", "", "Number[Plural]"), group("Counterpart", "", "Gender")])
                .inherent("Gender")
                .citation("Lemma Plural")
                .head_first(),
            pos("adj", "adj", vec![group("", "!before-vowel", "Gender? Number"), group("BeforeVowel", "before-vowel", "")]).citation("MasculineSingular FeminineSingular"),
            pos(
                "verb",
                "verb",
                vec![
                    group("Infinitive", "infinitive", ""),
                    group("PresentParticiple", "participle present", ""),
                    group("PastParticiple", "participle past", "").declines("adj", ""),
                    group("Indicative", "indicative", "Tense Person Number"),
                    group("Conditional", "conditional", "Person Number"),
                    group("Subjunctive", "subjunctive", "Tense[Present,Imperfect] Person Number"),
                    group("Imperative", "imperative", "Person[First,Second] Number"),
                ],
            )
            .citation("Infinitive PresentParticiple PastParticiple IndicativePresentFirstSingular IndicativePastHistoricFirstSingular")
            .head_first(),
            pos("pron", "pron", vec![group("", nominal, "Gender? Number")]),
            pos(
                "det",
                "det",
                vec![group("", &format!("{nominal} !before-vowel"), "Gender? Number"), group("BeforeVowel", "before-vowel", "")],
            ),
            pos("article", "article", vec![group("", nominal, "Gender? Number")]),
            pos("num", "num", vec![group("", "", "Gender? Number")]),
        ];
        s
    }

    fn keep_entry(&self, e: &Entry) -> bool {
        e.word.chars().any(char::is_alphabetic) && !e.word.chars().any(|c| c.is_ascii_digit())
    }

    fn adjust(&self, pos: &PosSpec, lemma: &str, forms: &mut Vec<(usize, String, bool)>) {
        static CELLS: OnceLock<Vec<(String, Vec<String>)>> = OnceLock::new();
        let cells = CELLS.get_or_init(|| {
            let s = self.spec();
            s.pos.iter().map(|p| (p.name.clone(), s.cells(p).into_iter().map(|c| c.name).collect())).collect()
        });
        let Some((_, names)) = cells.iter().find(|c| c.0 == pos.name) else { return };
        if pos.name == "verb" {
            // Pronominal-only verbs (s'absenter) list forms with their
            // reflexive pronoun: the verb's own forms are what's left.
            for f in forms.iter_mut() {
                for p in ["me ", "te ", "se ", "nous ", "vous ", "m'", "t'", "s'", "m’", "t’", "s’"] {
                    if let Some(rest) = f.1.strip_prefix(p) {
                        f.1 = rest.to_string();
                    }
                }
                for p in ["-toi", "-nous", "-vous"] {
                    if let Some(rest) = f.1.strip_suffix(p) {
                        f.1 = rest.to_string();
                    }
                }
            }
        }
        fill_lemma(names, lemma, forms);
    }

    fn samples(&self) -> Vec<&'static str> {
        vec!["cheval", "chien", "aller", "prendre", "beau", "rapide", "flurger", "glorbeau"]
    }

    fn readme(&self) -> String {
        r#"## Example

```rust
use french::*;
assert_eq!(noun("cheval", Number::Plural), "chevaux");
assert_eq!(noun_counterpart("chien", Gender::Feminine), "chienne");
assert_eq!(adj("beau", Gender::Feminine, Number::Singular), "belle");
assert_eq!(verb_indicative("aller", Tense::Present, Person::First, Number::Singular), "vais");
assert_eq!(verb_subjunctive("faire", Tense::Present, Person::First, Number::Plural), "fassions");
assert_eq!(verb_past_participle_declined("prendre", Gender::Feminine, Number::Plural), "prises");
println!("{:?}", noun_gender("maison"));
// Made-up words inflect by analogy:
println!("{}", verb_indicative("flurger", Tense::PastHistoric, Person::Third, Number::Plural));
```

Spelling is matched as written, accents included. Reflexive conjugations
(`se fier`) and compound tenses (`avoir` + past participle) are not
modelled: build them from the simple forms.
"#
        .into()
    }
}

/// The dictionary form is a cell Wiktionary doesn't list: the masculine
/// singular of adjectives and pronouns (or the feminine singular when the
/// lemma is feminine, `une` → `un`). Gender-invariable words (`rapide`) get
/// the lemma in both singular cells. Forms with spaces on one-word lemmas
/// are notes, not forms.
fn fill_lemma(names: &[String], lemma: &str, forms: &mut Vec<(usize, String, bool)>) {
    forms.retain(|f| f.1.chars().any(char::is_alphabetic) && (lemma.contains(' ') || !f.1.contains(' ')));
    // Entries without forms are form-of entries (`lentes`), not lemmas.
    if forms.is_empty() {
        return;
    }
    let idx = |n: &str| names.iter().position(|c| c == n);
    let Some((ms, fs, mp, fp)) = [("MasculineSingular", "FeminineSingular", "MasculinePlural", "FemininePlural"), ("Masculine", "Feminine", "", "")]
        .into_iter()
        .find_map(|(m, f, mp, fp)| Some((idx(m)?, idx(f)?, idx(mp), idx(fp))))
    else {
        return;
    };
    let set = |ci: Option<usize>, forms: &[(usize, String, bool)]| -> Vec<String> {
        let mut v: Vec<String> = forms.iter().filter(|f| Some(f.0) == ci).map(|f| f.1.clone()).collect();
        v.sort();
        v
    };
    let has = |ci: usize, forms: &[(usize, String, bool)]| forms.iter().any(|f| f.0 == ci);
    match (has(ms, forms), has(fs, forms)) {
        (false, false) => {
            // A feminine singular that differs would be listed (feliç,
            // feliços, felices).
            let invariable = set(mp, forms) == set(fp, forms) || !set(fp, forms).is_empty();
            forms.push((ms, lemma.to_string(), false));
            if invariable {
                forms.push((fs, lemma.to_string(), false));
            }
        }
        (true, false) => forms.push((fs, lemma.to_string(), false)),
        (false, true) => forms.push((ms, lemma.to_string(), false)),
        (true, true) => {}
    }
}

pub fn language() -> Box<dyn Language> {
    Box::new(French)
}
