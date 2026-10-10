use morphe_train::kaikki::Entry;
use morphe_train::spec::*;

pub struct Karelian;

impl Language for Karelian {
    fn spec(&self) -> Spec {
        let mut s = Spec::new("karelian", "Karelian", "krl");
        s.features = vec![
            feature(
                "Case",
                &[
                    ("Nominative", "nominative"),
                    ("Genitive", "genitive"),
                    ("Partitive", "partitive"),
                    ("Illative", "illative"),
                    ("Inessive", "inessive"),
                    ("Elative", "elative"),
                    ("Adessive", "adessive"),
                    ("Ablative", "ablative"),
                    ("Translative", "translative"),
                    ("Essive", "essive"),
                    ("Comitative", "comitative"),
                    ("Abessive", "abessive"),
                    ("Prolative", "prolative"),
                    ("Instructive", "instructive"),
                ],
            ),
            feature("Number", &[("Singular", "singular"), ("Plural", "plural")]),
        ];
        let nominal = || vec![group("", "", "Case Number")];
        s.pos = vec![
            pos("noun", "noun", nominal()).citation("NominativeSingular GenitiveSingular PartitiveSingular").head_last(),
            pos("adj", "adj", nominal()).head_last(),
            pos("pron", "pron", nominal()),
            pos("num", "num", nominal()),
        ];
        s
    }

    /// Entries have a North Karelian table and a Tver one; the first is
    /// read. Its rows tag the singular `plural` and leave the plural
    /// untagged. Headword lines give the genitive and partitive singular
    /// without their number.
    fn prepare(&self, e: &mut Entry) {
        let mut table = 0;
        e.forms.retain_mut(|f| {
            if f.source.is_none() {
                if f.tags.len() == 1 {
                    f.tags.push("singular".into());
                }
                return true;
            }
            if f.tags.iter().any(|t| t == "table-tags") {
                table += 1;
                return false;
            }
            if table > 1 {
                return false;
            }
            if f.tags.iter().any(|t| t.ends_with("ive")) {
                let plural = f.tags.iter().any(|t| t == "plural");
                f.tags.retain(|t| t != "plural" && t != "singular");
                f.tags.push(if plural { "singular" } else { "plural" }.into());
            }
            true
        });
    }

    fn samples(&self) -> Vec<&'static str> {
        vec!["kala", "talo", "käsi", "šuuri", "kirpakka"]
    }

    fn readme(&self) -> String {
        r#"## Example

```rust
use karelian::*;
println!("{}", noun("kala", Case::Inessive, Number::Singular)); // kalašša
println!("{}", noun("kala", Case::Partitive, Number::Plural)); // kaloja
```

North Karelian (Viena) nouns, adjectives, pronouns and numerals with
fourteen cases in both numbers. Wiktionary has no Karelian verb tables.
"#
        .into()
    }
}

pub fn language() -> Box<dyn Language> {
    Box::new(Karelian)
}
