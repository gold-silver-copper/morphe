use morphe_train::kaikki::Entry;
use morphe_train::spec::*;

pub struct OttomanTurkish;

/// The possessed table's layout: per possessor, per case, one thing owned
/// then many.
const POSSESSORS: [&str; 6] = [
    "first-person singular",
    "second-person singular",
    "third-person singular",
    "first-person plural",
    "second-person plural",
    "third-person plural",
];
const CASES: [&str; 6] = ["nominative", "accusative", "dative", "locative", "ablative", "genitive"];

impl Language for OttomanTurkish {
    fn spec(&self) -> Spec {
        let mut s = Spec::new("ottoman_turkish", "OttomanTurkish", "ota");
        s.features = vec![
            feature(
                "Case",
                &[
                    ("Nominative", "nominative"),
                    ("Accusative", "accusative"),
                    ("Dative", "dative"),
                    ("Locative", "locative"),
                    ("Ablative", "ablative"),
                    ("Genitive", "genitive"),
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

    /// The possessed table's number tags are scrambled (the possessor's
    /// and the owned thing's numbers mixed up), but its 72 cells always
    /// come in the same order. Read it by position.
    fn prepare(&self, e: &mut Entry) {
        let mut table = 0usize;
        let mut cells: Vec<usize> = Vec::new();
        for (i, f) in e.forms.iter().enumerate().filter(|(_, f)| f.source.is_some()) {
            if f.tags.iter().any(|t| t == "table-tags") {
                table += 1;
            } else if table == 2 && !f.tags.iter().any(|t| t == "inflection-template") {
                cells.push(i);
            }
        }
        let ok = cells.len() == 72;
        for (k, i) in cells.into_iter().enumerate() {
            let Some(f) = e.forms.get_mut(i) else { continue };
            let (Some(p), Some(c)) = (POSSESSORS.get(k / 12), CASES.get(k % 12 / 2)) else { continue };
            let owned = if k % 2 == 0 { "possessed-single" } else { "possessed-many" };
            f.tags = if ok { tags(&format!("possessive {p} {c} {owned}")) } else { tags("table-tags") };
        }
    }

    fn samples(&self) -> Vec<&'static str> {
        vec!["امام", "كتاب", "او", "كوز", "شرپل"]
    }

    fn readme(&self) -> String {
        r#"## Example

```rust
use ottoman_turkish::*;
println!("{}", noun("امام", Case::Dative, Number::Singular)); // امامه
println!("{}", noun("امام", Case::Nominative, Number::Plural)); // ائمه
println!("{}", noun_possessive("امام", Possessor::FirstSingular, Possessed::One, Case::Ablative)); // اماممدن
```

Nouns have six cases in both numbers (with the Arabic broken plurals
Wiktionary gives) and the possessed forms for six possessors, with one
thing or many owned, in every case. Wiktionary has no Ottoman Turkish
conjugation tables.
"#
        .into()
    }
}

pub fn language() -> Box<dyn Language> {
    Box::new(OttomanTurkish)
}
