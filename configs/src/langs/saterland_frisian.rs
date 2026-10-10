use morphe_train::kaikki::{Entry, Form};
use morphe_train::spec::*;
use std::collections::HashMap;
use std::sync::OnceLock;

pub struct SaterlandFrisian;

fn has(tags: &[String], t: &str) -> bool {
    tags.iter().any(|x| x == t)
}

/// Finite verb groups. Each also gets a subordinate-clause twin named
/// `Subordinate…` (separable verbs: "anner ou" / "ouannert").
const FINITE: &[(&str, &str)] = &[("Present", "present !imperative !participle"), ("Past", "past !participle")];

impl SaterlandFrisian {
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

impl Language for SaterlandFrisian {
    fn spec(&self) -> Spec {
        let mut s = Spec::new("saterland_frisian", "SaterlandFrisian", "stq");
        s.unmark = unmark("äa öo üu áa ée íi óo úu");
        s.inherent_form_tags = tags("auxiliary");
        s.features = vec![
            feature("Gender", &[("Masculine", "masculine"), ("Feminine", "feminine"), ("Neuter", "neuter")]),
            feature("Auxiliary", &[("Habe", "häbe"), ("Weze", "weze")]),
            feature(
                "PersonNumber",
                &[
                    ("FirstSingular", "first-person singular"),
                    ("SecondSingular", "second-person singular"),
                    ("ThirdSingular", "third-person singular"),
                    ("Plural", "plural"),
                ],
            ),
            feature("Number", &[("Singular", "singular"), ("Plural", "plural")]),
            feature(
                "Form",
                &[
                    // breede: after the definite article, and the plural.
                    ("Inflected", "definite plural"),
                    // breeden: the masculine after the indefinite article.
                    ("Masculine", "masculine"),
                    ("Neuter", "neuter"),
                    ("Comparative", "comparative"),
                    ("Superlative", "superlative"),
                ],
            ),
            feature(
                "DetForm",
                &[
                    ("Feminine", "feminine"),
                    ("Neuter", "neuter"),
                    ("Plural", "plural"),
                    ("Predicative", "predicative"),
                    ("Oblique", "oblique"),
                ],
            ),
        ];
        let mut verb = vec![
            group("Infinitive", "infinitive !te-infinitive", ""),
            // tou heren
            group("LongInfinitive", "infinitive te-infinitive", ""),
            group("ParticiplePresent", "participle present", ""),
            group("ParticiplePast", "participle past", ""),
            group("Imperative", "imperative !participle !auxiliary", "Number"),
        ];
        for (name, fixed) in FINITE {
            verb.push(group(name, &format!("{fixed} !subordinate-clause !auxiliary"), "PersonNumber"));
        }
        for (name, fixed) in FINITE {
            verb.push(group(&format!("Subordinate{name}"), &format!("{fixed} subordinate-clause"), "PersonNumber"));
        }
        s.pos = vec![
            pos("noun", "noun", vec![group("Plural", "plural", "")])
                .inherent("Gender")
                .citation("Lemma Plural")
                .head_last(),
            pos("adj", "adj", vec![group("", "", "Form")]).citation("Lemma Comparative Superlative"),
            pos("verb", "verb", verb)
                .inherent("Auxiliary")
                .citation("Infinitive PastThirdSingular ParticiplePast")
                .head_last(),
            pos("det", "det", vec![group("", "", "DetForm")]),
        ];
        s
    }

    fn prepare(&self, e: &mut Entry) {
        if !e.forms.iter().any(|f| has(&f.tags, "canonical")) {
            e.forms.push(Form { form: e.word.clone(), tags: vec!["canonical".into()], source: None });
        }
        // Plurals come with their article: "do Wege".
        if e.pos == "noun" {
            for f in &mut e.forms {
                if let Some(rest) = f.form.strip_prefix("do ") {
                    f.form = rest.to_string();
                }
            }
        }
        if e.pos != "verb" {
            return;
        }
        let separable = e.forms.iter().any(|f| has(&f.tags, "main-clause"));
        for f in &mut e.forms {
            if f.source.is_none() {
                continue;
            }
            // The bare infinitive heads the table untagged.
            if has(&f.tags, "error-unrecognized-form") {
                f.tags = vec!["infinitive".into()];
            } else if has(&f.tags, "infinitive") {
                if let Some(rest) = f.form.strip_prefix("tou ") {
                    f.form = rest.to_string();
                }
                f.tags.push("te-infinitive".into());
            }
            if !separable {
                continue;
            }
            // Separable tables misplace their headers: the participles and
            // the auxiliary row pick up "imperative main-clause", and the
            // subordinate present loses "subordinate-clause".
            if has(&f.tags, "participle") || has(&f.tags, "auxiliary") {
                f.tags.retain(|t| t != "imperative" && t != "main-clause");
            } else if has(&f.tags, "present")
                && !has(&f.tags, "main-clause")
                && !has(&f.tags, "imperative")
                && !has(&f.tags, "subordinate-clause")
            {
                f.tags.push("subordinate-clause".into());
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
        vec!["wai", "here", "ouannerje", "breed", "sin", "Wurm", "flomperje", "snoarich"]
    }

    fn readme(&self) -> String {
        r#"## Example

```rust
use saterland_frisian::*;
println!("{}", noun_plural("wai")); // wege
println!("{}", verb_past("here", PersonNumber::ThirdSingular)); // heerde
println!("{}", verb_present("ouannerje", PersonNumber::ThirdSingular)); // annert ou
println!("{}", verb_subordinate_present("ouannerje", PersonNumber::ThirdSingular)); // ouannert
println!("{}", adj("breed", Form::Comparative)); // brader
```

Output is lowercase (nouns are capitalized in writing). Plurals are given
without the article "do". Umlauts and accents are optional on input.
"#
        .into()
    }
}

pub fn language() -> Box<dyn Language> {
    Box::new(SaterlandFrisian)
}
