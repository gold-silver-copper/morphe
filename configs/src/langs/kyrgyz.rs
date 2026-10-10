use morphe_train::kaikki::Entry;
use morphe_train::spec::*;

pub struct Kyrgyz;

/// The finite blocks of Wiktionary's Kyrgyz conjugation table, in order.
/// Each block lists the persons; negative blocks follow their positive.
const BLOCKS: [&str; 27] = [
    "past",
    "past",
    "perfect",
    "perfect",
    "pluperfect",
    "pluperfect",
    "evidential",
    "evidential",
    "habitual",
    "habitual",
    "past-habitual",
    "past-habitual",
    "not-yet",
    "aorist",
    "aorist",
    "future",
    "future",
    "hortative",
    "hortative",
    "future-in-past",
    "future-in-past",
    "aorist-in-past",
    "aorist-in-past",
    "intentional",
    "intentional",
    "conditional",
    "conditional",
];

/// Whether a second-person form is the formal (сиз) one: its last word ends
/// in -ңыз, -сыз (… -дар).
fn formal(form: &str) -> bool {
    let w: Vec<char> = form.split_whitespace().last().unwrap_or("").chars().collect();
    let mut n = w.len();
    if n >= 3 && w.get(n - 3) == Some(&'д') && w.get(n - 1) == Some(&'р') {
        n -= 3;
    }
    n >= 3
        && w.get(n - 1) == Some(&'з')
        && w.get(n - 2).is_some_and(|c| "ыиуү".contains(*c))
        && w.get(n - 3).is_some_and(|c| "ңс".contains(*c))
}

const MARKERS: [&str; 7] = ["error-unrecognized-form", "colloquial", "nonstandard", "proscribed", "singular", "plural", "second-person"];

impl Language for Kyrgyz {
    fn spec(&self) -> Spec {
        let mut s = Spec::new("kyrgyz", "Kyrgyz", "ky");
        s.features = vec![
            feature(
                "Case",
                &[
                    ("Nominative", "nominative"),
                    ("Genitive", "genitive"),
                    ("Dative", "dative"),
                    ("Accusative", "accusative"),
                    ("Locative", "locative"),
                    ("Ablative", "ablative"),
                ],
            ),
            feature("Number", &[("Singular", "singular"), ("Plural", "plural")]),
            feature(
                "Person",
                &[
                    ("First", "first-person"),
                    ("Second", "second-person informal"),
                    ("SecondFormal", "second-person formal"),
                    ("Third", "third-person"),
                ],
            ),
            feature(
                "Tense",
                &[
                    ("Past", "t-past"),
                    ("Perfect", "t-perfect"),
                    ("Pluperfect", "t-pluperfect"),
                    ("Evidential", "t-evidential"),
                    ("Habitual", "t-habitual"),
                    ("PastHabitual", "t-past-habitual"),
                    ("NotYet", "t-not-yet"),
                    ("Aorist", "t-aorist"),
                    ("Future", "t-future"),
                    ("Hortative", "t-hortative"),
                    ("FutureInPast", "t-future-in-past"),
                    ("AoristInPast", "t-aorist-in-past"),
                    ("Intentional", "t-intentional"),
                    ("Conditional", "t-conditional"),
                ],
            ),
            feature("Polarity", &[("Positive", "!negative"), ("Negative", "negative")]),
            feature("ParticipleTense", &[("Past", "past"), ("Future", "future")]),
            feature("Converb", &[("Perfective", "perfective"), ("Imperfective", "imperfective")]),
        ];
        let negated = "Tense[Past,Perfect,Pluperfect,Evidential,Habitual,PastHabitual,Aorist,Future,Hortative,FutureInPast,AoristInPast,Intentional,Conditional] Person Number";
        s.pos = vec![
            pos(
                "noun",
                "noun",
                vec![group("", "!possessive", "Case Number"), group("Possessive", "possessive", "Person Case Number")],
            )
            .citation("NominativeSingular NominativePlural")
            .head_last(),
            pos(
                "verb",
                "verb",
                vec![
                    group("Infinitive", "infinitive", "Polarity"),
                    group("", "finite !negative", "Tense Person Number"),
                    group("Negative", "finite negative", negated),
                    group("Participle", "participle", "ParticipleTense Polarity"),
                    group("Converb", "converb !negative", "Converb"),
                    group("NegativeConverb", "converb negative", ""),
                ],
            )
            .citation("Infinitive PastThirdSingular AoristThirdSingular")
            .head_last(),
        ];
        s
    }

