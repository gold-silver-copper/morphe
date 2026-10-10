use morphe_train::kaikki::{Entry, Form};
use morphe_train::spec::*;

pub struct Punjabi;

const PERSONS: [&str; 6] = [
    "first-person singular",
    "second-person singular",
    "third-person singular",
    "first-person plural",
    "second-person plural",
    "third-person plural",
];
/// Shahmukhi tables put the second person plural before the first.
const PERSONS_SHAHMUKHI: [&str; 6] = [
    "first-person singular",
    "second-person singular",
    "third-person singular",
    "second-person plural",
    "first-person plural",
    "third-person plural",
];
/// Tags that tell the cells of a finite row apart, or are noise.
const ROW_NOISE: [&str; 7] = [
    "first-person",
    "second-person",
    "third-person",
    "singular",
    "plural",
    "error-unrecognized-form",
    "non-aspectual",
];

impl Language for Punjabi {
    fn spec(&self) -> Spec {
        let mut s = Spec::new("punjabi", "Punjabi", "pa");
        // Shahmukhi harakat; Gurmukhi tippi and bindi are interchangeable,
        // and Shahmukhi ݨ and ࣇ are often typed as ن and ل.
        s.marks = ('\u{64B}'..='\u{65F}').chain(['\u{670}', '\u{640}']).collect();
        s.fold = vec![('ੰ', 'ਂ'), ('ݨ', 'ن'), ('ࣇ', 'ل'), ('ي', 'ی'), ('ى', 'ی'), ('ك', 'ک'), ('ه', 'ہ')];
        s.drop_tags.extend(tags("dated adjectival x-passive"));
        s.features = vec![
            feature(
                "Case",
                &[
                    ("Direct", "direct"),
                    ("Oblique", "oblique"),
                    ("Vocative", "vocative"),
                    ("Ablative", "ablative"),
                    ("Instrumental", "instrumental"),
                    ("Locative", "locative"),
                ],
            ),
            feature("AdjCase", &[("Direct", "direct"), ("Oblique", "oblique"), ("Vocative", "vocative")]),
            feature("Number", &[("Singular", "singular"), ("Plural", "plural")]),
            feature("Gender", &[("Masculine", "masculine !feminine"), ("Feminine", "feminine !masculine")]),
            feature(
                "Agreement",
                &[
                    ("MasculineSingular", "masculine singular !oblique"),
                    ("MasculineOblique", "masculine singular oblique"),
                    ("MasculinePlural", "masculine plural"),
                    ("FeminineSingular", "feminine singular"),
                    ("FemininePlural", "feminine plural"),
                ],
            ),
            feature(
                "Person",
                &[
                    ("FirstSingular", "first-person singular"),
                    ("SecondSingular", "second-person singular"),
                    ("ThirdSingular", "third-person singular"),
                    ("FirstPlural", "first-person plural"),
                    ("SecondPlural", "second-person plural"),
                    ("ThirdPlural", "third-person plural"),
                ],
            ),
            feature(
                "Participle",
                &[("Imperfective", "imperfective"), ("Perfective", "perfective"), ("Agentive", "agentive")],
            ),
            feature("InfinitiveCase", &[("Direct", "direct"), ("Oblique", "oblique"), ("Ablative", "ablative")]),
        ];
        s.pos = vec![
            pos("noun", "noun", vec![group("", "", "Case Number")])
                .inherent("Gender")
                .citation("DirectSingular DirectPlural")
                .head_last(),
            pos("adj", "adj", vec![group("", "", "AdjCase Gender Number")])
                .citation("DirectMasculineSingular DirectFeminineSingular")
                .head_last(),
            pos(
                "verb",
                "verb",
                vec![
                    group("Stem", "stem", ""),
                    group("Infinitive", "x-infinitive", "InfinitiveCase"),
                    group("Conjunctive", "conjunctive", ""),
                    group("Progressive", "progressive !x-finite", ""),
                    group("Participle", "participle !gerund !conjunctive", "Participle Agreement"),
                    group("Future", "x-finite future !x-synthetic", "Person Gender"),
                    group("SyntheticFuture", "x-finite x-synthetic", "Person"),
                    group("Subjunctive", "x-finite subjunctive", "Person"),
                    group("Imperative", "x-finite imperative", "Number"),
                    group("Jussive", "x-finite jussive", "Number"),
                ],
            )
            .head_last(),
            pos("pron", "pron", vec![group("", "", "AdjCase Gender? Number")]),
        ];
        s
    }

