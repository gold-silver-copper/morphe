use morphe_train::kaikki::Entry;
use morphe_train::spec::*;

pub struct Tatar;

impl Language for Tatar {
    fn spec(&self) -> Spec {
        let mut s = Spec::new("tatar", "Tatar", "tt");
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
                ],
            ),
            feature("Number", &[("Singular", "singular"), ("Plural", "plural")]),
            feature(
                "Possessor",
                &[
                    ("FirstSingular", "first-person singular"),
                    ("SecondSingular", "second-person singular"),
                    ("ThirdSingular", "third-person singular"),
                    ("FirstPlural", "first-person plural"),
                    ("SecondPlural", "second-person plural"),
                    ("ThirdPlural", "third-person plural"),
                ],
            ),
            feature("Possessed", &[("One", "possessed-single"), ("Many", "possessed-many")]),
        ];
        s.pos = vec![pos(
            "noun",
            "noun",
            vec![group("", "!possessive", "Case Number"), group("Possessive", "possessive", "Possessor Possessed Case")],
        )
        .citation("NominativeSingular NominativePlural")
        .head_last()];
        s
    }

    /// The possessive table lists each cell twice, for one thing owned and
    /// for many, under the same tags.
    fn prepare(&self, e: &mut Entry) {
        let mut k = 0usize;
        for f in e.forms.iter_mut().filter(|f| f.source.is_some()) {
            if f.tags.iter().any(|t| t == "possessive") {
                f.tags.push(if k % 2 == 0 { "possessed-single" } else { "possessed-many" }.into());
                k += 1;
            }
        }
    }

    fn samples(&self) -> Vec<&'static str> {
        vec!["республика", "китап", "өй", "бала", "шырпыл"]
    }

    fn readme(&self) -> String {
        r#"## Example

```rust
use tatar::*;
println!("{}", noun("республика", Case::Dative, Number::Plural)); // республикаларга
println!("{}", noun_possessive("республика", Possessor::FirstSingular, Possessed::Many, Case::Nominative)); // республикаларым
```

Nouns have six cases in both numbers and the possessed forms for six
possessors, with one thing or many owned. Wiktionary has Tatar tables for
a few hundred nouns; verbs are too few to learn from and are left out.
"#
        .into()
    }
}

pub fn language() -> Box<dyn Language> {
    Box::new(Tatar)
}
