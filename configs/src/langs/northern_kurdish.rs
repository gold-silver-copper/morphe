use morphe_train::kaikki::Entry;
use morphe_train::spec::*;

pub struct NorthernKurdish;

impl Language for NorthernKurdish {
    fn spec(&self) -> Spec {
        let mut s = Spec::new("northern_kurdish", "NorthernKurdish", "kmr");
        s.features = vec![
            feature(
                "Case",
                &[
                    ("Nominative", "nominative"),
                    ("Construct", "construct"),
                    ("Oblique", "oblique"),
                    ("Demonstrative", "demonstrative"),
                    ("Vocative", "vocative"),
                ],
            ),
            feature("Number", &[("Singular", "singular"), ("Plural", "plural")]),
            feature("Gender", &[("Masculine", "masculine"), ("Feminine", "feminine")]),
        ];
        s.pos = vec![pos(
            "noun",
            "noun",
            vec![
                group("", "definite", "Case Number"),
                group("Indefinite", "indefinite", "Case[Nominative,Construct,Oblique] Number"),
            ],
        )
        .inherent("Gender")
        .citation("NominativeSingular ObliqueSingular")
        .head_last()];
        s
    }

    /// The table gives the definite forms, then the indefinite ones under
    /// the same tags; the demonstrative row (wê askê) is untagged.
    fn prepare(&self, e: &mut Entry) {
        let mut indefinite = false;
        let mut seen_vocative = false;
        for f in e.forms.iter_mut().filter(|f| f.source.is_some()) {
            if f.tags.iter().any(|t| t == "table-tags") {
                indefinite = false;
                seen_vocative = false;
                continue;
            }
            let vocative = f.tags.iter().any(|t| t == "vocative");
            if seen_vocative && !vocative {
                indefinite = true;
            }
            seen_vocative |= vocative;
            if let Some(t) = f.tags.iter_mut().find(|t| *t == "error-unrecognized-form") {
                *t = "demonstrative".into();
            }
            f.tags.push(if indefinite { "indefinite" } else { "definite" }.into());
        }
    }

    fn samples(&self) -> Vec<&'static str> {
        vec!["ask", "av", "ajo", "bajar", "mal", "zirpel"]
    }

    fn readme(&self) -> String {
        r#"## Example

```rust
use northern_kurdish::*;
println!("{}", noun("av", Case::Oblique, Number::Singular)); // avê
println!("{}", noun("av", Case::Construct, Number::Plural)); // avên
println!("{}", noun_indefinite("av", Case::Nominative, Number::Singular)); // avek
println!("{:?}", noun_gender("av")); // Feminine
```

Nouns have the direct (nominative), construct (ezafe), oblique,
demonstrative and vocative forms in both numbers, the indefinite direct,
construct and oblique, and their gender. Wiktionary's Kurmanji
conjugation tables come out of Kaikki with their template slots unfilled,
so verbs are left out.
"#
        .into()
    }
}

pub fn language() -> Box<dyn Language> {
    Box::new(NorthernKurdish)
}
