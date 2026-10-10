use morphe_train::kaikki::Entry;
use morphe_train::spec::*;

pub struct Bashkir;

impl Language for Bashkir {
    fn spec(&self) -> Spec {
        let mut s = Spec::new("bashkir", "Bashkir", "ba");
        s.features = vec![
            feature(
                "Case",
                &[
                    ("Nominative", "absolute"),
                    ("Genitive", "genitive"),
                    ("Dative", "dative"),
                    ("Accusative", "accusative"),
                    ("Locative", "locative"),
                    ("Ablative", "ablative"),
                ],
            ),
            feature("Number", &[("Singular", "singular"), ("Plural", "plural")]),
        ];
        s.pos = vec![pos("noun", "noun", vec![group("", "", "Case Number")])
            .citation("NominativeSingular NominativePlural")
            .head_last()];
        s
    }

    /// Uncountable nouns' tables say `singular-only` instead of `singular`.
    fn prepare(&self, e: &mut Entry) {
        for f in &mut e.forms {
            for t in &mut f.tags {
                if t == "singular-only" {
                    *t = "singular".into();
                }
            }
        }
    }

    fn samples(&self) -> Vec<&'static str> {
        vec!["он", "китап", "бала", "өй", "шырпыл"]
    }

    fn readme(&self) -> String {
        r#"## Example

```rust
use bashkir::*;
assert_eq!(noun("он", Case::Ablative, Number::Plural), "ондарҙан");
println!("{}", noun("китап", Case::Dative, Number::Singular));
```

Nouns have six cases in both numbers. Wiktionary has no Bashkir verb
tables.
"#
        .into()
    }
}

pub fn language() -> Box<dyn Language> {
    Box::new(Bashkir)
}
