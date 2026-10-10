use morphe_train::kaikki::Entry;
use morphe_train::spec::*;

pub struct Estonian;

const CASES: &str = "Case";

impl Language for Estonian {
    fn spec(&self) -> Spec {
        let mut s = Spec::new("estonian", "Estonian", "et");
        // š and ž appear in loanwords and are often typed as s and z.
        s.unmark = unmark("šs žz ŠS ŽZ");
        s.features = vec![
            feature(
                "Case",
                &[
                    ("Nominative", "nominative !accusative"),
                    ("Genitive", "genitive !accusative"),
                    ("Partitive", "partitive"),
                    ("Illative", "illative"),
                    ("Inessive", "inessive"),
                    ("Elative", "elative"),
                    ("Allative", "allative"),
                    ("Adessive", "adessive"),
                    ("Ablative", "ablative"),
                    ("Translative", "translative"),
                    ("Terminative", "terminative"),
                    ("Essive", "essive"),
                    ("Abessive", "abessive"),
                    ("Comitative", "comitative"),
                ],
            ),
            feature("Number", &[("Singular", "singular"), ("Plural", "plural")]),
            feature("Person", &[("First", "first-person"), ("Second", "second-person"), ("Third", "third-person")]),
            feature(
                "Mood",
                &[
                    ("IndicativePresent", "indicative present"),
                    ("IndicativePast", "indicative past"),
                    ("ConditionalPresent", "conditional present"),
                    ("ConditionalPast", "conditional perfect"),
                    ("Imperative", "imperative present"),
                ],
            ),
            feature("Voice", &[("Active", "active"), ("Passive", "passive")]),
            feature("Tense", &[("Present", "present"), ("Past", "past")]),
            feature(
                "SupineCase",
                &[
                    ("Illative", "illative"),
                    ("Inessive", "inessive"),
                    ("Elative", "elative"),
                    ("Translative", "translative"),
                    ("Abessive", "abessive"),
                ],
            ),
        ];
        let nominal = || vec![group("", "", &format!("{CASES} Number"))];
        let mut adj = nominal();
        adj.push(group("Comparative", "comparative", ""));
        adj.push(group("Superlative", "superlative", ""));
        s.pos = vec![
            pos("noun", "noun", nominal())
                .citation("NominativeSingular GenitiveSingular PartitiveSingular")
                .head_last(),
            pos("adj", "adj", adj).citation("NominativeSingular GenitiveSingular PartitiveSingular"),
            pos(
                "verb",
                "verb",
                vec![
                    group("InfinitiveMa", "infinitive-ma nominative active", ""),
                    group("InfinitiveMaPassive", "infinitive-ma nominative passive", ""),
                    group("InfinitiveDa", "infinitive-da present", ""),
                    group("Supine", "infinitive-ma active", "SupineCase"),
                    group("Gerund", "gerund", ""),
                    group("", "!impersonal !quotative !negative", "Mood Person Number"),
                    group("Impersonal", "impersonal", "Mood[IndicativePresent,IndicativePast,ConditionalPresent,Imperative]"),
                    group("Quotative", "quotative present", "Voice"),
                    group("Participle", "participle", "Voice Tense"),
                ],
            )
            .citation("InfinitiveMa InfinitiveDa IndicativePresentThirdSingular ParticiplePassivePast")
            .head_first(),
            pos("pron", "pron", nominal()),
            pos("num", "num", nominal()),
        ];
        s
    }

    /// Verb tables list compound tenses (olen teinud, ei tee) built from
    /// auxiliaries; only the verb's own one-word forms are read. The
    /// des-form (tehes) comes untagged. Headword lines give the genitive and
    /// partitive singular without their number.
    fn prepare(&self, e: &mut Entry) {
        let words = e.word.split_whitespace().count();
        let verb = e.pos == "verb";
        e.forms.retain_mut(|f| {
            let Some(_) = f.source else {
                if !verb && f.tags.len() == 1 && (f.tags[..] == ["genitive"] || f.tags[..] == ["partitive"]) {
                    f.tags.push("singular".into());
                }
                return true;
            };
            if !verb {
                return true;
            }
            if f.form.split_whitespace().count() > words {
                return false;
            }
            if f.tags.iter().any(|t| t == "error-unrecognized-form") && f.tags.iter().any(|t| t == "present") {
                f.tags = tags("gerund");
            }
            true
        });
    }

    fn samples(&self) -> Vec<&'static str> {
        vec!["maja", "jõgi", "inimene", "tegema", "olema", "hea", "lugema", "kribalik"]
    }

    fn readme(&self) -> String {
        r#"## Example

```rust
use estonian::*;
assert_eq!(noun("maja", Case::Inessive, Number::Plural), "majades");
println!("{}", noun("jõgi", Case::Genitive, Number::Singular)); // jõe
println!("{}", noun("inimene", Case::Partitive, Number::Plural)); // inimesi
println!("{}", verb("tegema", Mood::IndicativePast, Person::First, Number::Singular)); // tegin
println!("{}", verb_infinitive_da("tegema")); // teha
println!("{}", verb_supine("tegema", SupineCase::Inessive)); // tegemas
println!("{}", verb_participle("tegema", Voice::Passive, Tense::Past)); // tehtud
// Citations pin principal parts: ma- and da-infinitive, present, past participle.
println!("{}", verb("lugema, lugeda, loeb, loetud", Mood::ConditionalPresent, Person::Third, Number::Plural));
```

Nouns, adjectives, pronouns and numerals have fourteen cases in both
numbers; adjectives also have the comparative and superlative. Verbs have
their one-word forms: the indicative present and past, the conditional
present and past, the imperative, the impersonal, the quotative, the ma-
and da-infinitives, the des-form and the participles. Compound tenses
(olen teinud) and negatives (ei tee) are built from auxiliaries and left out.
"#
        .into()
    }
}

pub fn language() -> Box<dyn Language> {
    Box::new(Estonian)
}
