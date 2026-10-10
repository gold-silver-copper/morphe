use morphe_train::kaikki::{Entry, Form};
use morphe_train::spec::*;

pub struct Asturian;

fn has(tags: &[String], t: &str) -> bool {
    tags.iter().any(|x| x == t)
}

impl Language for Asturian {
    fn spec(&self) -> Spec {
        let mut s = Spec::new("asturian", "Asturian", "ast");
        s.unmark = unmark("áa ée íi óo úu üu ïi öo");
        s.fold_strs = vec![("ḥ".into(), "h".into()), ("ḷḷ".into(), "ll".into())];
        // Forms of other areas on the headword line.
        s.soft_tags.extend(tags("Western Eastern Central Northwestern Southwestern Cabrales A-Estierna"));
        s.features = vec![
            feature("Gender", &[("Masculine", "masculine"), ("Feminine", "feminine"), ("Neuter", "neuter")]),
            feature(
                "Form",
                &[
                    ("Feminine", "feminine singular"),
                    // The neuter of mass nouns: lo bono.
                    ("Neuter", "neuter singular"),
                    ("MasculinePlural", "masculine plural"),
                    ("FemininePlural", "feminine plural"),
                ],
            ),
            feature(
                "GenderNumber",
                &[
                    ("Masculine", "masculine singular"),
                    ("Feminine", "feminine singular"),
                    ("Neuter", "neuter singular"),
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
            pos("adj", "adj", vec![group("", "", "Form")]).citation("Lemma Feminine Neuter MasculinePlural"),
            pos(
                "verb",
                "verb",
                vec![
                    group("Infinitive", "infinitive", ""),
                    group("Gerund", "gerund", ""),
                    group("Participle", "participle", "GenderNumber"),
                    group("", "!imperative !participle", "Tense Person Number"),
                    group("Imperative", "imperative second-person", "Number"),
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
        let mut extra = Vec::new();
        for f in &mut e.forms {
            if has(&f.tags, "participle") {
                // "m creáu", "f pl creaes"; the headword line's is masculine.
                let (gn, rest) = match f.form.split_once(' ') {
                    Some(("m", r)) => match r.strip_prefix("pl ") {
                        Some(r) => ("masculine plural", r.to_string()),
                        None => ("masculine singular", r.to_string()),
                    },
                    Some(("f", r)) => match r.strip_prefix("pl ") {
                        Some(r) => ("feminine plural", r.to_string()),
                        None => ("feminine singular", r.to_string()),
                    },
                    Some(("n", r)) => ("neuter singular", r.to_string()),
                    _ => ("masculine singular", f.form.clone()),
                };
                f.form = rest;
                f.tags.extend(tags(gn));
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

    fn samples(&self) -> Vec<&'static str> {
        vec!["pie", "color", "crear", "abater", "facer", "vistu", "mental", "flumpar", "snorcu"]
    }

    fn readme(&self) -> String {
        r#"## Example

```rust
use asturian::*;
println!("{}", noun_plural("pie")); // pies
println!("{}", adj("vistu", Form::Neuter)); // visto
println!("{}", verb("abater", Tense::Preterite, Person::Third, Number::Singular)); // abatió
println!("{}", verb_participle("crear", GenderNumber::Feminine)); // creada
```

Adjectives and participles have a neuter for mass nouns (lo visto). Accents
are optional on input.
"#
        .into()
    }
}

pub fn language() -> Box<dyn Language> {
    Box::new(Asturian)
}
