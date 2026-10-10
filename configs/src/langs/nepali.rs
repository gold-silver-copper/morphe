use morphe_train::kaikki::{Entry, Form};
use morphe_train::spec::*;

pub struct Nepali;

/// The first and second persons of a finite row, in table order.
const SLOTS: [&str; 4] = ["first-person singular", "first-person plural", "second-person x-low", "second-person x-middle"];
/// Tags that don't tell finite rows apart.
const ROW_NOISE: [&str; 12] = [
    "error-unrecognized-form",
    "masculine",
    "feminine",
    "impolite",
    "polite",
    "deferential",
    "formal",
    "singular",
    "plural",
    "third-person",
    "first-person",
    "second-person",
];

impl Language for Nepali {
    fn spec(&self) -> Spec {
        let mut s = Spec::new("nepali", "Nepali", "ne");
        // Chandrabindu and anusvara are written interchangeably.
        s.fold = vec![('ँ', 'ं')];
        s.features = vec![
            feature(
                "Case",
                &[
                    ("Nominative", "nominative"),
                    ("Accusative", "accusative"),
                    ("Ergative", "ergative"),
                    ("Ablative", "ablative"),
                    ("Genitive", "genitive"),
                    ("Locative", "locative"),
                ],
            ),
            feature("Number", &[("Singular", "singular"), ("Plural", "plural")]),
            feature("Gender", &[("Masculine", "masculine"), ("Feminine", "feminine")]),
            feature(
                "Agreement",
                &[("MasculineSingular", "masculine singular"), ("FeminineSingular", "feminine singular"), ("Plural", "plural")],
            ),
            // तँ and ऊ are the low grade, तिमी and उनी the middle grade,
            // तपाईं the honorific.
            feature(
                "Person",
                &[
                    ("FirstSingular", "first-person singular"),
                    ("FirstPlural", "first-person plural"),
                    ("SecondLow", "second-person x-low"),
                    ("SecondMiddle", "second-person x-middle"),
                    ("ThirdLow", "third-person impolite"),
                    ("ThirdMiddle", "third-person polite"),
                    ("Honorific", "deferential"),
                ],
            ),
            feature(
                "Tense",
                &[
                    ("Present", "x-present"),
                    ("Past", "x-past"),
                    ("HabitualPast", "x-habitual"),
                    ("Evidential", "x-evidential"),
                    ("DefiniteFuture", "x-definite"),
                    ("ProbableFuture", "x-probable"),
                    ("Injunctive", "x-injunctive"),
                ],
            ),
            feature(
                "Imperfect",
                &[("Simultaneous", "x-simultaneous"), ("Progressive", "x-progressive"), ("Adjectival", "x-adjectival")],
            ),
        ];
        s.pos = vec![
            pos("noun", "noun", vec![group("", "", "Case Number")]).citation("NominativeSingular").head_last(),
            pos("adj", "adj", vec![group("", "direct", "Gender Number")])
                .citation("MasculineSingular FeminineSingular")
                .head_last(),
            pos(
                "verb",
                "verb",
                vec![
                    group("Stem", "stem", ""),
                    group("Infinitive", "x-infinitive", ""),
                    group("Purposive", "x-purposive", ""),
                    group("Passive", "passive !participle", ""),
                    group("Causative", "causative", ""),
                    group("Conjunctive", "conjunctive affirmative", ""),
                    group("PerfectParticiple", "perfect-i affirmative !formal", "Agreement"),
                    group("Perfect", "perfect-ii affirmative !formal", ""),
                    group("ImperfectParticiple", "x-imperfect", "Imperfect"),
                    group("Participle", "infinitive participle affirmative !formal", ""),
                    group("", "x-finite", "Tense Person Gender?"),
                    group("Imperative", "x-imperative", "Person[SecondLow,SecondMiddle,Honorific]"),
                ],
            )
            .citation("Infinitive PastThirdLowMasculine")
            .head_last(),
        ];
        s
    }

    fn prepare(&self, e: &mut Entry) {
        if e.pos == "noun" {
            noun_table(e);
        }
        let header = |f: &Form| f.tags.iter().any(|t| t == "table-tags" || t == "inflection-template");
        e.forms.retain(|f| f.source.is_none() || header(f) || !f.form.chars().any(|c| c.is_ascii_alphabetic() || ('\u{250}'..='\u{2FF}').contains(&c)));
        if e.pos == "verb" {
            let forms = std::mem::take(&mut e.forms);
            e.forms = verb_table(forms);
        }
    }

