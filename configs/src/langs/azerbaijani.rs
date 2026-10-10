use morphe_train::kaikki::Entry;
use morphe_train::spec::*;

pub struct Azerbaijani;

impl Language for Azerbaijani {
    fn spec(&self) -> Spec {
        let mut s = Spec::new("azerbaijani", "Azerbaijani", "az");
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
                "Possessor",
                &[
                    ("FirstSingular", "first-person singular"),
                    ("SecondSingular", "second-person singular"),
                    ("ThirdSingular", "third-person singular"),
                    ("FirstPlural", "first-person plural"),
                    ("SecondPlural", "second-person plural"),
                    ("ThirdPlural", "third-person plural"),
                ],
            ),
            feature("Possessed", &[("One", "possessed-single"), ("Many", "possessed-many")]),
            feature("Polarity", &[("Positive", "!negative"), ("Negative", "negative")]),
            feature(
                "Tense",
                &[
                    ("Present", "indicative present !perfect"),
                    ("Past", "indicative past !perfect !continuative"),
                    ("PerfectI", "perfect-i"),
                    ("PerfectII", "perfect-ii"),
                    ("Pluperfect", "past perfect"),
                    ("ContinuativePast", "continuative past"),
                    ("FutureIndefinite", "future indefinite"),
                    ("FutureDefinite", "future definite"),
                    ("PastFutureIndefinite", "past-future indefinite"),
                    ("PastFutureDefinite", "past-future definite"),
                    ("ConditionalPresent", "conditional present"),
                    ("ConditionalPast", "conditional past"),
                    ("Subjunctive", "subjunctive"),
                    ("ObligativePresent", "obligative present"),
                    ("ObligativePast", "obligative past"),
                    ("EvidentialPresent", "evidential present"),
                    ("EvidentialPast", "evidential past"),
                    ("Hortative", "hortative"),
                ],
            ),
            feature(
                "Participle",
                &[
                    ("NonPast", "non-past subjective"),
                    ("Future", "future subjective"),
                    ("Past", "past subjective"),
                    ("Temporal", "general temporal"),
                    ("Simultaneous", "specific temporal"),
                    ("Intensifier", "intensifier"),
                    ("Modal", "modal"),
                ],
            ),
        ];
        let nominal = || vec![group("", "!possessive", "Case Number")];
        let mut noun = nominal();
        noun.push(group("Possessive", "possessive", "Possessor Possessed Case"));
        let nonfinite = "!participle !converb !infinitive !noun-from-verb !imperative";
        s.pos = vec![
            pos("noun", "noun", noun).citation("NominativeSingular AccusativeSingular NominativePlural").head_last(),
            pos(
                "verb",
                "verb",
                vec![
                    group("Infinitive", "infinitive", "Polarity"),
                    group("VerbalNoun", "noun-from-verb", "Polarity"),
                    group("Converb", "converb", "Polarity"),
                    group("Participle", "participle !non-subject", "Polarity Participle"),
                    group("", nonfinite, "Polarity Tense Person Number"),
                    group("Imperative", "imperative", "Polarity Person Number"),
                ],
            )
            .citation("Infinitive PositivePastThirdSingular")
            .head_last(),
            pos("pron", "pron", nominal()),
        ];
        s
    }

    /// Verbs have three tables: positive, negative and impotential
    /// (gələmməmək, "be unable to come"). The negative table's infinitive,
    /// verbal noun and participles aren't tagged negative; the impotential
    /// one is left out.
    fn prepare(&self, e: &mut Entry) {
        if e.pos != "verb" {
            return;
        }
        let mut table = 0;
        e.forms.retain_mut(|f| {
            if f.source.is_none() {
                return true;
            }
            if f.tags.iter().any(|t| t == "table-tags") {
                table += 1;
            }
            if table == 2 && !f.tags.iter().any(|t| t == "negative") {
                f.tags.push("negative".into());
            }
            table <= 2
        });
    }

    fn samples(&self) -> Vec<&'static str> {
        vec!["ev", "kitab", "göz", "gəlmək", "yazmaq", "oxumaq", "zıpırtmaq", "fırtlaq"]
    }

    fn readme(&self) -> String {
        r#"## Example

```rust
use azerbaijani::*;
assert_eq!(noun("ev", Case::Locative, Number::Plural), "evlərdə");
println!("{}", noun("kitab", Case::Dative, Number::Singular)); // kitaba
println!("{}", noun_possessive("göz", Possessor::FirstSingular, Possessed::One, Case::Ablative)); // gözümdən
println!("{}", verb("gəlmək", Polarity::Positive, Tense::Present, Person::First, Number::Singular)); // gəlirəm
println!("{}", verb("yazmaq", Polarity::Negative, Tense::Past, Person::Third, Number::Plural)); // yazmadılar
println!("{}", verb_converb("oxumaq", Polarity::Positive)); // oxuyub
println!("{}", verb_infinitive("oxumaq", Polarity::Negative)); // oxumamaq
```

Nouns have six cases in both numbers and the possessed forms in every
case. Verbs have eighteen tenses and moods in both polarities, the
imperative, the infinitive, the verbal noun, the converb and seven
participles.
"#
        .into()
    }
}

pub fn language() -> Box<dyn Language> {
    Box::new(Azerbaijani)
}
