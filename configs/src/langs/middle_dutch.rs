use morphe_train::kaikki::Entry;
use morphe_train::spec::*;

pub struct MiddleDutch;

fn has(tags: &[String], t: &str) -> bool {
    tags.iter().any(|x| x == t)
}

impl Language for MiddleDutch {
    fn spec(&self) -> Spec {
        let mut s = Spec::new("middle_dutch", "MiddleDutch", "dum");
        // Length marks are editorial: ē, ê, ā, â, ō, ô.
        s.unmark = unmark("āa âa ēe êe ëe īi îi ōo ôo õo ūu ûu");
        s.features = vec![
            feature(
                "Case",
                &[("Nominative", "nominative"), ("Accusative", "accusative"), ("Genitive", "genitive"), ("Dative", "dative")],
            ),
            feature("Number", &[("Singular", "singular"), ("Plural", "plural")]),
            feature("Gender", &[("Masculine", "masculine"), ("Feminine", "feminine"), ("Neuter", "neuter")]),
            feature(
                "GenderNumber",
                &[
                    ("Masculine", "masculine singular"),
                    ("Feminine", "feminine singular"),
                    ("Neuter", "neuter singular"),
                    ("Plural", "plural"),
                ],
            ),
            feature("Definiteness", &[("Indefinite", "indefinite"), ("Definite", "definite")]),
            feature("Mood", &[("Indicative", "indicative"), ("Subjunctive", "subjunctive")]),
            feature("Tense", &[("Present", "present"), ("Past", "past")]),
            feature("Person", &[("First", "first-person"), ("Second", "second-person"), ("Third", "third-person")]),
        ];
        s.pos = vec![
            pos("noun", "noun", vec![group("", "", "Case Number")])
                .inherent("Gender")
                .citation("NominativeSingular GenitiveSingular NominativePlural")
                .head_last(),
            pos("adj", "adj", vec![group("", "", "Definiteness? Case GenderNumber")])
                .citation("IndefiniteNominativeMasculine"),
            pos(
                "verb",
                "verb",
                vec![
                    group("Infinitive", "infinitive !genitive !dative", ""),
                    // The inflected infinitive: drivens, (te) drivene.
                    group("InflectedInfinitive", "infinitive", "Case[Genitive,Dative]"),
                    group("", "!imperative !participle !infinitive", "Mood Tense Person Number"),
                    group("Imperative", "imperative !participle", "Number"),
                    group("Participle", "participle", "Tense"),
                ],
            )
            .citation("Infinitive IndicativePastThirdSingular IndicativePastThirdPlural ParticiplePast")
            .head_first(),
        ];
        s
    }

    fn prepare(&self, e: &mut Entry) {
        for f in &mut e.forms {
            if f.source.is_none() {
                continue;
            }
            // The plain infinitive heads the table half-recognized.
            if has(&f.tags, "infinitive") {
                f.tags.retain(|t| t != "error-unrecognized-form");
            }
            // The participle rows sit under the imperative header.
            if has(&f.tags, "participle") {
                f.tags.retain(|t| t != "imperative");
            }
        }
    }

    fn samples(&self) -> Vec<&'static str> {
        vec!["raven", "name", "driven", "seven", "doen", "arm", "open", "flompen", "snoor"]
    }

    fn readme(&self) -> String {
        r#"## Example

```rust
use middle_dutch::*;
println!("{}", noun("raven", Case::Genitive, Number::Singular)); // rāvens
println!("{:?}", noun_gender("raven"));
println!("{}", verb("driven", Mood::Indicative, Tense::Past, Person::Third, Number::Singular)); // drêef
println!("{}", verb_participle("driven", Tense::Past)); // gedrēven
println!("{}", adj("arm", Definiteness::Indefinite, Case::Dative, GenderNumber::Masculine)); // armen
```

The editorial length marks (ē, ê, ā…) are optional on input; output carries
them.
"#
        .into()
    }
}

pub fn language() -> Box<dyn Language> {
    Box::new(MiddleDutch)
}
