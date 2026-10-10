use morphe_train::kaikki::{Entry, Form};
use morphe_train::spec::*;

pub struct Marathi;

/// The conjugation table (`mr-conj`) is the same grid for every verb, and
/// Kaikki tags its cells only with a gender, so its finite forms are read
/// by position: (index among the tagged forms, tags). Merged cells stand
/// for several persons.
const VERB_CELLS: &[(usize, &str)] = &[
    (0, "x-infinitive"),
    (1, "x-infinitive colloquial"),
    (3, "x-completive"),
    (4, "x-prospective"),
    (5, "x-hortative"),
    (6, "x-desiderative"),
    (9, "x-obligative neuter singular"),
    (10, "x-obligative neuter singular colloquial"),
    (11, "x-obligative neuter plural"),
    (13, "x-obligative masculine singular"),
    (14, "x-obligative masculine plural"),
    (16, "x-obligative feminine singular"),
    (17, "x-obligative feminine plural"),
    (20, "x-past third-person neuter singular"),
    (21, "x-past third-person neuter singular colloquial"),
    (22, "x-past third-person neuter plural"),
    (23, "x-past first-person masculine singular"),
    (24, "x-past second-person masculine singular"),
    (25, "x-past third-person masculine singular"),
    (26, "x-past second-person masculine plural"),
    (27, "x-past third-person masculine plural"),
    (28, "x-past first-person feminine singular"),
    (29, "x-past second-person feminine singular"),
    (30, "x-past third-person feminine singular"),
    (31, "x-past first-person masculine plural"),
    (31, "x-past first-person feminine plural"),
    (33, "x-past second-person feminine plural"),
    (34, "x-past third-person feminine plural"),
    (49, "x-present third-person neuter singular"),
    (50, "x-present third-person neuter singular colloquial"),
    (51, "x-present third-person neuter plural"),
    (52, "x-present first-person masculine singular"),
    (52, "x-present third-person masculine singular"),
    (53, "x-present second-person masculine singular"),
    (54, "x-present second-person masculine plural"),
    (55, "x-present third-person masculine plural"),
    (56, "x-present first-person feminine singular"),
    (56, "x-present third-person feminine singular"),
    (57, "x-present second-person feminine singular"),
    (58, "x-present first-person masculine plural"),
    (58, "x-present first-person feminine plural"),
    (59, "x-present second-person feminine plural"),
    (60, "x-present third-person feminine plural"),
];
/// The tagged forms of an `mr-conj` table.
const VERB_TABLE_LEN: usize = 107;

impl Language for Marathi {
    fn spec(&self) -> Spec {
        let mut s = Spec::new("marathi", "Marathi", "mr");
        // Anusvara and chandrabindu are written interchangeably.
        s.fold = vec![('ँ', 'ं')];
        s.soft_tags.extend(tags("colloquial"));
        s.features = vec![
            feature(
                "Case",
                &[
                    ("Direct", "direct"),
                    ("Oblique", "oblique"),
                    ("Dative", "dative"),
                    ("Ergative", "ergative"),
                    ("Instrumental", "instrumental"),
                    ("Locative", "locative"),
                    ("Genitive", "genitive"),
                    ("Vocative", "vocative"),
                ],
            ),
            feature("Number", &[("Singular", "singular"), ("Plural", "plural")]),
            feature("Gender", &[("Masculine", "masculine"), ("Feminine", "feminine"), ("Neuter", "neuter")]),
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
        ];
        s.pos = vec![
            pos("noun", "noun", vec![group("", "", "Case Number")])
                .inherent("Gender")
                .citation("DirectSingular ObliqueSingular")
                .lemma_cell("DirectSingular")
                .head_last(),
            pos(
                "verb",
                "verb",
                vec![
                    group("Stem", "x-stem", ""),
                    group("Infinitive", "x-infinitive", ""),
                    group("ObliqueInfinitive", "x-oblique-infinitive", ""),
                    group("Completive", "x-completive", ""),
                    group("Prospective", "x-prospective", ""),
                    group("Hortative", "x-hortative", ""),
                    group("Desiderative", "x-desiderative", ""),
                    group("Progressive", "x-progressive", ""),
                    group("ObliquePerfective", "x-oblique-perfective", ""),
                    group("Obligative", "x-obligative", "Gender Number"),
                    group("Past", "x-past", "Person Gender"),
                    group("Present", "x-present", "Person Gender"),
                ],
            )
            .head_last(),
        ];
        s
    }

    /// Kaikki can't tag the Marathi tables: nouns are read by their endings
    /// (see `noun_table`), verbs by position. Nouns get their gender from
    /// the template's name.
    fn prepare(&self, e: &mut Entry) {
        let template = e.forms.iter().find(|f| f.tags.iter().any(|t| t == "inflection-template")).map(|f| f.form.clone());
        let forms = std::mem::take(&mut e.forms);
        let (head, table): (Vec<Form>, Vec<Form>) = forms.into_iter().partition(|f| f.source.is_none());
        let mut out: Vec<Form> = head.into_iter().filter(|f| f.tags.iter().any(|t| t == "canonical")).collect();
        match e.pos.as_str() {
            "noun" => {
                let gender = template.as_deref().and_then(|t| t.rsplit('-').next()).and_then(|g| match g {
                    "m" => Some("masculine"),
                    "f" => Some("feminine"),
                    "n" => Some("neuter"),
                    _ => None,
                });
                if let Some(g) = gender {
                    out.push(Form { form: e.word.clone(), tags: tags(&format!("canonical {g}")), source: None });
                }
                out.extend(noun_table(&table));
            }
            "verb" if template.as_deref() == Some("mr-conj") => out.extend(verb_table(&table)),
            _ => {}
        }
        e.forms = out;
    }

