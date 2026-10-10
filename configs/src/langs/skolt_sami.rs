use morphe_train::spec::*;

pub struct SkoltSami;

impl Language for SkoltSami {
    fn spec(&self) -> Spec {
        let mut s = Spec::new("skolt_sami", "SkoltSami", "sms");
        // Wiktionary marks vowel quality and consonant length (ǩiẹʹssmään)
        // for learners; the standard spelling leaves the marks out.
        s.marks = vec!['\u{323}', '\u{2c8}'];
        s.unmark = unmark("ẹe ạa ịi ọo ụu");
        s.features = vec![
            feature(
                "Case",
                &[
                    ("Nominative", "nominative"),
                    ("Genitive", "genitive"),
                    ("Accusative", "accusative"),
                    ("Illative", "illative"),
                    ("Locative", "locative"),
                    ("Comitative", "comitative"),
                    ("Abessive", "abessive"),
                ],
            ),
            feature("Number", &[("Singular", "singular"), ("Plural", "plural")]),
            feature("Person", &[("First", "first-person"), ("Second", "second-person"), ("Third", "third-person")]),
            feature(
                "Mood",
                &[
                    ("Present", "indicative present"),
                    ("Past", "indicative past"),
                    ("Imperative", "imperative"),
                    ("Conditional", "conditional"),
                    ("Potential", "potential"),
                ],
            ),
            feature("Tense", &[("Present", "present"), ("Past", "past")]),
        ];
        s.pos = vec![
            pos(
                "noun",
                "noun",
                vec![
                    group("", "!essive !partitive !possessive", "Case Number"),
                    group("Essive", "essive !plural !dual", ""),
                    group("Partitive", "partitive !plural !dual", ""),
                ],
            )
            .citation("NominativeSingular GenitiveSingular")
            .head_last(),
            pos(
                "verb",
                "verb",
                vec![
                    group("Infinitive", "infinitive", ""),
                    group("", "!connegative !fourth-person", "Mood Person Number"),
                    group("Impersonal", "fourth-person", "Mood[Present,Past,Conditional,Potential]"),
                    group("Connegative", "connegative", "Mood"),
                    group("Participle", "participle", "Tense"),
                ],
            )
            .citation("Infinitive PresentFirstSingular PastFirstSingular")
            .head_last(),
        ];
        s
    }

    fn samples(&self) -> Vec<&'static str> {
        vec!["buss", "euro", "kuõskkâd", "puättad", "sõrgg", "vuõddad", "škirpp", "vuhtsâd"]
    }

    fn readme(&self) -> String {
        r#"## Example

```rust
use skolt_sami::*;
println!("{}", noun("buss", Case::Locative, Number::Singular)); // buussâst
println!("{}", noun("buss", Case::Comitative, Number::Plural)); // buussivuiʹm
println!("{}", verb("kuõskkâd", Mood::Past, Person::First, Number::Singular)); // kuõʹsǩǩem
println!("{}", verb_impersonal("kuõskkâd", Mood::Present)); // kuõskât
```

Nouns have seven cases in both numbers, plus the essive and the
partitive. Verbs have the indicative present and past, the imperative, the
conditional and the potential for six persons and the impersonal (the
"fourth person"), the connegatives, the infinitive and the participles.
The palatalization mark (ʹ) and the separator (ʼ) are part of the
spelling; Wiktionary's learner marks (ẹ, ˈ) are ignored in input, and
`marks(false)` removes them from output.
"#
        .into()
    }
}

pub fn language() -> Box<dyn Language> {
    Box::new(SkoltSami)
}
