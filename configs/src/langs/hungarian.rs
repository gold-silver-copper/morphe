use morphe_train::kaikki::{Entry, Form};
use morphe_train::spec::*;

pub struct Hungarian;

const CASES: &str = "Case";

impl Language for Hungarian {
    fn spec(&self) -> Spec {
        let mut s = Spec::new("hungarian", "Hungarian", "hu");
        s.soft_tags.extend(tags("rare dated poetic"));
        s.features = vec![
            feature(
                "Case",
                &[
                    ("Nominative", "nominative"),
                    ("Accusative", "accusative"),
                    ("Dative", "dative"),
                    ("Instrumental", "instrumental"),
                    ("CausalFinal", "causal-final"),
                    ("Translative", "translative"),
                    ("Terminative", "terminative"),
                    ("EssiveFormal", "essive-formal"),
                    ("Inessive", "inessive"),
                    ("Superessive", "superessive"),
                    ("Adessive", "adessive"),
                    ("Illative", "illative"),
                    ("Sublative", "sublative"),
                    ("Allative", "allative"),
                    ("Elative", "elative"),
                    ("Delative", "delative"),
                    ("Ablative", "ablative"),
                ],
            ),
            feature("Number", &[("Singular", "singular"), ("Plural", "plural")]),
            feature("Person", &[("First", "first-person"), ("Second", "second-person"), ("Third", "third-person")]),
            feature(
                "Possessor",
                &[
                    ("FirstSingular", "first-person singular"),
                    ("SecondSingular", "second-person singular"),
                    ("ThirdSingular", "third-person singular"),
                    ("FirstPlural", "first-person plural"),
                    ("SecondPlural", "second-person plural"),
                    ("ThirdPlural", "third-person plural"),
                ],
            ),
            feature("Possessed", &[("One", "possessed-single"), ("Many", "possessed-many")]),
            feature(
                "Mood",
                &[("Present", "present"), ("Past", "past"), ("Conditional", "conditional"), ("Subjunctive", "subjunctive")],
            ),
            feature("Definiteness", &[("Indefinite", "indefinite"), ("Definite", "definite")]),
            feature("Tense", &[("Present", "present"), ("Past", "past"), ("Future", "future")]),
        ];
        let nominal = || vec![group("", "!possessive", &format!("{CASES} Number"))];
        let mut noun = nominal();
        noun.push(group("Possessive", "possessive", "Possessor Possessed"));
        let mut adj = nominal();
        adj.push(group("Comparative", "comparative", ""));
        adj.push(group("Superlative", "superlative", ""));
        s.pos = vec![
            pos("noun", "noun", noun)
                .citation("NominativeSingular AccusativeSingular NominativePlural PossessiveThirdSingularOne")
                .head_last(),
            pos("adj", "adj", adj).citation("NominativeSingular AccusativeSingular NominativePlural"),
            pos(
                "verb",
                "verb",
                vec![
                    group("Infinitive", "infinitive !personal", ""),
                    group("", "finite", "Mood Definiteness Person Number"),
                    group("Lak", "lak", "Mood"),
                    group("PersonalInfinitive", "infinitive personal", "Person Number"),
                    group("Participle", "participle !adverbial", "Tense").declines("adj", ""),
                    group("AdverbialParticiple", "participle adverbial", ""),
                ],
            )
            .citation("PresentIndefiniteThirdSingular PresentIndefiniteSecondSingular PastIndefiniteThirdSingular")
            .head_first(),
            pos("pron", "pron", nominal()),
            pos("num", "num", nominal()),
        ];
        s
    }

    /// Wiktionary's Hungarian conjugation tables come out of the parser with
    /// every person label shifted by one column and the tense labels lost,
    /// so the table is read by position instead.
    fn prepare(&self, e: &mut Entry) {
        if e.pos != "verb" {
            return;
        }
        let table: Vec<&Form> = e.forms.iter().filter(|f| f.source.is_some()).collect();
        let retagged = read_conjugation(&table);
        e.forms.retain(|f| f.source.is_none());
        e.forms.extend(retagged.into_iter().map(|(form, t)| Form { form, tags: tags(&t), source: Some("conjugation".into()) }));
    }

