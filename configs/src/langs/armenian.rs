use morphe_train::kaikki::Entry;
use morphe_train::spec::*;

pub struct Armenian;

const NOMINAL: &str = "Case Number";

impl Language for Armenian {
    fn spec(&self) -> Spec {
        let mut s = Spec::new("armenian", "Armenian", "hy");
        // Stress, emphasis and question marks written over a vowel (խոսի՛ր).
        s.marks = vec!['\u{55B}', '\u{55C}', '\u{55E}'];
        s.drop_tags.extend(tags("negative"));
        s.features = vec![
            feature(
                "Case",
                &[
                    ("Nominative", "nominative"),
                    ("Dative", "dative"),
                    ("Ablative", "ablative"),
                    ("Instrumental", "instrumental"),
                    ("Locative", "locative"),
                ],
            ),
            feature("Number", &[("Singular", "singular"), ("Plural", "plural")]),
            feature("Possessor", &[("First", "first-person"), ("Second", "second-person")]),
            feature("Person", &[("First", "first-person"), ("Second", "second-person"), ("Third", "third-person")]),
            feature(
                "Tense",
                &[
                    ("Present", "indicative present !perfect"),
                    ("Imperfect", "imperfective past"),
                    ("Future", "future indicative !past"),
                    ("FuturePast", "future indicative past"),
                    ("Perfect", "perfect present"),
                    ("Pluperfect", "pluperfect"),
                    ("Aorist", "aorist"),
                    ("PresentSubjunctive", "present subjunctive"),
                    ("PastSubjunctive", "past subjunctive"),
                    ("ConditionalFuture", "conditional future"),
                    ("ConditionalPast", "conditional past"),
                ],
            ),
            feature(
                "Converb",
                &[
                    ("Imperfective", "imperfective"),
                    ("Simultaneous", "simultaneous"),
                    ("Perfective", "perfective"),
                    ("FutureI", "converb-i"),
                    ("FutureII", "converb-ii"),
                ],
            ),
            feature("Participle", &[("Resultative", "resultative"), ("Subjective", "subjective")]),
        ];
        let nominal = || {
            vec![
                group("", "!definite !possessive", NOMINAL),
                group("Definite", "definite !possessive", "Case[Nominative,Dative] Number"),
                group("Possessive", "possessive", "Possessor Case Number"),
            ]
        };
        let mut adj = nominal();
        adj.push(group("Superlative", "superlative", ""));
        s.pos = vec![
            pos("noun", "noun", nominal()).citation("NominativeSingular DativeSingular NominativePlural").head_last(),
            pos("adj", "adj", adj).head_last(),
            pos(
                "verb",
                "verb",
                vec![
                    group("Infinitive", "infinitive", ""),
                    group("Converb", "converb", "Converb"),
                    group("Participle", "participle", "Participle"),
                    group("AoristStem", "aorist stem", ""),
                    group("Passive", "passive", ""),
                    group("Causative", "causative", ""),
                    group("", "!imperative !converb !participle !stem", "Tense Person Number"),
                    group("Imperative", "imperative", "Number"),
                ],
            )
            .citation("Infinitive AoristThirdSingular")
            .head_last(),
        ];
        s
    }

    /// Entries carry an Eastern and a Western Armenian table; the Eastern
    /// one is read. The parser tags the first person plural of verbs
    /// `connegative converb`, the imperative `rare`, and lists a second,
    /// spurious form for each aorist cell.
    fn prepare(&self, e: &mut Entry) {
        let mut skip = false;
        let mut seen: Vec<Vec<String>> = Vec::new();
        e.forms.retain_mut(|f| {
            if f.source.is_none() {
                return true;
            }
            if f.tags.iter().any(|t| t == "table-tags") {
                skip = f.form.contains("Western-Armenian");
                seen.clear();
                return false;
            }
            if skip {
                return false;
            }
            let has = |tags: &[String], t: &str| tags.iter().any(|x| x == t);
            if has(&f.tags, "first-person") && has(&f.tags, "connegative") {
                f.tags.retain(|t| t != "connegative" && t != "converb");
            }
            if has(&f.tags, "imperative") {
                f.tags.retain(|t| t != "rare");
            }
            if f.tags.iter().any(|t| t == "aorist") && f.tags.iter().any(|t| t.ends_with("-person")) {
                let mut key = f.tags.clone();
                key.sort();
                if seen.contains(&key) {
                    return false;
                }
                seen.push(key);
            }
            true
        });
    }

    fn samples(&self) -> Vec<&'static str> {
        vec!["հայ", "գիրք", "տուն", "խոսել", "կարդալ", "մեծ", "բլոկվել", "զրտուկ"]
    }

    fn readme(&self) -> String {
        r#"## Example

```rust
use armenian::*;
assert_eq!(noun("հայ", Case::Ablative, Number::Plural), "հայերից");
println!("{}", noun_definite("գիրք", Case::Nominative, Number::Singular)); // գիրքը
println!("{}", noun_possessive("տուն", Possessor::First, Case::Nominative, Number::Singular)); // տունս
println!("{}", verb("խոսել", Tense::Aorist, Person::First, Number::Singular)); // խոսեցի
println!("{}", verb("կարդալ", Tense::PresentSubjunctive, Person::Third, Number::Plural)); // կարդան
println!("{}", verb_converb("խոսել", Converb::Imperfective)); // խոսում
```

This is Eastern Armenian in reformed spelling; Wiktionary's Western Armenian
tables are left out. Nouns have five cases in both numbers, the definite
nominative and dative, and the first- and second-person possessive forms.
Verbs have the infinitive, converbs, participles, the synthetic tenses
(aorist, subjunctives, conditionals, imperative) and the periphrastic ones
built with the copula (խոսում եմ, խոսել էի). Negative forms are left out.
Input matching ignores the stress and emphasis marks.
"#
        .into()
    }
}

pub fn language() -> Box<dyn Language> {
    Box::new(Armenian)
}
