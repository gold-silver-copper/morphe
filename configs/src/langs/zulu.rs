use morphe_train::kaikki::Entry;
use morphe_train::spec::*;

pub struct Zulu;

/// Verb tables tag the first and second persons; each person's forms of a
/// tense come in a fixed order: (tag, index within the person, tense).
const VERB_CELLS: &[(&str, usize, &str)] = &[
    ("present", 0, "x-present"),
    ("present", 4, "x-present-negative"),
    ("present", 7, "x-perfect"),
    ("present", 11, "x-perfect-negative"),
    ("present", 14, "x-remote-past"),
    ("potential", 0, "x-potential"),
    ("potential", 2, "x-potential-negative"),
    ("immediate-future", 0, "x-near-future"),
    ("immediate-future", 2, "x-near-future-negative"),
    ("immediate-future", 5, "x-remote-future"),
    ("immediate-future", 7, "x-remote-future-negative"),
];

impl Language for Zulu {
    fn spec(&self) -> Spec {
        let mut s = Spec::new("zulu", "Zulu", "zu");
        // Dictionary tone and length marks.
        s.unmark = unmark("áa ée íi óo úu âa êe îi ôo ûu àa èe ìi òo ùu āa ēe īi ōo ūu");
        s.canonical_spelling = false;
        s.features = vec![
            feature(
                "Class",
                &[
                    ("C1", "class-1"),
                    ("C1a", "class-1a"),
                    ("C2", "class-2"),
                    ("C2a", "class-2a"),
                    ("C3", "class-3"),
                    ("C4", "class-4"),
                    ("C5", "class-5"),
                    ("C6", "class-6"),
                    ("C7", "class-7"),
                    ("C8", "class-8"),
                    ("C9", "class-9"),
                    ("C10", "class-10"),
                    ("C11", "class-11"),
                    ("C14", "class-14"),
                    ("C15", "class-15"),
                    ("C17", "class-17"),
                ],
            ),
            feature("Number", &[("Singular", "singular"), ("Plural", "plural")]),
            // The possessed noun's class, which the possessive agrees with
            // (classes sharing a concord share a cell).
            feature(
                "Concord",
                &[
                    ("C1", "class-1"),
                    ("C2", "class-2"),
                    ("C4", "class-4"),
                    ("C5", "class-5"),
                    ("C6", "class-6"),
                    ("C7", "class-7"),
                    ("C8", "class-8"),
                    ("C11", "class-11"),
                    ("C15", "class-15"),
                ],
            ),
            feature(
                "Person",
                &[
                    ("FirstSingular", "first-person singular"),
                    ("SecondSingular", "second-person singular"),
                    ("FirstPlural", "first-person plural"),
                    ("SecondPlural", "second-person plural"),
                ],
            ),
            feature(
                "Tense",
                &[
                    ("Present", "x-present"),
                    ("PresentNegative", "x-present-negative"),
                    ("Perfect", "x-perfect"),
                    ("PerfectNegative", "x-perfect-negative"),
                    ("RemotePast", "x-remote-past"),
                    ("NearFuture", "x-near-future"),
                    ("NearFutureNegative", "x-near-future-negative"),
                    ("RemoteFuture", "x-remote-future"),
                    ("RemoteFutureNegative", "x-remote-future-negative"),
                    ("Potential", "x-potential"),
                    ("PotentialNegative", "x-potential-negative"),
                    ("Subjunctive", "x-subjunctive"),
                    ("SubjunctiveNegative", "x-subjunctive-negative"),
                ],
            ),
        ];
        s.pos = vec![
            pos(
                "noun",
                "noun",
                vec![
                    group("", "x-full", "Number"),
                    group("Bare", "x-bare", "Number"),
                    group("Locative", "x-locative", "Number"),
                    group("Copulative", "x-copulative", "Number"),
                    group("Possessive", "x-possessive", "Concord Number"),
                ],
            )
            .inherent("Class")
            .citation("Singular Plural")
            .initial(3)
            .head_last(),
            pos(
                "verb",
                "verb",
                vec![
                    group("Infinitive", "infinitive !negative", ""),
                    group("NegativeInfinitive", "infinitive negative", ""),
                    group("Imperative", "imperative !object-concord", "Number"),
                    group("", "x-finite", "Tense Person"),
                ],
            )
            .head_first(),
        ];
        s
    }

