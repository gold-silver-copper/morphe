use morphe_train::kaikki::{Entry, Form};
use morphe_train::spec::*;

pub struct Sanskrit;

/// Vedic pitch accents: udātta and anudātta.
const ACCENTS: &[char] = &['\u{951}', '\u{952}'];

/// Words of a conjugation table's summary row ("निन्दति [present]").
const TENSES: &[&str] = &["present", "imperfect", "future", "conditional", "aorist", "perfect"];

fn devanagari(s: &str) -> bool {
    s.chars().all(|c| matches!(c, '\u{900}'..='\u{97f}' | '\u{1cd0}'..='\u{1cff}' | '\u{a8e0}'..='\u{a8ff}' | ' ' | '-'))
}

fn has(f: &Form, t: &str) -> bool {
    f.tags.iter().any(|x| x == t)
}

impl Language for Sanskrit {
    fn spec(&self) -> Spec {
        let mut s = Spec::new("sanskrit", "Sanskrit", "sa");
        // Accented input is accepted; output is unaccented (see `prepare`).
        s.marks = ACCENTS.to_vec();
        // Vedic-only forms and moods.
        s.drop_tags.extend(tags("Vedic Epic subjunctive injunctive"));
        s.features = vec![
            feature(
                "Case",
                &[
                    ("Nominative", "nominative"),
                    ("Accusative", "accusative"),
                    ("Instrumental", "instrumental"),
                    ("Dative", "dative"),
                    ("Ablative", "ablative"),
                    ("Genitive", "genitive"),
                    ("Locative", "locative"),
                    ("Vocative", "vocative"),
                ],
            ),
            feature("Number", &[("Singular", "singular"), ("Dual", "dual"), ("Plural", "plural")]),
            feature("Gender", &[("Masculine", "masculine"), ("Feminine", "feminine"), ("Neuter", "neuter")]),
            feature(
                "Tense",
                &[
                    ("Present", "present"),
                    ("Imperfect", "imperfect"),
                    ("Future", "future"),
                    ("Conditional", "conditional"),
                    ("Aorist", "aorist"),
                    ("Perfect", "perfect"),
                ],
            ),
            feature("Person", &[("First", "first-person"), ("Second", "second-person"), ("Third", "third-person")]),
            feature("Voice", &[("Active", "active"), ("Middle", "middle-voice"), ("Passive", "passive")]),
        ];
        let finite = "!participle !infinitive !gerund";
        s.pos = vec![
            pos("noun", "noun", vec![group("", "", "Case Number")]).inherent("Gender").head_last(),
            pos("adj", "adj", vec![group("", "", "Case Gender Number")]),
            pos(
                "verb",
                "verb",
                vec![
                    group("Indicative", &format!("indicative {finite}"), "Tense Voice[Active,Middle] Person Number"),
                    group("Imperative", &format!("imperative present {finite}"), "Voice[Active,Middle] Person Number"),
                    group("Optative", &format!("optative present {finite}"), "Voice[Active,Middle] Person Number"),
                    group("Benedictive", &format!("benedictive {finite}"), "Voice[Active,Middle] Person Number"),
                    group("Participle", "participle !past", "Tense[Present,Future,Perfect] Voice[Active,Middle]").declines("adj", ""),
                    group("PastParticiple", "participle past", "Voice[Active,Passive]").declines("adj", ""),
                    group("Infinitive", "infinitive", ""),
                    group("Gerund", "gerund", ""),
                ],
            )
            // Tense stems are lexical (aorist type, perfect reduplication):
            // a tense Wiktionary doesn't give can't be guessed from the
            // present, and many verbs lack the active or middle.
            .missing_is_absent()
            .head_first(),
            pos("pron", "pron", vec![group("", "", "Case Gender? Number")]),
            pos("det", "det", vec![group("", "", "Case Gender? Number")]),
            pos("num", "num", vec![group("", "", "Case Gender? Number")]),
        ];
        s
    }

    fn keep_entry(&self, e: &Entry) -> bool {
        devanagari(&e.word)
    }

