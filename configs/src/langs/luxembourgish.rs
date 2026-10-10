use morphe_train::spec::*;

pub struct Luxembourgish;

fn has(tags: &[String], t: &str) -> bool {
    tags.iter().any(|x| x == t)
}

impl Language for Luxembourgish {
    fn spec(&self) -> Spec {
        let mut s = Spec::new("luxembourgish", "Luxembourgish", "lb");
        // Compound tenses are auxiliary + participle.
        s.drop_tags.extend(tags("dated perfect future"));
        s.features = vec![
            feature("Gender", &[("Masculine", "masculine"), ("Feminine", "feminine"), ("Neuter", "neuter")]),
            feature(
                "GenderNumber",
                &[
                    ("Masculine", "masculine !plural"),
                    ("Feminine", "feminine !plural"),
                    ("Neuter", "neuter !plural"),
                    ("Plural", "plural"),
                ],
            ),
            feature(
                "PersonNumber",
                &[
                    ("FirstSingular", "first-person singular"),
                    ("SecondSingular", "second-person singular"),
                    ("ThirdSingular", "third-person singular"),
                    ("FirstPlural", "first-person plural"),
                    ("SecondPlural", "second-person plural"),
                    ("ThirdPlural", "third-person plural"),
                ],
            ),
            feature("Number", &[("Singular", "singular"), ("Plural", "plural")]),
        ];
        s.pos = vec![
            pos("noun", "noun", vec![group("Plural", "plural", ""), group("Diminutive", "diminutive", "")])
                .inherent("Gender")
                .citation("Lemma Plural")
                .head_last(),
            pos(
                "adj",
                "adj",
                vec![
                    // de schéine Mann
                    group("", "nominative with-determiner", "GenderNumber"),
                    // schéines Wieder
                    group("Strong", "nominative without-determiner", "GenderNumber"),
                    group("Dative", "dative !with-head", "GenderNumber"),
                    group("Comparative", "comparative", ""),
                    group("Superlative", "superlative", ""),
                ],
            )
            .citation("Lemma Masculine Neuter"),
            pos(
                "verb",
                "verb",
                vec![
                    group("Present", "indicative present", "PersonNumber"),
                    group("Past", "indicative past", "PersonNumber"),
                    group("Conditional", "conditional !present", "PersonNumber"),
                    group("Imperative", "imperative second-person", "Number"),
                    group("Participle", "participle", ""),
                    // hunn or sinn, for the perfect.
                    group("Auxiliary", "auxiliary", ""),
                ],
            )
            .citation("Lemma Participle")
            .head_first(),
        ];
        s
    }

    fn retag(&self, pos: &PosSpec, _form: &str, tags: &mut Vec<String>) {
        // The headword's "groussen, grousst" are the attributive forms.
        if pos.name == "adj" && tags.len() == 1 && (has(tags, "masculine") || has(tags, "neuter")) {
            tags.extend(["nominative", "with-determiner", "singular"].map(String::from));
        }
    }

    fn samples(&self) -> Vec<&'static str> {
        vec!["Buch", "Hond", "Haus", "grouss", "schéin", "hidden", "goen", "Flimpchen", "knaschteg"]
    }

    fn readme(&self) -> String {
        r#"## Example

```rust
use luxembourgish::*;
println!("{}", noun_plural("Hond")); // Hënn
println!("{:?}", noun_gender("Buch"));
assert_eq!(adj("grouss", GenderNumber::Masculine), "groussen");
assert_eq!(adj_strong("grouss", GenderNumber::Neuter), "grousses");
println!("{}", verb_present("hidden", PersonNumber::SecondSingular)); // hitts
println!("{}", verb_auxiliary("goen")); // sinn
```

Compound tenses are left to the caller: `verb_auxiliary` plus
`verb_participle`. Comparatives and superlatives are given as Wiktionary
writes them ("méi grouss", "am gréissten").
"#
        .into()
    }
}

pub fn language() -> Box<dyn Language> {
    Box::new(Luxembourgish)
}