    fn readme(&self) -> String {
        r#"## Example

```rust
use hungarian::*;
assert_eq!(noun("ház", Case::Inessive, Number::Singular), "házban");
println!("{}", noun("fa", Case::Accusative, Number::Plural)); // fákat
println!("{}", noun_possessive("kert", Possessor::FirstSingular, Possessed::One)); // kertem
println!("{}", verb("olvas", Mood::Present, Definiteness::Definite, Person::Third, Number::Singular)); // olvassa
println!("{}", verb_lak("lát", Mood::Present)); // látlak
println!("{}", adj_comparative("szép")); // szebb
// Citations pin principal parts: nominative, accusative, plural, possessed.
println!("{}", noun("ló, lovat", Case::Dative, Number::Plural)); // lovaknak
```

Cells cover the seventeen productive cases in both numbers, the possessed
forms (in the nominative), the indefinite and definite conjugations of the
present, past, conditional and subjunctive (the imperative is the
subjunctive), the `-lak/-lek` forms (I → you), the infinitive and its
personal forms, and the participles. Wiktionary's verb tables are read by
position; archaic tenses and the potential (`-hat/-het`) table are left
out. Verbs with an irregular table layout (`van`, `jön`…) are learned from
whatever parts fit and predicted for the rest.
"#
        .into()
    }
}

/// A table cell: its forms (variants) and whether it sits in a definite row.
struct Slot {
    forms: Vec<String>,
    definite: bool,
    tags: Vec<String>,
}

enum Token {
    Slot(Slot),
    Note(String),
}

const PERSONS: [&str; 6] = [
    "first-person singular",
    "second-person singular",
    "third-person singular",
    "first-person plural",
    "second-person plural",
    "third-person plural",
];

/// The first conjugation table of a verb, as cells in reading order.
fn tokens(forms: &[&Form]) -> Vec<Token> {
    let mut out: Vec<Token> = Vec::new();
    let mut tables = 0;
    let mut join_next = false;
    for f in forms {
        let has = |t: &str| f.tags.iter().any(|x| x == t);
        if has("table-tags") {
            tables += 1;
            continue;
        }
        if tables != 1 || has("inflection-template") || has("class") {
            continue;
        }
        // The formal-you column repeats the third-person cell.
        if has("second-person-semantically") || has("formal") {
            continue;
        }
        let mut text = f.form.trim();
        if let Some(i) = text.find(" (") {
            text = text.get(..i).unwrap_or(text);
        }
        let text = text.trim_end_matches('*');
        let (alt, text) = match text.strip_prefix("or ") {
            Some(t) => (true, t),
            None => (false, text),
        };
        let (more, text) = match text.strip_suffix(" or") {
            Some(t) => (true, t),
            None => (false, text),
        };
        if text.contains([' ', '.', ',', '/']) || text.is_empty() {
            out.push(Token::Note(text.to_string()));
            join_next = false;
            continue;
        }
        let joined = alt || join_next;
        join_next = more;
        if joined {
            if let Some(Token::Slot(s)) = out.last_mut() {
                if !s.forms.iter().any(|x| x == text) {
                    s.forms.push(text.to_string());
                }
                continue;
            }
        }
        // Consecutive repeats are the same cell read twice.
        if let Some(Token::Slot(s)) = out.last() {
            if s.forms.len() == 1 && s.forms.first().is_some_and(|x| x == text) && s.definite == has("definite") {
                continue;
            }
        }
        out.push(Token::Slot(Slot { forms: vec![text.to_string()], definite: has("definite"), tags: f.tags.clone() }));
    }
    out
}

/// One finite tense: six indefinite cells, six definite cells, and the
/// `-lak/-lek` form, when present.
struct Section<'a> {
    indef: Vec<&'a Slot>,
    def: Vec<&'a Slot>,
    lak: Option<&'a Slot>,
}

fn is_lak(s: &Slot) -> bool {
    !s.definite && s.forms.first().is_some_and(|f| f.ends_with("lak") || f.ends_with("lek"))
}

/// Splits a run of slots (between notes) into sections.
fn sections<'a>(run: &[&'a Slot]) -> Vec<Section<'a>> {
    let mut out: Vec<Section<'a>> = Vec::new();
    let mut cur = Section { indef: Vec::new(), def: Vec::new(), lak: None };
    for &s in run {
        if s.definite {
            cur.def.push(s);
        } else if !cur.def.is_empty() && cur.lak.is_none() && is_lak(s) {
            cur.lak = Some(s);
        } else if cur.lak.is_some() && is_lak(s) {
            // A second -lak cell is an archaic variant.
        } else if !cur.def.is_empty() || cur.lak.is_some() {
            out.push(std::mem::replace(&mut cur, Section { indef: vec![s], def: Vec::new(), lak: None }));
        } else {
            cur.indef.push(s);
        }
    }
    if !cur.indef.is_empty() || !cur.def.is_empty() {
        out.push(cur);
    }
    out
}

