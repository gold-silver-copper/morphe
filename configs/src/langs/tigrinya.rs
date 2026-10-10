use morphe_train::kaikki::Entry;
use morphe_train::spec::*;

pub struct Tigrinya;

/// The rows of the conjugation table, in order.
const ROWS: [&str; 5] = ["first-person", "second-person masculine", "second-person feminine", "third-person masculine", "third-person feminine"];
const TENSES: [&str; 5] = ["perfect", "gerund", "imperfect", "jussive", "imperative"];

impl Language for Tigrinya {
    fn spec(&self) -> Spec {
        let mut s = Spec::new("tigrinya", "Tigrinya", "ti");
        s.features = vec![
            feature(
                "Person",
                &[
                    ("First", "first-person"),
                    ("SecondMasculine", "second-person masculine"),
                    ("SecondFeminine", "second-person feminine"),
                    ("ThirdMasculine", "third-person masculine"),
                    ("ThirdFeminine", "third-person feminine"),
                ],
            ),
            feature("Number", &[("Singular", "singular"), ("Plural", "plural")]),
            feature("Gender", &[("Masculine", "masculine"), ("Feminine", "feminine")]),
            feature(
                "Tense",
                &[("Perfect", "perfect"), ("Gerund", "gerund"), ("Imperfect", "imperfect"), ("Jussive", "jussive")],
            ),
        ];
        s.pos = vec![
            pos("noun", "noun", vec![group("Plural", "plural", "")]).citation("Lemma Plural").head_last(),
            pos(
                "verb",
                "verb",
                vec![
                    group("Infinitive", "infinitive", ""),
                    group("", "x-finite !imperative", "Tense Person Number"),
                    group("Imperative", "x-finite imperative !negative", "Gender Number"),
                    group("NegativeImperative", "x-finite imperative negative", "Gender Number"),
                ],
            )
            .citation("PerfectThirdMasculineSingular ImperfectThirdMasculineSingular")
            .head_last(),
        ];
        s
    }

    /// The conjugation table tags the persons of only some cells. Each
    /// tense and number runs down the rows in order (first person, second
    /// masculine and feminine, third masculine and feminine; the imperative
    /// has only the second persons), so a form's row is how many forms of
    /// its tense and number came before it.
    fn prepare(&self, e: &mut Entry) {
        let mut seen: Vec<String> = Vec::new();
        for f in e.forms.iter_mut().filter(|f| f.source.is_some()) {
            let has = |t: &str| f.tags.iter().any(|x| x == t);
            let Some(tense) = TENSES.iter().find(|t| has(t)) else { continue };
            let number = if has("plural") { "plural" } else { "singular" };
            let key = format!("{tense} {number} {}", has("negative"));
            let n = seen.iter().filter(|k| **k == key).count();
            seen.push(key);
            let rows: &[&str] = if *tense == "imperative" { &["masculine", "feminine"] } else { &ROWS };
            let Some(row) = rows.get(n) else { continue };
            let negative = if has("negative") { "negative" } else { "" };
            f.tags = tags(&format!("x-finite {tense} {number} {row} {negative}"));
        }
    }

    fn samples(&self) -> Vec<&'static str> {
        vec!["ሰበረ", "ከፈተ", "ወዲ", "ቀተለ", "ሰለመ"]
    }

    fn readme(&self) -> String {
        r#"## Example

```rust
use tigrinya::*;
println!("{}", verb("ሰበረ", Tense::Perfect, Person::First, Number::Singular)); // ሰበርኩ
println!("{}", verb("ሰበረ", Tense::Imperfect, Person::ThirdMasculine, Number::Plural)); // ይሰብሩ
println!("{}", verb_imperative("ሰበረ", Gender::Feminine, Number::Singular)); // ስብሪ
println!("{}", verb_infinitive("ከፈተ")); // ምክፋት
println!("{}", noun_plural("ወዲ")); // ኣወዳት
```

Verbs are cited by the perfect third person masculine singular and have
the perfect, gerund, imperfect and jussive for ten persons, the
imperatives and the infinitive. Nouns have their plural.
"#
        .into()
    }
}

pub fn language() -> Box<dyn Language> {
    Box::new(Tigrinya)
}
