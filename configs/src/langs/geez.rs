use morphe_train::kaikki::Entry;
use morphe_train::spec::*;

pub struct Geez;

impl Language for Geez {
    fn spec(&self) -> Spec {
        let mut s = Spec::new("geez", "Geez", "gez");
        s.features = vec![feature("Number", &[("Singular", "singular"), ("Plural", "plural")])];
        s.pos = vec![pos(
            "noun",
            "noun",
            vec![group("Plural", "plural !accusative", ""), group("Accusative", "accusative", "Number")],
        )
        .citation("Lemma Plural")
        .head_last()];
        s
    }

    /// The declension table's first cells (tagged "absolute") are the
    /// accusative singular and plural (-ä); its possessed forms are too
    /// incomplete to place and are left out.
    fn prepare(&self, e: &mut Entry) {
        e.forms.retain(|f| !f.tags.iter().any(|t| t == "possessed-form"));
        for f in e.forms.iter_mut().filter(|f| f.source.is_some()) {
            if f.tags.iter().any(|t| t == "absolute") {
                let number = if f.tags.iter().any(|t| t == "plural") { "plural" } else { "singular" };
                f.tags = tags(&format!("accusative {number}"));
            }
        }
    }

    fn samples(&self) -> Vec<&'static str> {
        vec!["ገጽ", "እኅት", "ሰማይ", "ብእሲ"]
    }

    fn readme(&self) -> String {
        r#"## Example

```rust
use geez::*;
println!("{}", noun_plural("ገጽ")); // ገጻት
println!("{}", noun_accusative("ገጽ", Number::Singular)); // ገጸ
```

Nouns have their plural (often a broken plural) and the accusative in both
numbers. Wiktionary has no Ge'ez verb tables.
"#
        .into()
    }
}

pub fn language() -> Box<dyn Language> {
    Box::new(Geez)
}
