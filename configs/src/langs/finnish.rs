use morphe_train::kaikki::Entry;
use morphe_train::spec::*;

pub struct Finnish;

const CASES: &str = "Case[Nominative,Genitive,Partitive,Inessive,Elative,Illative,Adessive,Ablative,Allative,Essive,Translative,Abessive,Instructive]";

/// Possessive-suffix and accusative forms stay out of the plain case cells.
const PLAIN: &str = "!possessive !singular-possessive !plural-possessive";

impl Language for Finnish {
    fn spec(&self) -> Spec {
        let mut s = Spec::new("finnish", "Finnish", "fi");
        s.soft_tags.extend(tags("rare dated poetic"));
        s.features = vec![
            feature(
                "Case",
                &[
                    ("Nominative", "nominative !accusative"),
                    ("Genitive", "genitive !accusative"),
                    ("Partitive", "partitive"),
                    ("Inessive", "inessive"),
                    ("Elative", "elative"),
                    ("Illative", "illative"),
                    ("Adessive", "adessive"),
                    ("Ablative", "ablative"),
                    ("Allative", "allative"),
                    ("Essive", "essive"),
                    ("Translative", "translative"),
                    ("Abessive", "abessive"),
                    ("Instructive", "instructive"),
                    ("Comitative", "comitative"),
                    ("Accusative", "accusative !nominative !genitive"),
                ],
            ),
            feature("Number", &[("Singular", "singular"), ("Plural", "plural")]),
            feature("Person", &[("First", "first-person"), ("Second", "second-person"), ("Third", "third-person")]),
            feature(
                "Mood",
                &[
                    ("IndicativePresent", "indicative present"),
                    ("IndicativePast", "indicative past"),
                    ("Conditional", "conditional present"),
                    ("Potential", "potential present"),
                    ("Imperative", "imperative present"),
                ],
            ),
            feature("Voice", &[("Active", "active"), ("Passive", "passive")]),
            feature("Tense", &[("Present", "present"), ("Past", "past")]),
        ];
        let finite = "!negative !passive !perfect !pluperfect";
        s.pos = vec![
            pos("noun", "noun", vec![group("", PLAIN, &format!("{CASES} Number")), group("Comitative", "comitative plural third-person", "")])
                .citation("NominativeSingular GenitiveSingular PartitiveSingular PartitivePlural")
                .head_last(),
            pos(
                "adj",
                "adj",
                vec![
                    group("", PLAIN, &format!("{CASES} Number")),
                    group("Comitative", &format!("comitative plural {PLAIN}"), ""),
                    group("Comparative", "comparative", ""),
                    group("Superlative", "superlative", ""),
                ],
            )
            .citation("NominativeSingular GenitiveSingular PartitiveSingular PartitivePlural"),
            pos(
                "verb",
                "verb",
                vec![
                    group("Infinitive", "infinitive infinitive-i", ""),
                    group("", &format!("{finite} !imperative"), "Mood[IndicativePresent,IndicativePast,Conditional,Potential] Person Number"),
                    group("Imperative", &format!("imperative present {finite} !first-person"), "Person[Second,Third] Number"),
                    group("ImperativeFirstPlural", &format!("imperative present first-person plural {finite}"), ""),
                    group("Connegative", "connegative", ""),
                    group("Passive", "passive !negative !perfect !pluperfect !participle", "Mood"),
                    group("SecondInfinitiveInessive", "infinitive-ii inessive", "Voice"),
                    group("SecondInfinitiveInstructive", "infinitive-ii instructive", ""),
                    group("ThirdInfinitive", "infinitive-iii active", "Case[Inessive,Elative,Illative,Adessive,Abessive,Instructive]"),
                    group("FourthInfinitive", "infinitive-iv", ""),
                    group("Participle", "participle !agent !negative", "Voice Tense").declines("adj", ""),
                    group("AgentParticiple", "participle agent", "").declines("adj", ""),
                    group("NegativeParticiple", "participle negative", "").declines("adj", ""),
                ],
            )
            .citation("Infinitive IndicativePresentFirstSingular IndicativePastThirdSingular")
            .head_first(),
            pos("pron", "pron", vec![group("", PLAIN, "Case[Nominative,Genitive,Partitive,Inessive,Elative,Illative,Adessive,Ablative,Allative,Essive,Translative,Abessive,Instructive,Accusative] Number")]),
            pos("num", "num", vec![group("", PLAIN, &format!("{CASES} Number"))]),
        ];
        s
    }

