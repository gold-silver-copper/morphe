use morphe_train::kaikki::Entry;
use morphe_train::spec::*;

pub struct CrimeanTatar;

const CASES: [&str; 6] = ["nominative", "genitive", "dative", "accusative", "locative", "ablative"];

impl Language for CrimeanTatar {
    fn spec(&self) -> Spec {
        let mut s = Spec::new("crimean_tatar", "CrimeanTatar", "crh");
        // The circumflex (â) of loanwords is often left out.
        s.unmark = unmark("âa ÂA");
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
                    ("Present", "indicative present !perfect !continuative"),
                    ("Past", "indicative past !continuative"),
                    ("Perfect", "indicative perfect"),
                    ("Future", "indicative future"),
                    ("Continuative", "continuative present"),
                    ("PastContinuative", "continuative past"),
                    ("Conditional", "conditional"),
                    ("Optative", "optative"),
                    ("Imperative", "imperative"),
                ],
            ),
            feature("ParticipleTense", &[("Past", "past"), ("Future", "future")]),
        ];
        s.pos = vec![
            pos("noun", "noun", vec![group("", "", "Case Number")]).citation("NominativeSingular NominativePlural").head_last(),
            pos(
                "verb",
                "verb",
                vec![
                    group("Infinitive", "infinitive", ""),
                    group("", "", "Tense Person Number"),
                    group("Participle", "participle", "ParticipleTense"),
                    group("Gerund", "gerund !repeated !temporal", ""),
                    group("RepeatedGerund", "gerund repeated", ""),
                    group("TemporalGerund", "gerund temporal", ""),
                    group("Converb", "adverbial !indicative !conditional !optative !imperative !continuative", ""),
                ],
            )
            .citation("Infinitive PresentThirdSingular PastThirdSingular")
            .head_last(),
        ];
        s
    }

    /// Mass nouns' tables name only the case.
    fn prepare(&self, e: &mut Entry) {
        for f in e.forms.iter_mut().filter(|f| f.source.is_some()) {
            let case = f.tags.iter().any(|t| CASES.contains(&t.as_str()));
            let number = f.tags.iter().any(|t| t == "singular" || t == "plural");
            if case && !number {
                f.tags.push("singular".into());
            }
        }
    }

    fn samples(&self) -> Vec<&'static str> {
        vec!["ab", "kitap", "ev", "köz", "ekmek", "almaq", "körmek", "şırpıl", "zıbırmaq"]
    }

    fn readme(&self) -> String {
        r#"## Example

```rust
use crimean_tatar::*;
println!("{}", noun("kitap", Case::Dative, Number::Singular)); // kitapqa
println!("{}", noun("ev", Case::Locative, Number::Plural)); // evlerde
println!("{}", verb("ekmek", Tense::Past, Person::First, Number::Singular)); // ektim
println!("{}", verb("ekmek", Tense::Future, Person::Third, Number::Plural)); // ekecekler
```

Nouns have six cases in both numbers. Verbs have the present, past,
perfect, future, two continuatives, the conditional, the optative and the
imperative for six persons, plus the infinitive, the participles, the
gerunds and the converb. Input matching ignores the circumflex (â).
"#
        .into()
    }
}

pub fn language() -> Box<dyn Language> {
    Box::new(CrimeanTatar)
}
