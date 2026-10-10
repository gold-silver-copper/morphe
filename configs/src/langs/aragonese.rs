use morphe_train::kaikki::{Entry, Form};
use morphe_train::spec::*;

pub struct Aragonese;

fn has(tags: &[String], t: &str) -> bool {
    tags.iter().any(|x| x == t)
}

impl Language for Aragonese {
    fn spec(&self) -> Spec {
        let mut s = Spec::new("aragonese", "Aragonese", "an");
        s.unmark = unmark("áa ée íi óo úu üu");
        // Forms of other areas on the headword line.
        s.soft_tags.extend(tags("Western Eastern Central"));
        s.features = vec![
            feature("Gender", &[("Masculine", "masculine"), ("Feminine", "feminine")]),
            feature(
                "Form",
                &[
                    ("Feminine", "feminine !plural"),
                    ("MasculinePlural", "masculine plural"),
                    ("FemininePlural", "feminine plural"),
                ],
            ),
            feature(
                "GenderNumber",
                &[
                    ("Masculine", "masculine singular"),
                    ("Feminine", "feminine singular"),
                    ("MasculinePlural", "masculine plural"),
                    ("FemininePlural", "feminine plural"),
                ],
            ),
            feature(
                "Tense",
                &[
                    ("Present", "indicative present"),
                    ("Imperfect", "indicative imperfect"),
                    ("Preterite", "indicative preterite"),
                    ("Future", "indicative future"),
                    ("Conditional", "conditional"),
                    ("Subjunctive", "subjunctive present"),
                    ("ImperfectSubjunctive", "subjunctive imperfect"),
                ],
            ),
            feature("Person", &[("First", "first-person"), ("Second", "second-person"), ("Third", "third-person")]),
            feature("Number", &[("Singular", "singular"), ("Plural", "plural")]),
        ];
        s.pos = vec![
            pos("noun", "noun", vec![group("Plural", "plural !feminine", ""), group("Diminutive", "diminutive", "")])
                .inherent("Gender")
                .citation("Lemma Plural")
                .head_last(),
            pos("adj", "adj", vec![group("", "", "Form")]).citation("Lemma Feminine MasculinePlural"),
            pos(
                "verb",
                "verb",
                vec![
                    group("Infinitive", "infinitive", ""),
                    group("Gerund", "gerund", ""),
                    group("Participle", "participle", "GenderNumber"),
                    group("", "!imperative !participle", "Tense Person Number"),
                    group("Imperative", "imperative second-person", "Number"),
                ],
            )
            .citation("Infinitive PresentFirstSingular ParticipleMasculine")
            .head_first(),
        ];
        s
    }

    fn prepare(&self, e: &mut Entry) {
        if !e.forms.iter().any(|f| has(&f.tags, "canonical")) {
            e.forms.push(Form { form: e.word.clone(), tags: vec!["canonical".into()], source: None });
        }
        let mut extra = Vec::new();
        for f in &mut e.forms {
            if f.source.is_some() && has(&f.tags, "error-unrecognized-form") {
                // The second person plural (sigaz, estiez) loses its person.
                f.tags.retain(|t| t != "error-unrecognized-form");
                f.tags.push("second-person".into());
            } else if e.pos == "adj" && f.tags.len() == 1 && has(&f.tags, "plural") {
                // One plural for both genders.
                f.tags.push("masculine".into());
                let mut g = f.clone();
                g.tags = tags("feminine plural");
                extra.push(g);
            }
        }
        e.forms.extend(extra);
    }

    fn samples(&self) -> Vec<&'static str> {
        vec!["abet", "lobo", "estar", "cantar", "negro", "libre", "flumpar", "snorco"]
    }

    fn readme(&self) -> String {
        r#"## Example

```rust
use aragonese::*;
println!("{}", noun_plural("abet")); // abetz
println!("{}", adj("negro", Form::FemininePlural)); // negras
println!("{}", verb("estar", Tense::Imperfect, Person::Third, Number::Singular)); // yera
println!("{}", verb_participle("estar", GenderNumber::Feminine)); // estada
```

Accents are optional on input.
"#
        .into()
    }
}

pub fn language() -> Box<dyn Language> {
    Box::new(Aragonese)
}
