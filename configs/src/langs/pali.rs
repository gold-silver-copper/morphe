use morphe_train::kaikki::Entry;
use morphe_train::spec::*;

pub struct Pali;

impl Language for Pali {
    fn spec(&self) -> Spec {
        let mut s = Spec::new("pali", "Pali", "pi");
        // Long vowels and retroflex and nasal dots, often typed plain.
        s.unmark = unmark("āa īi ūu ṃm ṁm ṅn ñn ṭt ḍd ṇn ḷl");
        s.features = vec![
            feature(
                "Case",
                &[
                    ("Nominative", "nominative"),
                    ("Accusative", "accusative"),
                    ("Instrumental", "instrumental"),
                    ("Dative", "dative"),
                    ("Ablative", "ablative"),
                    ("Genitive", "genitive"),
                    ("Locative", "locative"),
                    ("Vocative", "vocative"),
                ],
            ),
            feature("Number", &[("Singular", "singular"), ("Plural", "plural")]),
            feature("Gender", &[("Masculine", "masculine"), ("Feminine", "feminine"), ("Neuter", "neuter")]),
            feature("Person", &[("First", "first-person"), ("Second", "second-person"), ("Third", "third-person")]),
            feature("Voice", &[("Active", "active"), ("Middle", "middle-voice")]),
            feature(
                "Mood",
                &[
                    ("Present", "present"),
                    ("Imperative", "imperative"),
                    ("Optative", "optative"),
                    ("Aorist", "aorist"),
                    ("Imperfect", "imperfect"),
                    ("Future", "future"),
                    ("Conditional", "conditional"),
                ],
            ),
        ];
        s.pos = vec![
            pos("noun", "noun", vec![group("", "", "Case Number")])
                .inherent("Gender")
                .citation("NominativeSingular GenitiveSingular")
                .head_last(),
            pos("adj", "adj", vec![group("", "", "Gender Case Number")]).head_last(),
            pos("verb", "verb", vec![group("", "", "Mood Voice Person Number")])
                .citation("PresentActiveThirdSingular")
                .head_last(),
            pos("pron", "pron", vec![group("", "", "Gender? Case Number")]),
            pos("num", "num", vec![group("", "", "Gender? Case Number?")]),
        ];
        s
    }

    /// Wiktionary has Pali in many scripts; the romanized entries are read.
    fn keep_entry(&self, e: &Entry) -> bool {
        e.word.chars().all(|c| c.is_ascii() || ('\u{C0}'..='\u{24F}').contains(&c) || ('\u{1E00}'..='\u{1EFF}').contains(&c))
    }

    /// Adjective tables come one per gender, named in the table header.
    fn prepare(&self, e: &mut Entry) {
        let mut gender: Option<String> = None;
        for f in e.forms.iter_mut().filter(|f| f.source.is_some()) {
            if f.tags.iter().any(|t| t == "table-tags") {
                gender = ["masculine", "feminine", "neuter"].into_iter().find(|g| f.form.contains(g)).map(String::from);
                continue;
            }
            if let Some(g) = &gender {
                if !f.tags.contains(g) {
                    f.tags.push(g.clone());
                }
            }
        }
    }

    fn samples(&self) -> Vec<&'static str> {
        vec!["dhamma", "buddha", "citta", "gacchati", "karoti", "kusala", "phalika", "dhamati"]
    }

    fn readme(&self) -> String {
        r#"## Example

```rust
use pali::*;
assert_eq!(noun("dhamma", Case::Locative, Number::Plural), "dhammesu");
println!("{}", noun("citta", Case::Instrumental, Number::Singular)); // cittena
println!("{:?}", noun_gender("citta"));
println!("{}", adj("kusala", Gender::Feminine, Case::Nominative, Number::Singular)); // kusalā
println!("{}", verb("gacchati", Mood::Optative, Voice::Active, Person::Third, Number::Singular));
println!("{}", verb("gacchati", Mood::Imperative, Voice::Active, Person::Second, Number::Singular)); // gaccha
```

Pali in roman script; entries in other scripts are left out. Nouns are
looked up by their stem (dhamma) and have eight cases in both numbers;
adjectives decline in three genders. Verbs are looked up by the third
person singular present and have the present, imperative, optative,
aorist, imperfect, future and conditional, active and middle. Input
matching ignores the macrons and dots (ā ṃ ṭ ṇ…).
"#
        .into()
    }
}

pub fn language() -> Box<dyn Language> {
    Box::new(Pali)
}
