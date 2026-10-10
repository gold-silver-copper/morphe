use morphe_train::kaikki::{Entry, Form};
use morphe_train::spec::*;

pub struct Kashubian;

fn has(tags: &[String], t: &str) -> bool {
    tags.iter().any(|x| x == t)
}

const ADJ_CASES: &str = "Case[Nominative,Genitive,Dative,Accusative,Instrumental,Locative]";

impl Language for Kashubian {
    fn spec(&self) -> Spec {
        let mut s = Spec::new("kashubian", "Kashubian", "csb");
        s.unmark = unmark("ëe òo ée óo ôo łl ùu ãa ąa żz ńn");
        s.soft_tags.extend(tags("poetic"));
        s.features = vec![
            feature(
                "Case",
                &[
                    ("Nominative", "nominative"),
                    ("Genitive", "genitive"),
                    ("Dative", "dative"),
                    ("Accusative", "accusative"),
                    ("Instrumental", "instrumental"),
                    ("Locative", "locative"),
                    ("Vocative", "vocative !nominative"),
                ],
            ),
            feature("Number", &[("Singular", "singular"), ("Plural", "plural")]),
            feature("Gender", &[("Masculine", "masculine"), ("Feminine", "feminine"), ("Neuter", "neuter")]),
            feature("Animacy", &[("Personal", "person"), ("Animal", "animal-not-person"), ("Inanimate", "inanimate")]),
            feature(
                "SingularGender",
                &[
                    ("Masculine", "masculine !animate"),
                    ("MasculineAnimate", "animate"),
                    ("Neuter", "neuter"),
                    ("Feminine", "feminine"),
                ],
            ),
            feature("Virility", &[("Virile", "virile"), ("Nonvirile", "nonvirile")]),
            feature("Degree", &[("Comparative", "comparative"), ("Superlative", "superlative")]),
        ];
        s.pos = vec![
            pos(
                "noun",
                "noun",
                vec![
                    group("", "", "Case Number"),
                    group("Diminutive", "diminutive", ""),
                    group("Augmentative", "augmentative", ""),
                ],
            )
            .inherent("Gender Animacy")
            .citation("NominativeSingular GenitiveSingular")
            .head_last(),
            pos(
                "adj",
                "adj",
                vec![
                    group("", "singular", &format!("{ADJ_CASES} SingularGender")),
                    group("Plural", "plural", &format!("{ADJ_CASES} Virility")),
                    // The headword line's comparative and superlative.
                    group("Simple", "", "Degree"),
                ],
            )
            .citation("NominativeMasculine NominativeFeminine NominativeNeuter"),
        ];
        s
    }

    fn prepare(&self, e: &mut Entry) {
        if !e.forms.iter().any(|f| has(&f.tags, "canonical")) {
            e.forms.push(Form { form: e.word.clone(), tags: vec!["canonical".into()], source: None });
        }
        for f in &mut e.forms {
            // The adjective's nonvirile plural column has no header.
            if e.pos == "adj" && f.source.is_some() && has(&f.tags, "error-unrecognized-form") {
                f.tags.retain(|t| t != "error-unrecognized-form");
                f.tags.push("nonvirile".into());
            }
        }
    }

    fn samples(&self) -> Vec<&'static str> {
        vec!["abstinent", "pies", "krowa", "chłodny", "czôrny", "snorôk", "flompny"]
    }

    fn readme(&self) -> String {
        r#"## Example

```rust
use kashubian::*;
println!("{}", noun("abstinent", Case::Dative, Number::Singular)); // abstinentowi
println!("{}", noun_diminutive("pies")); // piesk
println!("{}", adj("chłodny", Case::Genitive, SingularGender::Masculine)); // chłodnégò
println!("{}", adj_simple("chłodny", Degree::Comparative)); // chłodniészi
```

Diacritics are optional on input. Verbs are left out: Wiktionary has
conjugation tables for only a handful.
"#
        .into()
    }
}

pub fn language() -> Box<dyn Language> {
    Box::new(Kashubian)
}
