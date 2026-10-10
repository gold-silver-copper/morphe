use morphe_train::kaikki::{Entry, Form};
use morphe_train::spec::*;

pub struct Ladino;

fn has(tags: &[String], t: &str) -> bool {
    tags.iter().any(|x| x == t)
}

impl Language for Ladino {
    fn spec(&self) -> Spec {
        let mut s = Spec::new("ladino", "Ladino", "lad");
        s.unmark = unmark("áa ée íi óo úu üu");
        // Hebrew-script spellings; the negative imperative (no levantes)
        // and compound forms are multiword.
        s.drop_tags.extend(tags("Hebrew negative multiword-construction"));
        // Forms of other areas on the headword line.
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
                "GenderNumber",
                &[
                    ("Masculine", "masculine singular"),
                    ("Feminine", "feminine singular"),
                    ("MasculinePlural", "masculine plural"),
                    ("FemininePlural", "feminine plural"),
                ],
            ),
            feature(
                "Tense",
                &[
                    ("Present", "indicative present"),
                    ("Imperfect", "indicative imperfect"),
                    ("Preterite", "indicative preterite"),
                    ("Future", "indicative future"),
                    ("Conditional", "conditional"),
                    ("Subjunctive", "subjunctive present"),
                    ("ImperfectSubjunctive", "subjunctive imperfect"),
                    // dyera: the same tense, labeled "past".
                    ("PastSubjunctive", "subjunctive past"),
                ],
            ),
            feature("Person", &[("First", "first-person"), ("Second", "second-person"), ("Third", "third-person")]),
            feature("Number", &[("Singular", "singular"), ("Plural", "plural")]),
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
                    group("Infinitive", "infinitive", ""),
                    group("Gerund", "gerund", ""),
                    group("Participle", "participle", "GenderNumber"),
                    group("", "!imperative !participle", "Tense Person Number"),
                    group("Imperative", "imperative", "Person Number"),
                ],
            )
            .citation("Infinitive PresentFirstSingular ParticipleMasculine")
            .head_first(),
        ];
        s
    }

    fn prepare(&self, e: &mut Entry) {
        if !e.forms.iter().any(|f| has(&f.tags, "canonical")) {
            e.forms.push(Form { form: e.word.clone(), tags: vec!["canonical".into()], source: None });
        }
        // Unlabeled persons: a singular is the third person; a plural's
        // ending tells its person (-mos, -sh/-x/-d, -n). The imperative's
        // "second-person" is the singular.
        let mut extra = Vec::new();
        for f in &mut e.forms {
            let finite = !["infinitive", "gerund", "participle", "table-tags", "inflection-template"]
                .iter()
                .any(|t| has(&f.tags, t));
            if e.pos == "verb" && f.source.is_some() && finite {
                f.tags.retain(|t| t != "error-unrecognized-form");
                let person = ["first-person", "second-person", "third-person"].iter().any(|t| has(&f.tags, t));
                let w = f.form.as_str();
                if person {
                    if has(&f.tags, "imperative") && !has(&f.tags, "singular") && !has(&f.tags, "plural") {
                        f.tags.push("singular".into());
                    }
                } else if has(&f.tags, "singular") {
                    f.tags.push("third-person".into());
                } else if has(&f.tags, "plural") {
                    let p = if w.ends_with("mos") {
                        "first-person"
                    } else if w.ends_with('n') {
                        "third-person"
                    } else {
                        "second-person"
                    };
                    f.tags.push(p.into());
                }
            } else if e.pos == "adj" && f.tags.len() == 1 && has(&f.tags, "plural") {
                // One plural for both genders.
                f.tags.push("masculine".into());
                let mut g = f.clone();
                g.tags = tags("feminine plural");
                extra.push(g);
            }
        }
        e.forms.extend(extra);
    }

    fn keep_entry(&self, e: &Entry) -> bool {
        // Latin-script words only.
        !e.word.chars().any(|c| ('\u{0590}'..='\u{05FF}').contains(&c) || ('\u{FB1D}'..='\u{FB4F}').contains(&c))
    }

    fn split_form(&self, raw: &str) -> Vec<String> {
        // Compound infinitive and gerund, negative imperative.
        if ["aver ", "aviendo ", "no "].iter().any(|p| raw.starts_with(p)) {
            return Vec::new();
        }
        vec![raw.to_string()]
    }

    fn samples(&self) -> Vec<&'static str> {
        vec!["pariente", "pie", "amar", "levantar", "dar", "derecho", "espanyol", "flumpar", "snorko"]
    }

    fn readme(&self) -> String {
        r#"## Example

```rust
use ladino::*;
println!("{}", noun_plural("pariente")); // parientes
println!("{}", adj("espanyol", Form::FemininePlural)); // espanyolas
println!("{}", verb("amar", Tense::Imperfect, Person::Third, Number::Singular)); // amava
println!("{}", verb_participle("levantar", GenderNumber::Feminine)); // levantada
```

Covers the Latin-script spelling of Wiktionary's tables (Aki Yerushalayim
style); Hebrew-script words are left out. Accents are optional on input.
"#
        .into()
    }
}

pub fn language() -> Box<dyn Language> {
    Box::new(Ladino)
}
