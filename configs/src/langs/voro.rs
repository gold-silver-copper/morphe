use morphe_train::spec::*;

pub struct Voro;

impl Language for Voro {
    fn spec(&self) -> Spec {
        let mut s = Spec::new("voro", "Võro", "vro");
        s.features = vec![
            feature(
                "Case",
                &[
                    ("Nominative", "nominative"),
                    ("Accusative", "accusative"),
                    ("Genitive", "genitive"),
                    ("Partitive", "partitive"),
                    ("Illative", "illative"),
                    ("Inessive", "inessive"),
                    ("Elative", "elative"),
                    ("Allative", "allative"),
                    ("Adessive", "adessive"),
                    ("Ablative", "ablative"),
                    ("Translative", "translative"),
                    ("Terminative", "terminative"),
                    ("Abessive", "abessive"),
                    ("Comitative", "comitative"),
                ],
            ),
            feature("Number", &[("Singular", "singular"), ("Plural", "plural")]),
        ];
        let mut adj = vec![group("", "", "Case Number")];
        adj.push(group("Comparative", "comparative", ""));
        s.pos = vec![
            pos("noun", "noun", vec![group("", "", "Case Number")])
                .citation("NominativeSingular GenitiveSingular PartitiveSingular")
                .head_last(),
            pos("adj", "adj", adj).citation("NominativeSingular GenitiveSingular PartitiveSingular"),
            pos(
                "verb",
                "verb",
                vec![
                    group("InfinitiveMa", "infinitive-ma", ""),
                    group("InfinitiveDa", "infinitive-da", ""),
                    group("PresentFirstSingular", "first-person present singular", ""),
                ],
            )
            .lemma_cell("InfinitiveMa")
            .citation("InfinitiveMa InfinitiveDa PresentFirstSingular")
            .head_first(),
        ];
        s
    }

    fn samples(&self) -> Vec<&'static str> {
        vec!["jalg", "luu", "lammas", "pikk", "elämä", "kirotama", "tsirbak"]
    }

    fn readme(&self) -> String {
        r#"## Example

```rust
use voro::*;
println!("{}", noun("jalg", Case::Genitive, Number::Plural)); // jalgo
println!("{}", noun("lammas", Case::Illative, Number::Singular)); // lambahe
println!("{}", noun("luu", Case::Comitative, Number::Plural)); // luiõgaq
println!("{}", verb_infinitive_da("elämä")); // elläq
```

Nouns and adjectives have fourteen cases in both numbers. Wiktionary has
no Võro conjugation tables; verbs have only the principal parts the
headword line gives (the ma- and da-infinitives and the first person
present).
"#
        .into()
    }
}

pub fn language() -> Box<dyn Language> {
    Box::new(Voro)
}
