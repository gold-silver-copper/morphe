use morphe_train::kaikki::Entry;
use morphe_train::spec::*;

pub struct Gothic;

fn has(tags: &[String], t: &str) -> bool {
    tags.iter().any(|x| x == t)
}

const PERSONS: &[&str] = &["first-person", "second-person", "third-person"];

impl Language for Gothic {
    fn spec(&self) -> Spec {
        let mut s = Spec::new("gothic", "Gothic", "got");
        s.features = vec![
            feature(
                "Case",
                &[
                    ("Nominative", "nominative"),
                    ("Vocative", "vocative"),
                    ("Accusative", "accusative"),
                    ("Genitive", "genitive"),
                    ("Dative", "dative"),
                ],
            ),
            feature("Number", &[("Singular", "singular"), ("Dual", "dual"), ("Plural", "plural")]),
            feature("Gender", &[("Masculine", "masculine"), ("Feminine", "feminine"), ("Neuter", "neuter")]),
            feature("Mood", &[("Indicative", "indicative"), ("Subjunctive", "subjunctive")]),
            feature("Tense", &[("Present", "present"), ("Past", "past")]),
            feature("Person", &[("First", "first-person"), ("Second", "second-person"), ("Third", "third-person")]),
            feature("Voice", &[("Active", "!passive"), ("Passive", "passive")]),
        ];
        let adj_cases = "Case[Nominative,Accusative,Genitive,Dative] Number[Singular,Plural] Gender";
        s.pos = vec![
            pos("noun", "noun", vec![group("", "", "Case Number[Singular,Plural]")])
                .inherent("Gender")
                .citation("Lemma GenitiveSingular")
                .head_last(),
            pos(
                "adj",
                "adj",
                vec![
                    group("", "!weak !comparative !superlative", adj_cases),
                    group("Weak", "weak", adj_cases),
                    group("Comparative", "comparative", ""),
                    group("Superlative", "superlative", ""),
                ],
            ),
            pos(
                "verb",
                "verb",
                vec![
                    group("", "!participle !infinitive !imperative", "Mood Tense Person Number Voice"),
                    group("Imperative", "imperative", "Person[Second,Third] Number"),
                    group("ParticiplePresent", "participle present", ""),
                    group("ParticiplePast", "participle past", ""),
                ],
            )
            .citation("Lemma IndicativePastThirdSingularActive ParticiplePast")
            .head_first(),
        ];
        s
    }

    fn prepare(&self, e: &mut Entry) {
        // The conjugation table gives the indicative rows (present, past,
        // and an unlabeled present passive), then the same rows for the
        // subjunctive, then the imperative. A row seen twice starts the next
        // block. The adjective table gives the strong forms, then the weak.
        let moods = ["indicative", "subjunctive", "imperative"];
        let mut mood = 0;
        let mut seen: Vec<Vec<String>> = Vec::new();
        let mut weak = false;
        for f in &mut e.forms {
            if has(&f.tags, "table-tags") {
                (mood, weak) = (0, false);
                seen.clear();
                continue;
            }
            if f.source.is_none() {
                continue;
            }
            if e.pos == "adj" {
                if has(&f.tags, "masculine") && has(&f.tags, "nominative") && has(&f.tags, "singular") {
                    weak = !seen.is_empty();
                    seen.push(Vec::new());
                }
                if weak {
                    f.tags.push("weak".into());
                }
                continue;
            }
            if e.pos != "verb" || !PERSONS.iter().any(|p| has(&f.tags, p)) || has(&f.tags, "class-1") {
                continue;
            }
            if has(&f.tags, "error-unrecognized-form") {
                f.tags.retain(|t| t != "error-unrecognized-form");
                f.tags.extend(["passive".to_string(), "present".to_string()]);
            }
            let mut key = f.tags.clone();
            key.sort();
            if seen.contains(&key) {
                mood += 1;
                seen.clear();
            }
            seen.push(key);
            if let Some(m) = moods.get(mood) {
                f.tags.push((*m).into());
            }
        }
    }

    fn samples(&self) -> Vec<&'static str> {
        vec!["𐌷𐌿𐌽𐌳𐍃", "𐌰𐍂𐌼𐍃", "𐌲𐌹𐌱𐌰", "𐌽𐌹𐌼𐌰𐌽", "𐌷𐌰𐌱𐌰𐌽", "𐌼𐌰𐌲𐌰𐌽", "𐌱𐌻𐌹𐌽𐌳𐍃", "𐍃𐌺𐍂𐌿𐌼𐍀𐌰𐌽"]
    }

    fn readme(&self) -> String {
        r#"## Example

```rust
use gothic::*;
assert_eq!(noun("𐌷𐌿𐌽𐌳𐍃", Case::Dative, Number::Plural), "𐌷𐌿𐌽𐌳𐌰𐌼");
println!("{:?}", noun_gender("𐌲𐌹𐌱𐌰"));
println!("{}", verb("𐌽𐌹𐌼𐌰𐌽", Mood::Indicative, Tense::Past, Person::Third, Number::Singular, Voice::Active));
println!("{}", adj_weak("𐌱𐌻𐌹𐌽𐌳𐍃", Case::Nominative, Number::Singular, Gender::Masculine));
```

Words are written in the Gothic alphabet. The passive exists only in the
present.
"#
        .into()
    }
}

pub fn language() -> Box<dyn Language> {
    Box::new(Gothic)
}
