use morphe_train::spec::*;

pub struct LuleSami;

impl Language for LuleSami {
    fn spec(&self) -> Spec {
        let mut s = Spec::new("lule_sami", "LuleSami", "smj");
        // Wiktionary marks some long consonants (alˈle) and long vowels
        // (bå̄htin) for learners; the standard spelling leaves them out.
        s.marks = vec!['\u{2c8}', '\u{304}'];
        s.unmark = unmark("āa ēe īi ōo ūu");
        // Norway writes æ and ń where Sweden writes ä and ŋ.
        s.fold = vec![('æ', 'ä'), ('Æ', 'Ä'), ('ń', 'ŋ'), ('Ń', 'Ŋ')];
        s.features = vec![
            feature(
                "Case",
                &[
                    ("Nominative", "nominative"),
                    ("Genitive", "genitive"),
                    ("Accusative", "accusative"),
                    ("Illative", "illative"),
                    ("Inessive", "inessive"),
                    ("Elative", "elative"),
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
                    ("Optative", "optative"),
                    ("Conditional", "conditional"),
                    ("Potential", "potential"),
                ],
            ),
            feature(
                "ConnegativeMood",
                &[
                    ("Indicative", "indicative"),
                    ("Imperative", "imperative"),
                    ("Optative", "optative"),
                    ("Conditional", "conditional"),
                    ("Potential", "potential"),
                ],
            ),
            feature("Tense", &[("Present", "present"), ("Past", "past")]),
        ];
        let nominal = || {
            vec![
                group("", "!possessive !essive !attributive", "Case Number[Singular,Plural]"),
                group("Essive", "essive", ""),
                group("Possessive", "possessive", "Person Number"),
            ]
        };
        let mut adj = nominal();
        adj.push(group("Attributive", "attributive", ""));
        s.pos = vec![
            pos("noun", "noun", nominal()).citation("NominativeSingular GenitiveSingular").head_last(),
            pos("adj", "adj", adj).head_last(),
            pos(
                "verb",
                "verb",
                vec![
                    group("Infinitive", "infinitive", ""),
                    group("", "!connegative", "Mood Person Number"),
                    group("Connegative", "connegative", "ConnegativeMood"),
                    group("Participle", "participle", "Tense"),
                ],
            )
            .citation("Infinitive PresentFirstSingular PastFirstSingular")
            .head_last(),
        ];
        s
    }

    fn samples(&self) -> Vec<&'static str> {
        vec!["suola", "valle", "giella", "dahkat", "boahtet", "mannat", "buorre", "skirpa", "vuhtsat"]
    }

    fn readme(&self) -> String {
        r#"## Example

```rust
use lule_sami::*;
println!("{}", noun("suola", Case::Genitive, Number::Singular));
println!("{}", noun("valle", Case::Illative, Number::Singular)); // valláj
println!("{}", noun_possessive("suola", Person::First, Number::Singular)); // suollagim
println!("{}", verb("dahkat", Mood::Potential, Person::First, Number::Singular)); // dagátjav
println!("{}", verb_connegative("dahkat", ConnegativeMood::Indicative)); // dagá
```

Nouns and adjectives have eight cases in the singular and plural, the
essive, and the possessed forms for nine possessors; adjectives also have
their attributive form. Verbs have the indicative present and past, the
imperative, the optative, the conditional and the potential for nine
persons (singular, dual and plural), the connegatives, the infinitive and
the participles. Input matching ignores the length marks (ˈ, ā) and reads the
Norwegian æ and ń as ä and ŋ.
"#
        .into()
    }
}

pub fn language() -> Box<dyn Language> {
    Box::new(LuleSami)
}