fn read_conjugation(forms: &[&Form]) -> Vec<(String, String)> {
    let toks = tokens(forms);
    // Runs of slots between notes, with the note that ends each run.
    let mut runs: Vec<(Vec<&Slot>, String)> = vec![(Vec::new(), String::new())];
    for t in &toks {
        match t {
            Token::Slot(s) => {
                if let Some(r) = runs.last_mut() {
                    r.0.push(s);
                }
            }
            Token::Note(n) => {
                if let Some(r) = runs.last_mut() {
                    if r.0.is_empty() {
                        continue;
                    }
                    r.1 = n.clone();
                }
                runs.push((Vec::new(), String::new()));
            }
        }
    }
    let mut out: Vec<(String, String)> = Vec::new();
    let mut emit = |slot: &Slot, t: String| {
        for f in &slot.forms {
            out.push((f.clone(), t.clone()));
        }
    };
    let put = |sec: &Section, mood: &str, present_def: &[&Slot], emit: &mut dyn FnMut(&Slot, String)| {
        if sec.indef.len() != 6 {
            return;
        }
        for (s, p) in sec.indef.iter().zip(PERSONS) {
            emit(s, format!("finite {mood} indefinite {p}"));
        }
        // A subjunctive definite row that stops early repeats the present.
        let def: Vec<&Slot> = if sec.def.len() < 6 && sec.def.len() >= 2 && present_def.len() == 6 {
            sec.def.iter().copied().chain(present_def.iter().skip(sec.def.len()).copied()).collect()
        } else {
            sec.def.clone()
        };
        if def.len() == 6 {
            for (s, p) in def.iter().zip(PERSONS) {
                emit(s, format!("finite {mood} definite {p}"));
            }
        }
        if let Some(l) = sec.lak {
            emit(l, format!("lak {mood}"));
        }
    };

    // Present and past open the table.
    let Some((first, _)) = runs.first() else { return out };
    let first = sections(first);
    let present_def: Vec<&Slot> = first.first().map(|s| s.def.clone()).unwrap_or_default();
    if let Some(s) = first.first() {
        put(s, "present", &[], &mut emit);
    }
    if let Some(s) = first.get(1) {
        put(s, "past", &[], &mut emit);
    }
    // The conditional is the last tense before the note on "volna", the
    // subjunctive the tense before the note on "légyen".
    let volna = runs.iter().position(|r| r.1.contains("volna"));
    let legyen = runs.iter().position(|r| r.1.contains("légyen"));
    if let Some(i) = volna {
        if let Some(s) = runs.get(i).map(|r| sections(&r.0)).and_then(|v| v.into_iter().last()) {
            put(&s, "conditional", &[], &mut emit);
        }
    }
    if let Some(i) = legyen {
        if volna.is_some_and(|v| v < i) {
            if let Some(s) = runs.get(i).map(|r| sections(&r.0)).and_then(|v| v.into_iter().last()) {
                put(&s, "subjunctive", &present_def, &mut emit);
            }
        }
        // Then: infinitive, its six personal forms, the verbal noun, three
        // participles and the adverbial participle.
        if let Some((rest, _)) = runs.get(i + 1) {
            let mut it = rest.iter().copied().peekable();
            if let Some(inf) = it.next() {
                emit(inf, "infinitive".into());
            }
            let personal: Vec<&Slot> = it.by_ref().take(6).collect();
            if personal.len() == 6 {
                for (s, p) in personal.iter().zip(PERSONS) {
                    emit(s, format!("infinitive personal {p}"));
                }
            }
            while it.peek().is_some_and(|s| s.tags.iter().any(|t| t == "noun-from-verb")) {
                it.next();
            }
            for t in ["present", "past", "future"] {
                if let Some(s) = it.next() {
                    emit(s, format!("participle {t}"));
                }
            }
            if let Some(s) = it.next() {
                if s.tags.iter().any(|t| t == "adverbial") {
                    emit(s, "participle adverbial".into());
                }
            }
        }
    }
    out
}

pub fn language() -> Box<dyn Language> {
    Box::new(Hungarian)
}
