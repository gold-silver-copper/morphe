use morphe_train::kaikki::Entry;
use morphe_train::spec::*;
use std::sync::OnceLock;

pub struct Galician;

impl Language for Galician {
    fn spec(&self) -> Spec {
        let mut s = Spec::new("galician", "Galician", "gl");
        // Alternative spellings (including reintegrationist ones) are other
        // words.
        s.drop_tags.extend(tags("alternative abbreviation diminutive augmentative dated"));
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
            pos("adj", "adj", vec![group("", "", "Gender? Number")]).head_first(),
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
            pos("pron", "pron", vec![group("", "!accusative !dative !genitive", "Gender? Number")]),
            pos("det", "det", vec![group("", "", "Gender? Number")]),
            pos("article", "article", vec![group("", "", "Gender? Number")]),
            pos("num", "num", vec![group("", "", "Gender? Number")]),
        ];
        s
    }

    fn prepare(&self, e: &mut Entry) {
        // The standard conjugation comes first; a second table gives the
        // reintegrationist spellings (abatêrom, abaterám).
        if let Some(second) = e.forms.iter().enumerate().filter(|(_, f)| f.tags.iter().any(|t| t == "inflection-template")).nth(1).map(|x| x.0) {
            e.forms.truncate(second);
        }
        // The standard table's third-person rows lose their person label.
        for f in &mut e.forms {
            if f.source.is_some() && f.tags.iter().any(|t| t == "error-unrecognized-form") {
                f.tags.retain(|t| t != "error-unrecognized-form");
                let person = f.tags.iter().any(|t| t.ends_with("-person"));
                if !person && !f.tags.iter().any(|t| t == "participle") {
                    f.tags.push("third-person".into());
                }
            }
        }
    }

    fn keep_entry(&self, e: &Entry) -> bool {
        let reintegrationist = e.forms.iter().find(|f| f.tags.iter().any(|t| t == "inflection-template")).is_some_and(|f| f.form.contains("reinteg"));
        !reintegrationist && e.word.chars().any(char::is_alphabetic) && !e.word.chars().any(|c| c.is_ascii_digit())
    }

    fn adjust(&self, pos: &PosSpec, lemma: &str, forms: &mut Vec<(usize, String, bool)>) {
        static CELLS: OnceLock<Vec<(String, Vec<String>)>> = OnceLock::new();
        let cells = CELLS.get_or_init(|| {
            let s = self.spec();
            s.pos.iter().map(|p| (p.name.clone(), s.cells(p).into_iter().map(|c| c.name).collect())).collect()
        });
        let Some((_, names)) = cells.iter().find(|c| c.0 == pos.name) else { return };
        fill_lemma(names, lemma, forms);
    }

    fn samples(&self) -> Vec<&'static str> {
        vec!["can", "cidade", "falar", "abater", "frío", "feliz", "brugar", "flomento"]
    }

    fn readme(&self) -> String {
        r#"## Example

```rust
use galician::*;
assert_eq!(noun("can", Number::Plural), "cans");
assert_eq!(noun("animal", Number::Plural), "animais");
assert_eq!(adj("frío", Gender::Feminine, Number::Plural), "frías");
assert_eq!(verb_indicative("abater", Tense::Preterite, Person::First, Number::Singular), "abatín");
assert_eq!(verb_indicative("abater", Tense::Present, Person::Third, Number::Plural), "abaten");
assert_eq!(verb_personal_infinitive("falar", Person::First, Number::Plural), "falarmos");
println!("{:?}", noun_gender("mel"));
// Made-up words inflect by analogy:
println!("{}", verb_indicative("brugar", Tense::Preterite, Person::First, Number::Singular));
```

Forms follow the official (RAG) norm; the reintegrationist conjugations
Wiktionary also gives are left out. Negative imperatives (`non fales`) are
the present subjunctive.
"#
        .into()
    }
}

/// The dictionary form is a cell Wiktionary doesn't list: the masculine
/// singular of adjectives and pronouns (or the feminine singular when the
/// lemma is feminine, `unha` → `un`). Gender-invariable words (`livre`) get
/// the lemma in both singular cells. Forms with spaces on one-word lemmas
/// are notes, not forms.
fn fill_lemma(names: &[String], lemma: &str, forms: &mut Vec<(usize, String, bool)>) {
    forms.retain(|f| f.1.chars().any(char::is_alphabetic) && (lemma.contains(' ') || !f.1.contains(' ')));
    // Entries without forms are form-of entries (`frías`), not lemmas.
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
    Box::new(Galician)
}
