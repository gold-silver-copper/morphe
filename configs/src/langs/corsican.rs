use morphe_train::kaikki::{Entry, Form};
use morphe_train::spec::*;

pub struct Corsican;

fn has(tags: &[String], t: &str) -> bool {
    tags.iter().any(|x| x == t)
}

impl Language for Corsican {
    fn spec(&self) -> Spec {
        let mut s = Spec::new("corsican", "Corsican", "co");
        s.unmark = unmark("àa èe ée ìi òo ùu ïi");
        s.features = vec![
            feature("Gender", &[("Masculine", "masculine"), ("Feminine", "feminine")]),
            feature(
                "Form",
                &[
                    ("Feminine", "feminine singular"),
                    ("MasculinePlural", "masculine plural"),
                    ("FemininePlural", "feminine plural"),
                ],
            ),
            feature(
                "Tense",
                &[
                    ("Present", "indicative present !conditional"),
                    ("Imperfect", "indicative imperfect"),
                    ("Past", "indicative past"),
                    ("Future", "indicative future"),
                    ("Conditional", "conditional"),
                    ("Subjunctive", "subjunctive present"),
                    ("ImperfectSubjunctive", "subjunctive imperfect"),
                ],
            ),
            feature("Person", &[("First", "first-person"), ("Second", "second-person"), ("Third", "third-person")]),
            feature("PTense", &[("Present", "present"), ("Past", "past")]),
            feature("Number", &[("Singular", "singular"), ("Plural", "plural")]),
        ];
        s.pos = vec![
            pos("noun", "noun", vec![group("Plural", "plural !feminine", "")])
                .inherent("Gender")
                .citation("Lemma Plural")
                .head_last(),
            pos("adj", "adj", vec![group("", "", "Form")]).citation("Lemma Feminine MasculinePlural"),
            pos(
                "verb",
                "verb",
                vec![
                    group("Infinitive", "infinitive", ""),
                    group("Participle", "participle", "PTense"),
                    group("", "!imperative", "Tense Person Number"),
                    group("Imperative", "imperative", "Person Number"),
                ],
            )
            .citation("Infinitive PresentFirstSingular ParticiplePast")
            .head_first(),
        ];
        s
    }

    fn prepare(&self, e: &mut Entry) {
        if !e.forms.iter().any(|f| has(&f.tags, "canonical")) {
            e.forms.push(Form { form: e.word.clone(), tags: vec!["canonical".into()], source: None });
        }
        if e.pos == "adj" {
            // A bare plural serves both genders.
            let mut extra = Vec::new();
            for f in &mut e.forms {
                if f.tags.len() == 1 && has(&f.tags, "plural") {
                    f.tags.push("masculine".into());
                    let mut g = f.clone();
                    g.tags = vec!["feminine".into(), "plural".into()];
                    extra.push(g);
                }
            }
            e.forms.extend(extra);
        }
    }

    fn samples(&self) -> Vec<&'static str> {
        vec!["aria", "cane", "ride", "accumpagnà", "europeu", "inglese", "flumpà", "snorcu"]
    }

    fn readme(&self) -> String {
        r#"## Example

```rust
use corsican::*;
println!("{}", noun_plural("aria")); // arie
println!("{}", adj("europeu", Form::FemininePlural)); // europee
println!("{}", verb("ride", Tense::Past, Person::Third, Number::Singular)); // rise
println!("{}", verb_participle("ride", PTense::Past)); // risu
```

Forms follow Wiktionary's tables, mostly Cismontane. Accents are optional on
input.
"#
        .into()
    }
}

pub fn language() -> Box<dyn Language> {
    Box::new(Corsican)
}
