use morphe_train::kaikki::{Entry, Form};
use morphe_train::spec::*;

pub struct OldIrish;

fn has(tags: &[String], t: &str) -> bool {
    tags.iter().any(|x| x == t)
}

const TENSES: &[&str] =
    &["present", "imperfect", "preterite", "perfect", "future", "conditional", "subjunctive", "imperative", "past"];

impl Language for OldIrish {
    fn spec(&self) -> Spec {
        let mut s = Spec::new("old_irish", "OldIrish", "sga");
        // Length marks and editorial diaeresis are often left out.
        s.unmark = unmark("áa ée íi óo úu ïi öo üu äa ëe");
        s.features = vec![
            feature("Gender", &[("Masculine", "masculine"), ("Feminine", "feminine"), ("Neuter", "neuter")]),
            feature(
                "Case",
                &[
                    ("Nominative", "nominative"),
                    ("Vocative", "vocative"),
                    ("Accusative", "accusative"),
                    ("Genitive", "genitive"),
                    ("Dative", "dative"),
                ],
            ),
            feature("Number", &[("Singular", "singular"), ("Dual", "dual"), ("Plural", "plural")]),
            feature("Mutation", &[("Lenition", "lenition"), ("Nasalization", "nasalization")]),
            feature("Degree", &[("Equative", "equative"), ("Comparative", "comparative"), ("Superlative", "superlative")]),
            feature(
                "Tense",
                &[
                    ("Present", "present indicative"),
                    ("Imperfect", "imperfect"),
                    ("Preterite", "preterite"),
                    ("Perfect", "perfect"),
                    ("Future", "future"),
                    ("Conditional", "conditional"),
                    ("Subjunctive", "present subjunctive"),
                    ("PastSubjunctive", "past subjunctive"),
                    ("Imperative", "imperative"),
                ],
            ),
            feature(
                "Dependency",
                &[("Absolute", "!conjunct !relative"), ("Conjunct", "conjunct"), ("Relative", "relative")],
            ),
            feature(
                "Subject",
                &[
                    ("FirstSingular", "active first-person singular"),
                    ("SecondSingular", "active second-person singular"),
                    ("ThirdSingular", "active third-person singular"),
                    ("FirstPlural", "active first-person plural"),
                    ("SecondPlural", "active second-person plural"),
                    ("ThirdPlural", "active third-person plural"),
                    ("PassiveSingular", "passive singular"),
                    ("PassivePlural", "passive plural"),
                ],
            ),
        ];
        let mutated = || group("Mutated", "mutation", "Mutation");
        s.pos = vec![
            pos("noun", "noun", vec![group("", "!mutation", "Case Number"), mutated()])
                .inherent("Gender")
                .citation("Lemma GenitiveSingular NominativePlural")
                .initial(2)
                .head_first(),
            pos(
                "adj",
                "adj",
                vec![
                    group("", "!mutation !comparative !superlative !equative", "Case Number[Singular,Plural] Gender"),
                    group("Degree", "", "Degree"),
                    mutated(),
                ],
            )
            .initial(2),
            pos(
                "verb",
                "verb",
                vec![
                    group("", "!noun-from-verb !participle !mutation", "Tense Dependency Subject"),
                    group("VerbalNoun", "noun-from-verb", ""),
                    group("Participle", "participle", ""),
                    mutated(),
                ],
            )
            .citation("Lemma VerbalNoun")
            .initial(2),
        ];
        s
    }

    fn keep_entry(&self, e: &Entry) -> bool {
        // Entries for conjunct forms (·ber) carry only a mutation table.
        !e.forms.iter().any(|f| has(&f.tags, "canonical") && f.form.starts_with('·'))
    }

