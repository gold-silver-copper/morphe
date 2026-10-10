use morphe_train::kaikki::{Entry, Form};
use morphe_train::spec::*;
use std::collections::HashMap;
use std::sync::OnceLock;

pub struct LowGerman;

fn has(tags: &[String], t: &str) -> bool {
    tags.iter().any(|x| x == t)
}

/// Finite verb groups. Each also gets a subordinate-clause twin named
/// `Subordinate…` (separable verbs: "ding af" / "afding").
const FINITE: &[(&str, &str)] = &[("Present", "present !imperative !participle"), ("Preterite", "preterite !participle")];

impl LowGerman {
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

impl Language for LowGerman {
    fn spec(&self) -> Spec {
        let mut s = Spec::new("low_german", "LowGerman", "nds");
        s.unmark = unmark("äa öo üu åa");
        s.fold = vec![('’', '\'')];
        s.fold_strs = vec![("ß".into(), "ss".into())];
        s.inherent_form_tags = tags("auxiliary");
        // Headword-line forms of other dialects.
        s.soft_tags.extend(tags(
            "Westphalian Low-Prussian Gronings Lippisch Münsterland Sauerland Mecklenburgisch Paderbornisch",
        ));
        s.features = vec![
            feature("Gender", &[("Masculine", "masculine"), ("Feminine", "feminine"), ("Neuter", "neuter")]),
            feature("Auxiliary", &[("Hebben", "hebben"), ("Wesen", "wesen")]),
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
            feature("Tense", &[("Present", "present"), ("Past", "past")]),
            feature(
                "Degree",
                &[("Positive", "!comparative !superlative"), ("Comparative", "comparative"), ("Superlative", "superlative")],
            ),
            feature("Declension", &[("Strong", "strong"), ("Weak", "weak"), ("Mixed", "mixed")]),
            feature("Case", &[("Nominative", "nominative"), ("Oblique", "oblique")]),
            feature(
                "GenderNumber",
                &[("Masculine", "masculine"), ("Feminine", "feminine"), ("Neuter", "neuter"), ("Plural", "plural")],
            ),
        ];
        let mut verb = vec![
            group("Infinitive", "infinitive", ""),
            group("Participle", "participle", "Tense"),
            group("Imperative", "imperative !participle", "Number"),
        ];
        for (name, fixed) in FINITE {
            verb.push(group(name, &format!("{fixed} !subordinate-clause"), "PersonNumber"));
        }
        for (name, fixed) in FINITE {
            verb.push(group(&format!("Subordinate{name}"), &format!("{fixed} subordinate-clause"), "PersonNumber"));
        }
        s.pos = vec![
            pos("noun", "noun", vec![group("Plural", "plural !dative !genitive", ""), group("Diminutive", "diminutive", "")])
                .inherent("Gender")
                .citation("Lemma Plural")
                .head_last(),
            pos(
                "adj",
                "adj",
                vec![
                    group("", "", "Degree Declension Case GenderNumber"),
                    group("Predicative", "predicative", "Degree GenderNumber"),
                    group("Partitive", "partitive", "Degree GenderNumber"),
                    // The headword line's comparative and superlative.
                    group("Simple", "", "Degree[Comparative,Superlative]"),
                ],
            )
            .citation("Lemma SimpleComparative SimpleSuperlative"),
            pos("verb", "verb", verb)
                .inherent("Auxiliary")
                .citation("Infinitive PreteriteThirdSingular ParticiplePast")
                .head_last(),
        ];
        s
    }

    fn prepare(&self, e: &mut Entry) {
        if !e.forms.iter().any(|f| has(&f.tags, "canonical")) {
            e.forms.push(Form { form: e.word.clone(), tags: vec!["canonical".into()], source: None });
        }
        for f in &mut e.forms {
            let table = f.source.is_some();
            if e.pos == "adj" && table {
                // "de fre'e", "se is free", "wat Fre'es": keep the adjective.
                if has(&f.tags, "includes-article") || has(&f.tags, "predicative") || has(&f.tags, "partitive") {
                    f.form = f.form.rsplit(' ').next().unwrap_or_default().to_string();
                }
            }
            if e.pos != "verb" {
                continue;
            }
            if table {
                // Separable tables give the participles the headers of the
                // imperative row above them.
                if has(&f.tags, "participle") {
                    f.tags.retain(|t| t != "imperative" && t != "main-clause");
                }
            } else if !has(&f.tags, "participle") && !has(&f.tags, "auxiliary") && !has(&f.tags, "alternative") {
                // Headword line: "harr" (past), "kümmt vör" (present) are
                // third person singular.
                if has(&f.tags, "past") || has(&f.tags, "preterite") {
                    f.tags = vec!["preterite".into(), "singular".into(), "third-person".into()];
                } else if has(&f.tags, "present") {
                    f.tags = vec!["present".into(), "singular".into(), "third-person".into()];
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
        vec!["Kind", "Fisch", "fallen", "gaon", "afdingen", "free", "groot", "flompen", "snoorig"]
    }

    fn readme(&self) -> String {
        r#"## Example

```rust
use low_german::*;
println!("{}", noun_plural("Kind"));
println!("{}", verb_preterite("fallen", PersonNumber::ThirdSingular)); // full
println!("{}", verb_present("afdingen", PersonNumber::ThirdSingular)); // dingt af
println!("{}", verb_subordinate_present("afdingen", PersonNumber::ThirdSingular)); // afdingt
println!("{:?}", verb_auxiliary("fallen")); // Wesen
println!("{}", adj("free", Degree::Positive, Declension::Weak, Case::Oblique, GenderNumber::Masculine)); // fre'en
```

Adjective forms are given without the article or predicate the tables show
them with ("den fre'en" → "fre'en", "he is de Freeste" → "freeste").
Output is lowercase. Umlauts are optional on input, and ß matches ss.
"#
        .into()
    }
}

pub fn language() -> Box<dyn Language> {
    Box::new(LowGerman)
}
