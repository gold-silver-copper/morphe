use morphe_train::kaikki::Entry;
use morphe_train::spec::*;

pub struct Mongolian;

impl Language for Mongolian {
    fn spec(&self) -> Spec {
        let mut s = Spec::new("mongolian", "Mongolian", "mn");
        s.marks = vec!['\u{301}'];
        s.features = vec![
            feature(
                "Case",
                &[
                    ("Nominative", "nominative"),
                    ("Genitive", "genitive"),
                    ("Accusative", "accusative"),
                    ("DativeLocative", "dative locative"),
                    ("Ablative", "ablative"),
                    ("Instrumental", "instrumental"),
                    ("Comitative", "comitative"),
                    ("Privative", "privative"),
                    ("Directive", "directive"),
                    ("Substantive", "substantive"),
                ],
            ),
            feature("Number", &[("Singular", "singular"), ("Plural", "plural")]),
        ];
        s.pos = vec![pos(
            "noun",
            "noun",
            vec![
                group("", "!possessive", "Case Number"),
                group("Reflexive", "possessive", "Case[Nominative,Genitive,Accusative,DativeLocative,Ablative,Instrumental,Comitative,Privative,Directive] Number"),
                group("Attributive", "attributive", ""),
            ],
        )
        .citation("NominativeSingular GenitiveSingular NominativePlural")
        .head_last()];
        s
    }

    /// The genitive-based substantive form (зөгийнх, "the bee's one")
    /// comes untagged.
    fn prepare(&self, e: &mut Entry) {
        for f in e.forms.iter_mut().filter(|f| f.source.is_some()) {
            if let Some(t) = f.tags.iter_mut().find(|t| *t == "error-unrecognized-form") {
                *t = "substantive".into();
            }
        }
    }

    fn samples(&self) -> Vec<&'static str> {
        vec!["зөгий", "ном", "гэр", "хүн", "мод", "шавхар"]
    }

    fn readme(&self) -> String {
        r#"## Example

```rust
use mongolian::*;
println!("{}", noun("зөгий", Case::Genitive, Number::Singular)); // зөгийн
println!("{}", noun("ном", Case::Ablative, Number::Plural));
println!("{}", noun_reflexive("гэр", Case::DativeLocative, Number::Singular)); // гэртээ
```

Nouns have nine cases and the substantive (зөгийнх, "the bee's") in both
numbers, the reflexive-possessive forms (-аа, "one's own"), and the
attributive form. Wiktionary has Mongolian tables for a few hundred nouns;
verbs have almost none and are left out.
"#
        .into()
    }
}

pub fn language() -> Box<dyn Language> {
    Box::new(Mongolian)
}
