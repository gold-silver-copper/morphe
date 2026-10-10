use morphe_train::spec::*;

pub struct NorthernSami;

impl Language for NorthernSami {
    fn spec(&self) -> Spec {
        let mut s = Spec::new("northern_sami", "NorthernSami", "se");
        // Wiktionary's tables mark vowel length and quality (ō, ẹ, ạ) for
        // learners; standard spelling leaves them out.
        s.unmark = unmark("āa ēe īi ōo ūu ạa ẹe ịi ọo ụu");
        s.marks = vec!['\u{304}', '\u{323}'];
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
            feature("ActionCase", &[("Inessive", "inessive"), ("Elative", "elative"), ("Comitative", "comitative")]),
        ];
        let nominal = || {
            vec![
                group("", "!possessive !essive", "Case Number[Singular,Plural]"),
                group("Essive", "essive", ""),
                group("Possessive", "possessive", "Person Number"),
            ]
        };
        s.pos = vec![
            pos("noun", "noun", nominal()).citation("NominativeSingular GenitiveSingular").head_last(),
            pos("adj", "adj", nominal()).head_last(),
            pos(
                "verb",
                "verb",
                vec![
                    group("Infinitive", "infinitive", ""),
                    group("VerbalNoun", "noun-from-verb !inessive !elative !comitative", ""),
                    group("Action", "noun-from-verb", "ActionCase"),
                    group("Participle", "participle !agent", "Tense"),
                    group("AgentParticiple", "agent participle", ""),
                    group("Abessive", "abessive", ""),
                    group("", "!connegative", "Mood Person Number"),
                    group("Connegative", "connegative", "Mood"),
                ],
            )
            .citation("Infinitive PresentFirstSingular PastFirstSingular")
            .head_last(),
            pos("pron", "pron", nominal()),
        ];
        s
    }

    fn samples(&self) -> Vec<&'static str> {
        vec!["giella", "viessu", "boahtit", "mannat", "čáppat", "bulvá", "skihpat"]
    }

    fn readme(&self) -> String {
        r#"## Example

```rust
use northern_sami::*;
println!("{}", noun("giella", Case::Genitive, Number::Singular)); // giela
println!("{}", noun("viessu", Case::Locative, Number::Plural)); // viesuin
println!("{}", noun_possessive("viessu", Person::First, Number::Singular)); // viessun
println!("{}", verb("boahtit", Mood::Past, Person::Third, Number::Singular)); // bođii
println!("{}", verb("boahtit", Mood::Present, Person::First, Number::Dual)); // boahte
println!("{}", verb_connegative("boahtit", Mood::Present)); // boađe
```

Nouns have six cases in the singular and plural, the essive, and the
possessed forms for nine possessors (singular, dual and plural). Verbs have
the indicative present and past, the imperative, the conditional and the
potential for nine persons, the connegatives, the infinitive, the action
forms and the participles. Wiktionary's tables mark vowel length and
quality (ō, ẹ, ạ) the standard spelling leaves out: input matching ignores
them, and `marks(false)` removes them from output.
"#
        .into()
    }
}

pub fn language() -> Box<dyn Language> {
    Box::new(NorthernSami)
}