    /// Kaikki's Sanskrit tables need help:
    /// - a conjugation table names its tense only in a summary row
    ///   ("निन्दति [present]"); the precative table has none;
    /// - the optative comes labelled as a second imperative block;
    /// - the non-finite table marks its forms "error-unrecognized-form";
    /// - an adjective's tables are masculine, feminine, neuter in turn,
    ///   without gender tags;
    /// - footnoted forms (a trailing digit) are Vedic;
    /// - the canonical form reads "विश्व ॑ stem or".
    fn prepare(&self, e: &mut Entry) {
        // Split forms into tables at each table-tags row.
        let mut tables: Vec<Vec<Form>> = vec![Vec::new()];
        let mut head = Vec::new();
        for f in std::mem::take(&mut e.forms) {
            if f.source.is_none() {
                head.push(f);
            } else if has(&f, "table-tags") {
                tables.push(vec![f]);
            } else if let Some(t) = tables.last_mut() {
                t.push(f);
            }
        }
        let templates: Vec<String> = tables
            .iter()
            .map(|t| t.iter().find(|f| has(f, "inflection-template")).map(|f| f.form.clone()).unwrap_or_default())
            .collect();
        let adj_tables: Vec<usize> = (0..tables.len()).filter(|&i| templates[i].starts_with("sa-decl-adj")).collect();
        for (ti, table) in tables.iter_mut().enumerate() {
            let template = &templates[ti];
            // Gender of a declension table.
            let gender = if let Some(g) = template.strip_prefix("sa-decl-noun-") {
                match g {
                    "m" => Some("masculine"),
                    "f" => Some("feminine"),
                    "n" => Some("neuter"),
                    _ => None,
                }
            } else if let Some(k) = adj_tables.iter().position(|&i| i == ti) {
                let n = adj_tables.len();
                Some(if n % 3 == 0 {
                    ["masculine", "feminine", "neuter"][k % 3]
                } else if k == 0 {
                    "masculine"
                } else if k + 1 == n {
                    "neuter"
                } else {
                    "feminine"
                })
            } else {
                None
            };
            let conj = template == "sa-conj";
            let tense = table
                .iter()
                .find(|f| f.tags.len() == 1 && TENSES.contains(&f.tags[0].as_str()))
                .map(|f| f.tags[0].clone());
            let nonfinite = table.iter().any(|f| has(f, "error-unrecognized-form"));
            let mut seen: Vec<Vec<String>> = Vec::new();
            let mut prev: Vec<String> = Vec::new();
            let mut optative = false;
            for f in table.iter_mut() {
                if has(f, "table-tags") || has(f, "inflection-template") || has(f, "class") {
                    continue;
                }
                let tags = f.tags.clone();
                if let Some(g) = gender {
                    if !f.tags.iter().any(|t| matches!(t.as_str(), "masculine" | "feminine" | "neuter")) {
                        f.tags.push(g.into());
                    }
                }
                if !conj {
                    continue;
                }
                if nonfinite {
                    f.tags.retain(|t| t != "error-unrecognized-form");
                    // Past participles: -ta/-na passive, -tavat/-navat active.
                    let plain: String = f.form.chars().filter(|c| !ACCENTS.contains(c)).collect();
                    if has(f, "participle") {
                        if plain.ends_with("वत्") {
                            f.tags.extend(["past".into(), "active".into()]);
                        } else if plain.ends_with('त') || plain.ends_with('न') {
                            f.tags.extend(["past".into(), "passive".into()]);
                        } else {
                            f.tags.push("gerundive".into());
                        }
                    }
                    continue;
                }
                if has(f, "imperative") {
                    if tags != prev && seen.contains(&tags) {
                        optative = true;
                    }
                    if optative {
                        for t in f.tags.iter_mut().filter(|t| *t == "imperative") {
                            *t = "optative".into();
                        }
                    }
                    seen.push(tags.clone());
                } else {
                    optative = false;
                }
                prev = tags;
                match &tense {
                    Some(t) => f.tags.push(t.clone()),
                    None => f.tags.push("benedictive".into()),
                }
            }
        }
        for mut f in head {
            if has(&f, "canonical") {
                // "निन्द॑ति third-singular indicative", "विश्व ॑ stem or"
                f.form = f.form.split_whitespace().take_while(|w| devanagari(w)).collect();
            }
            e.forms.push(f);
        }
        for f in tables.into_iter().flatten() {
            let footnote = f.form.chars().any(|c| c.is_ascii_digit() || matches!(c, '¹' | '²' | '³'));
            if !footnote && devanagari(&f.form) || has(&f, "table-tags") || has(&f, "inflection-template") {
                e.forms.push(f);
            }
        }
        // Vedic accents are marked on some entries and not others, and
        // Classical Sanskrit doesn't write them: train without them.
        for f in &mut e.forms {
            f.form.retain(|c| !ACCENTS.contains(&c));
        }
    }

    fn samples(&self) -> Vec<&'static str> {
        vec!["देव", "सेना", "नामन्", "भवति", "लभते", "प्रिय", "कपोत", "झिम्पति"]
    }

    fn readme(&self) -> String {
        r#"## Example

```rust
use sanskrit::*;
println!("{}", noun("देव", Case::Instrumental, Number::Plural)); // देवैः
println!("{:?}", noun_gender("सेना"));
println!("{}", adj("प्रिय", Case::Nominative, Gender::Feminine, Number::Singular)); // प्रिया
println!("{}", verb_indicative("भवति", Tense::Imperfect, Voice::Active, Person::Third, Number::Singular)); // अभवत्
println!("{}", verb_optative("निन्दति", Voice::Active, Person::Third, Number::Singular));
println!("{}", verb_past_participle_declined("गच्छति", Voice::Passive, Case::Nominative, Gender::Feminine, Number::Singular));
println!("{}", noun("अ॒ग्नि", Case::Genitive, Number::Singular)); // accents in input are ignored
```

Classical Sanskrit in Devanagari, as Wiktionary lemmatizes it: nouns and
adjectives by stem, verbs by the third-person singular present, personal
pronouns by stem (म, त्व). Output is unaccented; accented input (udātta ॑,
anudātta ॒) is accepted. Vedic-only forms, the subjunctive and the injunctive
are left out. The precative (benedictive) is its own function. Wiktionary
gives many verbs only their present-system tables; other tenses of those
verbs are left empty rather than guessed. Unknown verbs get a whole
predicted paradigm.
"#
        .into()
    }
}

pub fn language() -> Box<dyn Language> {
    Box::new(Sanskrit)
}