    /// Forms are read without Shahmukhi harakat, which some tables carry
    /// and others don't. Verb tables come in a Gurmukhi and a Shahmukhi
    /// layout, both largely untagged; see `gurmukhi_verb` and
    /// `shahmukhi_verb`.
    fn prepare(&self, e: &mut Entry) {
        let header = |f: &Form| f.tags.iter().any(|t| t == "table-tags" || t == "inflection-template");
        e.forms.retain(|f| f.source.is_none() || header(f) || !f.form.chars().any(|c| c.is_ascii_alphabetic()));
        for f in e.forms.iter_mut().filter(|f| !header(f)) {
            f.form.retain(|c| !(('\u{64B}'..='\u{65F}').contains(&c) || c == '\u{670}'));
        }
        if e.pos != "verb" {
            e.forms.retain(|f| f.source.as_deref() != Some("conjugation"));
            return;
        }
        let template = e.forms.iter().find(|f| f.tags.iter().any(|t| t == "inflection-template")).map(|f| f.form.clone());
        let forms = std::mem::take(&mut e.forms);
        e.forms = match template.as_deref() {
            Some("pa-conj") => verb_tables(forms, true),
            Some(t) if t.starts_with("pnb-conj") => verb_tables(forms, false),
            _ => forms.into_iter().filter(|f| f.source.is_none()).collect(),
        };
    }

    fn samples(&self) -> Vec<&'static str> {
        vec!["ਜਾਣਨਾ", "ਮੁੰਡਾ", "ਕੁੜੀ", "ਸੋਹਣਾ", "سوچنا", "پانی", "ਝਪਟਣਾ", "ਗੁਲਬਾ"]
    }

    fn readme(&self) -> String {
        r#"## Example

```rust
use punjabi::*;
println!("{}", noun("ਮੁੰਡਾ", Case::Oblique, Number::Plural)); // ਮੁੰਡਿਆਂ
println!("{:?}", noun_gender("ਕੁੜੀ")); // Feminine
println!("{}", adj("ਸੋਹਣਾ", AdjCase::Direct, Gender::Feminine, Number::Singular)); // ਸੋਹਣੀ
println!("{}", verb_future("ਜਾਣਨਾ", Person::FirstSingular, Gender::Masculine)); // ਜਾਣਾਂਗਾ
println!("{}", verb_subjunctive("ਜਾਣਨਾ", Person::ThirdPlural)); // ਜਾਣਨ
println!("{}", verb_participle("ਜਾਣਨਾ", Participle::Perfective, Agreement::FeminineSingular)); // ਜਾਣੀ
println!("{}", verb_future("سوچنا", Person::ThirdSingular, Gender::Masculine)); // سوچے گا
```

Words are written in Gurmukhi or Shahmukhi, as Wiktionary has them; each
script has its own entries. Nouns have six cases in both numbers and their
gender. Verbs have the stem, the infinitive (direct, oblique, ablative),
the conjunctive, the imperfective, perfective and agentive participles in
five agreement forms, the future, the synthetic -ਸ- future, the
subjunctive, the imperative and the jussive. Tenses built with auxiliaries
are left out. Shahmukhi input ignores harakat, and ن, ل stand for ݨ, ࣇ.
"#
        .into()
    }
}

fn with(f: &Form, t: &str) -> Form {
    Form { form: f.form.clone(), tags: tags(t), source: f.source.clone() }
}

fn has(f: &Form, t: &str) -> bool {
    f.tags.iter().any(|x| x == t)
}

