use morphe_train::kaikki::Entry;
use morphe_train::spec::*;

pub struct Chechen;

impl Language for Chechen {
    fn spec(&self) -> Spec {
        let mut s = Spec::new("chechen", "Chechen", "ce");
        // Wiktionary marks long vowels (уьно̄) for learners; the standard
        // spelling leaves them out.
        s.marks = vec!['\u{304}'];
        s.unmark = unmark("ӯу ӮУ");
        // The palochka is written with I, 1 or ӏ.
        s.fold = vec![('I', 'ӏ'), ('1', 'ӏ'), ('Ӏ', 'ӏ')];
        s.features = vec![
            feature(
                "Case",
                &[
                    ("Nominative", "nominative"),
                    ("Genitive", "genitive"),
                    ("Dative", "dative"),
                    ("Ergative", "ergative"),
                    ("Allative", "allative"),
                    ("Instrumental", "instrumental"),
                    ("Lative", "lative"),
                    ("Comparative", "comparative-case"),
                ],
            ),
            feature("Number", &[("Singular", "singular"), ("Plural", "plural")]),
            // Gender class in the singular and the plural (в, й, б, д).
            feature(
                "Class",
                &[
                    ("JJ", "class-jj"),
                    ("DD", "class-dd"),
                    ("BD", "class-bd"),
                    ("VB", "class-vb"),
                    ("BB", "class-bb"),
                    ("JB", "class-jb"),
                    ("J", "class-j"),
                    ("D", "class-d"),
                    ("B", "class-b"),
                    ("V", "class-v"),
                ],
            ),
        ];
        s.pos = vec![
            pos("noun", "noun", vec![group("", "", "Case Number")])
                .inherent("Class")
                .citation("NominativeSingular GenitiveSingular")
                .head_last(),
            pos("num", "num", vec![group("", "!plural", "Case")]),
        ];
        s
    }

    /// The headword names the noun's class ("у class dd").
    fn prepare(&self, e: &mut Entry) {
        for f in e.forms.iter_mut().filter(|f| f.tags.iter().any(|t| t == "canonical")) {
            if let Some((_, class)) = f.form.split_once(" class ") {
                let class = class.trim();
                if !class.is_empty() && class.chars().all(|c| "jdbv".contains(c)) {
                    f.tags.push(format!("class-{class}"));
                }
            }
        }
        // Numerals name the comparative without "-case".
        for f in e.forms.iter_mut().filter(|f| f.source.is_some()) {
            if let Some(t) = f.tags.iter_mut().find(|t| *t == "comparative") {
                *t = "comparative-case".into();
            }
        }
    }

    fn samples(&self) -> Vec<&'static str> {
        vec!["у", "кор", "стаг", "нана", "цхьаъ", "шырпал"]
    }

    fn readme(&self) -> String {
        r#"## Example

```rust
use chechen::*;
println!("{}", noun("у", Case::Genitive, Number::Singular)); // уьнан
println!("{}", noun("у", Case::Nominative, Number::Plural)); // аннаш
println!("{:?}", noun_class("у")); // DD
```

Nouns have eight cases in both numbers, and numerals the eight cases;
nouns also have their
gender class in the singular and plural (JJ: й/й, BD: б/д, VB: в/б…).
Wiktionary marks long vowels (уьно̄) the standard spelling leaves out:
input matching ignores them, and `marks(false)` removes them from output.
Input may write the palochka as ӏ, I or 1.
"#
        .into()
    }
}

pub fn language() -> Box<dyn Language> {
    Box::new(Chechen)
}
