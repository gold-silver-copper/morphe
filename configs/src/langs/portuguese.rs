use morphe_train::kaikki::Entry;
use morphe_train::spec::*;
use std::sync::OnceLock;

pub struct Portuguese;

impl Language for Portuguese {
    fn spec(&self) -> Spec {
        let mut s = Spec::new("portuguese", "Portuguese", "pt");
        // Alternative spellings, diminutives and the like are other words;
        // comparatives are analytic (mais famoso).
        s.drop_tags.extend(tags("alternative abbreviation diminutive augmentative comparative dated"));
        s.soft_tags.extend(tags("poetic"));
        s.features = vec![
            feature("Number", &[("Singular", "!plural"), ("Plural", "plural")]),
            feature("Gender", &[("Masculine", "masculine"), ("Feminine", "feminine")]),
            feature("Person", &[("First", "first-person"), ("Second", "second-person"), ("Third", "third-person")]),
            feature(
                "Tense",
                &[
                    ("Present", "present"),
                    ("Imperfect", "imperfect"),
                    ("Preterite", "preterite"),
                    ("Pluperfect", "pluperfect"),
                    ("Future", "future"),
                ],
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
            .head_first(),
            pos(
                "adj",
                "adj",
                vec![
                    group("", "!superlative", "Gender? Number"),
                    group("Superlative", "superlative !feminine !plural", "").declines("adj", ""),
                ],
            )
            .head_first(),
            pos(
                "verb",
                "verb",
                vec![
                    group("Infinitive", "infinitive impersonal", ""),
                    group("PersonalInfinitive", "infinitive !impersonal", "Person Number"),
                    group("Gerund", "gerund", ""),
                    group("Participle", "participle past !short-form", "Gender Number"),
                    group("ShortParticiple", "participle past short-form !feminine !plural", "").declines("adj", ""),
                    group("Indicative", indicative, "Tense Person Number"),
                    group("Conditional", "conditional", "Person Number"),
                    group("Subjunctive", "subjunctive", "Tense[Present,Imperfect,Future] Person Number"),
                    group("Imperative", "imperative !negative", "Person Number"),
                ],
            )
            .citation("Infinitive IndicativePresentFirstSingular IndicativePreteriteFirstSingular ParticipleMasculineSingular")
            .head_first(),
            pos("pron", "pron", vec![group("", "", "Gender? Number")]),
            pos("det", "det", vec![group("", "", "Gender? Number")]),
            pos("article", "article", vec![group("", "", "Gender? Number")]),
            pos("num", "num", vec![group("", "", "Gender? Number")]),
        ];
        s
    }

    fn prepare(&self, e: &mut Entry) {
        // A second conjugation table is the pronominal verb (grisar-se,
        // grisar-me-ei…): another lemma.
        if let Some(second) = e.forms.iter().enumerate().filter(|(_, f)| f.tags.iter().any(|t| t == "inflection-template")).nth(1).map(|x| x.0) {
            e.forms.truncate(second);
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
        if pos.name == "verb" && !lemma.contains('-') {
            // Forms with enclitic pronouns (queixo-me) belong to pronominal verbs.
            forms.retain(|f| !f.1.contains('-'));
        }
        fill_lemma(names, lemma, forms);
    }

    fn samples(&self) -> Vec<&'static str> {
        vec!["cidade", "mão", "ter", "falar", "feliz", "famoso", "brugar", "flomento"]
    }

    fn readme(&self) -> String {
        r#"## Example

```rust
use portuguese::*;
assert_eq!(noun("cidade", Number::Plural), "cidades");
assert_eq!(noun("pão", Number::Plural), "pães");
assert_eq!(adj("feliz", Gender::Masculine, Number::Plural), "felizes");
assert_eq!(verb_indicative("ter", Tense::Present, Person::First, Number::Singular), "tenho");
assert_eq!(verb_subjunctive("fazer", Tense::Future, Person::Third, Number::Plural), "fizerem");
assert_eq!(verb_personal_infinitive("falar", Person::First, Number::Plural), "falarmos");
println!("{:?}", noun_gender("mão"));
// Made-up words inflect by analogy:
println!("{}", verb_indicative("brugar", Tense::Preterite, Person::First, Number::Singular));
```

Spelling is matched as written, accents included. Both Brazilian and
European spellings of a form are kept where Wiktionary lists both
(`amamos`, `amámos`). Negative imperatives (`não fales`) are the
present subjunctive.
"#
        .into()
    }
}

/// The dictionary form is a cell Wiktionary doesn't list: the masculine
/// singular of adjectives and pronouns (or the feminine singular when the
/// lemma is feminine, `uma` → `um`). Gender-invariable words (`livre`) get
/// the lemma in both singular cells. Forms with spaces on one-word lemmas
/// are notes, not forms.
fn fill_lemma(names: &[String], lemma: &str, forms: &mut Vec<(usize, String, bool)>) {
    forms.retain(|f| f.1.chars().any(char::is_alphabetic) && (lemma.contains(' ') || !f.1.contains(' ')));
    // Entries without forms are form-of entries (`belas`), not lemmas.
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
    Box::new(Portuguese)
}
