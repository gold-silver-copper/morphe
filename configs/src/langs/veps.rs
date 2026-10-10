use morphe_train::kaikki::Entry;
use morphe_train::spec::*;

pub struct Veps;

const LOCAL: [&str; 6] = ["inessive", "instructive", "illative", "elative", "adessive", "abessive"];

impl Language for Veps {
    fn spec(&self) -> Spec {
        let mut s = Spec::new("veps", "Veps", "vep");
        s.features = vec![
            feature(
                "Case",
                &[
                    ("Nominative", "nominative"),
                    ("Accusative", "accusative"),
                    ("Genitive", "genitive"),
                    ("Partitive", "partitive"),
                    ("EssiveInstructive", "essive-instructive"),
                    ("Translative", "translative"),
                    ("Inessive", "inessive"),
                    ("Elative", "elative"),
                    ("Illative", "illative"),
                    ("Adessive", "adessive"),
                    ("Ablative", "ablative"),
                    ("Allative", "allative"),
                    ("Abessive", "abessive"),
                    ("Comitative", "comitative"),
                    ("Prolative", "prolative"),
                    ("ApproximativeI", "approximative-i"),
                    ("ApproximativeII", "approximative-ii"),
                    ("Egressive", "egressive"),
                    ("TerminativeI", "terminative-i"),
                    ("TerminativeII", "terminative-ii"),
                    ("TerminativeIII", "terminative-iii"),
                    ("AdditiveI", "additive-i"),
                    ("AdditiveII", "additive-ii"),
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
                    ("Conditional", "conditional present"),
                    ("ConditionalPast", "conditional past"),
                    ("Potential", "potential"),
                ],
            ),
            feature("Tense", &[("Present", "present"), ("Past", "past")]),
            feature("Voice", &[("Active", "active"), ("Passive", "passive")]),
            feature("InfinitiveII", &[("Inessive", "inessive"), ("Instructive", "instructive")]),
            feature(
                "InfinitiveIII",
                &[
                    ("Inessive", "inessive"),
                    ("Illative", "illative"),
                    ("Elative", "elative"),
                    ("Adessive", "adessive"),
                    ("Abessive", "abessive"),
                ],
            ),
        ];
        let nominal = || vec![group("", "", "Case Number")];
        s.pos = vec![
            pos("noun", "noun", nominal()).citation("NominativeSingular GenitiveSingular PartitiveSingular").head_last(),
            pos("adj", "adj", nominal()).head_last(),
            pos(
                "verb",
                "verb",
                vec![
                    group("Infinitive", "infinitive-i", ""),
                    group("InfinitiveII", "infinitive-ii", "InfinitiveII"),
                    group("InfinitiveIII", "infinitive-iii", "InfinitiveIII"),
                    group("", "!connegative !infinitive-ii !infinitive-iii", "Mood Person Number"),
                    group("Connegative", "connegative", "Mood Number?"),
                    group("Participle", "participle !infinitive-iii", "Voice Tense"),
                ],
            )
            .citation("Infinitive PresentFirstSingular PastFirstSingular")
            .head_last(),
            pos("pron", "pron", nominal()),
            pos("num", "num", nominal()),
        ];
        s
    }

    /// The conjugation's second and third infinitives come tagged as
    /// conditional and potential forms with a case.
    fn prepare(&self, e: &mut Entry) {
        if e.pos != "verb" {
            return;
        }
        for f in e.forms.iter_mut().filter(|f| f.source.is_some()) {
            let has = |t: &str| f.tags.iter().any(|x| x == t);
            let Some(case) = LOCAL.iter().find(|c| has(c)) else { continue };
            if has("connegative") {
                continue;
            }
            let inf = if has("conditional") { "infinitive-ii" } else if has("potential") { "infinitive-iii" } else { continue };
            f.tags = tags(&format!("{inf} {case}"));
        }
    }

    fn samples(&self) -> Vec<&'static str> {
        vec!["kala", "sana", "torta", "olda", "kirpak", "plokta"]
    }

    fn readme(&self) -> String {
        r#"## Example

```rust
use veps::*;
println!("{}", noun("kala", Case::Inessive, Number::Plural)); // kaloiš
println!("{}", noun("kala", Case::Illative, Number::Singular)); // kalaha
println!("{}", verb("torta", Mood::Past, Person::First, Number::Singular)); // torzin
println!("{}", verb("torta", Mood::Conditional, Person::Third, Number::Plural)); // tordaižiba
```

Nouns have twenty-three cases in both numbers, including the
approximatives, terminatives and additives. Verbs have the indicative
present and past, the imperative, the conditional present and past and the
potential in all persons, the connegatives, the three infinitives and the
participles; compound tenses and negatives are left out.
"#
        .into()
    }
}

pub fn language() -> Box<dyn Language> {
    Box::new(Veps)
}
