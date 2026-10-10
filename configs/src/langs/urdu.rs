use morphe_train::kaikki::{Entry, Form};
use morphe_train::spec::*;

pub struct Urdu;

/// The finite rows of the conjugation table name tense and gender but not
/// the person: their forms come in a fixed order, one per person column
/// (میں; تو، یہ، وہ; ہم، یہ، وہ، آپ; تم). Each index lists the persons it
/// stands for.
const PERSON_COLUMNS: [&[&str]; 4] = [
    &["first-person singular"],
    &["second-person singular", "third-person singular"],
    &["first-person plural", "third-person plural", "second-person plural formal"],
    &["second-person plural"],
];
/// The imperative rows: intimate, familiar, formal.
const ADDRESS_COLUMNS: [&str; 3] = ["second-person singular", "second-person plural", "second-person plural formal"];

impl Language for Urdu {
    fn spec(&self) -> Spec {
        let mut s = Spec::new("urdu", "Urdu", "ur");
        // Harakat, the superscript alef, the noon ghunna mark and tatweel.
        s.marks = ('\u{64B}'..='\u{65F}').chain(['\u{670}', '\u{640}']).collect();
        // Arabic letters typed for the Urdu ones.
        s.fold = vec![('ي', 'ی'), ('ى', 'ی'), ('ك', 'ک'), ('ه', 'ہ')];
        s.drop_tags.extend(tags("dated poetic"));
        s.features = vec![
            feature("Case", &[("Direct", "direct"), ("Oblique", "oblique"), ("Vocative", "vocative")]),
            feature(
                "PronCase",
                &[
                    ("Direct", "direct"),
                    ("Indirect", "indirect"),
                    ("Ergative", "ergative"),
                    ("Dative", "dative"),
                    ("Genitive", "genitive"),
                ],
            ),
            feature("Number", &[("Singular", "singular"), ("Plural", "plural")]),
            feature("Gender", &[("Masculine", "masculine !feminine"), ("Feminine", "feminine !masculine")]),
            feature(
                "Agreement",
                &[
                    ("MasculineSingular", "masculine singular"),
                    ("MasculinePlural", "masculine plural"),
                    ("FeminineSingular", "feminine singular"),
                    ("FemininePlural", "feminine plural"),
                ],
            ),
            // The six persons; تو، تم and آپ are the intimate, familiar and
            // formal second persons.
            feature(
                "Person",
                &[
                    ("FirstSingular", "first-person singular"),
                    ("SecondIntimate", "second-person singular"),
                    ("ThirdSingular", "third-person singular"),
                    ("FirstPlural", "first-person plural"),
                    ("SecondFamiliar", "second-person plural !formal"),
                    ("SecondFormal", "second-person plural formal"),
                    ("ThirdPlural", "third-person plural"),
                ],
            ),
            feature(
                "Address",
                &[
                    ("Intimate", "second-person singular"),
                    ("Familiar", "second-person plural !formal"),
                    ("Formal", "second-person plural formal"),
                ],
            ),
            feature(
                "Participle",
                &[("Habitual", "habitual"), ("Perfective", "perfective"), ("Agentive", "agentive prospective")],
            ),
        ];
        s.pos = vec![
            pos("noun", "noun", vec![group("", "", "Case Number")])
                .inherent("Gender")
                .citation("DirectSingular DirectPlural")
                .head_last(),
            pos("adj", "adj", vec![group("", "", "Case Gender? Number")])
                .citation("DirectMasculineSingular DirectFeminineSingular DirectMasculinePlural")
                .head_last(),
            pos(
                "verb",
                "verb",
                vec![
                    group("Stem", "stem", ""),
                    group("Infinitive", "infinitive !oblique !masculine !feminine", ""),
                    group("InfinitiveOblique", "infinitive oblique", ""),
                    group("InfinitiveAgreeing", "infinitive", "Agreement"),
                    group("Conjunctive", "conjunctive", ""),
                    group("Progressive", "progressive", ""),
                    group("Participle", "!infinitive !indicative !subjunctive", "Participle Agreement"),
                    group("Future", "future indicative", "Person Gender"),
                    group("Subjunctive", "subjunctive", "Person"),
                    group("Imperative", "imperative present", "Address"),
                    group("FutureImperative", "imperative future", "Address"),
                ],
            )
            .head_last(),
            pos(
                "pron",
                "pron",
                vec![group("", "!masculine !feminine", "PronCase Number?"), group("Possessive", "", "Case Gender Number")],
            ),
            pos("num", "num", vec![group("", "", "Case Number")]),
            pos("det", "det", vec![group("", "", "Case Gender? Number")]),
        ];
        s
    }

