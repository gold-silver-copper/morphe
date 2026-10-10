use morphe_train::kaikki::Entry;
use morphe_train::spec::*;

pub struct Greenlandic;

const CASES: [&str; 7] = ["absolutive", "ergative", "instrumental", "allative", "ablative", "locative", "prolative"];

impl Language for Greenlandic {
    fn spec(&self) -> Spec {
        let mut s = Spec::new("greenlandic", "Greenlandic", "kl");
        s.features = vec![
            feature(
                "Case",
                &[
                    ("Absolutive", "absolutive"),
                    ("Ergative", "ergative"),
                    ("Instrumental", "instrumental"),
                    ("Allative", "allative"),
                    ("Ablative", "ablative"),
                    ("Locative", "locative"),
                    ("Prolative", "prolative"),
                ],
            ),
            feature("Number", &[("Singular", "singular"), ("Plural", "plural")]),
        ];
        s.pos = vec![pos(
            "noun",
            "noun",
            vec![group("", "!equative", "Case Number"), group("Equative", "equative !plural", "")],
        )
        .citation("AbsolutiveSingular AbsolutivePlural")
        .head_last()];
        s
    }

    /// Nouns used in one number only name just the case.
    fn prepare(&self, e: &mut Entry) {
        for f in e.forms.iter_mut().filter(|f| f.source.is_some()) {
            let case = f.tags.iter().any(|t| CASES.contains(&t.as_str()));
            if case && !f.tags.iter().any(|t| t == "singular" || t == "plural") {
                f.tags.push("singular".into());
            }
        }
    }

    fn samples(&self) -> Vec<&'static str> {
        vec!["radio", "illu", "nuna", "panna", "qimmeq", "sikuttaaq"]
    }

    fn readme(&self) -> String {
        r#"## Example

```rust
use greenlandic::*;
println!("{}", noun("illu", Case::Locative, Number::Singular)); // illumi
println!("{}", noun("radio", Case::Ablative, Number::Plural)); // radioniit
println!("{}", noun_equative("radio")); // radiotut
```

Nouns have seven cases (absolutive, ergative, instrumental, allative,
ablative, locative, prolative) in both numbers, plus the equative.
Wiktionary has conjugation tables for a handful of verbs only, too few to
learn from; verbs are left out.
"#
        .into()
    }
}

pub fn language() -> Box<dyn Language> {
    Box::new(Greenlandic)
}
