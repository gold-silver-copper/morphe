use morphe_train::kaikki::Entry;
use morphe_train::spec::*;
use std::sync::OnceLock;

pub struct Catalan;

impl Language for Catalan {
    fn spec(&self) -> Spec {
        let mut s = Spec::new("catalan", "Catalan", "ca");
        // Alternative spellings (including Valencian and Balearic ones) are
        // other words.
        s.drop_tags.extend(tags("alternative abbreviation diminutive augmentative dated"));
        s.soft_tags.extend(tags("poetic"));
        s.features = vec![
            feature("Number", &[("Singular", "!plural"), ("Plural", "plural")]),
            feature("Gender", &[("Masculine", "masculine"), ("Feminine", "feminine")]),
            feature("Person", &[("First", "first-person"), ("Second", "second-person"), ("Third", "third-person")]),
            feature(
                "Tense",
                &[("Present", "present"), ("Imperfect", "imperfect"), ("Preterite", "preterite"), ("Future", "future")],
            ),
        ];
        // Headword lines give principal parts without the mood tag.
        let indicative = "!subjunctive !conditional !imperative !infinitive !participle !gerund";
        s.pos = vec![
            pos(
                "noun",
                "noun",
                vec![group("", "", "Number[Plural]"), group("Counterpart", "", "Gender Number")],
            )
            .inherent("Gender")
            .citation("Lemma Plural")
            // Only nouns with a gendered counterpart (gat, gata) have one.
            .missing_is_absent()
            .head_first(),
            pos("adj", "adj", vec![group("", "", "Gender? Number")]).citation("MasculineSingular FeminineSingular").head_first(),
            pos(
                "verb",
                "verb",
                vec![
                    group("Infinitive", "infinitive", ""),
                    group("Gerund", "gerund", ""),
                    group("Participle", "participle past", "Gender Number"),
                    group("Indicative", indicative, "Tense Person Number"),
                    group("Conditional", "conditional", "Person Number"),
                    group("Subjunctive", "subjunctive", "Tense[Present,Imperfect] Person Number"),
                    group("Imperative", "imperative !negative", "Person Number"),
                ],
            )
            .citation("Infinitive IndicativePresentFirstSingular IndicativePreteriteFirstSingular ParticipleMasculineSingular")
            .head_first(),
            // The personal pronoun table is one suppletive paradigm shared by
            // every personal pronoun (jo, em, -me…), not an inflection.
            pos("pron", "pron", vec![group("", "!personal !contracted", "Gender? Number")]),
            pos("det", "det", vec![group("", "", "Gender? Number")]),
            pos("article", "article", vec![group("", "", "Gender? Number")]),
        ];
        s
    }

    /// The formal imperative (parli, parlin: vostè, vostès) is grammatically
    /// third person.
    fn retag(&self, _pos: &PosSpec, _form: &str, tags: &mut Vec<String>) {
        if tags.iter().any(|t| t == "second-person-semantically") {
            tags.push("third-person".into());
        }
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
        if pos.name == "noun" {
            fill_counterpart(names, lemma, forms);
        } else {
            fill_lemma(names, lemma, forms);
        }
    }

    fn samples(&self) -> Vec<&'static str> {
        vec!["gat", "ciutat", "estar", "parlar", "bo", "feliç", "brugar", "flometa"]
    }

    fn readme(&self) -> String {
        r#"## Example

```rust
use catalan::*;
assert_eq!(noun("ciutat", Number::Plural), "ciutats");
assert_eq!(noun_counterpart("gat", Gender::Feminine, Number::Singular), "gata");
assert_eq!(adj("feliç", Gender::Masculine, Number::Plural), "feliços");
assert_eq!(verb_indicative("estar", Tense::Present, Person::First, Number::Singular), "estic");
assert_eq!(verb_subjunctive("ser", Tense::Present, Person::Third, Number::Singular), "sigui");
assert_eq!(verb_participle("viure", Gender::Feminine, Number::Plural), "viscudes");
println!("{:?}", noun_gender("mà"));
// Made-up words inflect by analogy:
println!("{}", verb_indicative("brugar", Tense::Preterite, Person::First, Number::Singular));
```

Spelling is matched as written, accents included. Forms are Central
Catalan; Valencian and Balearic spellings Wiktionary lists as alternatives
are left out. The periphrastic past (`vaig parlar`) and negative imperatives
(`no parlis`) are built from these forms.
"#
        .into()
    }
}

/// A noun with a counterpart of the other gender (gat, gata) is itself the
/// counterpart of its own gender: gata's feminine counterpart is gata.
fn fill_counterpart(names: &[String], lemma: &str, forms: &mut Vec<(usize, String, bool)>) {
    let idx = |n: &str| names.iter().position(|c| c == n);
    let (Some(plural), Some(ms), Some(mp), Some(fs), Some(fp)) = (
        idx("Plural"),
        idx("CounterpartMasculineSingular"),
        idx("CounterpartMasculinePlural"),
        idx("CounterpartFeminineSingular"),
        idx("CounterpartFemininePlural"),
    ) else {
        return;
    };
    let has = |ci: usize| forms.iter().any(|f| f.0 == ci);
    let own = match (has(ms) || has(mp), has(fs) || has(fp)) {
        (true, false) => (fs, fp),
        (false, true) => (ms, mp),
        _ => return,
    };
    let plurals: Vec<(usize, String, bool)> = forms.iter().filter(|f| f.0 == plural).map(|f| (own.1, f.1.clone(), f.2)).collect();
    forms.push((own.0, lemma.to_string(), false));
    forms.extend(plurals);
}

/// The dictionary form is a cell Wiktionary doesn't list: the masculine
/// singular of adjectives and pronouns (or the feminine singular when the
/// lemma is feminine, `les` → `el`). Gender-invariable words (`abaxial`) get
/// the lemma in both singular cells. Forms with spaces on one-word lemmas
/// are notes, not forms.
fn fill_lemma(names: &[String], lemma: &str, forms: &mut Vec<(usize, String, bool)>) {
    forms.retain(|f| f.1.chars().any(char::is_alphabetic) && (lemma.contains(' ') || !f.1.contains(' ')));
    // Entries without forms are form-of entries (`augustes`), not lemmas.
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
    Box::new(Catalan)
}