    /// Nouns and adjectives call the oblique "indirect" in some tables.
    fn retag(&self, p: &PosSpec, _form: &str, t: &mut Vec<String>) {
        if p.name != "pron" {
            for x in t.iter_mut() {
                if x == "indirect" {
                    *x = "oblique".into();
                }
            }
        }
    }

    /// Forms are read without harakat, which Wiktionary writes on some
    /// tables and not others. Other parts of speech drop conjugation tables
    /// (verbal nouns carry them) and read a headword plural as the direct
    /// plural. Verb tables come in two layouts, read by `auto_table` and
    /// `person_table`.
    fn prepare(&self, e: &mut Entry) {
        // Romanized duplicates are interleaved in some tables.
        let header = |f: &Form| f.tags.iter().any(|t| t == "table-tags" || t == "inflection-template");
        e.forms.retain(|f| f.source.is_none() || header(f) || !f.form.chars().any(|c| c.is_ascii_alphabetic()));
        for f in &mut e.forms {
            f.form.retain(|c| !is_mark(c));
        }
        if e.pos != "verb" {
            e.forms.retain(|f| f.source.as_deref() != Some("conjugation"));
            for f in &mut e.forms {
                if f.source.is_none() && f.tags.iter().any(|t| t == "plural") && f.tags.len() <= 3 {
                    f.tags.retain(|t| t != "formal");
                    f.tags.push("direct".into());
                }
            }
            return;
        }
        let forms = std::mem::take(&mut e.forms);
        let auto = forms.iter().any(|f| f.tags.iter().any(|t| t == "inflection-template") && f.form == "ur-conj-auto");
        e.forms = if auto { auto_table(forms) } else { person_table(forms) };
    }

    /// Arabic-script lists use the Arabic comma.
    fn split_form(&self, raw: &str) -> Vec<String> {
        raw.split('،').map(|x| x.trim().to_string()).collect()
    }

    fn samples(&self) -> Vec<&'static str> {
        vec!["لڑکا", "کتاب", "پانی", "کالا", "پینا", "کرنا", "میں", "گلبار", "جھپٹانا"]
    }

    fn readme(&self) -> String {
        r#"## Example

```rust
use urdu::*;
println!("{}", noun("لڑکا", Case::Oblique, Number::Plural)); // لڑکوں
println!("{:?}", noun_gender("کتاب")); // Feminine
println!("{}", adj("کالا", Case::Direct, Gender::Feminine, Number::Singular)); // کالی
println!("{}", verb_future("پینا", Person::FirstSingular, Gender::Masculine)); // پیوں گا
println!("{}", verb_subjunctive("پینا", Person::SecondFamiliar)); // پیو
println!("{}", verb_participle("پینا", Participle::Habitual, Agreement::FeminineSingular)); // پیتی
println!("{}", verb_imperative("پینا", Address::Familiar)); // پیو
```

Nouns have the direct, oblique and vocative in both numbers; adjectives
agree in gender too. Verbs have the stem, the infinitive and its oblique and
agreeing forms, the conjunctive, the habitual, perfective and agentive
participles, the simple future, the subjunctive and the imperatives. Tenses
built with auxiliaries (پیتا ہوں، پی رہا تھا) are a participle plus a form
of ہونا and are left out. Forms are written without harakat, as in everyday
Urdu, and input matching ignores them.
"#
        .into()
    }
}

fn is_mark(c: char) -> bool {
    ('\u{64B}'..='\u{65F}').contains(&c) || c == '\u{670}' || c == '\u{640}'
}

fn with(f: &Form, form: &str, t: &str) -> Form {
    Form { form: form.into(), tags: tags(t), source: f.source.clone() }
}

