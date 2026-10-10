use morphe_train::kaikki::{Entry, Form};
use morphe_train::spec::*;

pub struct Romanian;

const CASES: &str = "Case[Nominative,Accusative,Genitive,Dative]";

impl Language for Romanian {
    fn spec(&self) -> Spec {
        let mut s = Spec::new("romanian", "Romanian", "ro");
        // ş ţ (cedilla) are the old encoding of ș ț (comma).
        s.fold = vec![('ş', 'ș'), ('ţ', 'ț')];
        // Alternative spellings (including Cyrillic ones) are other words.
        s.drop_tags.extend(tags("alternative abbreviation diminutive augmentative dated"));
        s.soft_tags.extend(tags("poetic regional"));
        s.features = vec![
            feature(
                "Case",
                &[
                    ("Nominative", "nominative"),
                    ("Accusative", "accusative"),
                    ("Genitive", "genitive"),
                    ("Dative", "dative"),
                    ("Vocative", "vocative"),
                ],
            ),
            feature("Definiteness", &[("Indefinite", "indefinite"), ("Definite", "definite")]),
            feature("Number", &[("Singular", "!plural"), ("Plural", "plural")]),
            feature("Gender", &[("Masculine", "masculine"), ("Feminine", "feminine"), ("Neuter", "neuter")]),
            feature("Person", &[("First", "first-person"), ("Second", "second-person"), ("Third", "third-person")]),
            feature(
                "Tense",
                &[
                    ("Present", "present"),
                    ("Imperfect", "imperfect"),
                    ("SimplePerfect", "perfect"),
                    ("Pluperfect", "pluperfect"),
                ],
            ),
        ];
        // Headword lines give principal parts without the mood tag.
        let indicative = "!subjunctive !imperative !infinitive !participle !gerund";
        let nominal = "!possessive";
        s.pos = vec![
            pos(
                "noun",
                "noun",
                vec![
                    group("", "!vocative", &format!("{CASES} Definiteness Number")),
                    group("Vocative", "vocative", "Number"),
                    group("Counterpart", "!plural !nominative !genitive !accusative !dative !vocative", "Gender[Masculine,Feminine]"),
                ],
            )
            .inherent("Gender")
            .citation("NominativeIndefiniteSingular NominativeIndefinitePlural")
            .head_first(),
            pos("adj", "adj", vec![group("", "", &format!("{CASES} Definiteness Gender Number"))])
                .citation("NominativeIndefiniteMasculineSingular NominativeIndefiniteFeminineSingular"),
            pos(
                "verb",
                "verb",
                vec![
                    group("Infinitive", "infinitive", ""),
                    group("Gerund", "gerund", ""),
                    group("Participle", "participle past", "").declines("adj", ""),
                    group("Indicative", indicative, "Tense Person Number"),
                    group("Subjunctive", "subjunctive present", "Person Number"),
                    group("Imperative", "imperative !negative", "Number"),
                    group("ImperativeNegative", "imperative negative", "Number"),
                ],
            )
            .citation("Infinitive IndicativePresentThirdSingular Participle")
            .head_first(),
            pos("pron", "pron", vec![group("", nominal, &format!("{CASES} Gender? Number"))]),
            pos("det", "det", vec![group("", nominal, &format!("{CASES} Definiteness? Gender? Number"))]),
            pos("article", "article", vec![group("", "", &format!("{CASES} Definiteness? Gender? Number"))]),
            pos("num", "num", vec![group("", "", "Gender? Number")]),
        ];
        s
    }

    fn prepare(&self, e: &mut Entry) {
        if e.pos == "adj" || e.pos == "det" {
            adjective_cases(e);
        }
    }

    fn split_form(&self, raw: &str) -> Vec<String> {
        // Gender counterparts are written "equivalent absolutistă".
        vec![raw.strip_prefix("equivalent ").unwrap_or(raw).to_string()]
    }

    fn keep_entry(&self, e: &Entry) -> bool {
        e.word.chars().any(char::is_alphabetic) && !e.word.chars().any(|c| c.is_ascii_digit())
    }

    fn adjust(&self, pos: &PosSpec, lemma: &str, forms: &mut Vec<(usize, String, bool)>) {
        for f in forms.iter_mut() {
            // Particles of the infinitive, subjunctive and negative imperative.
            if pos.name == "verb" {
                for p in ["a ", "să ", "nu "] {
                    if let Some(rest) = f.1.strip_prefix(p) {
                        f.1 = rest.to_string();
                    }
                }
            }
            f.1 = written(&f.1);
        }
        forms.retain(|f| f.1.chars().any(char::is_alphabetic) && (lemma.contains(' ') || !f.1.contains(' ')));
    }

