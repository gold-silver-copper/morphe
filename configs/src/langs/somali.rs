use morphe_train::kaikki::Entry;
use morphe_train::spec::*;

pub struct Somali;

/// The untagged possessed forms of the determiner table, in order (the
/// third persons singular come tagged between them).
const POSSESSORS: [&str; 6] = [
    "first-person singular",
    "second-person singular",
    "first-person plural exclusive",
    "first-person plural inclusive",
    "second-person plural",
    "third-person plural",
];

impl Language for Somali {
    fn spec(&self) -> Spec {
        let mut s = Spec::new("somali", "Somali", "so");
        // Dictionary tone and stress marks.
        s.unmark = unmark("áa ée íi óo úu àa èe ìi òo ùu");
        s.canonical_spelling = false;
        s.features = vec![
            feature("Gender", &[("Masculine", "masculine"), ("Feminine", "feminine")]),
            feature("Number", &[("Singular", "singular"), ("Plural", "plural")]),
            feature(
                "Case",
                &[("Absolutive", "absolutive"), ("Nominative", "nominative"), ("Genitive", "genitive"), ("Vocative", "vocative")],
            ),
            feature(
                "Possessor",
                &[
                    ("FirstSingular", "first-person singular"),
                    ("SecondSingular", "second-person singular"),
                    ("ThirdMasculine", "third-person masculine"),
                    ("ThirdFeminine", "third-person feminine"),
                    ("FirstPluralExclusive", "first-person plural exclusive"),
                    ("FirstPluralInclusive", "first-person plural inclusive"),
                    ("SecondPlural", "second-person plural"),
                    ("ThirdPlural", "third-person plural"),
                ],
            ),
            feature(
                "Person",
                &[
                    ("FirstSingular", "first-person singular"),
                    ("SecondSingular", "second-person singular"),
                    ("ThirdMasculine", "third-person masculine singular"),
                    ("ThirdFeminine", "third-person feminine singular"),
                    ("FirstPlural", "first-person plural"),
                    ("SecondPlural", "second-person plural"),
                    ("ThirdPlural", "third-person plural"),
                ],
            ),
            feature(
                "Tense",
                &[
                    ("Present", "x-present"),
                    ("PresentProgressive", "x-present-progressive"),
                    ("Past", "x-past"),
                    ("PastProgressive", "x-past-progressive"),
                ],
            ),
        ];
        s.pos = vec![
            pos(
                "noun",
                "noun",
                vec![
                    group("Plural", "plural !x-table", ""),
                    group("", "x-table", "Case Number"),
                    group("Definite", "x-table definite !x-remote", "Number"),
                    group("RemoteDefinite", "x-table definite x-remote", "Number"),
                    group("Proximal", "x-table proximal", "Number"),
                    group("Medial", "x-table medial", "Number"),
                    group("Distal", "x-table distal", "Number"),
                    group("Possessive", "x-table possessive !x-plural-possessum", "Possessor"),
                ],
            )
            .inherent("Gender")
            .citation("Lemma Plural")
            .head_last(),
            pos(
                "verb",
                "verb",
                vec![
                    group("Infinitive", "infinitive", ""),
                    group("Causative", "causative", ""),
                    group("Middle", "middle", ""),
                    group("Passive", "passive", ""),
                    group("Imperative", "imperative", "Number"),
                    group("", "x-finite", "Tense Person"),
                ],
            )
            .citation("ImperativeSingular Infinitive")
            .lemma_cell("ImperativeSingular")
            .head_first(),
        ];
        s
    }

    /// The determiner table tags few of its cells; the second definite is
    /// the remote one (-tii), and the untagged possessed forms come in a
    /// fixed order. In verb tables each person's row has the present, the
    /// present progressive, then two forms tagged "continuative past": the
    /// simple past and the past progressive.
    fn prepare(&self, e: &mut Entry) {
        e.forms.retain(|f| !f.form.contains("{{{"));
        let (mut definite, mut possessor) = (0usize, 0usize);
        let mut past: Vec<String> = Vec::new();
        for f in e.forms.iter_mut().filter(|f| f.source.is_some()) {
            let has = |t: &str| f.tags.iter().any(|x| x == t);
            if has("table-tags") {
                (definite, possessor) = (0, 0);
                past.clear();
                continue;
            }
            if has("inflection-template") {
                continue;
            }
            if e.pos == "noun" {
                let error = has("error-unrecognized-form");
                f.tags.retain(|t| t != "error-unrecognized-form");
                let has = |t: &str| f.tags.iter().any(|x| x == t);
                if has("definite") {
                    if definite % 2 == 1 {
                        f.tags.push("x-remote".into());
                    }
                    definite += 1;
                } else if has("possessive") {
                    // The possessed noun's number, apart from the possessor's.
                    let plural = has("plural");
                    f.tags.retain(|t| t != "singular" && t != "plural");
                    if plural {
                        f.tags.push("x-plural-possessum".into());
                    }
                    if error {
                        match POSSESSORS.get(possessor) {
                            Some(p) => f.tags.extend(tags(p)),
                            None => f.tags.clear(),
                        }
                        possessor += 1;
                    }
                } else if error && f.tags.len() == 1 {
                    f.tags.clear();
                    continue;
                }
                f.tags.push("x-table".into());
                continue;
            }
            if has("indicative") && !has("future") {
                let person: Vec<&String> = f.tags.iter().filter(|t| t.ends_with("-person") || *t == "singular" || *t == "plural" || *t == "masculine" || *t == "feminine").collect();
                let person = person.into_iter().cloned().collect::<Vec<_>>().join(" ");
                let tense = if has("present") && has("continuative") {
                    "x-present-progressive"
                } else if has("present") {
                    "x-present"
                } else {
                    let n = past.iter().filter(|p| **p == person).count();
                    past.push(person.clone());
                    if n == 0 { "x-past" } else { "x-past-progressive" }
                };
                f.tags = tags(&format!("x-finite {tense} {person}"));
            }
        }
    }

    fn samples(&self) -> Vec<&'static str> {
        vec!["beer", "bad", "gabdho", "door", "far", "qor", "baraas"]
    }

    fn readme(&self) -> String {
        r#"## Example

```rust
use somali::*;
println!("{}", noun_plural("beer")); // beero
println!("{}", noun_definite("beer", Number::Singular)); // beerta
println!("{}", noun_possessive("beer", Possessor::FirstSingular)); // beertayda
println!("{:?}", noun_gender("beer"));
println!("{}", verb("door", Tense::Past, Person::FirstPlural)); // doortnay
println!("{}", verb_imperative("door", Number::Plural)); // doorta
```

Nouns have the plural, their gender, the definite (near -ka/-ta and remote
-kii/-tii), the demonstratives, the possessed forms for eight possessors
and, for some, the case forms. Verbs are cited by the imperative singular
and have the present, past and their progressives for seven persons, the
plural imperative and the derived causative, middle and passive stems.
Input matching ignores the tone marks dictionaries write.
"#
        .into()
    }
}

pub fn language() -> Box<dyn Language> {
    Box::new(Somali)
}
