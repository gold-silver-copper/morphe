use morphe_train::kaikki::Entry;
use morphe_train::spec::*;

pub struct Uzbek;

const CASES: [&str; 7] = ["nominative", "genitive", "dative", "accusative", "locative", "ablative", "equative"];
const POSSESSORS: [&str; 5] = ["poss-first-singular", "poss-second-singular", "poss-third", "poss-first-plural", "poss-second-plural"];

impl Language for Uzbek {
    fn spec(&self) -> Spec {
        let mut s = Spec::new("uzbek", "Uzbek", "uz");
        // The ʻ of oʻ and gʻ is typed as any apostrophe.
        s.fold = vec![('\'', 'ʻ'), ('’', 'ʻ'), ('‘', 'ʻ'), ('`', 'ʻ')];
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
                    ("Equative", "equative"),
                ],
            ),
            feature("Number", &[("Singular", "singular"), ("Plural", "plural")]),
            feature(
                "Possessor",
                &[
                    ("FirstSingular", "poss-first-singular"),
                    ("SecondSingular", "poss-second-singular"),
                    ("Third", "poss-third"),
                    ("FirstPlural", "poss-first-plural"),
                    ("SecondPlural", "poss-second-plural"),
                ],
            ),
        ];
        s.pos = vec![pos(
            "noun",
            "noun",
            vec![group("", "!possessive", "Case Number"), group("Possessive", "possessive", "Possessor Case Number")],
        )
        .citation("NominativeSingular NominativePlural")
        .head_last()];
        s
    }

    /// The declension's equative (-dek) comes untagged, and the possessive
    /// table's person and number tags are muddled. Both tables have a fixed
    /// layout: cases in rows, singular and plural in turn, and in the
    /// possessive table one block of rows per possessor.
    fn prepare(&self, e: &mut Entry) {
        let mut table = 0;
        let mut k = 0usize;
        for f in e.forms.iter_mut().filter(|f| f.source.as_deref() == Some("declension")) {
            if f.tags.iter().any(|t| t == "table-tags") {
                table += 1;
                k = 0;
                continue;
            }
            if f.tags.iter().any(|t| t == "inflection-template") {
                continue;
            }
            let num = if k % 2 == 0 { "singular" } else { "plural" };
            let case = CASES.get((k % 14) / 2).copied().unwrap_or("");
            f.tags = match table {
                1 => tags(&format!("{case} {num}")),
                2 => tags(&format!("possessive {} {case} {num}", POSSESSORS.get(k / 14).copied().unwrap_or("table-tags"))),
                _ => tags("table-tags"),
            };
            k += 1;
        }
    }

    fn samples(&self) -> Vec<&'static str> {
        vec!["kitob", "uy", "bola", "oʻqituvchi", "tog'", "shirpak"]
    }

    fn readme(&self) -> String {
        r#"## Example

```rust
use uzbek::*;
assert_eq!(noun("kitob", Case::Dative, Number::Plural), "kitoblarga");
println!("{}", noun("uy", Case::Locative, Number::Singular)); // uyda
println!("{}", noun("bola", Case::Equative, Number::Singular)); // boladek
println!("{}", noun_possessive("kitob", Possessor::FirstSingular, Case::Accusative, Number::Singular)); // kitobimni
```

Nouns have seven cases, including the equative (-dek, "like"), in both
numbers, and the possessed forms for five possessors (the third person is
the same for one owner and many). Input matching takes any apostrophe for
the ʻ of oʻ and gʻ. Wiktionary's Uzbek verb tables don't label person, so
verbs are left out.
"#
        .into()
    }
}

pub fn language() -> Box<dyn Language> {
    Box::new(Uzbek)
}
