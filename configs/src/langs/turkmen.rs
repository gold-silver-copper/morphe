use morphe_train::kaikki::Entry;
use morphe_train::spec::*;

pub struct Turkmen;

impl Language for Turkmen {
    fn spec(&self) -> Spec {
        let mut s = Spec::new("turkmen", "Turkmen", "tk");
        // Wiktionary marks long vowels (āt, atýār) for learners; the standard
        // spelling leaves them out.
        s.unmark = unmark("āa ēe īi ōo ūu ȳy ǟä ȫö ǖü ĀA ĒE ĪI ŌO ŪU");
        s.marks = vec!['\u{304}'];
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
                ],
            ),
            feature("Number", &[("Singular", "singular"), ("Plural", "plural")]),
            feature("Person", &[("First", "first-person"), ("Second", "second-person"), ("Third", "third-person")]),
            feature(
                "Tense",
                &[
                    ("Present", "indicative present"),
                    ("Past", "indicative past proximal"),
                    ("DistalPast", "indicative past distal"),
                    ("Unwitnessed", "indicative past unwitnessed"),
                    ("Future", "indicative future indefinite"),
                    ("PastFuture", "indicative past-future indefinite"),
                    ("DefinitePastFuture", "indicative past-future definite"),
                    ("Conditional", "conditional"),
                    ("Imperative", "imperative"),
                    ("IntentivePast", "intentive past"),
                    ("ObligativePast", "obligative past"),
                ],
            ),
            feature("ParticipleTense", &[("Present", "present"), ("Past", "past"), ("Future", "future")]),
        ];
        let nominal = || vec![group("", "", "Case Number")];
        s.pos = vec![
            pos("noun", "noun", nominal()).citation("NominativeSingular AccusativeSingular NominativePlural").head_last(),
            pos(
                "verb",
                "verb",
                vec![
                    group("Infinitive", "infinitive", ""),
                    group("", "", "Tense Person Number"),
                    group("DefiniteFuture", "indicative future definite", ""),
                    group("Intentive", "intentive present", ""),
                    group("Obligative", "obligative present", ""),
                    group("Participle", "participle", "ParticipleTense"),
                    group("Gerund", "gerund", ""),
                ],
            )
            .citation("Infinitive PresentThirdSingular PastThirdSingular")
            .head_last(),
            pos("pron", "pron", nominal()),
        ];
        s
    }

    /// Mass nouns are tagged `singular-only`; the conditional's compound
    /// rows are descriptions ("past participle + conditional of bolmak").
    fn prepare(&self, e: &mut Entry) {
        e.forms.retain(|f| !f.form.contains(" + "));
        for f in e.forms.iter_mut() {
            for t in f.tags.iter_mut() {
                if t == "singular-only" {
                    *t = "singular".into();
                }
            }
        }
    }

    fn samples(&self) -> Vec<&'static str> {
        vec!["at", "kitap", "göz", "atmak", "gelmek", "okamak", "şyrpyl", "zykmak"]
    }

    fn readme(&self) -> String {
        r#"## Example

```rust
use turkmen::*;
println!("{}", noun("kitap", Case::Dative, Number::Singular)); // kitaba
println!("{}", noun("at", Case::Ablative, Number::Plural)); // atlardan
println!("{}", verb("atmak", Tense::Present, Person::First, Number::Singular)); // atýaryn
println!("{}", verb("atmak", Tense::Past, Person::Third, Number::Plural)); // atdylar
```

Nouns have six cases in both numbers. Verbs have the present, three pasts
(proximal, distal, unwitnessed), the indefinite future, two past-futures,
the conditional, the imperative and the intentive and obligative pasts for
six persons, plus the infinitive, the participles and the gerund.
Wiktionary marks long vowels (āt, atýār) the standard spelling leaves out:
input matching ignores them, and `marks(false)` removes them from output.
"#
        .into()
    }
}

pub fn language() -> Box<dyn Language> {
    Box::new(Turkmen)
}
