use morphe_train::kaikki::Entry;
use morphe_train::spec::*;

pub struct EasternMari;

impl Language for EasternMari {
    fn spec(&self) -> Spec {
        let mut s = Spec::new("eastern_mari", "EasternMari", "mhr");
        s.features = vec![
            feature(
                "Case",
                &[
                    ("Nominative", "nominative"),
                    ("Genitive", "genitive"),
                    ("Accusative", "accusative"),
                    ("Dative", "dative"),
                    ("Comitative", "comitative"),
                    ("Comparative", "comparative"),
                    ("Illative", "illative long-form"),
                    ("ShortIllative", "illative short-form"),
                    ("Inessive", "inessive"),
                    ("Lative", "lative"),
                ],
            ),
            feature("Number", &[("Singular", "singular"), ("Plural", "plural")]),
            feature("Person", &[("First", "first-person"), ("Second", "second-person"), ("Third", "third-person")]),
            feature(
                "Tense",
                &[
                    ("Present", "indicative present"),
                    ("Desiderative", "desiderative"),
                    ("Past", "direct past"),
                    ("CompoundPast", "compound-past"),
                    ("Imperative", "imperative"),
                ],
            ),
            feature("Participle", &[("Active", "active"), ("Passive", "passive"), ("Future", "future"), ("Negative", "negative")]),
        ];
        let nominal = || vec![group("", "!possessive", "Case Number"), group("Possessive", "possessive", "Person Number")];
        s.pos = vec![
            pos("noun", "noun", nominal()).head_last(),
            pos("adj", "adj", nominal()).head_last(),
            pos(
                "verb",
                "verb",
                vec![
                    group("Infinitive", "infinitive", ""),
                    group("VerbalNoun", "noun-from-verb", ""),
                    group("Participle", "participle", "Participle"),
                    group("", "!negative !participle", "Tense Person Number"),
                    group("Negative", "negative !participle", "Tense Person Number"),
                ],
            )
            .citation("Infinitive PresentThirdSingular PastThirdSingular")
            .head_first(),
        ];
        s
    }

    /// Verb tables leave the first and second person singular untagged
    /// (in that order; the imperative's lone one is the second), and name
    /// the desiderative and the compound past nothing at all.
    fn prepare(&self, e: &mut Entry) {
        if e.pos != "verb" {
            return;
        }
        let mut seen: Vec<(Vec<String>, usize)> = Vec::new();
        for f in e.forms.iter_mut().filter(|f| f.source.is_some()) {
            if f.tags.iter().any(|x| x == "error-unrecognized-form") {
                f.tags.retain(|t| t != "error-unrecognized-form");
                if f.tags.iter().any(|x| x == "present") {
                    f.tags.retain(|t| t != "present");
                    f.tags.push("desiderative".into());
                } else if f.tags.iter().any(|x| x == "past") {
                    f.tags.retain(|t| t != "past");
                    f.tags.push("compound-past".into());
                } else {
                    f.tags = tags("table-tags");
                    continue;
                }
            }
            let has = |t: &str| f.tags.iter().any(|x| x == t);
            if !has("singular") || has("first-person") || has("second-person") || has("third-person") {
                continue;
            }
            if has("imperative") {
                f.tags.push("second-person".into());
                continue;
            }
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
            match n {
                0 => f.tags.push("first-person".into()),
                1 => f.tags.push("second-person".into()),
                _ => f.tags = tags("table-tags"),
            }
        }
    }

    fn samples(&self) -> Vec<&'static str> {
        vec!["ю", "пӧрт", "мланде", "колаш", "лудаш", "ончаш", "шырпыл", "кыбыраш"]
    }

    fn readme(&self) -> String {
        r#"## Example

```rust
use eastern_mari::*;
println!("{}", noun("пӧрт", Case::Inessive, Number::Singular)); // пӧртышто
println!("{}", noun("ю", Case::Dative, Number::Plural)); // ю-влаклан
println!("{}", noun_possessive("ю", Person::First, Number::Singular)); // юэм
println!("{}", verb("колаш", Tense::Present, Person::First, Number::Singular)); // колам
println!("{}", verb_negative("колаш", Tense::Past, Person::Third, Number::Singular)); // ыш кол
```

Nouns and adjectives have ten cases (with the long and short illative) in
both numbers and the possessed forms for six possessors. Verbs have the
present, the desiderative, the two pasts and the imperative for six
persons, positive and negative (the negatives are analytic: ом кол), plus
the infinitive, the verbal noun and the participles. Wiktionary has
conjugation tables for a few dozen verbs only.
"#
        .into()
    }
}

pub fn language() -> Box<dyn Language> {
    Box::new(EasternMari)
}
