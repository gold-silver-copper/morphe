use morphe_train::spec::*;

pub struct Basque;

impl Language for Basque {
    fn spec(&self) -> Spec {
        let mut s = Spec::new("basque", "Basque", "eu");
        s.features = vec![
            feature(
                "Case",
                &[
                    ("Absolutive", "absolutive"),
                    ("Ergative", "ergative"),
                    ("Dative", "dative"),
                    ("Genitive", "genitive"),
                    ("Comitative", "comitative"),
                    ("Causative", "causative"),
                    ("Benefactive", "benefactive"),
                    ("Instrumental", "instrumental"),
                    ("Inessive", "inessive"),
                    ("Locative", "locative"),
                    ("Allative", "allative"),
                    ("Terminative", "terminative"),
                    ("Directive", "directive"),
                    ("Destinative", "destinative"),
                    ("Ablative", "ablative"),
                    ("Partitive", "partitive"),
                    ("Prolative", "prolative"),
                ],
            ),
            // The table's four columns. The parser tags the definite
            // singular and plural columns `indefinite` too.
            feature(
                "Number",
                &[
                    ("Indefinite", "indefinite !singular !plural"),
                    ("Singular", "singular"),
                    ("Plural", "plural !proximal"),
                    ("Proximal", "proximal"),
                ],
            ),
            feature("Participle", &[("Imperfect", "imperfect"), ("Future", "future")]),
        ];
        let nominal = || vec![group("", "", "Case Number")];
        s.pos = vec![
            pos("noun", "noun", nominal()).citation("AbsolutiveIndefinite AbsolutiveSingular").head_last(),
            pos("adj", "adj", nominal()).head_last(),
            pos(
                "verb",
                "verb",
                vec![
                    group("Participle", "participle", "Participle"),
                    group("VerbalNoun", "noun-from-verb", ""),
                    group("Stem", "short-form", ""),
                ],
            )
            .head_last(),
            pos("pron", "pron", nominal()),
            pos("num", "num", nominal()),
        ];
        s
    }

    fn samples(&self) -> Vec<&'static str> {
        vec!["etxe", "mendi", "gizon", "robot", "on", "etorri", "hartu", "zirtola"]
    }

    fn readme(&self) -> String {
        r#"## Example

```rust
use basque::*;
assert_eq!(noun("etxe", Case::Inessive, Number::Singular), "etxean");
println!("{}", noun("mendi", Case::Ablative, Number::Plural)); // mendietatik
println!("{}", noun("gizon", Case::Ergative, Number::Singular)); // gizonak
println!("{}", noun("robot", Case::Inessive, Number::Indefinite)); // robotengan
println!("{}", verb_participle("hartu", Participle::Future)); // hartuko
println!("{}", verb_verbal_noun("hartu")); // hartze
```

Nouns, adjectives, pronouns and numerals have seventeen cases in four
forms: indefinite, definite singular, definite plural and proximal plural
(`Number::Proximal`, "these"). Animate nouns take -gan in the local cases.
Verbs are looked up by their perfect participle and have the imperfect and
future participles, the verbal noun and the short stem; Basque finite
verbs are mostly periphrastic, built on these with an auxiliary.
"#
        .into()
    }
}

pub fn language() -> Box<dyn Language> {
    Box::new(Basque)
}