    /// The headword line's plural is the full plural form; tables are read
    /// by position (see `NOUN_CELLS`, `VERB_CELLS`).
    fn prepare(&self, e: &mut Entry) {
        for f in e.forms.iter_mut().filter(|f| f.source.is_none()) {
            if f.tags.iter().any(|t| t == "plural") {
                f.tags.push("x-full".into());
            }
        }
        let template = e.forms.iter().find(|f| f.tags.iter().any(|t| t == "inflection-template")).map(|f| f.form.clone());
        let table: Vec<usize> = (0..e.forms.len())
            .filter(|&i| {
                e.forms.get(i).is_some_and(|f| f.source.is_some() && !f.tags.iter().any(|t| t == "table-tags" || t == "inflection-template"))
            })
            .collect();
        if template.as_deref() == Some("zu-infl-noun") {
            noun_table(e, &table);
            return;
        }
        if e.pos != "verb" {
            return;
        }
        // Occurrences of each (tense tag, person) so far.
        let mut seen: Vec<String> = Vec::new();
        for &i in &table {
            let Some(f) = e.forms.get_mut(i) else { continue };
            let has = |t: &str| f.tags.iter().any(|x| x == t);
            let person = ["first-person", "second-person"].into_iter().find(|p| has(p));
            let number = if has("plural") { "plural" } else { "singular" };
            let Some(person) = person else { continue };
            let new = if has("subjunctive") {
                if has("past") {
                    None
                } else if has("negative") {
                    Some("x-subjunctive-negative".to_string())
                } else {
                    Some("x-subjunctive".to_string())
                }
            } else if let Some(tag) = ["present", "potential", "immediate-future"].into_iter().find(|t| has(t)) {
                let key = format!("{tag} {person} {number}");
                let n = seen.iter().filter(|k| **k == key).count();
                seen.push(key);
                VERB_CELLS.iter().find(|c| c.0 == tag && c.1 == n).map(|c| c.2.to_string())
            } else {
                None
            };
            f.tags = match new {
                Some(t) => tags(&format!("x-finite {t} {person} {number}")),
                None => tags("table-tags"),
            };
        }
    }

    fn samples(&self) -> Vec<&'static str> {
        vec!["impala", "umuntu", "isilo", "hamba", "bona", "funda", "isikhwepha", "gqomba"]
    }

    fn readme(&self) -> String {
        r#"## Example

```rust
use zulu::*;
println!("{}", noun("umuntu", Number::Plural)); // abantu
println!("{}", noun_locative("impala", Number::Singular)); // empaleni
println!("{}", noun_possessive("impala", Concord::C7, Number::Plural)); // sezimpala
println!("{:?}", noun_class("isilo")); // C7
println!("{}", verb("hamba", Tense::Present, Person::FirstSingular)); // ngiyahamba
println!("{}", verb("hamba", Tense::Perfect, Person::SecondPlural)); // nihambile
println!("{}", verb_imperative("hamba", Number::Plural)); // hambani
```

Nouns have their singular and plural, the forms without the initial vowel,
the locative and copulative, the possessive forms agreeing with the
possessed noun's class, and their noun class; the tree reads the start of
the word as well as its end, since Zulu marks class and number with
prefixes. Verbs are looked up by their stem (hamba) and have the
infinitive, the imperative, and the present, perfect, remote past, near
and remote future, potential and subjunctive (with their negatives) for the
first and second persons. Third person forms agree with the subject's noun
class; Wiktionary's tables merge identical cells, so those are left out.
Input matching ignores tone marks.
"#
        .into()
    }
}

/// Kaikki tags the noun table's cells only with a number, and drops cells
/// that repeat an earlier one, so positions shift. The untagged plain
/// forms come in order (full, without the initial vowel, copulative); a
/// possessive's concord is read from its first letters, and the long
/// possessives (o- + short form) are left out.
fn noun_table(e: &mut Entry, table: &[usize]) {
    let possessives: Vec<String> = table
        .iter()
        .filter_map(|&i| e.forms.get(i))
        .filter(|f| f.tags.iter().any(|t| t == "possessive"))
        .map(|f| f.form.clone())
        .collect();
    let mut plain = [0usize; 2];
    for &i in table {
        let Some(f) = e.forms.get_mut(i) else { continue };
        let has = |t: &str| f.tags.iter().any(|x| x == t);
        let number = if has("plural") { "plural" } else { "singular" };
        let n = usize::from(number == "plural");
        let new = if has("locative") {
            Some("x-locative".to_string())
        } else if has("possessive") {
            let rest: String = f.form.chars().skip(1).collect();
            let long = f.form.starts_with(['a', 'e', 'o']) && possessives.contains(&rest);
            let class = if f.form.starts_with("lw") {
                "class-11"
            } else if f.form.starts_with('k') {
                "class-15"
            } else if f.form.starts_with('w') {
                "class-1"
            } else if f.form.starts_with('b') {
                "class-2"
            } else if f.form.starts_with('y') {
                "class-4"
            } else if f.form.starts_with('l') {
                "class-5"
            } else if f.form.starts_with('s') {
                "class-7"
            } else if f.form.starts_with('z') {
                "class-8"
            } else {
                "class-6"
            };
            (!long).then(|| format!("x-possessive {class}"))
        } else {
            let kind = ["x-full", "x-bare", "x-copulative"].get(plain[n]).map(|k| k.to_string());
            plain[n] += 1;
            kind
        };
        f.tags = match new {
            Some(t) => tags(&format!("{t} {number}")),
            None => tags("table-tags"),
        };
    }
}

pub fn language() -> Box<dyn Language> {
    Box::new(Zulu)
}