    /// The conjugation table's nonfinite section comes out of the parser with
    /// wrong tags (active and passive past participles both read
    /// `infinitive-i-long past`, the third infinitive reads `third-person`):
    /// retag it by the order the template lists it in. Negative present forms
    /// ("en sano") give the connegative ("sano").
    fn prepare(&self, e: &mut Entry) {
        if e.pos != "verb" {
            return;
        }
        let mut seen: Vec<String> = Vec::new();
        e.forms.retain_mut(|f| {
            if f.source.as_deref() != Some("conjugation") {
                return true;
            }
            let mut key = f.tags.clone();
            key.sort();
            let key = key.join(" ");
            let n = seen.iter().filter(|k| **k == key).count();
            seen.push(key.clone());
            let new: &str = match (key.as_str(), n) {
                ("infinitive-i-long past", 0) => "participle active past",
                ("infinitive-i-long past", 1) => "participle passive past",
                ("inessive second-person", 0) => "infinitive-ii inessive active",
                ("inessive second-person", 1) => "infinitive-ii inessive passive",
                ("instructive second-person", 0) => "infinitive-ii instructive",
                ("inessive negative second-person", 0) => "participle negative",
                ("agent", 0) => "participle agent",
                ("fourth-person negative noun-from-verb", 0) => "infinitive-iv",
                ("first-person indicative negative present singular", 0) => {
                    if let Some(last) = f.form.split_whitespace().last() {
                        f.form = last.to_string();
                    }
                    "connegative"
                }
                // Possessive forms of the infinitives say nothing about which
                // infinitive they belong to.
                (k, _) if k.contains("possessive") => return false,
                (k, 0) if k.split(' ').count() == 2 && k.ends_with(" third-person") => {
                    let case = k.trim_end_matches(" third-person");
                    if ["inessive", "elative", "illative", "adessive", "abessive", "instructive"].contains(&case) {
                        f.tags = tags(&format!("infinitive-iii active {case}"));
                    }
                    return true;
                }
                _ => return true,
            };
            f.tags = tags(new);
            true
        });
    }

    fn samples(&self) -> Vec<&'static str> {
        vec!["talo", "käsi", "tietää", "hyvä", "kuusi, kuuden", "hölökkä", "sulputtaa", "minä"]
    }

    fn readme(&self) -> String {
        r#"## Example

```rust
use finnish::*;
assert_eq!(noun("talo", Case::Inessive, Number::Plural), "taloissa");
println!("{}", noun("käsi", Case::Partitive, Number::Singular)); // kättä
println!("{}", verb("puhua", Mood::IndicativePast, Person::Third, Number::Plural)); // puhuivat
println!("{}", verb_connegative("tietää")); // tiedä
println!("{}", adj_comparative("hyvä")); // parempi
// Citations pin principal parts: nominative, genitive, partitive, partitive plural.
println!("{}", noun("kuusi, kuuden", Case::Essive, Number::Singular)); // kuutena
```

Cells cover the fourteen cases in both numbers, the comitative, the finite
moods (indicative present and past, conditional, potential, imperative), the
passive, the connegative, the infinitives and the participles. Possessive
suffixes and compound tenses (perfect, pluperfect, negatives) are left out:
Wiktionary's possessive tables don't tag noun number, and compound tenses
are built from `olla`/`ei` plus a participle or the connegative.
"#
        .into()
    }
}

pub fn language() -> Box<dyn Language> {
    Box::new(Finnish)
}
