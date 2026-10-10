use morphe_train::spec::*;

pub struct Latgalian;

impl Language for Latgalian {
    fn spec(&self) -> Spec {
        let mut s = Spec::new("latgalian", "Latgalian", "ltg");
        s.features = vec![
            feature(
                "Case",
                &[
                    ("Nominative", "nominative"),
                    ("Genitive", "genitive"),
                    ("Dative", "dative"),
                    ("Accusative", "accusative"),
                    ("Instrumental", "instrumental"),
                    ("Locative", "locative"),
                    ("Vocative", "vocative"),
                ],
            ),
            feature("Number", &[("Singular", "singular"), ("Plural", "plural")]),
            feature("Gender", &[("Masculine", "masculine"), ("Feminine", "feminine")]),
            feature("Definiteness", &[("Indefinite", "indefinite"), ("Definite", "definite")]),
        ];
        s.pos = vec![
            pos("noun", "noun", vec![group("", "", "Case Number")])
                .inherent("Gender")
                .citation("NominativeSingular GenitiveSingular")
                .head_last(),
            pos("adj", "adj", vec![group("", "", "Definiteness Gender Case Number")]).head_last(),
            pos("pron", "pron", vec![group("", "", "Gender? Case Number?")]),
            pos("num", "num", vec![group("", "", "Gender? Case Number?")]),
        ];
        s
    }

    fn samples(&self) -> Vec<&'static str> {
        vec!["ass", "mama", "muoja", "rats", "lels", "kirpaks"]
    }

    fn readme(&self) -> String {
        r#"## Example

```rust
use latgalian::*;
println!("{}", noun("mama", Case::Dative, Number::Singular)); // mamai
println!("{}", adj("rats", Definiteness::Definite, Gender::Feminine, Case::Nominative, Number::Singular)); // ratuo
```

Nouns have seven cases in both numbers; dialectal forms are kept as
low-weight alternatives. Adjectives decline in both genders, definite and
indefinite. Wiktionary has Latgalian tables for a few hundred words; verbs
are too few to learn from and are left out.
"#
        .into()
    }
}

pub fn language() -> Box<dyn Language> {
    Box::new(Latgalian)
}
