use morphe_train::spec::*;

pub struct Erzya;

impl Language for Erzya {
    fn spec(&self) -> Spec {
        let mut s = Spec::new("erzya", "Erzya", "myv");
        s.features = vec![
            feature(
                "Case",
                &[
                    ("Nominative", "nominative"),
                    ("Genitive", "genitive"),
                    ("Dative", "dative"),
                    ("Ablative", "ablative"),
                    ("Inessive", "inessive"),
                    ("Elative", "elative"),
                    ("Illative", "illative"),
                    ("Prolative", "prolative"),
                    ("Translative", "translative"),
                    ("Abessive", "abessive"),
                    ("Comparative", "comparative"),
                ],
            ),
            feature("Number", &[("Singular", "singular"), ("Plural", "plural")]),
        ];
        // Only some nouns have a definite table; a separate part of speech
        // lets the others get predicted definite forms rather than none.
        s.pos = vec![
            pos(
                "noun",
                "noun",
                vec![group("", "indefinite", "Case Number")],
            )
            .citation("NominativeSingular NominativePlural")
            .head_last(),
            pos(
                "noun_definite",
                "noun",
                vec![group("", "definite", "Case[Nominative,Genitive,Dative,Ablative,Inessive,Elative,Illative,Prolative,Abessive,Comparative] Number")],
            )
            .head_last(),
        ];
        s
    }

    fn samples(&self) -> Vec<&'static str> {
        vec!["он", "кудо", "вирь", "ялга", "шырпе"]
    }

    fn readme(&self) -> String {
        r#"## Example

```rust
use erzya::*;
println!("{}", noun("он", Case::Inessive, Number::Singular)); // онсо
println!("{}", noun("он", Case::Nominative, Number::Plural)); // онт
println!("{}", noun_definite("он", Case::Genitive, Number::Plural)); // онтнэнь
```

Nouns have eleven cases in the indefinite singular and the indefinite
nominative plural (the other indefinite plural cases are not used, and
have no form), and ten cases of the definite declension in both numbers. Wiktionary has no
Erzya conjugation tables.
"#
        .into()
    }
}

pub fn language() -> Box<dyn Language> {
    Box::new(Erzya)
}
