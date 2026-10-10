use morphe_train::kaikki::{Entry, Form};
use morphe_train::spec::*;

pub struct Amharic;

impl Language for Amharic {
    fn spec(&self) -> Spec {
        let mut s = Spec::new("amharic", "Amharic", "am");
        s.features = vec![
            feature("Gender", &[("Masculine", "masculine"), ("Feminine", "feminine")]),
            feature(
                "Possessor",
                &[
                    ("FirstSingular", "first-person singular"),
                    ("FirstPlural", "first-person plural"),
                    ("SecondMasculine", "second-person masculine singular"),
                    ("SecondFeminine", "second-person feminine singular"),
                    ("SecondPlural", "second-person plural"),
                    ("SecondFormal", "second-person formal"),
                    ("ThirdMasculine", "third-person masculine singular"),
                    ("ThirdFeminine", "third-person feminine singular"),
                    ("ThirdPlural", "third-person plural"),
                    ("ThirdFormal", "third-person formal"),
                ],
            ),
        ];
        let nominal = |plural: bool| {
            let mut g = vec![
                group("Definite", "x-definite", "Gender"),
                group("DefiniteObject", "definite direct-object", "Gender"),
                group("Object", "x-accusative", ""),
                group("Possessive", "possessive", "Possessor"),
            ];
            if plural {
                g.insert(0, group("Plural", "plural !possessive", ""));
            }
            g
        };
        s.pos = vec![
            pos("noun", "noun", nominal(true)).inherent("Gender").citation("Lemma Plural").head_last(),
            pos("adj", "adj", nominal(false)).head_last(),
        ];
        s
    }

    /// The table leaves the definite (-ው, -ዋ) and the indefinite object
    /// (-ን) untagged; the gender of the indefinite form is the noun's.
    fn prepare(&self, e: &mut Entry) {
        let mut gender = None;
        for f in e.forms.iter_mut() {
            let has = |t: &str| f.tags.iter().any(|x| x == t);
            if has("indefinite") && f.source.is_some() {
                gender = ["masculine", "feminine"].into_iter().find(|g| has(g));
            }
            if has("error-unrecognized-form") {
                f.tags.retain(|t| t != "error-unrecognized-form");
                if f.form.ends_with('ን') {
                    f.tags = tags("x-accusative");
                } else {
                    f.tags.push("x-definite".into());
                }
            }
        }
        if let Some(g) = gender {
            e.forms.push(Form { form: e.word.clone(), tags: tags(&format!("canonical {g}")), source: None });
        }
    }

    fn samples(&self) -> Vec<&'static str> {
        vec!["ቤት", "ጓደኛ", "ልጅ", "አማርኛ", "ፀሓይ", "ቡላሎ"]
    }

    fn readme(&self) -> String {
        r#"## Example

```rust
use amharic::*;
println!("{}", noun_plural("ጓደኛ")); // ጓደኞች
println!("{}", noun_definite("ቤት", Gender::Masculine)); // ቤቱ
println!("{}", noun_definite_object("ቤት", Gender::Masculine)); // ቤቱን
println!("{}", noun_possessive("ቤት", Possessor::FirstSingular)); // ቤቴ
println!("{}", noun_possessive("ቤት", Possessor::SecondPlural)); // ቤታችሁ
```

Nouns have the plural, the definite (masculine -ው, feminine -ዋ/-ይቱ),
the definite and indefinite object (-ን), and the possessed forms for ten
possessors; adjectives the same without the plural. Wiktionary has no
Amharic verb tables, so verbs are left out.
"#
        .into()
    }
}

pub fn language() -> Box<dyn Language> {
    Box::new(Amharic)
}