    fn samples(&self) -> Vec<&'static str> {
        vec!["तीर", "केटा", "राम्रो", "खानु", "गर्नु", "जानु", "झप्टनु", "गुल्बा"]
    }

    fn readme(&self) -> String {
        r#"## Example

```rust
use nepali::*;
println!("{}", noun("तीर", Case::Ergative, Number::Plural)); // तीरहरूले
println!("{}", adj("राम्रो", Gender::Feminine, Number::Singular)); // राम्री
println!("{}", verb("खानु", Tense::Present, Person::FirstSingular, Gender::Masculine)); // खान्छु
println!("{}", verb("खानु", Tense::Past, Person::ThirdLow, Gender::Feminine)); // खाई
println!("{}", verb("खानु", Tense::Present, Person::Honorific, Gender::Masculine)); // खानुहुन्छ
println!("{}", verb_imperative("खानु", Person::SecondMiddle)); // खाऊ
println!("{}", verb_perfect_participle("खानु", Agreement::FeminineSingular)); // खाएकी
```

Nouns have six cases in both numbers. Verbs have the stem, infinitives,
the derived passive and causative, the participles, and the affirmative
present, past, habitual past, evidential past, definite and probable
future and injunctive for seven persons (म; हामी; तँ; तिमी; ऊ; उनी;
तपाईं) and two genders, plus the imperative. Wiktionary's tables don't
tag the first and second persons; they are read by their order in the row.
Negative forms are left out.
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

/// The accusative/dative and instrumental/ergative rows come as a header
/// word followed by forms tagged only with a number.
fn noun_table(e: &mut Entry) {
    let mut case = "";
    for f in e.forms.iter_mut().filter(|f| f.source.is_some()) {
        match f.form.as_str() {
            "accusative" | "dative" => {
                case = "accusative";
                f.tags = tags("table-tags");
            }
            "instrumental" | "ergative" => {
                case = "ergative";
                f.tags = tags("table-tags");
            }
            _ if f.tags.len() == 1 && (has(f, "singular") || has(f, "plural")) && !case.is_empty() => f.tags.push(case.into()),
            _ => {}
        }
    }
}

fn verb_table(forms: Vec<Form>) -> Vec<Form> {
    let mut out = Vec::new();
    let mut table = 0;
    let (mut infinitive, mut imperfect) = (0usize, 0usize);
    let mut block: Vec<Form> = Vec::new();
    let mut seen: Vec<String> = Vec::new();
    for f in forms {
        if f.source.is_none() {
            out.push(f);
            continue;
        }
        if has(&f, "table-tags") {
            table += 1;
            continue;
        }
        if has(&f, "inflection-template") {
            continue;
        }
        if table == 1 {
            if has(&f, "infinitive") && !has(&f, "participle") {
                out.push(with(&f, if infinitive == 0 { "x-infinitive" } else { "x-purposive" }));
                infinitive += 1;
            } else if has(&f, "imperfect") && has(&f, "affirmative") && !has(&f, "formal") {
                let kind = ["x-simultaneous", "x-progressive", "x-adjectival"].get(imperfect).copied().unwrap_or("table-tags");
                imperfect += 1;
                out.push(with(&f, &format!("x-imperfect {kind}")));
            } else if has(&f, "stem") {
                out.push(Form { form: f.form.trim_end_matches('-').to_string(), ..f });
            } else {
                out.push(f);
            }
            continue;
        }
        // A finite block ends with its honorific form.
        let last = has(&f, "deferential");
        block.push(f);
        if last {
            finite_block(std::mem::take(&mut block), &mut seen, &mut out);
        }
    }
    out
}

/// One affirmative or negative block of a tense: first and second persons
/// untagged (each one form for both genders, or masculine then feminine
/// forms), then the third persons and the honorific, tagged. Only the
/// first block of each tense (the affirmative) is read.
fn finite_block(block: Vec<Form>, seen: &mut Vec<String>, out: &mut Vec<Form>) {
    let Some(last) = block.last() else { return };
    let mut key: Vec<&String> = last.tags.iter().filter(|t| !ROW_NOISE.contains(&t.as_str())).collect();
    key.sort();
    let key = key.into_iter().cloned().collect::<Vec<_>>().join(" ");
    if seen.contains(&key) {
        return;
    }
    seen.push(key.clone());
    let k = |t: &str| key.split(' ').any(|x| x == t);
    let tense = if k("imperative") {
        "x-imperative"
    } else if k("injunctive") {
        "x-finite x-injunctive"
    } else if k("definite") {
        "x-finite x-definite"
    } else if k("indefinite") {
        "x-finite x-probable"
    } else if k("unknown") {
        "x-finite x-evidential"
    } else if k("habitual") {
        "x-finite x-habitual"
    } else if k("past") {
        "x-finite x-past"
    } else if k("present") {
        "x-finite x-present"
    } else {
        return;
    };
    let gender = |f: &Form| match (has(f, "masculine"), has(f, "feminine")) {
        (true, false) => "masculine",
        (false, true) => "feminine",
        _ => "masculine feminine",
    };
    let (untagged, tagged): (Vec<&Form>, Vec<&Form>) =
        block.iter().filter(|f| f.form != "-").partition(|f| !has(f, "third-person") && !has(f, "deferential"));
    for f in tagged {
        let who = if has(f, "deferential") {
            "deferential"
        } else if has(f, "impolite") {
            "third-person impolite"
        } else {
            "third-person polite"
        };
        out.push(with(f, &format!("{tense} {who} {}", gender(f))));
    }
    if tense == "x-imperative" {
        for (i, f) in untagged.iter().enumerate() {
            let who = if i == 0 { "second-person x-low" } else { "second-person x-middle" };
            out.push(with(f, &format!("{tense} {who}")));
        }
        return;
    }
    // Each slot is one form for both genders, or a run of masculine forms
    // and a run of feminine ones.
    let mut slots: Vec<Vec<&Form>> = Vec::new();
    let mut i = 0;
    while let Some(f) = untagged.get(i) {
        let mut slot = vec![*f];
        i += 1;
        if gender(f) != "masculine feminine" {
            let mut phase = gender(f);
            while let Some(g) = untagged.get(i) {
                let gg = gender(g);
                if gg == "masculine feminine" || (phase == "feminine" && gg == "masculine") {
                    break;
                }
                phase = gg;
                slot.push(*g);
                i += 1;
            }
        }
        slots.push(slot);
    }
    if slots.len() != SLOTS.len() {
        // Ambiguous alternatives: only the first persons are certain.
        slots.truncate(if slots.len() > SLOTS.len() { 2 } else { 0 });
    }
    for (slot, who) in slots.iter().zip(SLOTS) {
        for f in slot {
            out.push(with(f, &format!("{tense} {who} {}", gender(f))));
        }
    }
}

pub fn language() -> Box<dyn Language> {
    Box::new(Nepali)
}
