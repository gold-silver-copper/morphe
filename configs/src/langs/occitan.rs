use morphe_train::kaikki::Entry;
use morphe_train::spec::*;

pub struct Occitan;

fn has(tags: &[String], t: &str) -> bool {
    tags.iter().any(|x| x == t)
}

impl Language for Occitan {
    fn spec(&self) -> Spec {
        let mut s = Spec::new("occitan", "Occitan", "oc");
        s.unmark = unmark("àa èe ée íi òo óo úu ïi üu");
        // Compound tenses are aver/èsser + participle.
        s.drop_tags.extend(tags("multiword-construction"));
        s.features = vec![
            feature("Gender", &[("Masculine", "masculine"), ("Feminine", "feminine")]),
            feature(
                "Form",
                &[
                    ("Feminine", "feminine !plural"),
                    ("MasculinePlural", "masculine plural"),
                    ("FemininePlural", "feminine plural"),
                ],
            ),
            feature(
                "Tense",
                &[
                    ("Present", "indicative present"),
                    ("Imperfect", "imperfect indicative"),
                    ("Preterite", "preterite"),
                    ("Future", "future"),
                    ("Conditional", "conditional !conditional-ii"),
                    // abandonèra
                    ("ConditionalII", "conditional-ii"),
                    ("Subjunctive", "present subjunctive"),
                    ("ImperfectSubjunctive", "imperfect subjunctive"),
                ],
            ),
            feature("Person", &[("First", "first-person"), ("Second", "second-person"), ("Third", "third-person")]),
            feature("Number", &[("Singular", "singular"), ("Plural", "plural")]),
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
            pos("noun", "noun", vec![group("Plural", "plural !feminine", "")])
                .inherent("Gender")
                .citation("Lemma Plural")
                .head_last(),
            pos("adj", "adj", vec![group("", "", "Form")]).citation("Lemma Feminine MasculinePlural"),
            pos(
                "verb",
                "verb",
                vec![
                    group("", "!imperative !participle !infinitive !gerund", "Tense Person Number"),
                    group("Imperative", "imperative", "Imperative"),
                    group("Gerund", "gerund", ""),
                    group("ParticiplePast", "participle past", ""),
                ],
            )
            .citation("Lemma PresentFirstSingular PreteriteThirdSingular ParticiplePast")
            .head_first(),
        ];
        s
    }

    fn prepare(&self, e: &mut Entry) {
        // The subjunctive's first and second person singular lose their
        // person: the first has none at all, the second is "unrecognized".
        for f in &mut e.forms {
            if f.source.is_none() || !has(&f.tags, "subjunctive") {
                continue;
            }
            if has(&f.tags, "first-person") || has(&f.tags, "second-person") || has(&f.tags, "third-person") {
                continue;
            }
            if has(&f.tags, "error-unrecognized-form") {
                f.tags.retain(|t| t != "error-unrecognized-form");
                if has(&f.tags, "singular") {
                    f.tags.push("second-person".into());
                }
            } else if !has(&f.tags, "plural") {
                f.tags.extend(["first-person".to_string(), "singular".to_string()]);
            }
        }
        // A noun's "feminine" is its female counterpart, not an inflection;
        // an adjective's bare "feminine" is the feminine singular.
        if e.pos == "adj" {
            for f in &mut e.forms {
                if f.tags.len() == 1 && has(&f.tags, "plural") {
                    f.tags.push("masculine".into());
                }
            }
        }
    }

    fn split_form(&self, raw: &str) -> Vec<String> {
        // Some tables leak pronunciations (flunchˈavi).
        if raw.contains(['ˈ', 'ˌ', 'ʀ', 'ɛ', 'ɔ', 'ŋ', 'ʃ', 'ʒ', 'ɾ', '\u{32F}']) {
            return Vec::new();
        }
        vec![raw.to_string()]
    }

    fn samples(&self) -> Vec<&'static str> {
        vec!["can", "ostal", "gratuit", "polit", "abandonar", "èsser", "aver", "flunchar", "glorbèl"]
    }

    fn readme(&self) -> String {
        r#"## Example

```rust
use occitan::*;
println!("{}", noun_plural("ostal")); // ostals
assert_eq!(adj("gratuit", Form::Feminine), "gratuita");
assert_eq!(verb("abandonar", Tense::Preterite, Person::Third, Number::Singular), "abandonèt");
assert_eq!(verb("abandonar", Tense::Present, Person::First, Number::Singular), "abandoni");
```

Forms follow the Languedocien standard as in Wiktionary's tables. Accents
are optional on input.
"#
        .into()
    }
}

pub fn language() -> Box<dyn Language> {
    Box::new(Occitan)
}