    fn samples(&self) -> Vec<&'static str> {
        vec!["विश्व", "वृक्ष", "आत्मा", "करणे", "जाणे", "हंबरणे", "झपटणे", "गुलबा"]
    }

    fn readme(&self) -> String {
        r#"## Example

```rust
use marathi::*;
println!("{}", noun("वृक्ष", Case::Dative, Number::Plural)); // वृक्षांना
println!("{}", noun("आत्मा", Case::Oblique, Number::Singular)); // आत्म्या
println!("{:?}", noun_gender("विश्व")); // Neuter
println!("{}", verb_past("जाणे", Person::FirstSingular, Gender::Masculine)); // गेलो
println!("{}", verb_present("करणे", Person::SecondSingular, Gender::Feminine)); // करतेस
println!("{}", verb_obligative("करणे", Gender::Masculine, Number::Singular)); // करावा
println!("{}", verb_completive("करणे")); // करून
```

Nouns have the direct, oblique, dative, ergative, instrumental, locative,
genitive and vocative in both numbers, and their gender. Verbs have the
stem, the infinitives, the completive (-ऊन), prospective (-णार),
hortative, desiderative (-आयला) and progressive forms, the obligative
(-आवा) agreeing with its object, and the simple past and habitual present
by person and gender. Tenses built with auxiliaries are left out.
"#
        .into()
    }
}

fn with(form: &str, t: &str) -> Form {
    Form { form: form.trim_end_matches('-').to_string(), tags: tags(t), source: Some("declension".into()) }
}

/// The declension tables list direct forms, the oblique singular and
/// plural stems (the plural one ends in a hyphen), then the case forms,
/// each a stem plus an ending; colloquial and archaic variants are
/// interleaved. A second table has the genitives.
fn noun_table(forms: &[Form]) -> Vec<Form> {
    let mut out = Vec::new();
    let mut tables: Vec<Vec<&Form>> = Vec::new();
    for f in forms {
        if f.tags.iter().any(|t| t == "table-tags") {
            tables.push(Vec::new());
        } else if f.tags.iter().any(|t| t == "error-unrecognized-form" || t == "ergative" || t == "instrumental")
            && !f.tags.iter().any(|t| t == "archaic")
        {
            if let Some(t) = tables.last_mut() {
                t.push(f);
            }
        }
    }
    let Some(first) = tables.first() else { return out };
    let Some(pl_at) = first.iter().rposition(|f| f.form.ends_with('-')) else { return out };
    let Some(pl) = first.get(pl_at).map(|f| f.form.trim_end_matches('-')) else { return out };
    let sg_at = pl_at.saturating_sub(1);
    if pl_at == 0 {
        return out;
    }
    for (i, f) in first.iter().enumerate() {
        let form = f.form.trim_end_matches('-');
        let number = if i > pl_at && form.starts_with(pl) { "plural" } else { "singular" };
        let t = if i < sg_at {
            if i == 0 { "direct singular" } else { "direct plural" }
        } else if i == sg_at {
            "oblique singular"
        } else if i == pl_at {
            "oblique plural"
        } else if form.ends_with("नो") {
            "vocative plural"
        } else if form.ends_with("ने") || form.ends_with("नं") || form.ends_with("नी") {
            if number == "plural" { "ergative plural" } else { "ergative singular" }
        } else if form.ends_with("ला") || form.ends_with("ना") {
            if number == "plural" { "dative plural" } else { "dative singular" }
        } else if form.ends_with("शी") {
            if number == "plural" { "instrumental plural" } else { "instrumental singular" }
        } else if form.ends_with('त') {
            if number == "plural" { "locative plural" } else { "locative singular" }
        } else if form.ends_with('स') || form.ends_with("ते") {
            continue;
        } else {
            "vocative singular"
        };
        out.push(with(form, t));
        // A consonant-stem masculine has one direct form for both numbers.
        if i == 0 && sg_at == 1 {
            out.push(with(form, "direct plural"));
        }
    }
    if let Some(genitives) = tables.get(1) {
        if let Some(f) = genitives.first() {
            out.push(with(&f.form, "genitive singular"));
        }
        if let Some(f) = genitives.iter().find(|f| f.form.starts_with(pl)) {
            out.push(with(&f.form, "genitive plural"));
        }
    }
    out
}

/// The untagged nonfinite forms follow their row labels; the finite ones
/// are read by position.
fn verb_table(forms: &[Form]) -> Vec<Form> {
    let mut out = Vec::new();
    let label = |l: &str, t: &str, out: &mut Vec<Form>| {
        if let Some(i) = forms.iter().position(|f| f.form == l) {
            if let Some(f) = forms.get(i + 1) {
                out.push(with(&f.form, t));
            }
        }
    };
    label("धातू", "x-stem", &mut out);
    label("Oblique Infinitive", "x-oblique-infinitive", &mut out);
    label("Progressive", "x-progressive", &mut out);
    label("Oblique Perfective", "x-oblique-perfective", &mut out);
    let skip = ["+", "-", "Affirmative", "होकारार्थी", "Negative", "नकारार्थी", "m f n"];
    let cells: Vec<&Form> = forms
        .iter()
        .filter(|f| !f.tags.is_empty())
        .filter(|f| !f.tags.iter().any(|t| t == "table-tags" || t == "inflection-template"))
        .filter(|f| !skip.contains(&f.form.as_str()))
        .collect();
    if cells.len() != VERB_TABLE_LEN {
        return out;
    }
    for &(i, t) in VERB_CELLS {
        if let Some(f) = cells.get(i) {
            out.push(with(&f.form, t));
        }
    }
    out
}

pub fn language() -> Box<dyn Language> {
    Box::new(Marathi)
}
