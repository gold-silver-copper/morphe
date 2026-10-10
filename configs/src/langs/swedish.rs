use morphe_train::spec::*;

pub struct Swedish;

impl Language for Swedish {
    fn spec(&self) -> Spec {
        let mut s = Spec::new("swedish", "Swedish", "sv");
        // Kaikki marks the common-gender indefinite and the plain definite
        // adjective forms as unrecognized; they are the most frequent ones.
        s.drop_tags.retain(|t| t != "error-unrecognized-form");
        // Subjunctives and plural verb forms are dated or archaic.
        s.drop_tags.extend(tags("dated"));
        s.features = vec![
            feature("Definiteness", &[("Indefinite", "indefinite"), ("Definite", "definite")]),
            feature("Case", &[("Nominative", "!genitive"), ("Genitive", "genitive")]),
            feature("Number", &[("Singular", "singular"), ("Plural", "plural")]),
            feature("Gender", &[("Common", "common-gender"), ("Neuter", "neuter")]),
            feature("Voice", &[("Active", "!passive"), ("Passive", "passive")]),
            feature("Degree", &[("Positive", "positive"), ("Comparative", "comparative"), ("Superlative", "superlative")]),
            feature(
                "Form",
                &[
                    // en stor bil
                    ("Common", "indefinite !neuter !plural !masculine"),
                    // ett stort hus
                    ("Neuter", "indefinite neuter singular"),
                    // stora bilar
                    ("Plural", "indefinite plural"),
                    // den stora bilen
                    ("Definite", "definite !masculine"),
                    // den store mannen
                    ("DefiniteMasculine", "definite masculine singular"),
                ],
            ),
            feature("Agreement", &[("Neuter", "neuter !plural"), ("Plural", "plural")]),
        ];
        let noun = || vec![group("", "!positive !comparative !superlative", "Definiteness Case Number")];
        s.pos = vec![
            pos("noun", "noun", noun())
                .inherent("Gender")
                .citation("IndefiniteNominativeSingular DefiniteNominativeSingular IndefiniteNominativePlural")
                .head_last(),
            pos("adj", "adj", vec![group("", "", "Degree Form")])
                .citation("Lemma ComparativeCommon SuperlativeCommon"),
            pos(
                "verb",
                "verb",
                vec![
                    group("Infinitive", "infinitive", "Voice"),
                    group("Present", "present !participle !subjunctive", "Voice"),
                    group("Past", "past !participle !subjunctive", "Voice"),
                    group("Supine", "supine", "Voice"),
                    group("Imperative", "imperative", "Voice"),
                    group("ParticiplePresent", "participle present", ""),
                    group("ParticiplePast", "participle past !present", ""),
                ],
            )
            .citation("InfinitiveActive PresentActive PastActive SupineActive")
            .head_first(),
            pos("pron", "pron", vec![group("", "!personal", "Agreement")]),
            pos("det", "det", vec![group("", "!personal", "Agreement")]),
            pos("num", "num", noun()),
        ];
        s
    }

    fn samples(&self) -> Vec<&'static str> {
        vec!["hus", "bil", "skriva", "stor", "god", "tala", "flumpa", "snorkig"]
    }

    fn readme(&self) -> String {
        r#"## Example

```rust
use swedish::*;
assert_eq!(noun("bil", Definiteness::Definite, Case::Nominative, Number::Plural), "bilarna");
println!("{:?}", noun_gender("hus"));
assert_eq!(verb_past("skriva", Voice::Active), "skrev");
assert_eq!(verb_supine("skriva", Voice::Active), "skrivit");
assert_eq!(adj("stor", Degree::Positive, Form::Neuter), "stort");
assert_eq!(adj("stor", Degree::Comparative, Form::Common), "större");
assert_eq!(adj("liten", Degree::Positive, Form::Plural), "små");
// Citations pin principal parts: singular, definite singular, plural.
println!("{}", noun("mus, musen, möss", Definiteness::Definite, Case::Nominative, Number::Plural));
```

Subjunctive and plural verb forms are left out as dated. Personal pronouns
are not covered.
"#
        .into()
    }
}

pub fn language() -> Box<dyn Language> {
    Box::new(Swedish)
}
