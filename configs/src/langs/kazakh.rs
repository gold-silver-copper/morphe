use morphe_train::kaikki::Entry;
use morphe_train::spec::*;

pub struct Kazakh;

impl Language for Kazakh {
    fn spec(&self) -> Spec {
        let mut s = Spec::new("kazakh", "Kazakh", "kk");
        s.features = vec![
            feature(
                "Case",
                &[
                    ("Nominative", "nominative"),
                    ("Genitive", "genitive"),
                    ("Dative", "dative"),
                    ("Accusative", "accusative"),
                    ("Locative", "locative"),
                    ("Ablative", "ablative"),
                    ("Instrumental", "instrumental"),
                    ("Equative", "equative"),
                ],
            ),
            feature("Number", &[("Singular", "singular"), ("Plural", "plural")]),
            feature(
                "Person",
                &[
                    ("First", "first-person"),
                    ("Second", "second-person informal"),
                    ("SecondFormal", "second-person formal"),
                    ("Third", "third-person"),
                ],
            ),
            feature(
                "Tense",
                &[
                    ("Aorist", "aorist"),
                    ("Preterite", "preterite"),
                    ("Perfect", "perfect"),
                    ("Pluperfect", "pluperfect"),
                    ("Renarrative", "renarrative"),
                    ("PastHabitual", "habitual past"),
                    ("Presumptive", "presumptive"),
                    ("Intentive", "intentive"),
                    ("Optative", "optative"),
                    ("Conditional", "conditional"),
                    ("Imperative", "imperative"),
                ],
            ),
        ];
        s.pos = vec![
            pos("noun", "noun", vec![group("", "", "Case Number")])
                .citation("NominativeSingular NominativePlural")
                .head_last(),
            pos("verb", "verb", vec![group("", "", "Tense Person Number")]).head_last(),
        ];
        s
    }

    /// The parser leaves the noun equative (-дай) and the verb imperative
    /// untagged; other untagged verb rows (converb tables) are left out.
    fn prepare(&self, e: &mut Entry) {
        for f in e.forms.iter_mut().filter(|f| f.source.is_some()) {
            if !f.tags.iter().any(|t| t == "error-unrecognized-form") {
                continue;
            }
            f.tags.retain(|t| t != "error-unrecognized-form");
            if e.pos == "noun" {
                f.tags.push("equative".into());
            } else if f.tags.iter().all(|t| {
                ["first-person", "second-person", "third-person", "singular", "plural", "formal", "informal"].contains(&t.as_str())
            }) {
                f.tags.push("imperative".into());
            } else {
                f.tags = tags("table-tags");
            }
        }
    }

    fn samples(&self) -> Vec<&'static str> {
        vec!["үй", "кітап", "бала", "келу", "бару", "жазу", "шырпыл", "қырбаю"]
    }

    fn readme(&self) -> String {
        r#"## Example

```rust
use kazakh::*;
assert_eq!(noun("кітап", Case::Dative, Number::Singular), "кітапқа");
println!("{}", noun("үй", Case::Locative, Number::Plural)); // үйлерде
println!("{}", noun("бала", Case::Equative, Number::Singular)); // баладай
println!("{}", verb("келу", Tense::Preterite, Person::First, Number::Singular)); // келдім
println!("{}", verb("келу", Tense::Imperative, Person::SecondFormal, Number::Singular)); // келіңіз
```

Nouns have eight cases, including the equative (-дай, "like"), in both
numbers. Verbs are looked up by their verbal noun (келу) and have eleven
tenses and moods for the first person, the informal and formal second
person, and the third person. Wiktionary has conjugation tables for a few
hundred verbs only.
"#
        .into()
    }
}

pub fn language() -> Box<dyn Language> {
    Box::new(Kazakh)
}
