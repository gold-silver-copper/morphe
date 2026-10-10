use morphe_train::kaikki::Entry;
use morphe_train::spec::*;

pub struct Udmurt;

impl Language for Udmurt {
    fn spec(&self) -> Spec {
        let mut s = Spec::new("udmurt", "Udmurt", "udm");
        s.features = vec![
            feature(
                "Case",
                &[
                    ("Nominative", "nominative"),
                    ("Accusative", "accusative"),
                    ("Genitive", "genitive"),
                    ("Dative", "dative"),
                    ("Ablative", "ablative"),
                    ("Instrumental", "instrumental"),
                ],
            ),
            feature("Number", &[("Singular", "singular"), ("Plural", "plural")]),
            feature("Person", &[("First", "first-person"), ("Second", "second-person"), ("Third", "third-person")]),
            feature(
                "Tense",
                &[
                    ("Present", "present"),
                    ("Future", "future"),
                    ("Past", "preterite-1"),
                    ("Evidential", "preterite-2"),
                    ("Conditional", "conditional"),
                ],
            ),
            feature("Polarity", &[("Affirmative", "affirmative"), ("Caritive", "!affirmative")]),
            feature("ParticipleTense", &[("Present", "present"), ("Past", "past"), ("Future", "future")]),
            feature(
                "Gerund",
                &[
                    ("Plain", "!instrumental !elative !temporal !unnamed"),
                    ("Instrumental", "instrumental"),
                    ("Elative", "elative"),
                    ("Temporal", "temporal"),
                    ("Terminative", "unnamed"),
                ],
            ),
        ];
        s.pos = vec![
            pos("noun", "noun", vec![group("Possessive", "possessive", "Case Number")]).head_last(),
            pos(
                "verb",
                "verb",
                vec![
                    group("Infinitive", "infinitive affirmative", ""),
                    group("", "finite !negative !imperative", "Tense Person Number"),
                    group("Negative", "finite negative", "Tense[Present,Future,Past,Evidential] Person Number"),
                    group("Imperative", "finite imperative second-person !negative", "Number"),
                    group("Participle", "participle !gerund", "ParticipleTense Polarity"),
                    group("VerbalNoun", "unnamed affirmative !gerund", ""),
                    group("Gerund", "gerund affirmative", "Gerund"),
                    group("CaritiveGerund", "gerund !affirmative", "Gerund[Plain,Instrumental,Elative]"),
                ],
            )
            .citation("Infinitive PresentFirstSingular PastFirstSingular")
            .head_last(),
        ];
        s
    }

    /// The finite table repeats "past" for the two preterites (and the
    /// negative for three); tell them apart by position. Compound tenses
    /// are described rather than given ("preterite I of … + вал").
    fn prepare(&self, e: &mut Entry) {
        if e.pos != "verb" {
            return;
        }
        e.forms.retain(|f| !f.form.contains(" + "));
        let mut table = 0usize;
        // (block, third person seen) for affirmative and negative pasts.
        let mut past = [(0usize, false); 2];
        for f in e.forms.iter_mut().filter(|f| f.source.is_some()) {
            if f.tags.iter().any(|t| t == "table-tags") {
                table += 1;
                continue;
            }
            if table < 2 {
                // The terminative gerund (-тозь) and the verbal noun (-он).
                if let Some(t) = f.tags.iter_mut().find(|t| *t == "error-unrecognized-form") {
                    *t = "unnamed".into();
                }
                continue;
            }
            f.tags.push("finite".into());
            let has = |t: &str| f.tags.iter().any(|x| x == t);
            if !has("past") || has("auxiliary") {
                continue;
            }
            let neg = usize::from(has("negative"));
            let Some((block, third)) = past.get_mut(neg) else { continue };
            if has("first-person") && has("singular") && *third {
                *block += 1;
                *third = false;
            }
            if has("third-person") {
                *third = true;
            }
            let b = *block;
            f.tags.retain(|t| t != "past" && t != "error-unrecognized-form");
            match b {
                0 => f.tags.push("preterite-1".into()),
                1 => f.tags.push("preterite-2".into()),
                _ => f.tags = tags("table-tags"),
            }
        }
    }

    fn samples(&self) -> Vec<&'static str> {
        vec!["ю", "гурт", "книга", "мыныны", "лыдӟыны", "вераны", "шырпыл", "кыбырьяны"]
    }

    fn readme(&self) -> String {
        r#"## Example

```rust
use udmurt::*;
println!("{}", verb("мыныны", Tense::Present, Person::First, Number::Singular)); // мынӥсько
println!("{}", verb("мыныны", Tense::Future, Person::Third, Number::Plural)); // мынозы
println!("{}", verb_negative("мыныны", Tense::Present, Person::Third, Number::Singular)); // уг мыны
println!("{}", noun_possessive("ю", Case::Dative, Number::Singular)); // юзылы
```

Verbs have the present, the future, two preterites (the direct past and
the evidential), and the conditional for six persons, the negatives of the
first four (analytic, with уг/ӧй), the imperative, the participles, the
verbal noun and the gerunds (with their caritive forms). Wiktionary's
Udmurt noun tables come out of Kaikki with only the forms possessed by the
third person singular (юзы, юзылы), in six cases and both numbers.
"#
        .into()
    }
}

pub fn language() -> Box<dyn Language> {
    Box::new(Udmurt)
}
