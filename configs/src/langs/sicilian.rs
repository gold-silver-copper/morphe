use morphe_train::kaikki::{Entry, Form};
use morphe_train::spec::*;

pub struct Sicilian;

fn has(tags: &[String], t: &str) -> bool {
    tags.iter().any(|x| x == t)
}

impl Language for Sicilian {
    fn spec(&self) -> Spec {
        let mut s = Spec::new("sicilian", "Sicilian", "scn");
        s.unmark = unmark("àa èe ìi òo ùu âa êe îi ôo ûu ëe ïi");
        s.fold = vec![('‘', '\''), ('’', '\'')];
        s.soft_tags.extend(tags("pronunciation-spelling"));
        s.features = vec![
            feature("Gender", &[("Masculine", "masculine"), ("Feminine", "feminine")]),
            feature(
                "Form",
                &[
                    ("Feminine", "feminine singular"),
                    ("Plural", "plural"),
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
            pos("noun", "noun", vec![group("Plural", "plural !feminine", "")])
                .inherent("Gender")
                .citation("Lemma Plural")
                .head_last(),
            pos("adj", "adj", vec![group("", "", "Form")]).citation("Lemma Feminine Plural"),
            pos(
                "verb",
                "verb",
                vec![
                    group("Infinitive", "infinitive", ""),
                    group("Gerund", "gerund", ""),
                    group("ParticiplePast", "participle past", ""),
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
        // The conditional (identical to the imperfect subjunctive) comes
        // labeled only "indicative".
        for f in &mut e.forms {
            if f.source.is_some() && has(&f.tags, "error-unrecognized-form") && has(&f.tags, "indicative") {
                f.tags.retain(|t| t != "error-unrecognized-form" && t != "indicative");
                f.tags.push("conditional".into());
            }
        }
    }

    fn samples(&self) -> Vec<&'static str> {
        vec!["guerra", "lettu", "fari", "amari", "jiri", "duru", "flumpari", "snorcu"]
    }

    fn readme(&self) -> String {
        r#"## Example

```rust
use sicilian::*;
println!("{}", noun_plural("guerra")); // guerri
println!("{}", adj("duru", Form::Feminine)); // dura
println!("{}", verb("fari", Tense::Preterite, Person::Third, Number::Singular)); // fici
println!("{}", verb_participle_past("fari")); // fattu
```

Accents are optional on input.
"#
        .into()
    }
}

pub fn language() -> Box<dyn Language> {
    Box::new(Sicilian)
}