    fn prepare(&self, e: &mut Entry) {
        // Headword lines carry no canonical form; the title keeps its length
        // marks (cú, mór), which the plain key drops.
        if !e.forms.iter().any(|f| has(&f.tags, "canonical")) {
            e.forms.push(Form { form: e.word.clone(), tags: vec!["canonical".into()], source: None });
        }
        let mut seen: Vec<(Vec<String>, bool)> = Vec::new();
        let mut table_done = false;
        for f in e.forms.iter_mut() {
            let src = f.source.as_deref().unwrap_or("");
            if src == "mutation" {
                // radical, lenition, then nasalization, unlabeled.
                if !has(&f.tags, "mutation") {
                    f.tags.clear();
                } else if has(&f.tags, "error-unrecognized-form") {
                    f.tags = tags("mutation nasalization");
                }
                continue;
            }
            if e.pos == "adj" && (src == "declension" || src == "inflection") {
                // Plural columns come as "feminine neuter singular" with an
                // unrecognized marker, or with no number at all.
                let t = &mut f.tags;
                if !has(t, "class") && !has(t, "table-tags") && !has(t, "inflection-template") {
                    if has(t, "error-unrecognized-form") || (!has(t, "singular") && !has(t, "plural") && !has(t, "dual")) {
                        t.retain(|x| x != "error-unrecognized-form" && x != "singular");
                        t.push("plural".into());
                        if !has(t, "feminine") {
                            if has(t, "masculine") && (has(t, "genitive") || has(t, "dative")) {
                                t.extend(["feminine".to_string(), "neuter".to_string()]);
                            }
                            if !has(t, "masculine") {
                                t.extend(["masculine".to_string(), "feminine".to_string(), "neuter".to_string()]);
                            }
                        }
                    }
                }
                continue;
            }
            if e.pos != "verb" || (src != "conjugation" && src != "inflection") {
                continue;
            }
            let t = &mut f.tags;
            if has(t, "table-tags") || has(t, "inflection-template") {
                continue;
            }
            if has(t, "noun-from-verb") {
                *t = tags("noun-from-verb");
                table_done = true;
                continue;
            }
            if has(t, "participle") {
                *t = tags("participle");
                continue;
            }
            if table_done {
                // The verbal of necessity, mislabeled.
                t.clear();
                continue;
            }
            t.retain(|x| x != "error-unrecognized-form");
            if has(t, "imperative") {
                t.retain(|x| x != "past");
            }
            if !TENSES.iter().any(|x| has(t, x)) {
                t.push("imperfect".into());
            }
            // Within a row, absolute forms come first, then conjunct ones
            // (·beir), then relative ones (beres).
            let conjunct = f.form.starts_with('·');
            let mut key = t.clone();
            key.sort();
            if conjunct {
                t.push("conjunct".into());
                seen.push((key, true));
            } else if seen.iter().any(|s| s.0 == key && s.1) {
                t.push("relative".into());
                // not·beir: a relative with an infixed pronoun ranks after beres.
                if f.form.contains('·') {
                    t.push("rare".into());
                }
            }
        }
        // Reconstructed forms (*bindem).
        e.forms.retain(|f| !f.form.starts_with('*'));
    }

    fn split_form(&self, raw: &str) -> Vec<String> {
        vec![raw.strip_prefix("also ").unwrap_or(raw).to_string()]
    }

    fn retag(&self, pos: &PosSpec, _form: &str, tags: &mut Vec<String>) {
        if pos.name == "noun" && has(tags, "genitive") && !has(tags, "singular") && !has(tags, "plural") && !has(tags, "dual") {
            tags.push("singular".into());
        }
        if pos.name == "verb" && has(tags, "conjunct") && tags.len() == 1 {
            tags.extend(["present", "indicative", "active", "third-person", "singular"].map(String::from));
        }
    }

    fn samples(&self) -> Vec<&'static str> {
        vec!["fer", "ben", "mann", "abb", "beirid", "canaid", "mass", "trén", "glonnad", "scarbaid"]
    }

    fn readme(&self) -> String {
        r#"## Example

```rust
use old_irish::*;
assert_eq!(noun("fer", Case::Genitive, Number::Singular), "fir");
assert_eq!(noun("fer", Case::Accusative, Number::Plural), "firu");
println!("{:?}", noun_gender("ben"));
assert_eq!(verb("beirid", Tense::Present, Dependency::Conjunct, Subject::ThirdSingular), "·beir");
assert_eq!(verb("beirid", Tense::Future, Dependency::Absolute, Subject::ThirdSingular), "béraid");
println!("{}", adj("mass", Case::Nominative, Number::Plural, Gender::Feminine));
```

Verbs have absolute, conjunct (written with a leading ·) and relative
forms; the trainer tells them apart by their place in Wiktionary's tables,
which is sometimes wrong for relative forms. Length marks are optional on
input.
"#
        .into()
    }
}

pub fn language() -> Box<dyn Language> {
    Box::new(OldIrish)
}
