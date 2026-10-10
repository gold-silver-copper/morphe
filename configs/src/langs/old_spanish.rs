use morphe_train::kaikki::{Entry, Form};
use morphe_train::spec::*;

pub struct OldSpanish;

fn has(tags: &[String], t: &str) -> bool {
    tags.iter().any(|x| x == t)
}

impl Language for OldSpanish {
    fn spec(&self) -> Spec {
        let mut s = Spec::new("old_spanish", "OldSpanish", "osp");
        s.unmark = unmark("áa ée íi óo úu ýy");
        // The negative imperative (non) and compound forms are multiword.
        s.drop_tags.extend(tags("negative multiword-construction"));
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
                    ("Pluperfect", "indicative pluperfect"),
                    ("Future", "indicative future"),
                    ("Conditional", "conditional"),
                    ("Subjunctive", "subjunctive present"),
                    ("ImperfectSubjunctive", "subjunctive imperfect"),
                    ("FutureSubjunctive", "subjunctive future"),
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
                    group("Participle", "participle past !present", "GenderNumber"),
                    group("ParticiplePresent", "participle present", "Number"),
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
        // Unlabeled persons and moods. In a run of singulars, forms in -s
        // are the second person, those before them the first and those
        // after them the third; a plural's ending tells its person (-mos,
        // -des/-s, -n). Tenses without a mood are indicative.
        let mut seen_s: Vec<Vec<String>> = Vec::new();
        let mut runs: Vec<(Vec<String>, usize)> = Vec::new();
        let mut firsts: Vec<Form> = Vec::new();
        let mut prev: Option<Vec<String>> = None;
        let mut extra = Vec::new();
        for f in &mut e.forms {
            let finite = !["infinitive", "gerund", "participle", "table-tags", "inflection-template"]
                .iter()
                .any(|t| has(&f.tags, t));
            if e.pos == "verb" && f.source.is_some() && finite {
                f.tags.retain(|t| t != "error-unrecognized-form");
                let moods = ["indicative", "subjunctive", "conditional", "imperative"];
                if !moods.iter().any(|t| has(&f.tags, t)) {
                    // A tense's second run of unlabeled rows is the subjunctive.
                    let key = f.tags.clone();
                    if prev.as_ref() != Some(&key) {
                        match runs.iter_mut().find(|r| r.0 == key) {
                            Some(r) => r.1 += 1,
                            None => runs.push((key.clone(), 0)),
                        }
                    }
                    let n = runs.iter().find(|r| r.0 == key).map_or(0, |r| r.1);
                    prev = Some(key);
                    f.tags.push(if n == 0 { "indicative" } else { "subjunctive" }.into());
                } else {
                    prev = None;
                }
                let person = ["first-person", "second-person", "third-person"].iter().any(|t| has(&f.tags, t));
                let key = f.tags.clone();
                let w = f.form.as_str();
                if person {
                } else if has(&f.tags, "singular") {
                    let p = if w.ends_with('s') {
                        seen_s.push(key);
                        "second-person"
                    } else if seen_s.contains(&key) {
                        "third-person"
                    } else {
                        firsts.push(f.clone());
                        "first-person"
                    };
                    f.tags.push(p.into());
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
        // Rows merging the first and third person singular.
        for mut f in firsts {
            let mut third = f.tags.clone();
            third.push("third-person".into());
            if !e.forms.iter().any(|g| g.tags.len() == third.len() && third.iter().all(|t| has(&g.tags, t))) {
                f.tags = third;
                extra.push(f);
            }
        }
        e.forms.extend(extra);
    }

    fn split_form(&self, raw: &str) -> Vec<String> {
        // The negative imperative.
        if raw == "non" || raw.starts_with("non ") {
            return Vec::new();
        }
        vec![raw.to_string()]
    }

    fn samples(&self) -> Vec<&'static str> {
        vec!["pie", "lobo", "ser", "vender", "loco", "falso", "flumpar", "snorco"]
    }

    fn readme(&self) -> String {
        r#"## Example

```rust
use old_spanish::*;
println!("{}", noun_plural("pie")); // pies
println!("{}", adj("loco", Form::FemininePlural)); // locas
println!("{}", verb("ser", Tense::Imperfect, Person::Third, Number::Singular)); // era
println!("{}", verb("vender", Tense::Present, Person::Second, Number::Plural)); // vendedes
```

Accents are optional on input.
"#
        .into()
    }
}

pub fn language() -> Box<dyn Language> {
    Box::new(OldSpanish)
}
