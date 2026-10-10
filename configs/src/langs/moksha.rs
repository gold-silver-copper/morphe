use morphe_train::kaikki::Entry;
use morphe_train::spec::*;

pub struct Moksha;

const CASES: [&str; 14] = [
    "nominative",
    "genitive",
    "dative",
    "ablative",
    "inessive",
    "elative",
    "illative",
    "lative",
    "prolative",
    "causative",
    "translative",
    "abessive",
    "comparative",
    "accusative",
];
const PLURALS: [&str; 3] = ["first-person plural", "second-person plural", "third-person plural"];

impl Language for Moksha {
    fn spec(&self) -> Spec {
        let mut s = Spec::new("moksha", "Moksha", "mdf");
        s.features = vec![
            feature(
                "Case",
                &[
                    ("Nominative", "nominative"),
                    ("Genitive", "genitive"),
                    ("Dative", "dative"),
                    ("Ablative", "ablative"),
                    ("Inessive", "inessive"),
                    ("Elative", "elative"),
                    ("Illative", "illative"),
                    ("Lative", "lative"),
                    ("Prolative", "prolative"),
                    ("Causative", "causative"),
                    ("Translative", "translative"),
                    ("Abessive", "abessive"),
                    ("Comparative", "comparative"),
                ],
            ),
            feature("Number", &[("Singular", "singular"), ("Plural", "plural")]),
            feature("Person", &[("First", "first-person"), ("Second", "second-person"), ("Third", "third-person")]),
            feature(
                "Tense",
                &[
                    ("Present", "present !multiword-construction"),
                    ("Past", "past-i"),
                    ("PastII", "past-ii !conditional"),
                    ("Future", "future multiword-construction"),
                    ("Conditional", "conditional !subjunctive"),
                    ("ConditionalSubjunctive", "conditional subjunctive"),
                    ("Desiderative", "desiderative"),
                    ("Optative", "optative"),
                ],
            ),
            feature(
                "Participle",
                &[("PassivePast", "passive past"), ("PassivePresent", "passive present"), ("ActivePresent", "active present")],
            ),
        ];
        s.pos = vec![
            pos("noun", "noun", vec![group("", "!definite !possessive", "Case Number")])
                .citation("NominativeSingular NominativePlural")
                .head_last(),
            pos("noun_definite", "noun", vec![group("", "definite", "Case[Nominative,Genitive,Dative] Number")]).head_last(),
            pos(
                "verb",
                "verb",
                vec![
                    group("Infinitive", "infinitive !infinitive-ma !infinitive-mda", ""),
                    group("InfinitiveMa", "infinitive-ma", ""),
                    group("InfinitiveMda", "infinitive-mda", ""),
                    group("", "!negative !imperative", "Tense Person Number"),
                    group("Negative", "negative !imperative !gerund", "Tense Person Number"),
                    group("Imperative", "imperative second-person !negative", "Number"),
                    group("NegativeImperative", "imperative second-person negative", "Number"),
                    group("Participle", "participle", "Participle"),
                    group("Gerund", "gerund !negative", ""),
                    group("NegativeGerund", "gerund negative", ""),
                ],
            )
            .citation("Infinitive PresentFirstSingular PastThirdSingular")
            .head_first(),
        ];
        s
    }

    /// Noun tables name only the case of singular forms. Verb tables tag
    /// the plural persons as nothing at all; each block lists the first,
    /// second and third person plural in order, each positive form
    /// followed by its negatives. The second verb table (the objective
    /// conjugation) is left out.
    fn prepare(&self, e: &mut Entry) {
        if e.pos == "noun" {
            for f in e.forms.iter_mut().filter(|f| f.source.is_some()) {
                let case = f.tags.iter().any(|t| CASES.contains(&t.as_str()));
                if case && !f.tags.iter().any(|t| t == "singular" || t == "plural") {
                    f.tags.push("singular".into());
                }
            }
            return;
        }
        if e.pos != "verb" {
            return;
        }
        let mut table = 0usize;
        let mut seen: Vec<(Vec<String>, usize)> = Vec::new();
        let mut person = "";
        for f in e.forms.iter_mut().filter(|f| f.source.is_some()) {
            if f.tags.iter().any(|t| t == "table-tags") {
                table += 1;
                continue;
            }
            if table > 1 {
                f.tags = tags("table-tags");
                continue;
            }
            if f.tags.iter().any(|t| t == "imperative") && !f.tags.iter().any(|t| t == "second-person") {
                f.tags.retain(|t| t != "error-unrecognized-form");
                f.tags.extend(tags("second-person plural"));
                continue;
            }
            if !f.tags.iter().any(|t| t == "error-unrecognized-form") {
                continue;
            }
            f.tags.retain(|t| t != "error-unrecognized-form");
            let negative = f.tags.iter().any(|t| t == "negative");
            if !negative {
                let key = f.tags.clone();
                let n = match seen.iter_mut().find(|s| s.0 == key) {
                    Some(s) => {
                        s.1 += 1;
                        s.1
                    }
                    None => {
                        seen.push((key, 0));
                        0
                    }
                };
                person = PLURALS.get(n).copied().unwrap_or("");
            }
            if person.is_empty() {
                f.tags = tags("table-tags");
            } else {
                f.tags.extend(tags(person));
            }
        }
    }

    fn samples(&self) -> Vec<&'static str> {
        vec!["он", "кудо", "цёра", "морамс", "ваномс", "шырпе", "кыбырдемс"]
    }

    fn readme(&self) -> String {
        r#"## Example

```rust
use moksha::*;
println!("{}", noun("он", Case::Inessive, Number::Singular)); // онца
println!("{}", noun_definite("он", Case::Genitive, Number::Plural)); // онтнень
println!("{}", verb("морамс", Tense::Past, Person::First, Number::Singular)); // морань
println!("{}", verb_negative("морамс", Tense::Present, Person::Third, Number::Plural)); // аф морайхть
println!("{}", verb_imperative("морамс", Number::Singular)); // морак
```

Nouns have thirteen cases in the indefinite singular and the indefinite
nominative plural, and the definite nominative, genitive and dative in
both numbers. Verbs have the present, two pasts, the periphrastic future,
the conditional, the conditional subjunctive, the desiderative and the
optative for six persons, positive and negative (the negatives are
analytic: аф моран), the imperative, the infinitives, the participles and
the gerunds.
"#
        .into()
    }
}

pub fn language() -> Box<dyn Language> {
    Box::new(Moksha)
}
