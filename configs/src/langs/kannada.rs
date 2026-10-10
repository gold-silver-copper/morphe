use morphe_train::kaikki::Entry;
use morphe_train::spec::*;

pub struct Kannada;

impl Language for Kannada {
    fn spec(&self) -> Spec {
        let mut s = Spec::new("kannada", "Kannada", "kn");
        s.features = vec![
            feature(
                "Case",
                &[
                    ("Nominative", "nominative"),
                    ("Accusative", "accusative"),
                    ("Dative", "dative"),
                    ("Genitive", "genitive"),
                    ("Instrumental", "instrumental"),
                ],
            ),
            feature("Number", &[("Singular", "singular"), ("Plural", "plural")]),
            feature(
                "Person",
                &[
                    ("FirstSingular", "first-person singular"),
                    ("SecondSingular", "second-person singular"),
                    ("ThirdMasculine", "third-person masculine singular !feminine"),
                    ("ThirdFeminine", "third-person feminine singular !masculine"),
                    ("ThirdNeuter", "third-person neuter singular"),
                    ("FirstPlural", "first-person plural"),
                    ("SecondPlural", "second-person plural"),
                    ("ThirdPlural", "third-person masculine feminine plural"),
                    ("ThirdNeuterPlural", "third-person neuter plural"),
                ],
            ),
            feature(
                "Tense",
                &[
                    ("Present", "present"),
                    ("Past", "past"),
                    ("Future", "future"),
                    ("Negative", "negative"),
                    ("Contingent", "contingent"),
                ],
            ),
            feature("Adverbial", &[("Present", "present"), ("Past", "past"), ("Negative", "negative")]),
            feature("Adjectival", &[("NonPast", "non-past"), ("Past", "past"), ("Negative", "negative")]),
        ];
        s.pos = vec![
            pos("noun", "noun", vec![group("", "", "Case Number")])
                .citation("NominativeSingular NominativePlural")
                .head_last(),
            pos(
                "verb",
                "verb",
                vec![
                    group("Infinitive", "infinitive !dative", ""),
                    group("DativeInfinitive", "infinitive dative", ""),
                    group("Conditional", "conditional", ""),
                    group("Imperative", "imperative", "Number"),
                    group("Cohortative", "cohortative cohortative-i", ""),
                    group("CohortativeII", "cohortative cohortative-ii", ""),
                    group("Optative", "optative", ""),
                    group("Hortative", "suihortative", ""),
                    group("AdverbialParticiple", "adverbial participle", "Adverbial"),
                    group("AdjectivalParticiple", "adjectival participle", "Adjectival"),
                    group(
                        "",
                        "!participle !infinitive !imperative !cohortative !conditional !optative !suihortative",
                        "Tense Person",
                    ),
                ],
            )
            .citation("ImperativeSingular PastFirstSingular")
            .lemma_cell("ImperativeSingular")
            .head_last(),
        ];
        s
    }

    /// Tables built from a template missing its past stem show "{{{3}}}".
    fn prepare(&self, e: &mut Entry) {
        e.forms.retain(|f| !f.form.contains("{{{"));
    }

    /// Table cells carry a romanization in parentheses.
    fn split_form(&self, raw: &str) -> Vec<String> {
        vec![raw.split(" (").next().unwrap_or(raw).trim().to_string()]
    }

    fn samples(&self) -> Vec<&'static str> {
        vec!["ನುಡಿ", "ಮನೆ", "ಅನಿಲ", "ಮಾಡು", "ಟೊಣೆ", "ಹೋಗು", "ಗುಲಬ", "ಜಪಟು"]
    }

    fn readme(&self) -> String {
        r#"## Example

```rust
use kannada::*;
println!("{}", noun("ನುಡಿ", Case::Dative, Number::Singular)); // ನುಡಿಗೆ
println!("{}", noun("ಅನಿಲ", Case::Genitive, Number::Plural)); // ಅನಿಲಗಳ
println!("{}", verb("ಟೊಣೆ", Tense::Present, Person::FirstSingular)); // ಟೊಣೆಯುತ್ತೇನೆ
println!("{}", verb("ಟೊಣೆ", Tense::Future, Person::ThirdFeminine)); // ಟೊಣೆಯುವಳು
println!("{}", verb_infinitive("ಟೊಣೆ")); // ಟೊಣೆಯಲು
```

Nouns have five cases in both numbers. Verbs are cited by the imperative
singular, as in Wiktionary, and have the present, past, future, negative
and contingent for nine persons (the third person distinguishes masculine,
feminine and neuter singular, and human and neuter plural), the
infinitives, conditional, imperatives, cohortatives, optative and the
adverbial and adjectival participles.
"#
        .into()
    }
}

pub fn language() -> Box<dyn Language> {
    Box::new(Kannada)
}
