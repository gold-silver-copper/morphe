use morphe_train::kaikki::{Entry, Form};
use morphe_train::spec::*;

pub struct Friulian;

fn has(tags: &[String], t: &str) -> bool {
    tags.iter().any(|x| x == t)
}

impl Language for Friulian {
    fn spec(&self) -> Spec {
        let mut s = Spec::new("friulian", "Friulian", "fur");
        s.unmark = unmark("âa àa êe èe îi ìi ôo òo ûu ùu çc");
        s.features = vec![
            feature("Gender", &[("Masculine", "masculine"), ("Feminine", "feminine")]),
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
            feature("Number", &[("Singular", "singular"), ("Plural", "plural")]),
            feature(
                "Imperative",
                &[
                    ("SecondSingular", "second-person singular"),
                    ("FirstPlural", "first-person plural"),
                    ("SecondPlural", "second-person plural"),
                ],
            ),
        ];
        s.pos = vec![
            pos("noun", "noun", vec![group("Plural", "plural", "")])
                .inherent("Gender")
                .citation("Lemma Plural")
                .head_last(),
            pos("adj", "adj", vec![group("Feminine", "feminine", "")]).citation("Lemma Feminine"),
            pos(
                "verb",
                "verb",
                vec![
                    group("Infinitive", "infinitive", ""),
                    group("Gerund", "gerund", ""),
                    group("ParticiplePast", "participle past", "Gender Number"),
                    group("", "!imperative !participle", "Tense Person Number"),
                    group("Imperative", "imperative", "Imperative"),
                ],
            )
            .citation("Infinitive PresentFirstSingular ParticiplePastMasculineSingular")
            .head_first(),
        ];
        s
    }

    fn prepare(&self, e: &mut Entry) {
        if !e.forms.iter().any(|f| has(&f.tags, "canonical")) {
            e.forms.push(Form { form: e.word.clone(), tags: vec!["canonical".into()], source: None });
        }
        // The headword line's participle is the masculine singular.
        for f in &mut e.forms {
            if f.source.is_none() && has(&f.tags, "participle") && !has(&f.tags, "feminine") {
                f.tags.extend(["masculine".to_string(), "singular".to_string()]);
            }
        }
    }

    fn split_form(&self, raw: &str) -> Vec<String> {
        // Finite forms come with their subject clitic: "o bat", "tu batis",
        // and "al/e bat" split into "al" and "e bat".
        match raw.split_once(' ') {
            None if raw == "al" => Vec::new(),
            Some((clitic, rest)) if matches!(clitic, "o" | "tu" | "al" | "e" | "a") => vec![rest.to_string()],
            _ => vec![raw.to_string()],
        }
    }

    fn samples(&self) -> Vec<&'static str> {
        vec!["abandon", "sang", "bati", "cjantâ", "lei", "rot", "flompâ", "snorc"]
    }

    fn readme(&self) -> String {
        r#"## Example

```rust
use friulian::*;
println!("{}", noun_plural("abandon")); // abandons
println!("{}", verb("bati", Tense::Imperfect, Person::Second, Number::Singular)); // batevis
println!("{}", verb_participle_past("bati", Gender::Feminine, Number::Plural)); // batûdis
```

Finite forms are given without their subject clitic ("tu batis" → "batis").
Accents and ç are optional on input.
"#
        .into()
    }
}

pub fn language() -> Box<dyn Language> {
    Box::new(Friulian)
}
