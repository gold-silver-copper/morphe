use morphe_train::kaikki::{Entry, Form};
use morphe_train::spec::*;

pub struct Neapolitan;

fn has(tags: &[String], t: &str) -> bool {
    tags.iter().any(|x| x == t)
}

impl Language for Neapolitan {
    fn spec(&self) -> Spec {
        let mut s = Spec::new("neapolitan", "Neapolitan", "nap");
        s.unmark = unmark("àa áa âa èe ée êe ëe ìi íi îi òo óo ôo ùu");
        s.fold = vec![('’', '\'')];
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
                    ("Present", "indicative present"),
                    ("Imperfect", "indicative imperfect"),
                    ("Preterite", "indicative preterite"),
                    ("Future", "indicative future"),
                    ("Conditional", "conditional"),
                    ("ImperfectSubjunctive", "subjunctive imperfect"),
                ],
            ),
            feature("Person", &[("First", "first-person"), ("Second", "second-person"), ("Third", "third-person")]),
            feature("PTense", &[("Present", "present"), ("Past", "past")]),
            feature("Number", &[("Singular", "singular"), ("Plural", "plural")]),
        ];
        s.pos = vec![
            pos("noun", "noun", vec![group("Plural", "plural !feminine !masculine", "")])
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
        for f in &mut e.forms {
            if f.source.is_none() || !has(&f.tags, "error-unrecognized-form") {
                continue;
            }
            // The first person singular and the whole preterite come
            // half-labeled: "facevo" (imperfect, singular), "facette"
            // (indicative, singular).
            f.tags.retain(|t| t != "error-unrecognized-form");
            let tense = ["present", "imperfect", "future", "conditional"].iter().any(|t| has(&f.tags, t));
            if has(&f.tags, "indicative") && !tense {
                f.tags.push("preterite".into());
            }
            let person = ["first-person", "second-person", "third-person"].iter().any(|t| has(&f.tags, t));
            if !person {
                f.tags.push("first-person".into());
            }
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

    fn split_form(&self, raw: &str) -> Vec<String> {
        // The negative imperative: "nun fà".
        if raw.starts_with("nun ") {
            return Vec::new();
        }
        vec![raw.to_string()]
    }

    fn samples(&self) -> Vec<&'static str> {
        vec!["mana", "lupo", "fare", "parlà", "auto", "santo", "flumpà", "snorchio"]
    }

    fn readme(&self) -> String {
        r#"## Example

```rust
use neapolitan::*;
println!("{}", noun_plural("mana")); // mmane
println!("{}", adj("auto", Form::Feminine)); // auta
println!("{}", verb("fare", Tense::Preterite, Person::Third, Number::Singular)); // facette
println!("{}", verb_participle("fare", PTense::Past)); // fatto
```

Accents are optional on input.
"#
        .into()
    }
}

pub fn language() -> Box<dyn Language> {
    Box::new(Neapolitan)
}