    /// Kaikki leaves most Kyrgyz table cells half-tagged: possessed noun
    /// forms lack their number and the third-person possessor, verb forms
    /// lack the first and third person and the tense names, and the
    /// negative non-finite forms are untagged. Rebuild the tags from the
    /// table's fixed layout.
    fn prepare(&self, e: &mut Entry) {
        // Romanized rows repeat some cells in Latin script.
        e.forms.retain(|f| f.source.is_none() || f.form.chars().all(|c| !c.is_alphabetic() || ('\u{400}'..='\u{4ff}').contains(&c)));
        if e.pos == "noun" {
            for f in e.forms.iter_mut().filter(|f| f.source.is_some()) {
                let has = |t: &str| f.tags.iter().any(|x| x == t);
                let third = has("error-unrecognized-form");
                if !(third || has("first-person") || has("second-person")) {
                    continue;
                }
                f.tags.retain(|t| t != "error-unrecognized-form");
                f.tags.push("possessive".into());
                if third {
                    f.tags.push("third-person".into());
                } else if !f.tags.iter().any(|t| t == "singular") {
                    f.tags.push("plural".into());
                }
            }
            return;
        }
        if e.pos != "verb" {
            return;
        }
        let mut block: Option<usize> = None;
        let mut norm: Vec<String> = Vec::new();
        let (mut seen_pl, mut seen_sg2, mut seen_pl2) = (false, false, false);
        let mut last_nonfinite = String::new();
        let mut finite: Vec<(usize, usize)> = Vec::new();
        for (i, f) in e.forms.iter_mut().enumerate().filter(|(_, f)| f.source.as_deref() == Some("conjugation")) {
            let has = |t: &str| f.tags.iter().any(|x| x == t);
            if has("table-tags") || has("inflection-template") {
                continue;
            }
            let person_tags = has("singular") || has("plural");
            if !person_tags {
                // Non-finite forms; an untagged one negates the one before.
                let new = if has("infinitive") {
                    "infinitive".to_string()
                } else if has("participle") && has("past") {
                    "participle past".into()
                } else if has("participle") && has("future") {
                    "participle future".into()
                } else if has("perfective") {
                    "converb perfective".into()
                } else if has("imperfective") {
                    "converb imperfective".into()
                } else if has("error-unrecognized-form") {
                    match last_nonfinite.split_whitespace().next() {
                        Some("converb") => "converb negative".into(),
                        Some(_) => format!("{last_nonfinite} negative"),
                        None => "table-tags".into(),
                    }
                } else {
                    "table-tags".into()
                };
                if !new.contains("negative") && new != "table-tags" {
                    last_nonfinite = new.clone();
                }
                let soft = has("proscribed");
                f.tags = tags(&new);
                if soft {
                    f.tags.push("proscribed".into());
                }
                continue;
            }
            let sg = has("singular");
            let second = has("second-person");
            let n: Vec<String> = f.tags.iter().filter(|t| !MARKERS.contains(&t.as_str())).cloned().collect();
            let main = !has("colloquial") && !has("nonstandard") && !has("proscribed");
            if block.is_none() || n != norm || (sg && !second && main && seen_pl) {
                block = Some(block.map_or(0, |b| b + 1));
                norm = n;
                (seen_pl, seen_sg2, seen_pl2) = (false, false, false);
            }
            let person = if second {
                if formal(&f.form) { "second-person formal" } else { "second-person informal" }
            } else if (sg && !seen_sg2) || (!sg && !seen_pl2) {
                "first-person"
            } else {
                "third-person"
            };
            match (sg, second) {
                (true, true) => seen_sg2 = true,
                (false, true) => seen_pl2 = true,
                (false, false) => seen_pl = true,
                _ => {}
            }
            let mut new = tags(person);
            new.push(if sg { "singular" } else { "plural" }.into());
            for t in ["negative", "colloquial", "nonstandard", "proscribed"] {
                if has(t) {
                    new.push(t.into());
                }
            }
            f.tags = new;
            finite.push((i, block.unwrap_or(0)));
        }
        let ok = block == Some(BLOCKS.len() - 1);
        for (i, b) in finite {
            if let Some(f) = e.forms.get_mut(i) {
                match BLOCKS.get(b).filter(|_| ok) {
                    Some(t) => {
                        f.tags.push("finite".into());
                        f.tags.push(format!("t-{t}"));
                    }
                    None => f.tags = tags("table-tags"),
                }
            }
        }
    }

    fn samples(&self) -> Vec<&'static str> {
        vec!["уу", "китеп", "бала", "үй", "өлүү", "жазуу", "келүү", "шырпыл", "кыбыроо"]
    }

    fn readme(&self) -> String {
        r#"## Example

```rust
use kyrgyz::*;
println!("{}", noun("китеп", Case::Dative, Number::Singular)); // китепке
println!("{}", noun("үй", Case::Locative, Number::Plural)); // үйлөрдө
println!("{}", noun_possessive("үй", Person::First, Case::Ablative, Number::Singular)); // үйүмдөн
println!("{}", verb("жазуу", Tense::Past, Person::First, Number::Singular)); // жаздым
println!("{}", verb_negative("жазуу", Tense::Aorist, Person::Third, Number::Singular)); // жазбайт
```

Nouns have six cases in both numbers, and the possessed forms for the
first, second (informal and formal) and third person. Verbs have fourteen
tenses and moods (past, perfect, pluperfect, evidential, habitual, aorist,
future, hortative, conditional and others), positive and negative, for
the first, second (informal and formal) and third person; plus the verbal
noun, the participles and the converbs. Analytic forms (жазган жокмун) are
listed as Wiktionary gives them.
"#
        .into()
    }
}

pub fn language() -> Box<dyn Language> {
    Box::new(Kyrgyz)
}