/// `ur-conj-auto`: the participle table is tagged; the finite table lists
/// each row's forms by person column without saying the person. Only the
/// one-word subjunctive, the simple future and the imperatives are read from
/// it; the other rows are participles plus an auxiliary.
fn auto_table(forms: Vec<Form>) -> Vec<Form> {
    let mut out: Vec<Form> = Vec::new();
    let mut tables = 0;
    let mut seen: Vec<Vec<String>> = Vec::new();
    let mut i = 0;
    while let Some(f) = forms.get(i) {
        if f.source.is_none() {
            out.push(f.clone());
            i += 1;
            continue;
        }
        if f.tags.iter().any(|t| t == "table-tags") {
            tables += 1;
        }
        if tables < 2 {
            out.push(f.clone());
            i += 1;
            continue;
        }
        // A run of forms with the same tags is one row.
        let mut row: Vec<&Form> = Vec::new();
        while let Some(g) = forms.get(i).filter(|g| g.tags == f.tags) {
            row.push(g);
            i += 1;
        }
        let mut key = f.tags.clone();
        key.sort();
        let first = !seen.contains(&key);
        seen.push(key);
        let has = |t: &str| f.tags.iter().any(|x| x == t);
        let simple = first && (has("subjunctive") && has("error-unrecognized-form") || has("future") && has("indicative") || has("imperative"));
        if !simple {
            continue;
        }
        let base = f.tags.iter().filter(|t| *t != "error-unrecognized-form").cloned().collect::<Vec<_>>().join(" ");
        if has("imperative") && row.len() == 3 {
            for (g, who) in row.iter().zip(ADDRESS_COLUMNS) {
                out.push(with(g, &g.form, &format!("{base} {who}")));
            }
        } else if !has("imperative") && row.len() == 4 {
            for (g, whos) in row.iter().zip(PERSON_COLUMNS) {
                for who in whos {
                    out.push(with(g, &g.form, &format!("{base} {who}")));
                }
            }
        }
    }
    out
}

/// `ur-conj`, `ur-conj-v`, `ur-conj-y`: sections headed "imperfective",
/// "perfective" and "future", persons tagged but written with their
/// pronouns (مَیں کَرُوں). The first section's three untagged forms are the
/// imperatives; the fourth section is the subjunctive; later sections are
/// periphrastic.
fn person_table(forms: Vec<Form>) -> Vec<Form> {
    let mut out: Vec<Form> = Vec::new();
    let mut section = 0usize;
    let mut name = String::new();
    let mut imperative = 0usize;
    for f in forms {
        if f.source.is_none() {
            out.push(f);
            continue;
        }
        let has = |t: &str| f.tags.iter().any(|x| x == t);
        if has("table-tags") {
            section += 1;
            name = f.form.clone();
            continue;
        }
        if has("inflection-template") {
            continue;
        }
        let words: Vec<&str> = f.form.split_whitespace().collect();
        let last = |n: usize| words.get(words.len().saturating_sub(n)..).map(|w| w.join(" ")).unwrap_or_default();
        let t = f.tags.join(" ");
        let new = match (section, name.as_str()) {
            (1, _) if has("error-unrecognized-form") => {
                let who = ADDRESS_COLUMNS.get(imperative).copied().unwrap_or("table-tags");
                imperative += 1;
                // The intimate imperative is the bare stem.
                if imperative == 1 {
                    out.push(with(&f, &f.form, "stem"));
                }
                with(&f, &f.form, &format!("imperative present {who}"))
            }
            (1, _) if has("direct") => with(&f, &f.form, "infinitive"),
            (1, _) if has("indirect") => with(&f, &f.form, "infinitive oblique"),
            (1, _) if has("agentive") => with(&f, &f.form, "agentive prospective masculine singular"),
            (1, _) => f.clone(),
            (_, "imperfective") => with(&f, &f.form, &format!("habitual {t}")),
            (_, "perfective") => with(&f, &f.form, &format!("perfective {t}")),
            (4, "no-table-tags") => with(&f, &last(1), &format!("subjunctive {t}")),
            (_, "future") => with(&f, &last(2), &format!("future indicative {t}")),
            _ => continue,
        };
        // آپ takes the third person plural.
        if has("third-person") && has("plural") {
            let formal = new.tags.join(" ").replace("third-person", "second-person formal");
            out.push(with(&f, &new.form, &formal));
        }
        out.push(new);
    }
    out
}

pub fn language() -> Box<dyn Language> {
    Box::new(Urdu)
}
