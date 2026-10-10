use morphe_train::kaikki::{Entry, Form};
use morphe_train::spec::*;

pub struct Tagalog;

/// Focus tags of the conjugation rows, in table order of precedence.
const FOCI: [&str; 5] = ["objective", "directional", "locative", "benefactive", "causative"];

impl Language for Tagalog {
    fn spec(&self) -> Spec {
        let mut s = Spec::new("tagalog", "Tagalog", "tl");
        // Dictionary stress marks.
        s.unmark = unmark("áa àa âa ée èe êe íi ìi îi óo òo ôo úu ùu ûu");
        s.first_table_only = true;
        // Tables are written without the headword's stress marks.
        s.canonical_spelling = false;
        s.drop_tags.extend(tags("dialectal emphatic"));
        s.features = vec![
            feature(
                "Focus",
                &[
                    ("Actor", "x-actor"),
                    ("Object", "objective"),
                    ("Directional", "directional"),
                    ("Locative", "locative"),
                    ("Benefactive", "benefactive"),
                    ("Instrumental", "x-instrumental"),
                ],
            ),
            feature(
                "Aspect",
                &[
                    ("Infinitive", "x-infinitive"),
                    ("Completed", "x-completed"),
                    ("Progressive", "x-progressive"),
                    ("Contemplative", "x-contemplative"),
                ],
            ),
            feature("Number", &[("Singular", "singular"), ("Plural", "plural")]),
        ];
        s.pos = vec![
            pos(
                "verb",
                "verb",
                vec![
                    group("Root", "x-root", ""),
                    group("", "x-finite", "Focus Aspect"),
                    group("RecentPast", "x-recent", ""),
                ],
            )
            .initial(3)
            .head_first(),
            pos(
                "adj",
                "adj",
                vec![
                    group("Root", "root", ""),
                    group("Plural", "plural !superior !inferior !relative !absolute", ""),
                    group("Equal", "equal", ""),
                    group("Superior", "superior", "Number"),
                    group("Inferior", "inferior", "Number"),
                    group("Relative", "relative", "Number"),
                    group("Absolute", "absolute", "Number"),
                ],
            )
            .initial(3)
            .head_first(),
        ];
        s
    }

    /// The conjugation table tags only some of its cells. Its rows (one per
    /// focus) each start with the infinitive; the completed and
    /// contemplative forms are the untagged ones before and after the
    /// progressive. The first untagged row is the actor focus, the second
    /// the instrumental (ipang-).
    fn prepare(&self, e: &mut Entry) {
        if e.pos != "verb" {
            return;
        }
        let forms = std::mem::take(&mut e.forms);
        let mut out = Vec::new();
        let mut tables = 0;
        let mut rows: Vec<Vec<Form>> = Vec::new();
        let mut root: Option<Form> = None;
        for f in forms {
            let has = |t: &str| f.tags.iter().any(|x| x == t);
            if f.source.is_none() {
                out.push(f);
                continue;
            }
            if has("table-tags") {
                tables += 1;
                continue;
            }
            if tables != 1 || has("inflection-template") || has("dialectal") || has("rare") {
                continue;
            }
            let form = f.form.trim_start_matches('\u{2060}');
            if form == "—" || form == "-" {
                continue;
            }
            if has("infinitive") {
                rows.push(Vec::new());
            }
            match rows.last_mut() {
                Some(row) => row.push(f),
                None if root.is_none() => root = Some(f),
                None => {}
            }
        }
        let with = |f: &Form, t: &str| Form { form: f.form.trim_start_matches('\u{2060}').to_string(), tags: tags(t), source: f.source.clone() };
        if let Some(r) = &root {
            out.push(with(r, "x-root"));
        }
        let mut untagged = 0;
        for row in &rows {
            let Some(first) = row.first() else { continue };
            let focus = match FOCI.iter().find(|t| first.tags.iter().any(|x| x == *t)) {
                Some(&"causative") => continue,
                Some(t) => (*t).to_string(),
                None => {
                    untagged += 1;
                    match untagged {
                        1 => "x-actor".into(),
                        2 => "x-instrumental".into(),
                        _ => continue,
                    }
                }
            };
            let mut seen_progressive = false;
            let (mut completed, mut contemplative, mut progressive, mut recent) = (false, false, false, false);
            for f in row {
                let has = |t: &str| f.tags.iter().any(|x| x == t);
                let aspect = if has("infinitive") {
                    "x-infinitive"
                } else if has("progressive") {
                    seen_progressive = true;
                    if progressive {
                        continue;
                    }
                    progressive = true;
                    "x-progressive"
                } else if has("formal") {
                    if focus == "x-actor" && !recent {
                        recent = true;
                        out.push(with(f, "x-recent"));
                    }
                    continue;
                } else if has("error-unrecognized-form") && !seen_progressive && !completed {
                    completed = true;
                    "x-completed"
                } else if has("error-unrecognized-form") && seen_progressive && !contemplative {
                    contemplative = true;
                    "x-contemplative"
                } else {
                    continue;
                };
                out.push(with(f, &format!("x-finite {focus} {aspect}")));
            }
        }
        e.forms = out;
    }

    fn samples(&self) -> Vec<&'static str> {
        vec!["bumili", "dalhin", "kumain", "maganda", "sumulat", "lumakad", "gumipad", "plorkin"]
    }

    fn readme(&self) -> String {
        r#"## Example

```rust
use tagalog::*;
println!("{}", verb("bumili", Focus::Actor, Aspect::Progressive)); // bumibili
println!("{}", verb("bumili", Focus::Actor, Aspect::Contemplative)); // bibili
println!("{}", verb("bumili", Focus::Object, Aspect::Completed)); // binili
println!("{}", verb("dalhin", Focus::Object, Aspect::Progressive)); // dinadala
println!("{}", verb_root("dalhin")); // dala
println!("{}", adj_plural("maganda")); // magaganda
```

Verbs are looked up by their dictionary form (an infinitive, like bumili or
dalhin) and have the root, the infinitive, completed, progressive and
contemplative aspects for the actor, object, directional, locative,
benefactive and instrumental foci, and the recent past. Affixes and
reduplication come at the start of the word and infixes inside it, so the
tree reads the first letters of a word as well as its end. Adjectives have
the plural and the comparison forms. Input matching ignores stress marks.
"#
        .into()
    }
}

pub fn language() -> Box<dyn Language> {
    Box::new(Tagalog)
}
