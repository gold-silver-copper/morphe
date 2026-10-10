use morphe_train::kaikki::Entry;
use morphe_train::spec::*;

pub struct Tamil;

/// The case forms the parser leaves untagged, in table order: two genitives,
/// two locatives (-இல், and -இடம் for people), two sociatives.
const UNTAGGED_CASES: [&str; 6] = ["genitive", "genitive", "locative", "locative", "sociative", "sociative"];

/// The participial nouns' referents, in table order, for each of the four
/// blocks (present, past, future, negative).
const REFERENTS: [&str; 6] = ["masculine", "feminine", "honorific", "neuter", "epicene", "neuter-plural"];
const NOUN_TENSES: [&str; 4] = ["present", "past", "future", "negative"];

impl Language for Tamil {
    fn spec(&self) -> Spec {
        let mut s = Spec::new("tamil", "Tamil", "ta");
        s.features = vec![
            feature(
                "Case",
                &[
                    ("Nominative", "nominative"),
                    ("Accusative", "accusative"),
                    ("Dative", "dative"),
                    ("Benefactive", "benefactive"),
                    ("Genitive", "genitive"),
                    ("Locative", "locative"),
                    ("Sociative", "sociative"),
                    ("Instrumental", "instrumental"),
                    ("Ablative", "ablative"),
                    ("Vocative", "vocative"),
                ],
            ),
            feature("Number", &[("Singular", "singular"), ("Plural", "plural")]),
            feature(
                "Tense",
                &[
                    ("Present", "present !negative"),
                    ("Past", "past !negative"),
                    ("Future", "future !negative"),
                    ("FutureNegative", "future negative"),
                ],
            ),
            feature(
                "Person",
                &[
                    ("FirstSingular", "first-person singular"),
                    ("SecondSingular", "second-person singular"),
                    ("ThirdMasculine", "third-person masculine singular"),
                    ("ThirdFeminine", "third-person feminine singular"),
                    ("ThirdHonorific", "third-person honorific"),
                    ("ThirdNeuter", "third-person neuter singular"),
                    ("FirstPlural", "first-person plural"),
                    ("SecondPlural", "second-person plural"),
                    ("ThirdPlural", "third-person epicene plural"),
                    ("ThirdNeuterPlural", "third-person neuter plural"),
                ],
            ),
            feature(
                "Referent",
                &[
                    ("Masculine", "masculine"),
                    ("Feminine", "feminine"),
                    ("Honorific", "honorific"),
                    ("Neuter", "neuter"),
                    ("Plural", "epicene"),
                    ("NeuterPlural", "neuter-plural"),
                ],
            ),
            feature(
                "Participle",
                &[("Present", "present"), ("Past", "past"), ("Future", "future"), ("Negative", "negative")],
            ),
        ];
        let nominal = || vec![group("", "", "Case Number")];
        s.pos = vec![
            pos("noun", "noun", nominal()).citation("NominativeSingular AccusativeSingular").head_last(),
            pos(
                "verb",
                "verb",
                vec![
                    group("", "finite", "Tense Person"),
                    group("Imperative", "imperative !negative", "Number"),
                    group("NegativeImperative", "imperative negative", "Number"),
                    group("Infinitive", "x-infinitive", ""),
                    group("Potential", "x-potential", ""),
                    group("Cohortative", "x-cohortative", ""),
                    group("Conditional", "x-conditional", ""),
                    group("Causal", "x-causal", ""),
                    group("AdverbialParticiple", "x-adverbial", ""),
                    group("NegativeAdverbialParticiple", "x-adverbial-negative", ""),
                    group("AdjectivalParticiple", "x-adjectival", "Participle"),
                    group("ParticipialNoun", "x-participial", "Participle Referent"),
                    group("VerbalNoun", "x-verbal-noun", ""),
                    group("Progressive", "x-progressive", ""),
                    group("Passive", "x-passive", ""),
                ],
            )
            .citation("ImperativeSingular Infinitive AdverbialParticiple")
            .head_last(),
            pos("pron", "pron", nominal()),
            pos("num", "num", nominal()),
        ];
        s
    }

