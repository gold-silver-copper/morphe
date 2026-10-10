use morphe_train::spec::*;
use std::collections::HashMap;
use std::sync::OnceLock;

pub struct Dutch;

/// Finite verb groups. Each also gets a subordinate-clause twin named
/// `Subordinate…` (separable verbs: "valt aan" / "aanvalt").
const FINITE: &[(&str, &str)] = &[("Present", "present !imperative !participle"), ("Past", "past !participle")];

impl Dutch {
    /// Cell names per part of speech, for `adjust`.
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

impl Language for Dutch {
    fn spec(&self) -> Spec {
        let mut s = Spec::new("dutch", "Dutch", "nl");
        // Subjunctives, the plural imperative and "gij" forms are archaic,
        // Flemish or both; old spellings come tagged.
        s.drop_tags.extend(tags("dated Flanders majestic"));
        s.features = vec![
            feature("Number", &[("Singular", "singular"), ("Plural", "plural")]),
            feature("Gender", &[("Masculine", "masculine"), ("Feminine", "feminine"), ("Neuter", "neuter")]),
            feature(
                "PersonNumber",
                &[
                    ("FirstSingular", "first-person singular"),
                    ("SecondSingular", "second-person singular !formal"),
                    ("Formal", "second-person formal"),
                    ("ThirdSingular", "third-person singular"),
                    ("Plural", "plural"),
                ],
            ),
            feature("Degree", &[("Positive", "positive"), ("Comparative", "comparative"), ("Superlative", "superlative")]),
            feature(
                "Form",
                &[
                    ("Predicative", "!indefinite !definite !partitive"),
                    // Common-gender singular after "een": een grote hond.
                    ("Common", "indefinite masculine singular"),
                    // Neuter singular after "een": een groot huis.
                    ("Neuter", "indefinite neuter singular"),
                    ("Plural", "indefinite plural"),
                    ("Definite", "definite"),
                    // After iets, niets, veel: iets moois.
                    ("Partitive", "partitive"),
                ],
            ),
        ];
        let mut verb = vec![
            group("Infinitive", "infinitive", ""),
            group("ParticiplePresent", "participle present", ""),
            group("ParticiplePast", "participle past", ""),
            group("Imperative", "imperative singular", ""),
        ];
        for (name, fixed) in FINITE {
            verb.push(group(name, &format!("{fixed} !subordinate-clause"), "PersonNumber"));
        }
        for (name, fixed) in FINITE {
            verb.push(group(&format!("Subordinate{name}"), &format!("{fixed} subordinate-clause"), "PersonNumber"));
        }
        s.pos = vec![
            pos("noun", "noun", vec![group("", "!diminutive", "Number[Plural]"), group("Diminutive", "diminutive", "")])
                .inherent("Gender")
                .citation("Lemma Plural")
                .head_last(),
            pos("adj", "adj", vec![group("", "", "Degree Form")])
                .citation("Lemma ComparativePredicative SuperlativePredicative"),
            pos("verb", "verb", verb).citation("Infinitive PastThirdSingular ParticiplePast").head_last(),
            pos("det", "det", vec![group("", "", "Degree[Positive] Form")]),
            pos("num", "num", vec![group("", "", "Degree[Positive] Form")]),
        ];
        s
    }

    fn adjust(&self, pos: &PosSpec, _lemma: &str, forms: &mut Vec<(usize, String, bool)>) {
        // The predicative superlative is listed with its article: "het snelst".
        for f in forms.iter_mut() {
            if let Some(rest) = f.1.strip_prefix("het ") {
                f.1 = rest.to_string();
            }
        }
        if pos.name != "verb" || forms.is_empty() {
            return;
        }
        // Only separable verbs have their own subordinate-clause forms; the
        // others use the main-clause forms.
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
        vec!["huis", "lopen", "aanvallen", "groot", "goed", "kat", "vlomperen", "snoezig"]
    }

    fn readme(&self) -> String {
        r#"## Example

```rust
use dutch::*;
assert_eq!(noun("huis", Number::Plural), "huizen");
assert_eq!(noun_diminutive("huis"), "huisje");
println!("{:?}", noun_gender("huis"));
assert_eq!(verb_past("lopen", PersonNumber::ThirdSingular), "liep");
// Separable verbs split in main clauses and join in subordinate ones:
assert_eq!(verb_present("aanvallen", PersonNumber::ThirdSingular), "valt aan");
assert_eq!(verb_subordinate_present("aanvallen", PersonNumber::ThirdSingular), "aanvalt");
assert_eq!(adj("groot", Degree::Positive, Form::Definite), "grote");
assert_eq!(adj("goed", Degree::Comparative, Form::Predicative), "beter");
// Citations pin principal parts: infinitive, past, participle.
println!("{}", verb_past("vragen, vroeg, gevraagd", PersonNumber::Plural));
```

The superlative predicative is given without "het" ("snelst"). Subjunctive,
plural imperative and "gij" forms are left out as archaic or regional.
"#
        .into()
    }
}

pub fn language() -> Box<dyn Language> {
    Box::new(Dutch)
}
