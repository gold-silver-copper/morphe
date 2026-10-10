use morphe_train::kaikki::{Entry, Form};
use morphe_train::spec::*;

pub struct Telugu;

/// Persons in the order of the full conjugation table (`te-regular-verb`).
const PERSONS: [&str; 7] = [
    "first-person singular",
    "second-person singular",
    "third-person masculine singular",
    "third-person feminine neuter singular",
    "first-person plural",
    "second-person plural",
    "third-person neuter plural",
];
/// Persons in the order of the past-tense table (`l`).
const PAST_PERSONS: [&str; 8] = [
    "first-person singular",
    "first-person plural",
    "second-person singular",
    "second-person plural",
    "third-person masculine singular",
    "third-person feminine singular",
    "third-person neuter singular",
    "third-person neuter plural",
];

impl Language for Telugu {
    fn spec(&self) -> Spec {
        let mut s = Spec::new("telugu", "Telugu", "te");
        // The vocative particle ఓ is written into the table cells.
        s.strip_words = vec!["ఓ".into()];
        s.features = vec![
            feature(
                "Case",
                &[
                    ("Nominative", "nominative"),
                    ("Accusative", "accusative"),
                    ("Dative", "dative"),
                    ("Genitive", "genitive"),
                    ("Instrumental", "instrumental"),
                    ("Locative", "locative"),
                    ("Ablative", "ablative"),
                    ("Vocative", "vocative"),
                ],
            ),
            feature("Number", &[("Singular", "singular"), ("Plural", "plural")]),
            // The second person plural also serves the third person plural
            // of people (వారు).
            feature(
                "Person",
                &[
                    ("FirstSingular", "first-person singular"),
                    ("SecondSingular", "second-person singular"),
                    ("ThirdMasculine", "third-person masculine singular"),
                    ("ThirdFeminine", "third-person feminine singular"),
                    ("ThirdNeuter", "third-person neuter singular"),
                    ("FirstPlural", "first-person plural"),
                    ("SecondPlural", "second-person plural"),
                    ("ThirdNeuterPlural", "third-person neuter plural"),
                ],
            ),
            feature(
                "Tense",
                &[("Past", "x-past"), ("Future", "x-future"), ("Present", "x-present"), ("Negative", "x-negative")],
            ),
            feature("Number2", &[("Singular", "singular"), ("Plural", "plural")]),
        ];
        s.pos = vec![
            pos("noun", "noun", vec![group("", "", "Case Number")])
                .citation("NominativeSingular NominativePlural")
                .lemma_cell("NominativeSingular")
                .head_last(),
            pos(
                "verb",
                "verb",
                vec![
                    group("Infinitive", "infinitive", ""),
                    group("Conditional", "conditional", ""),
                    group("PastParticiple", "participle past", ""),
                    group("Imperative", "imperative", "Number2"),
                    group("Hortative", "hortative", ""),
                    group("", "x-finite", "Tense Person"),
                ],
            )
            .citation("ImperativeSingular PastFirstSingular")
            .lemma_cell("ImperativeSingular")
            .head_last(),
        ];
        s
    }

    /// Headword plurals come split (కర్రీ లు); they are joined and read as
    /// nominatives. Verb tables tag few persons; their order is fixed.
    fn prepare(&self, e: &mut Entry) {
        let spaced = e.word.contains(' ');
        for f in e.forms.iter_mut().filter(|f| f.source.is_none()) {
            if !spaced {
                f.form = f.form.replace(' ', "");
            }
            if e.pos == "noun" && f.tags.len() == 1 {
                match f.tags.first().map(String::as_str) {
                    Some("plural") => f.tags.push("nominative".into()),
                    Some("genitive" | "oblique") => f.tags = tags("genitive singular"),
                    _ => {}
                }
            }
        }
        if e.pos != "verb" {
            return;
        }
        let forms = std::mem::take(&mut e.forms);
        let mut out = Vec::new();
        // (template, forms) per table.
        let mut tables: Vec<(String, Vec<Form>)> = Vec::new();
        for f in forms {
            if f.source.is_none() {
                out.push(f);
            } else if f.tags.iter().any(|t| t == "table-tags") {
                tables.push((String::new(), Vec::new()));
            } else if let Some((template, table)) = tables.last_mut() {
                if f.tags.iter().any(|t| t == "inflection-template") {
                    *template = f.form;
                } else {
                    table.push(f);
                }
            }
        }
        let with = |f: &Form, t: &str| Form { form: f.form.clone(), tags: tags(t), source: f.source.clone() };
        for (template, table) in &tables {
            match template.as_str() {
                "l" if table.len() >= 7 => {
                    for (f, p) in table.iter().zip(PAST_PERSONS) {
                        out.push(with(f, &format!("x-finite x-past {p}")));
                    }
                }
                "te-regular-verb" => {
                    let mut rows: Vec<(&str, Vec<&Form>)> = Vec::new();
                    let mut imperative = 0;
                    for f in table {
                        let has = |t: &str| f.tags.iter().any(|x| x == t);
                        let row = if has("negative") {
                            "x-negative"
                        } else if has("past") && has("indicative") {
                            "x-past"
                        } else if has("future") {
                            "x-future"
                        } else if has("indicative") {
                            "x-present"
                        } else {
                            if has("imperative") {
                                out.push(with(f, &format!("imperative {}", if imperative == 0 { "singular" } else { "plural" })));
                                imperative += 1;
                            } else if !has("error-unrecognized-form") {
                                out.push(f.clone());
                            }
                            continue;
                        };
                        match rows.last_mut() {
                            Some((r, v)) if *r == row => v.push(f),
                            _ => rows.push((row, vec![f])),
                        }
                    }
                    for (row, forms) in rows {
                        for (f, p) in forms.iter().zip(PERSONS) {
                            out.push(with(f, &format!("x-finite {row} {p}")));
                        }
                    }
                }
                _ => {}
            }
        }
        e.forms = out;
    }

    fn samples(&self) -> Vec<&'static str> {
        vec!["ఇల్లు", "కుమారుడు", "పుస్తకం", "అరుచు", "విస్తరించు", "చదువు", "గులబం", "జపటించు"]
    }

    fn readme(&self) -> String {
        r#"## Example

```rust
use telugu::*;
println!("{}", noun("ఇల్లు", Case::Dative, Number::Singular)); // ఇంటికి
println!("{}", noun("పుస్తకం", Case::Nominative, Number::Plural)); // పుస్తకాలు
println!("{}", verb("అరుచు", Tense::Past, Person::FirstSingular)); // అరిచాను
println!("{}", verb("అరుచు", Tense::Future, Person::ThirdMasculine)); // అరుస్తాడు
println!("{}", verb("విస్తరించు", Tense::Past, Person::SecondPlural)); // విస్తరించారు
```

Nouns have eight cases in both numbers, though Wiktionary gives most nouns
only their plural. Verbs are cited by the imperative singular, as in
Wiktionary, and have the past for eight persons; a few have full tables
with the future, the present progressive, the negative, the infinitive,
conditional, past participle, imperatives and hortative. The second person
plural also serves the human third person plural (వారు).
"#
        .into()
    }
}

pub fn language() -> Box<dyn Language> {
    Box::new(Telugu)
}
