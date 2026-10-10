use morphe_train::kaikki::{Entry, Form};
use morphe_train::spec::*;

pub struct Slovak;

fn has(tags: &[String], t: &str) -> bool {
    tags.iter().any(|x| x == t)
}

const CASES: [&str; 7] = ["nominative", "genitive", "dative", "accusative", "locative", "instrumental", "vocative"];

/// A noun table being rebuilt: case → (singular forms, plural forms).
struct Rows(Vec<(&'static str, Vec<String>, Vec<String>)>);

impl Rows {
    fn get(&self, case: &str, plural: bool) -> Vec<String> {
        self.0.iter().find(|r| r.0 == case).map(|r| if plural { r.2.clone() } else { r.1.clone() }).unwrap_or_default()
    }

    /// Fill an empty cell (a form Kaikki dropped as a repeat).
    fn fill(&mut self, case: &str, plural: bool, forms: Vec<String>) {
        if let Some(r) = self.0.iter_mut().find(|r| r.0 == case) {
            let slot = if plural { &mut r.2 } else { &mut r.1 };
            if slot.is_empty() {
                *slot = forms;
            }
        }
    }
}

impl Language for Slovak {
    fn spec(&self) -> Spec {
        let mut s = Spec::new("slovak", "Slovak", "sk");
        s.drop_tags.extend(tags("dated"));
        s.soft_tags.extend(tags("poetic literary"));
        s.features = vec![
            feature(
                "Case",
                &[
                    ("Nominative", "nominative"),
                    ("Genitive", "genitive"),
                    ("Dative", "dative"),
                    ("Accusative", "accusative"),
                    ("Locative", "locative"),
                    ("Instrumental", "instrumental"),
                ],
            ),
            feature("Number", &[("Singular", "singular"), ("Plural", "plural")]),
            feature(
                "Gender",
                &[
                    ("MasculinePersonal", "masculine person"),
                    ("MasculineAnimal", "masculine animal-not-person"),
                    ("MasculineInanimate", "masculine inanimate"),
                    ("Feminine", "feminine"),
                    ("Neuter", "neuter"),
                ],
            ),
            feature(
                "GenderNumber",
                &[
                    ("Masculine", "masculine !plural"),
                    ("Feminine", "feminine !plural"),
                    ("Neuter", "neuter !plural"),
                    ("Virile", "virile"),
                    ("Nonvirile", "nonvirile"),
                ],
            ),
            feature(
                "PastGenderNumber",
                &[
                    ("Masculine", "masculine !plural"),
                    ("Feminine", "feminine !plural"),
                    ("Neuter", "neuter !plural"),
                    ("Plural", "plural"),
                ],
            ),
            feature("Animacy", &[("Animate", "animate"), ("Inanimate", "inanimate")]),
            feature("Aspect", &[("Imperfective", "imperfective"), ("Perfective", "perfective")]),
            feature("Person", &[("First", "first-person"), ("Second", "second-person"), ("Third", "third-person")]),
            feature("Tense", &[("Present", "present"), ("Past", "past")]),
        ];
        let adjlike = "Case GenderNumber Animacy?";
        s.pos = vec![
            pos("noun", "noun", vec![group("", "", "Case Number")])
                .inherent("Gender")
                .citation("NominativeSingular GenitiveSingular")
                .head_last(),
            pos(
                "adj",
                "adj",
                vec![
                    group("", "!comparative !superlative", adjlike),
                    group("Comparative", "comparative", ""),
                    group("Superlative", "superlative", ""),
                ],
            ),
            pos(
                "verb",
                "verb",
                vec![
                    group("Infinitive", "infinitive !indicative !conditional", ""),
                    group("Present", "indicative present !past !future !infinitive", "Person Number"),
                    group("Imperative", "imperative second-person !noun-from-verb", "Number"),
                    group("ImperativeFirstPlural", "imperative first-person plural", ""),
                    group("Past", "l-participle", "PastGenderNumber"),
                    group("ActiveParticiple", "active participle", "Tense").declines("adj", ""),
                    group("PassiveParticiple", "passive participle", "").declines("adj", ""),
                    group("Transgressive", "transgressive", ""),
                    group("VerbalNoun", "noun-from-verb", ""),
                ],
            )
            .inherent("Aspect")
            .citation("Infinitive PresentFirstSingular PastMasculine"),
            pos("pron", "pron", vec![group("", "", "Case Number GenderNumber? Animacy?")]),
            pos("det", "det", vec![group("", "", adjlike)]),
            pos("num", "num", vec![group("", "", "Case Number? GenderNumber? Animacy?")]),
        ];
        s
    }

    /// Kaikki reads Slovak noun tables as a flat list of untagged strings:
    /// column headers, then each case name followed by its forms. It also
    /// drops a form that repeats an earlier one, so a row may be short. The
    /// forms are put back into cells by their endings and the missing ones
    /// filled with the form they repeat.
    fn reshape(&self, pos: &PosSpec, e: &Entry) -> Option<Vec<Form>> {
        if pos.name != "noun" {
            return None;
        }
        let flat = |f: &&Form| f.source.is_some() && f.tags.is_empty();
        let mut seq = e.forms.iter().filter(flat).map(|f| f.form.trim()).peekable();
        seq.peek()?;
        let mut cols: Vec<bool> = Vec::new(); // true = plural
        while let Some(h) = seq.peek() {
            match *h {
                "singular" => cols.push(false),
                h if h.starts_with("plural") => cols.push(true),
                _ => break,
            }
            seq.next();
        }
        if cols.is_empty() {
            return None;
        }
        let mut raw: Vec<(&'static str, Vec<&str>)> = Vec::new();
        for f in seq {
            match CASES.iter().find(|c| **c == f) {
                Some(case) => raw.push((case, Vec::new())),
                None => {
                    if let Some(r) = raw.last_mut() {
                        r.1.push(f);
                    }
                }
            }
        }
        let both = cols.contains(&false) && cols.contains(&true);
        let mut rows = Rows(Vec::new());
        for (case, forms) in raw {
            let (mut sg, mut pl) = (Vec::new(), Vec::new());
            for (i, f) in forms.iter().enumerate() {
                let plural = if !both {
                    cols.first().copied().unwrap_or(false)
                } else {
                    match (case, forms.len()) {
                        ("dative", _) => f.ends_with('m'),
                        ("locative", _) => f.ends_with("ch"),
                        ("instrumental", _) => f.ends_with("mi"),
                        ("accusative", 1) => false,
                        ("genitive", 1) => true,
                        ("genitive", n) => f.ends_with("ov") || i + 1 == n,
                        (_, n) if n == cols.len() => cols.get(i).copied().unwrap_or(true),
                        _ => i > 0,
                    }
                };
                if plural { pl.push(f.to_string()) } else { sg.push(f.to_string()) }
            }
            rows.0.push((case, sg, pl));
        }
        if both {
            let senses: Vec<&str> = e.senses.iter().flat_map(|s| s.tags.iter()).map(String::as_str).collect();
            let person = senses.contains(&"person");
            let animate = person || senses.contains(&"animal-not-person") || senses.contains(&"animate");
            let masculine = senses.contains(&"masculine");
            rows.fill("nominative", true, rows.get("nominative", false));
            rows.fill("genitive", false, rows.get("nominative", true));
            let gen_sg = rows.get("genitive", false);
            let not_a: Vec<String> = gen_sg.iter().filter(|f| !f.ends_with('a')).cloned().collect();
            rows.fill("dative", false, if not_a.is_empty() { gen_sg.clone() } else { not_a });
            rows.fill("locative", false, rows.get("dative", false));
            // Masculine and neuter instrumental singular -om repeats the dative plural.
            rows.fill("instrumental", false, rows.get("dative", true));
            rows.fill("accusative", false, if masculine && animate { gen_sg } else { rows.get("nominative", false) });
            rows.fill("accusative", true, rows.get(if masculine && person { "genitive" } else { "nominative" }, true));
        }
        let mut out: Vec<Form> = e.forms.iter().filter(|f| !flat(f)).cloned().collect();
        for (case, sg, pl) in rows.0 {
            for (forms, number) in [(sg, "singular"), (pl, "plural")] {
                for form in forms {
                    out.push(Form { form, tags: vec![case.into(), number.into()], source: Some("declension".into()) });
                }
            }
        }
        Some(out)
    }

    /// Merged headers lose tags: the nonvirile plural column comes back as
    /// `error-unrecognized-form` or without "plural", genderless plurals
    /// stand for both, "singular-only" tables, and the l-participle forms
    /// come tagged "plural" whatever their gender (read off -l/-la/-lo/-li).
    fn retag(&self, pos: &PosSpec, form: &str, t: &mut Vec<String>) {
        for (only, plain) in [("singular-only", "singular"), ("plural-only", "plural")] {
            if let Some(i) = t.iter().position(|x| x == only) {
                t[i] = plain.into();
            }
        }
        if let Some(i) = t.iter().position(|x| x == "error-unrecognized-form") {
            t.remove(i);
            t.push("nonvirile".into());
        }
        if (has(t, "virile") || has(t, "nonvirile")) && !has(t, "plural") {
            t.push("plural".into());
        }
        if pos.name != "verb" && has(t, "plural") && !has(t, "virile") && !has(t, "nonvirile") {
            t.extend(tags("virile nonvirile"));
        }
        if has(t, "l-participle") {
            t.retain(|x| x != "plural" && x != "singular");
            let form = form.split_whitespace().next().unwrap_or("");
            let gn = if form.ends_with("li") {
                "plural"
            } else if form.ends_with("la") {
                "feminine singular"
            } else if form.ends_with("lo") {
                "neuter singular"
            } else {
                "masculine singular"
            };
            t.extend(tags(gn));
        }
    }

    /// Verb entries carry the reflexive table (robiť sa) after the plain one.
    fn adjust(&self, _pos: &PosSpec, lemma: &str, forms: &mut Vec<(usize, String, bool)>) {
        let reflexive = |w: &str| w.ends_with(" sa") || w.ends_with(" si");
        if !reflexive(lemma) {
            forms.retain(|(_, f, _)| !reflexive(f));
        }
    }

    fn readme(&self) -> String {
        r#"## Example

```rust
use slovak::*;
println!("{}", noun("žena", Case::Instrumental, Number::Plural));
println!("{}", noun("dub", Case::Locative, Number::Singular));
println!("{:?}", noun_gender("chlap"));
println!("{}", verb_present("robiť", Person::First, Number::Singular));
println!("{}", verb_past("robiť", PastGenderNumber::Feminine));
println!("{}", adj("bystrý", Case::Genitive, GenderNumber::Feminine, Animacy::Inanimate));
// Citations pin principal parts: nominative and genitive singular.
println!("{}", noun("pes, psa", Case::Dative, Number::Singular));
```

Input is matched as written: Slovak diacritics (á, ä, č, ď, ľ, ĺ, ň, ô, ŕ, š,
ť, ž…) are letters. The past tense, conditional and imperfective future are
analytic (`robil som`, `robil by som`, `budem robiť`): build them from
`verb_past` and `verb_infinitive`.
"#
        .into()
    }
}

pub fn language() -> Box<dyn Language> {
    Box::new(Slovak)
}
