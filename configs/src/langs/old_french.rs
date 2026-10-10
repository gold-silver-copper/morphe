use morphe_train::spec::*;

pub struct OldFrench;

fn has(tags: &[String], t: &str) -> bool {
    tags.iter().any(|x| x == t)
}

impl Language for OldFrench {
    fn spec(&self) -> Spec {
        let mut s = Spec::new("old_french", "OldFrench", "fro");
        // Editorial accents and diaeresis.
        s.unmark = unmark("ée ëe ïi üu");
        // Compound tenses are avoir/estre + participle.
        s.drop_tags.extend(tags("multiword-construction gerund"));
        s.soft_tags.extend(tags("Anglo-Norman Northern"));
        s.features = vec![
            feature("Gender", &[("Masculine", "masculine"), ("Feminine", "feminine"), ("Neuter", "neuter")]),
            feature("Case", &[("Nominative", "nominative"), ("Oblique", "oblique")]),
            feature("Number", &[("Singular", "singular"), ("Plural", "plural")]),
            feature(
                "Tense",
                &[
                    ("Present", "indicative present"),
                    ("Imperfect", "imperfect indicative"),
                    ("Preterite", "preterite"),
                    ("Future", "future"),
                    ("Conditional", "conditional"),
                    ("Subjunctive", "present subjunctive"),
                    ("ImperfectSubjunctive", "imperfect subjunctive"),
                ],
            ),
            feature("Person", &[("First", "first-person"), ("Second", "second-person"), ("Third", "third-person")]),
            feature(
                "Imperative",
                &[
                    ("SecondSingular", "second-person singular"),
                    ("FirstPlural", "first-person plural"),
                    ("SecondPlural", "second-person plural"),
                ],
            ),
        ];
        s.pos = vec![
            pos("noun", "noun", vec![group("", "", "Case Number")])
                .inherent("Gender")
                .citation("Lemma NominativeSingular ObliquePlural")
                .head_last(),
            pos(
                "adj",
                "adj",
                vec![
                    group("", "!comparative !superlative", "Case Number Gender?"),
                    group("Comparative", "comparative", ""),
                ],
            ),
            pos(
                "verb",
                "verb",
                vec![
                    group("", "!imperative !participle !infinitive", "Tense Person Number"),
                    group("Imperative", "imperative", "Imperative"),
                    group("ParticiplePresent", "participle present", ""),
                    group("ParticiplePast", "participle past", ""),
                ],
            )
            .citation("Lemma PresentThirdSingular PreteriteThirdSingular ParticiplePast")
            .head_first(),
        ];
        s
    }

    fn retag(&self, pos: &PosSpec, _form: &str, tags: &mut Vec<String>) {
        // Tables say "subjective" for the nominative.
        for t in tags.iter_mut() {
            if t == "subjective" {
                *t = "nominative".into();
            }
        }
        // The headword's feminine (livre) is the feminine oblique singular.
        if pos.name == "adj" && tags.len() == 1 && has(tags, "feminine") {
            tags.extend(["oblique".to_string(), "singular".to_string()]);
        }
    }

    fn split_form(&self, raw: &str) -> Vec<String> {
        // "cat oblique singular or"
        vec![raw.split(" oblique").next().unwrap_or(raw).to_string()]
    }

    fn samples(&self) -> Vec<&'static str> {
        vec!["cat", "chevalier", "livre", "abandoner", "avoir", "bon", "able", "flunchier", "glorbe"]
    }

    fn readme(&self) -> String {
        r#"## Example

```rust
use old_french::*;
println!("{}", noun("chevalier", Case::Nominative, Number::Singular)); // chevaliers
println!("{:?}", noun_gender("livre"));
assert_eq!(verb("abandoner", Tense::Preterite, Person::Third, Number::Singular), "abandona");
println!("{}", adj("bon", Case::Oblique, Number::Singular, Gender::Feminine)); // bone
```

Nouns and adjectives have two cases: the nominative (subject) and the
oblique (everything else, and the dictionary form). Compound tenses are left
to the caller. Editorial accents (é, ë, ï) are optional on input.
"#
        .into()
    }
}

pub fn language() -> Box<dyn Language> {
    Box::new(OldFrench)
}
