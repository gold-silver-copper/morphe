use morphe_train::kaikki::Entry;
use morphe_train::spec::*;

pub struct Ossetian;

impl Language for Ossetian {
    fn spec(&self) -> Spec {
        let mut s = Spec::new("ossetian", "Ossetian", "os");
        s.marks = vec!['\u{301}'];
        // The Latin æ is often typed for the Cyrillic ӕ.
        s.fold = vec![('æ', 'ӕ'), ('Æ', 'Ӕ')];
        s.features = vec![
            feature(
                "Case",
                &[
                    ("Nominative", "nominative"),
                    ("Genitive", "genitive"),
                    ("Dative", "dative"),
                    ("Allative", "allative"),
                    ("Ablative", "ablative"),
                    ("Inessive", "inessive"),
                    ("Adessive", "adessive"),
                    ("Equative", "equative"),
                    ("Comitative", "comitative"),
                ],
            ),
            feature("Number", &[("Singular", "singular"), ("Plural", "plural")]),
        ];
        s.pos = vec![pos("noun", "noun", vec![group("", "", "Case Number")])
            .citation("NominativeSingular NominativePlural")
            .head_last()];
        s
    }

    /// The table's number headers are swapped: the singular column is
    /// tagged plural and the plural column has no number. A few tables
    /// lose the plural's т (тӕрсыӕ for тӕрсытӕ); those are left out.
    fn prepare(&self, e: &mut Entry) {
        if e.pos != "noun" {
            return;
        }
        if e.forms.iter().any(|f| f.source.is_some() && f.form.contains("ыӕ")) {
            e.forms.retain(|f| f.source.is_none());
        }
        for f in e.forms.iter_mut().filter(|f| f.source.is_some()) {
            if f.tags.iter().any(|t| t == "table-tags" || t == "inflection-template") {
                continue;
            }
            match f.tags.iter().position(|t| t == "plural") {
                Some(i) => {
                    if let Some(t) = f.tags.get_mut(i) {
                        *t = "singular".into();
                    }
                }
                None => f.tags.push("plural".into()),
            }
        }
    }

    /// Some cells carry a romanization: "къуыпп (k’°ypp)".
    fn split_form(&self, raw: &str) -> Vec<String> {
        vec![raw.split(" (").next().unwrap_or(raw).trim().to_string()]
    }

    fn samples(&self) -> Vec<&'static str> {
        vec!["сӕр", "хур", "адӕймаг", "бӕстӕ", "къӕвда", "дур"]
    }

    fn readme(&self) -> String {
        r#"## Example

```rust
use ossetian::*;
println!("{}", noun("сӕр", Case::Dative, Number::Singular)); // сӕрӕн
println!("{}", noun("адӕймаг", Case::Nominative, Number::Plural)); // адӕймӕгтӕ
println!("{}", noun("хур", Case::Comitative, Number::Plural)); // хуртимӕ
```

Nouns have nine cases in both numbers. Wiktionary has declension tables
for a few dozen Ossetian nouns only (the declension is regular), and
conjugation tables for too few verbs to learn from.
"#
        .into()
    }
}

pub fn language() -> Box<dyn Language> {
    Box::new(Ossetian)
}
