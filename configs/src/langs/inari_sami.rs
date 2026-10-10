use morphe_train::spec::*;

pub struct InariSami;

impl Language for InariSami {
    fn spec(&self) -> Spec {
        let mut s = Spec::new("inari_sami", "InariSami", "smn");
        // Wiktionary marks consonant length and gradation (maṇe, jieˈm) for
        // learners; the standard spelling leaves the marks out.
        s.marks = vec!['\u{323}', '\u{2c8}'];
        s.unmark = unmark("ṇn ḍd ṭt ḷl ṃm ṛr ẹe ạa ịi ọo ụu");
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
            feature("Number", &[("Singular", "singular"), ("Dual", "dual"), ("Plural", "plural")]),
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
                    group("", "!essive !partitive !possessive", "Case Number[Singular,Plural]"),
                    group("Essive", "essive !plural", ""),
                    group("Partitive", "partitive !plural", ""),
                ],
            )
            .citation("NominativeSingular GenitiveSingular")
            .head_last(),
            pos(
                "verb",
                "verb",
                vec![
                    group("Infinitive", "infinitive", ""),
                    group("", "!connegative", "Mood Person Number"),
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
        vec!["euro", "mane", "kyele", "jaannâđ", "lávluđ", "puáttiđ", "skirpâ", "vuhtsâđ"]
    }

    fn readme(&self) -> String {
        r#"## Example

```rust
use inari_sami::*;
println!("{}", noun("euro", Case::Locative, Number::Singular)); // eurost
println!("{}", noun("euro", Case::Comitative, Number::Plural)); // euroiguin
println!("{}", verb("jaannâđ", Mood::Past, Person::Third, Number::Singular)); // jaanâi
println!("{}", verb("jaannâđ", Mood::Potential, Person::First, Number::Plural)); // janâžep
println!("{}", verb_connegative("jaannâđ", Mood::Present)); // jaanâ
```

Nouns have seven cases in the singular and plural, plus the essive and
the partitive. Verbs have the indicative present and past, the
imperative, the conditional and the potential for nine persons (singular,
dual and plural), the connegatives, the infinitive and the participles.
Wiktionary marks consonant length and gradation (maṇe, jieˈm) the
standard spelling leaves out: input matching ignores the marks, and
`marks(false)` removes them from output.
"#
        .into()
    }
}

pub fn language() -> Box<dyn Language> {
    Box::new(InariSami)
}
