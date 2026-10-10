use morphe_train::kaikki::{Entry, Form};
use morphe_train::spec::*;
use std::collections::HashMap;

pub struct Icelandic;

const CASES: [&str; 4] = ["nominative", "accusative", "dative", "genitive"];
const GENDERS: [&str; 3] = ["masculine", "feminine", "neuter"];

fn has(f: &Form, t: &str) -> bool {
    f.tags.iter().any(|x| x == t)
}

fn with(f: &Form, add: &[&str]) -> Form {
    let mut tags = f.tags.clone();
    tags.extend(add.iter().map(|t| t.to_string()));
    Form { form: f.form.clone(), tags, source: f.source.clone() }
}

/// Kaikki reads the adjective and participle tables without the number
/// (or, for weak forms, the case) of each column. They come in a fixed
/// order, so the n-th repeat of a tag set says which column it is.
fn retag_tables(e: &mut Entry) {
    let mut seen: HashMap<Vec<String>, usize> = HashMap::new();
    let mut table = 0;
    let mut out = Vec::new();
    for f in std::mem::take(&mut e.forms) {
        if f.source.is_none() {
            out.push(f);
            continue;
        }
        if has(&f, "table-tags") {
            table += 1;
            out.push(f);
            continue;
        }
        let mut key = f.tags.clone();
        key.sort();
        let n = *seen.entry(key.clone()).and_modify(|n| *n += 1).or_insert(0);
        let case = CASES.iter().any(|c| has(&f, c));
        let gender = GENDERS.iter().any(|g| has(&f, g));
        if e.pos == "verb" {
            if case && gender {
                // Past participle: strong columns, then weak.
                out.push(with(&f, &["participle", "past", if n == 0 { "strong" } else { "weak" }]));
            } else if has(&f, "error-unrecognized-form") && !has(&f, "singular") && !has(&f, "plural") {
                // Each conjugation table opens with infinitive, supine and
                // present participle; the second one is mediopassive.
                let mut g = Form { form: f.form.trim_start_matches("að ").to_string(), tags: Vec::new(), source: f.source.clone() };
                g.tags = match (table, n % 3) {
                    (1, 0) => tags("infinitive"),
                    (1, 2) => tags("participle present"),
                    (2, 0) => tags("infinitive mediopassive"),
                    _ => continue,
                };
                out.push(g);
            }
            continue;
        }
        let mut f = f;
        f.tags.retain(|t| t != "error-unrecognized-form");
        let weak = has(&f, "weak");
        if has(&f, "strong") {
            if gender {
                out.push(with(&f, &[if n == 0 { "singular" } else { "plural" }]));
            } else {
                // Plural dative and genitive: one form for all genders.
                out.extend(GENDERS.iter().map(|g| with(&f, &[g, "plural"])));
            }
        } else if weak && has(&f, "plural") {
            for c in CASES {
                out.extend(GENDERS.iter().map(|g| with(&f, &[c, g])));
            }
        } else if weak && case {
            out.push(with(&f, &["singular"]));
        } else if weak && gender {
            // Weak singular oblique cases share a form; so do all cases of a
            // comparative.
            let cases: &[&str] = if has(&f, "comparative") { &CASES } else { &CASES[1..] };
            out.extend(cases.iter().map(|c| with(&f, &[c, "singular"])));
        } else {
            out.push(f);
        }
    }
    e.forms = out;
}

impl Language for Icelandic {
    fn spec(&self) -> Spec {
        let mut s = Spec::new("icelandic", "Icelandic", "is");
        s.drop_tags.extend(tags("dated literary"));
        s.features = vec![
            feature("Definiteness", &[("Indefinite", "indefinite"), ("Definite", "definite")]),
            feature(
                "Case",
                &[("Nominative", "nominative"), ("Accusative", "accusative"), ("Dative", "dative"), ("Genitive", "genitive")],
            ),
            feature("Number", &[("Singular", "singular"), ("Plural", "plural")]),
            feature("Gender", &[("Masculine", "masculine"), ("Feminine", "feminine"), ("Neuter", "neuter")]),
            feature(
                "Degree",
                &[("Positive", "!comparative !superlative"), ("Comparative", "comparative"), ("Superlative", "superlative")],
            ),
        ];
        s.pos = vec![
            pos("noun", "noun", vec![group("", "", "Definiteness Case Number")])
                .inherent("Gender")
                .citation("IndefiniteNominativeSingular IndefiniteGenitiveSingular IndefiniteNominativePlural")
                .head_last(),
            pos(
                "adj",
                "adj",
                vec![
                    group("Strong", "strong", "Degree[Positive,Superlative] Case Gender Number"),
                    group("Weak", "weak", "Degree Case Gender Number"),
                ],
            ),
            pos(
                "verb",
                "verb",
                vec![
                    group("Infinitive", "infinitive !mediopassive", ""),
                    group("InfinitiveMediopassive", "infinitive mediopassive", ""),
                    group("Supine", "supine", ""),
                    // Only strong verbs list the plural, too few to learn from.
                    group("Past", "indicative past singular third-person", ""),
                    group("ParticiplePresent", "participle present", ""),
                    group("ParticiplePastStrong", "participle past strong", "Case Gender Number"),
                    group("ParticiplePastWeak", "participle past weak", "Case Gender Number"),
                ],
            )
            .citation("Lemma Past Supine")
            // Most verbs have only their participle table.
            .lemma_cell("Infinitive")
            .head_first(),
        ];
        s
    }

    fn prepare(&self, e: &mut Entry) {
        if e.pos == "adj" || e.pos == "verb" {
            retag_tables(e);
        }
    }

    fn samples(&self) -> Vec<&'static str> {
        vec!["hestur", "kona", "barn", "kalla", "stór", "góður", "flumpa", "snorkinn"]
    }

    fn readme(&self) -> String {
        r#"## Example

```rust
use icelandic::*;
assert_eq!(noun("hestur", Definiteness::Definite, Case::Dative, Number::Plural), "hestunum");
println!("{:?}", noun_gender("kona"));
assert_eq!(adj_strong("stór", Degree::Positive, Case::Accusative, Gender::Masculine, Number::Singular), "stóran");
assert_eq!(verb_past("kalla"), "kallaði");
assert_eq!(verb_supine("kalla"), "kallað");
// Citations pin principal parts: nominative, genitive, plural.
println!("{}", noun("maður, manns, menn", Definiteness::Indefinite, Case::Dative, Number::Plural));
```

Wiktionary's Icelandic conjugation tables don't say which form is which
person and tense, so the only finite verb form covered is the third-person
singular past. Personal pronouns are not covered.
"#
        .into()
    }
}

pub fn language() -> Box<dyn Language> {
    Box::new(Icelandic)
}
