use morphe_train::kaikki::Entry;
use morphe_train::spec::*;

pub struct KildinSami;

impl Language for KildinSami {
    fn spec(&self) -> Spec {
        let mut s = Spec::new("kildin_sami", "KildinSami", "sjd");
        // Long vowels (кӣлл, по̄ррк) are often typed without the macron.
        s.marks = vec!['\u{304}'];
        s.unmark = unmark("ӣи ӯу ӢИ ӮУ");
        s.features = vec![
            feature(
                "Case",
                &[
                    ("Nominative", "nominative"),
                    ("Accusative", "accusative"),
                    ("Genitive", "genitive"),
                    ("Illative", "illative"),
                    ("Locative", "locative"),
                    ("Comitative", "comitative"),
                    ("Abessive", "abessive"),
                ],
            ),
            feature("Number", &[("Singular", "singular"), ("Plural", "plural")]),
        ];
        s.pos = vec![pos(
            "noun",
            "noun",
            vec![
                group("", "!essive !partitive !possessive", "Case Number"),
                group("Essive", "essive", ""),
                group("Partitive", "partitive", ""),
            ],
        )
        .citation("NominativeSingular GenitiveSingular")
        .head_last()];
        s
    }

    /// The illative row comes untagged; the possessive rows are unfilled
    /// template slots.
    fn prepare(&self, e: &mut Entry) {
        e.forms.retain(|f| !f.form.contains("{{{") && !f.form.contains('?'));
        for f in e.forms.iter_mut().filter(|f| f.source.is_some()) {
            if let Some(t) = f.tags.iter_mut().find(|t| *t == "error-unrecognized-form") {
                *t = "illative".into();
            }
        }
    }

    fn samples(&self) -> Vec<&'static str> {
        vec!["кӣлл", "мырр", "по̄ррк", "шырпэ"]
    }

    fn readme(&self) -> String {
        r#"## Example

```rust
use kildin_sami::*;
println!("{}", noun("кӣлл", Case::Locative, Number::Singular)); // кӣлэсьт
println!("{}", noun("мырр", Case::Comitative, Number::Plural)); // мырэгуэйм
println!("{}", noun_essive("мырр")); // мыррэнҍ
```

Nouns have seven cases in both numbers, plus the essive and the partitive.
Input matching ignores the long-vowel macron. Wiktionary has no Kildin
Sami conjugation tables.
"#
        .into()
    }
}

pub fn language() -> Box<dyn Language> {
    Box::new(KildinSami)
}