/// The participle table, then the simple finite table; later tables are
/// periphrastic. In the finite table each row lists its persons in order
/// without tagging them (dialectal variants interleaved).
fn verb_tables(forms: Vec<Form>, gurmukhi: bool) -> Vec<Form> {
    let mut out = Vec::new();
    let mut table = 0;
    // Untagged nonfinite forms come in a fixed order.
    let (mut gerund, mut shahmukhi_seen) = (0usize, Vec::<String>::new());
    let mut rows: Vec<(String, Vec<Form>)> = Vec::new();
    for f in forms {
        if f.source.is_none() {
            out.push(f);
            continue;
        }
        if has(&f, "table-tags") {
            table += 1;
            continue;
        }
        if has(&f, "inflection-template") || f.form.contains(' ') && table == 2 && gurmukhi {
            continue;
        }
        let t = f.tags.iter().filter(|t| *t != "error-unrecognized-form").cloned().collect::<Vec<_>>().join(" ");
        match table {
            1 if gurmukhi => {
                if has(&f, "gerund") && has(&f, "active") && !has(&f, "passive") {
                    let case = ["direct", "oblique", "ablative"].get(gerund).copied().unwrap_or("table-tags");
                    gerund += 1;
                    out.push(with(&f, &format!("x-infinitive {case}")));
                } else if has(&f, "participle") && !has(&f, "gerund") {
                    let passive = has(&f, "imperfective")
                        && has(&f, "passive")
                        && (has(&f, "masculine") && has(&f, "active") || has(&f, "feminine") && !has(&f, "active"));
                    let t = if passive { format!("{t} x-passive") } else { t };
                    // The masculine oblique singular comes untagged after the direct.
                    let t = if has(&f, "masculine") && has(&f, "singular") && !has(&f, "direct") && !has(&f, "oblique") {
                        format!("{t} oblique")
                    } else {
                        t
                    };
                    out.push(with(&f, &t));
                } else if !has(&f, "gerund") || has(&f, "conjunctive") {
                    out.push(with(&f, &t));
                }
            }
            1 => {
                let error = has(&f, "error-unrecognized-form");
                if has(&f, "infinitive") {
                    out.push(with(&f, "x-infinitive direct"));
                } else if error && f.tags.len() == 1 {
                    out.push(with(&f, "x-infinitive oblique"));
                } else if error && has(&f, "participle") && !has(&f, "feminine") {
                    // Masculine singular, then plural (also the oblique singular).
                    let kind = ["imperfective", "perfective", "habitual"].into_iter().find(|k| has(&f, k)).unwrap_or("agentive");
                    let key = format!("{kind} {}", has(&f, "adjectival"));
                    let first = !shahmukhi_seen.contains(&key);
                    shahmukhi_seen.push(key);
                    if first {
                        out.push(with(&f, &format!("{t} {kind} masculine singular")));
                    } else {
                        out.push(with(&f, &format!("{t} {kind} masculine plural")));
                        out.push(with(&f, &format!("{t} {kind} masculine singular oblique")));
                    }
                } else if has(&f, "participle") && has(&f, "feminine") && !has(&f, "imperfective") && !has(&f, "perfective") {
                    out.push(with(&f, &format!("{t} agentive")));
                } else {
                    out.push(with(&f, &t));
                }
            }
            2 => {
                let mut key: Vec<&String> = f.tags.iter().filter(|t| !ROW_NOISE.contains(&t.as_str())).collect();
                key.sort();
                let key = key.into_iter().cloned().collect::<Vec<_>>().join(" ");
                if has(&f, "dialectal") {
                    continue;
                }
                match rows.last_mut() {
                    Some((k, row)) if *k == key => row.push(f),
                    _ => rows.push((key, vec![f])),
                }
            }
            _ => {}
        }
    }
    let mut seen: Vec<String> = Vec::new();
    for (key, row) in rows {
        if seen.contains(&key) {
            continue;
        }
        seen.push(key.clone());
        let k = |t: &str| key.split(' ').any(|x| x == t);
        let both = k("masculine") && k("feminine");
        let row_tags = if k("imperative") && k("polite") || k("jussive") {
            Some("jussive")
        } else if k("imperative") {
            Some("imperative")
        } else if k("subjunctive") {
            Some("subjunctive")
        } else if k("future") && k("indicative") && both {
            Some("future x-synthetic")
        } else if k("future") && k("indicative") {
            Some(if k("feminine") { "future feminine" } else { "future masculine" })
        } else {
            None
        };
        let Some(row_tags) = row_tags else { continue };
        // Shahmukhi's familiar imperative is the imperative, its polite one
        // the jussive.
        let persons = if gurmukhi { PERSONS } else { PERSONS_SHAHMUKHI };
        if row.len() == 6 {
            for (f, p) in row.iter().zip(persons) {
                out.push(with(f, &format!("x-finite {row_tags} {p}")));
            }
        } else if row.len() == 2 && (row_tags == "imperative" || row_tags == "jussive") {
            for (f, n) in row.iter().zip(["singular", "plural"]) {
                out.push(with(f, &format!("x-finite {row_tags} {n}")));
            }
        }
    }
    out
}

pub fn language() -> Box<dyn Language> {
    Box::new(Punjabi)
}
