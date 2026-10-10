use morphe_train::kaikki::{Entry, Form};
use morphe_train::spec::*;

pub struct Aromanian;

fn has(tags: &[String], t: &str) -> bool {
    tags.iter().any(|x| x == t)
}

fn set(f: &mut Form, t: &str) {
    f.tags = tags(t);
}

impl Language for Aromanian {
    fn spec(&self) -> Spec {
        let mut s = Spec::new("aromanian", "Aromanian", "rup");
        s.unmark = unmark("ãa ăa şs");
        s.features = vec![
            feature("Gender", &[("Masculine", "masculine"), ("Feminine", "feminine"), ("Neuter", "neuter")]),
            feature("Definiteness", &[("Indefinite", "indefinite"), ("Definite", "definite")]),
            feature("Case", &[("Direct", "nominative accusative"), ("Oblique", "genitive dative")]),
            feature("Number", &[("Singular", "singular"), ("Plural", "plural")]),
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
                    ("Imperfect", "indicative imperfect"),
                    ("Perfect", "indicative perfect"),
                    ("Conditional", "conditional"),
                ],
            ),
            feature("Person", &[("First", "first-person"), ("Second", "second-person"), ("Third", "third-person")]),
        ];
        s.pos = vec![
            pos("noun", "noun", vec![group("", "", "Definiteness Case Number")])
                .inherent("Gender")
                .lemma_cell("IndefiniteDirectSingular")
                .citation("IndefiniteDirectSingular IndefiniteDirectPlural")
                .head_last(),
            pos("adj", "adj", vec![group("", "", "Form")]).citation("Lemma Feminine MasculinePlural"),
            pos(
                "verb",
                "verb",
                vec![
                    group("Infinitive", "infinitive", ""),
                    group("Gerund", "gerund", ""),
                    group("ParticiplePast", "participle past", ""),
                    group("", "!imperative !participle", "Tense Person Number"),
                    group("Imperative", "imperative second-person", "Number"),
                ],
            )
            .lemma_cell("PresentFirstSingular")
            .citation("PresentFirstSingular PerfectFirstSingular ParticiplePast Infinitive")
            .head_first(),
        ];
        s
    }

    fn prepare(&self, e: &mut Entry) {
        if !e.forms.iter().any(|f| has(&f.tags, "canonical")) {
            e.forms.push(Form { form: e.word.clone(), tags: vec!["canonical".into()], source: None });
        }
        // An oblique cell may first list the article's forms ("ali",
        // "ale dzuã") before the real one, which comes with the particle
        // "a" ("a dzuãljei"): drop what precedes it.
        let mut drop = vec![false; e.forms.len()];
        let mut i = 0;
        while i < e.forms.len() {
            let mut j = i;
            while j < e.forms.len() && e.forms.get(j).map(|f| &f.tags) == e.forms.get(i).map(|f| &f.tags) {
                j += 1;
            }
            let run = e.forms.get(i..j).unwrap_or_default();
            if run.first().is_some_and(|f| f.source.is_some() && has(&f.tags, "genitive")) {
                if let Some(k) = run.iter().position(|f| f.form.starts_with("a ")) {
                    for d in drop.iter_mut().skip(i).take(k) {
                        *d = true;
                    }
                }
            }
            i = j;
        }
        let mut keep = drop.iter().map(|d| !d);
        e.forms.retain(|_| keep.next().unwrap_or(true));
        for f in &mut e.forms {
            if f.source.is_some() {
                // "(nishti, nishte) dzãli", "a (unei) dzuã", "a fratelui",
                // "unã dzuã": drop the articles and the genitive particle.
                let mut out = String::new();
                let mut depth = 0;
                for c in f.form.chars() {
                    match c {
                        '(' => depth += 1,
                        ')' => depth -= 1,
                        _ if depth == 0 => out.push(c),
                        _ => {}
                    }
                }
                let words: Vec<&str> = out.split_whitespace().collect();
                let skip = words.len().saturating_sub(1).min(
                    words.iter().take_while(|w| matches!(**w, "a" | "unã" | "un" | "ali" | "ale")).count(),
                );
                f.form = words.get(skip..).unwrap_or_default().join(" ");
                continue;
            }
            if has(&f.tags, "alternative") || has(&f.tags, "canonical") {
                continue;
            }
            // The headword line names its forms tersely.
            let t = f.tags.join(" ");
            match (e.pos.as_str(), t.as_str()) {
                ("noun", "plural") => set(f, "indefinite nominative accusative plural"),
                ("noun", "definite") | ("noun", "definite singular") => {
                    set(f, "definite nominative accusative singular")
                }
                ("noun", "definite plural") => set(f, "definite nominative accusative plural"),
                ("verb", "participle") => set(f, "participle past"),
                ("verb", "singular third-person") => set(f, "indicative present singular third-person"),
                ("verb", "imperfect") => set(f, "indicative imperfect singular first-person"),
                ("adj", "plural") => set(f, "masculine plural"),
                ("adj", "feminine singular") => set(f, "feminine"),
                _ => {}
            }
        }
    }

    fn samples(&self) -> Vec<&'static str> {
        vec!["frate", "os", "dzã", "lucredzu", "bat", "alb", "amar", "flumpedzu", "snorcu"]
    }

    fn readme(&self) -> String {
        r#"## Example

```rust
use aromanian::*;
println!("{}", noun("frate", Definiteness::Definite, Case::Oblique, Number::Singular)); // fratelui
println!("{}", adj("alb", Form::FemininePlural)); // albi
println!("{}", verb("lucredzu", Tense::Imperfect, Person::Third, Number::Singular)); // lucra
println!("{}", verb_participle_past("lucredzu")); // lucratã
```

Verbs are cited by their first person singular present, as in Wiktionary.
Oblique noun forms are given without the particle "a". ã is optional on
input.
"#
        .into()
    }
}

pub fn language() -> Box<dyn Language> {
    Box::new(Aromanian)
}
