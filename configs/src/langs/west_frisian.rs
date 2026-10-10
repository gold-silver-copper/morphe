use morphe_train::kaikki::{Entry, Form};
use morphe_train::spec::*;
use std::collections::HashMap;
use std::sync::OnceLock;

pub struct WestFrisian;

fn has(tags: &[String], t: &str) -> bool {
    tags.iter().any(|x| x == t)
}

/// Keep the headword's spelling (with its accents) as the lemma.
fn keep_spelling(e: &mut Entry) {
    if !e.forms.iter().any(|f| has(&f.tags, "canonical")) {
        e.forms.push(Form { form: e.word.clone(), tags: vec!["canonical".into()], source: None });
    }
}

/// Finite verb groups. Each also gets a subordinate-clause twin named
/// `Subordinate…` (separable verbs: "set oer" / "oerset").
const FINITE: &[(&str, &str)] = &[("Present", "present !imperative !participle"), ("Past", "past !participle")];

impl WestFrisian {
    fn names(&self, pos: &PosSpec) -> Vec<String> {
        static NAMES: OnceLock<HashMap<String, Vec<String>>> = OnceLock::new();
        NAMES
            .get_or_init(|| {
                let s = self.spec();
                s.pos.iter().map(|p| (p.name.clone(), s.cells(p).into_iter().map(|c| c.name).collect())).collect()
            })
            .get(&pos.name)
            .cloned()
            .unwrap_or_default()
    }
}

impl Language for WestFrisian {
    fn spec(&self) -> Spec {
        let mut s = Spec::new("west_frisian", "WestFrisian", "fy");
        s.unmark = unmark("âa êe ôo ûu úu ée");
        s.inherent_form_tags = tags("auxiliary");
        s.soft_tags.extend(tags("emphatic Wood"));
        s.features = vec![
            feature("Gender", &[("Common", "common-gender"), ("Neuter", "neuter")]),
            feature("Auxiliary", &[("Hawwe", "hawwe"), ("Weze", "wêze")]),
            feature(
                "PersonNumber",
                &[
                    ("FirstSingular", "first-person singular"),
                    ("SecondSingular", "second-person singular !clitic"),
                    ("ThirdSingular", "third-person singular"),
                    ("Plural", "plural"),
                    // loofsto: the second person with the clitic pronoun.
                    ("SecondClitic", "second-person singular clitic"),
                ],
            ),
            feature("Degree", &[("Positive", "positive"), ("Comparative", "comparative"), ("Superlative", "superlative")]),
            feature(
                "Form",
                &[
                    ("Predicative", "predicative"),
                    ("Common", "indefinite common-gender singular"),
                    ("Neuter", "indefinite neuter singular"),
                    ("Plural", "indefinite plural"),
                    ("Definite", "definite"),
                    ("Partitive", "partitive"),
                ],
            ),
        ];
        let mut verb = vec![
            group("Infinitive", "infinitive !infinitive-i-long", ""),
            // The long infinitive (te loven), also the verbal noun.
            group("LongInfinitive", "infinitive-i-long", ""),
            group("ParticiplePresent", "participle present", ""),
            group("ParticiplePast", "participle past", ""),
            group("Imperative", "imperative", ""),
        ];
        for (name, fixed) in FINITE {
            verb.push(group(name, &format!("{fixed} !subordinate-clause"), "PersonNumber"));
        }
        for (name, fixed) in FINITE {
            verb.push(group(&format!("Subordinate{name}"), &format!("{fixed} subordinate-clause"), "PersonNumber"));
        }
        s.pos = vec![
            pos("noun", "noun", vec![group("Plural", "plural", ""), group("Diminutive", "diminutive", "")])
                .inherent("Gender")
                .citation("Lemma Plural")
                .head_last(),
            pos("adj", "adj", vec![group("", "", "Degree Form")])
                .citation("Lemma ComparativePredicative SuperlativePredicative"),
            pos("verb", "verb", verb)
                .inherent("Auxiliary")
                .citation("Infinitive PastThirdSingular ParticiplePast")
                .head_last(),
        ];
        s
    }

    fn prepare(&self, e: &mut Entry) {
        keep_spelling(e);
        // Weak class-2 and irregular tables leave the singular persons
        // untagged, in order (the past often merges first and third); a
        // bare "unrecognized" present or past is the clitic -sto form.
        for tense in ["present", "past"] {
            let mut sing = Vec::new();
            for (i, f) in e.forms.iter_mut().enumerate() {
                if f.source.is_none() || !has(&f.tags, "error-unrecognized-form") || !has(&f.tags, tense) {
                    continue;
                }
                f.tags.retain(|t| t != "error-unrecognized-form");
                if has(&f.tags, "singular") {
                    sing.push(i);
                } else {
                    f.tags.extend(["second-person".to_string(), "singular".to_string(), "clitic".to_string()]);
                }
            }
            let persons: &[&[&str]] = match sing.len() {
                2 => &[&["first-person", "third-person"], &["second-person"]],
                _ => &[&["first-person"], &["second-person"], &["third-person"]],
            };
            let mut extra = Vec::new();
            for (&i, ps) in sing.iter().zip(persons) {
                let Some(f) = e.forms.get_mut(i) else { continue };
                if let Some((first, rest)) = ps.split_first() {
                    for p in rest {
                        let mut g = f.clone();
                        g.tags.push((*p).into());
                        extra.push(g);
                    }
                    f.tags.push((*first).into());
                }
            }
            e.forms.extend(extra);
        }
        if e.pos == "adj" {
            for f in &mut e.forms {
                if let Some(rest) = f.form.strip_prefix("it ") {
                    f.form = rest.to_string();
                }
            }
        }
    }

    fn adjust(&self, pos: &PosSpec, _lemma: &str, forms: &mut Vec<(usize, String, bool)>) {
        if pos.name != "verb" || forms.is_empty() {
            return;
        }
        // Only separable verbs have their own subordinate-clause forms.
        let names = self.names(pos);
        let mut extra = Vec::new();
        for (ci, name) in names.iter().enumerate() {
            let Some(src) = name.strip_prefix("Subordinate").and_then(|m| names.iter().position(|x| x == m)) else {
                continue;
            };
            if !forms.iter().any(|f| f.0 == ci) {
                extra.extend(forms.iter().filter(|f| f.0 == src).map(|f| (ci, f.1.clone(), f.2)));
            }
        }
        forms.extend(extra);
    }

    fn samples(&self) -> Vec<&'static str> {
        vec!["winter", "hûs", "love", "sykje", "gean", "oersette", "bot", "goed", "flompje", "snoarich"]
    }

    fn readme(&self) -> String {
        r#"## Example

```rust
use west_frisian::*;
println!("{}", noun_plural("winter")); // winters
println!("{}", noun_diminutive("winter")); // winterke
println!("{:?}", noun_gender("hûs"));
println!("{}", verb_past("love", PersonNumber::ThirdSingular)); // loofde
// Separable verbs split in main clauses and join in subordinate ones:
println!("{}", verb_present("oersette", PersonNumber::ThirdSingular)); // set oer
println!("{}", verb_subordinate_present("oersette", PersonNumber::ThirdSingular)); // oerset
println!("{}", adj("bot", Degree::Comparative, Form::Definite)); // bottere
```

Circumflexes and acute accents are optional on input. The predicative
superlative is given without "it".
"#
        .into()
    }
}

pub fn language() -> Box<dyn Language> {
    Box::new(WestFrisian)
}
