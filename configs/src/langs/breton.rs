use morphe_train::kaikki::{Entry, Form};
use morphe_train::spec::*;

pub struct Breton;

fn has(tags: &[String], t: &str) -> bool {
    tags.iter().any(|x| x == t)
}

impl Language for Breton {
    fn spec(&self) -> Spec {
        let mut s = Spec::new("breton", "Breton", "br");
        s.fold = vec![('’', '\'')];
        s.unmark = unmark("éê");
        s.features = vec![
            feature("Gender", &[("Masculine", "masculine"), ("Feminine", "feminine")]),
            feature("Number", &[("Plural", "plural"), ("Singulative", "singulative"), ("Dual", "dual")]),
            feature(
                "Mutation",
                &[
                    ("Soft", "mutation-soft"),
                    ("Hard", "mutation-hard"),
                    ("Aspirate", "mutation-aspirate"),
                    ("Mixed", "mutation-mixed"),
                ],
            ),
            feature("Degree", &[("Comparative", "comparative"), ("Superlative", "superlative")]),
            feature(
                "Tense",
                &[
                    ("Present", "indicative present"),
                    ("Future", "future"),
                    ("Imperfect", "imperfect indicative"),
                    ("Preterite", "preterite"),
                    ("Conditional", "present subjunctive"),
                    ("PastConditional", "imperfect subjunctive"),
                    ("Imperative", "imperative"),
                ],
            ),
            feature(
                "PersonNumber",
                &[
                    ("FirstSingular", "first-person singular"),
                    ("SecondSingular", "second-person singular"),
                    ("ThirdSingular", "third-person singular"),
                    ("FirstPlural", "first-person plural"),
                    ("SecondPlural", "second-person plural"),
                    ("ThirdPlural", "third-person plural"),
                    ("Impersonal", "impersonal"),
                ],
            ),
        ];
        s.pos = vec![
            pos(
                "noun",
                "noun",
                vec![
                    group("", "!mutation", "Number"),
                    group("Mutated", "mutation !plural", "Mutation"),
                    group("MutatedPlural", "mutation plural", "Mutation"),
                ],
            )
            .inherent("Gender")
            .citation("Lemma Plural")
            .initial(2)
            .head_first(),
            pos(
                "adj",
                "adj",
                vec![
                    group("", "!mutation-soft !mutation-hard !mutation-aspirate !mutation-mixed", "Degree"),
                    group("Mutated", "!comparative !superlative", "Mutation"),
                ],
            )
            .citation("Lemma Comparative Superlative")
            .initial(2),
            pos(
                "verb",
                "verb",
                vec![
                    group("", "!participle !mutation", "Tense PersonNumber"),
                    group("ParticiplePresent", "participle present", ""),
                    group("ParticiplePast", "participle past", ""),
                    group("Mutated", "mutation", "Mutation"),
                ],
            )
            .initial(2)
            .head_first(),
        ];
        s
    }

    fn prepare(&self, e: &mut Entry) {
        // "never occurs": the mutation leaves the word as it is. The table
        // gives the unmutated form first in each row.
        let mut plain = [e.word.clone(), e.word.clone()];
        for f in e.forms.iter_mut() {
            if !has(&f.tags, "mutation") || has(&f.tags, "comparative") || has(&f.tags, "superlative") {
                continue;
            }
            f.tags.retain(|t| t != "error-unrecognized-form");
            let slot = usize::from(has(&f.tags, "plural"));
            if has(&f.tags, "unmutated") {
                if let Some(p) = plain.get_mut(slot) {
                    *p = f.form.clone();
                }
            } else if f.form == "never occurs" {
                if let Some(p) = plain.get(slot) {
                    f.form = p.clone();
                }
            }
        }
        // A row the table leaves out is a mutation the word doesn't undergo.
        for (slot, number) in ["singular", "plural"].iter().enumerate() {
            let row = |f: &Form| {
                has(&f.tags, "mutation")
                    && !has(&f.tags, "comparative")
                    && !has(&f.tags, "superlative")
                    && has(&f.tags, "plural") == (slot == 1)
            };
            if !e.forms.iter().any(|f| row(f) && !has(&f.tags, "unmutated")) {
                continue;
            }
            for m in ["mutation-soft", "mutation-hard", "mutation-aspirate", "mutation-mixed"] {
                if !e.forms.iter().any(|f| row(f) && has(&f.tags, m)) {
                    let form = plain.get(slot).cloned().unwrap_or_default();
                    let tags = vec!["mutation".into(), m.into(), (*number).into()];
                    e.forms.push(Form { form, tags, source: Some("mutation".into()) });
                }
            }
        }
    }

    fn split_form(&self, raw: &str) -> Vec<String> {
        // "barnet (auxiliary verb: kaout)"
        let raw = raw.split(" (").next().unwrap_or(raw);
        vec![raw.to_string()]
    }

    fn samples(&self) -> Vec<&'static str> {
        vec!["bro", "person", "barn", "kana", "brav", "du", "glazig", "trabouilh"]
    }

    fn readme(&self) -> String {
        r#"## Example

```rust
use breton::*;
assert_eq!(noun("bro", Number::Plural), "broioù");
assert_eq!(noun_mutated("bro", Mutation::Soft), "vro");
assert_eq!(noun_mutated_plural("bro", Mutation::Hard), "proioù");
assert_eq!(verb("barn", Tense::Future, PersonNumber::FirstSingular), "barnin");
assert_eq!(adj("brav", Degree::Comparative), "bravoc'h");
```

The data is small: about 60 verb tables and 300 noun mutation tables.
Typographic and straight apostrophes are interchangeable on input (c’h = c'h).
"#
        .into()
    }
}

pub fn language() -> Box<dyn Language> {
    Box::new(Breton)
}