    /// The declension table leaves six case forms untagged and the
    /// conjugation table mislabels its nonfinite half; both come in a fixed
    /// order, read here. Nonfinite forms get private `x-` tags.
    fn prepare(&self, e: &mut Entry) {
        let (mut sg, mut pl) = (0usize, 0usize);
        let mut nouns: Option<usize> = None;
        for f in e.forms.iter_mut().filter(|f| f.source.is_some()) {
            let has = |t: &str| f.tags.iter().any(|x| x == t);
            if e.pos != "verb" {
                if has("error-unrecognized-form") {
                    let (k, num) = if has("plural") { (&mut pl, "plural") } else { (&mut sg, "singular") };
                    let case = UNTAGGED_CASES.get(*k).copied().unwrap_or("table-tags");
                    f.tags = tags(&format!("{case} {num}"));
                    *k += 1;
                }
                continue;
            }
            if let Some(k) = nouns {
                nouns = (k + 1 < 24).then_some(k + 1);
                let tense = NOUN_TENSES.get(k / 6).copied().unwrap_or("");
                let who = REFERENTS.get(k % 6).copied().unwrap_or("");
                f.tags = tags(&format!("x-participial {tense} {who}"));
                continue;
            }
            let negative = has("negative");
            let new = if has("adjectival") && has("participle") {
                if negative {
                    nouns = Some(0);
                    "x-adjectival negative".to_string()
                } else {
                    let t = ["present", "past", "future"].into_iter().find(|t| has(t)).unwrap_or("");
                    format!("x-adjectival {t}")
                }
            } else if has("adverbial") && has("participle") {
                if negative { "x-adverbial-negative".into() } else { "x-adverbial".into() }
            } else if negative && !has("imperative") && !(has("future") && f.tags.iter().any(|t| t.ends_with("-person"))) {
                "table-tags".into()
            } else if has("infinitive") {
                "x-infinitive".into()
            } else if has("potential") {
                "x-potential".into()
            } else if has("cohortative") {
                "x-cohortative".into()
            } else if has("conditional") {
                if has("informal") { "x-causal".into() } else { "x-conditional".into() }
            } else if has("gerund-i") || has("gerund-ii") {
                "x-verbal-noun".into()
            } else if has("gerund") {
                "table-tags".into()
            } else if has("progressive") {
                "x-progressive".into()
            } else if has("effective") {
                "x-passive".into()
            } else if has("perfect") {
                "table-tags".into()
            } else if has("imperative") {
                continue;
            } else if f.tags.iter().any(|t| t.ends_with("-person")) {
                f.tags.push("finite".into());
                continue;
            } else {
                continue;
            };
            f.tags = tags(&new);
        }
    }

    fn samples(&self) -> Vec<&'static str> {
        vec!["வீடு", "மரம்", "அக்கா", "செய்", "படி", "போ", "நான்", "குலம்பு"]
    }

    fn readme(&self) -> String {
        r#"## Example

```rust
use tamil::*;
assert_eq!(noun("வீடு", Case::Dative, Number::Singular), "வீட்டுக்கு");
println!("{}", noun("மரம்", Case::Locative, Number::Plural)); // மரங்களில்
println!("{}", verb("செய்", Tense::Past, Person::FirstSingular)); // செய்தேன்
println!("{}", verb("படி", Tense::Future, Person::ThirdPlural)); // படிப்பார்கள்
println!("{}", verb_infinitive("செய்")); // செய்ய
println!("{}", verb_adverbial_participle("போ")); // போய்
println!("{}", verb_participial_noun("செய்", Participle::Past, Referent::Masculine)); // செய்தவன்
```

Nouns have ten cases in both numbers; the genitive (-உடைய, -இன்), the
locative (-இல், -இடம்) and the sociative (-ஓடு, -உடன்) have two forms each.
Verbs are cited by the imperative singular, as in Wiktionary. They have the
present, past and future for all ten persons, the negative future, the
imperatives, the infinitive, the potential, cohortative, conditional and
causal forms, the adverbial and adjectival participles, the participial
nouns and the verbal noun.
"#
        .into()
    }
}

pub fn language() -> Box<dyn Language> {
    Box::new(Tamil)
}