    fn samples(&self) -> Vec<&'static str> {
        vec!["casă", "om", "abate", "face", "frumos", "mare", "brumi", "flătură"]
    }

    fn readme(&self) -> String {
        r#"## Example

```rust
use romanian::*;
assert_eq!(noun("casă", Case::Nominative, Definiteness::Indefinite, Number::Plural), "case");
assert_eq!(noun("om", Case::Genitive, Definiteness::Definite, Number::Singular), "omului");
assert_eq!(noun_vocative("prieten", Number::Singular), "prietene");
assert_eq!(adj("frumos", Case::Nominative, Definiteness::Indefinite, Gender::Feminine, Number::Singular), "frumoasă");
assert_eq!(verb_indicative("face", Tense::Present, Person::First, Number::Singular), "fac");
assert_eq!(verb_subjunctive("face", Person::Third, Number::Singular), "facă");
println!("{:?}", noun_gender("tren"));
// Made-up words inflect by analogy:
println!("{}", noun("flătură", Case::Dative, Definiteness::Definite, Number::Plural));
```

Forms are given without their particles: the infinitive `face` (not
`a face`), the subjunctive `facă` (not `să facă`). Input written with
cedilla letters (ş ţ) is read as the comma letters (ș ț); output uses the
comma letters. Stress marks Wiktionary puts on a few forms are removed.
"#
        .into()
    }
}

/// Standard spelling: comma letters, no stress marks.
fn written(form: &str) -> String {
    form.chars()
        .map(|c| match c {
            'ş' => 'ș',
            'ţ' => 'ț',
            'á' => 'a',
            'é' => 'e',
            'í' => 'i',
            'ó' => 'o',
            'ú' => 'u',
            _ => c,
        })
        .collect()
}

/// Wiktionary's adjective table loses its case labels: its rows are the
/// nominative-accusative, then the genitive-dative, which leaves out the
/// indefinite forms equal to the nominative. Headword-only adjectives give
/// the indefinite nominative.
fn adjective_cases(e: &mut Entry) {
    const UNKNOWN: &str = "error-unrecognized-form";
    let has_table = e.forms.iter().any(|f| f.source.is_some() && f.tags.iter().any(|t| t == UNKNOWN));
    let mut seen: Vec<Vec<String>> = Vec::new();
    let mut out: Vec<Form> = Vec::new();
    let nom_acc = || ["nominative", "accusative"].map(String::from);
    let gen_dat = || ["genitive", "dative"].map(String::from);
    let mut lemma_listed = false;
    for f in std::mem::take(&mut e.forms) {
        if has_table && f.source.is_some() && f.tags.iter().any(|t| t == UNKNOWN) {
            let mut tags: Vec<String> = f.tags.iter().filter(|t| *t != UNKNOWN).cloned().collect();
            tags.sort();
            let again = seen.contains(&tags);
            if !again {
                seen.push(tags.clone());
            }
            tags.extend(if again { gen_dat() } else { nom_acc() });
            out.push(Form { form: f.form, tags, source: f.source });
        } else if !has_table && f.source.is_none() && f.tags.iter().any(|t| t == "singular" || t == "plural") {
            let mut tags = f.tags.clone();
            if tags.len() == 1 && tags.iter().any(|t| t == "plural") {
                tags.extend(["masculine", "feminine", "neuter"].map(String::from));
            }
            lemma_listed |= tags.iter().any(|t| t == "feminine") && tags.iter().any(|t| t == "singular");
            tags.push("indefinite".into());
            tags.extend(nom_acc());
            out.push(Form { form: f.form, tags, source: None });
        } else {
            out.push(f);
        }
    }
    if !has_table && out.iter().any(|f| f.tags.iter().any(|t| t == "indefinite")) {
        let mut tags: Vec<String> = ["masculine", "neuter", "singular", "indefinite"].map(String::from).to_vec();
        if !lemma_listed {
            tags.push("feminine".into());
        }
        tags.extend(nom_acc());
        out.push(Form { form: e.word.clone(), tags, source: None });
    }
    // Indefinite genitive-dative forms the table leaves out: as the
    // nominative, except the feminine singular, which is the plural form.
    let indefinite_nom = |g: &str, n: &str, out: &[Form]| -> Vec<Form> {
        out.iter()
            .filter(|f| {
                let has = |t: &str| f.tags.iter().any(|x| x == t);
                has("indefinite") && has("nominative") && has(g) && has(n)
            })
            .map(|f| Form { form: f.form.clone(), tags: Vec::new(), source: f.source.clone() })
            .collect()
    };
    let has_gen = |g: &str, n: &str, out: &[Form]| {
        out.iter().any(|f| {
            let has = |t: &str| f.tags.iter().any(|x| x == t);
            has("indefinite") && has("genitive") && has(g) && has(n)
        })
    };
    for (g, n, from_g, from_n) in [
        ("masculine", "singular", "masculine", "singular"),
        ("neuter", "singular", "neuter", "singular"),
        ("masculine", "plural", "masculine", "plural"),
        ("feminine", "plural", "feminine", "plural"),
        ("neuter", "plural", "neuter", "plural"),
        ("feminine", "singular", "feminine", "plural"),
    ] {
        if has_gen(g, n, &out) {
            continue;
        }
        for mut f in indefinite_nom(from_g, from_n, &out) {
            f.tags = [g, n, "indefinite"].map(String::from).into_iter().chain(gen_dat()).collect();
            out.push(f);
        }
    }
    e.forms = out;
}

pub fn language() -> Box<dyn Language> {
    Box::new(Romanian)
}
