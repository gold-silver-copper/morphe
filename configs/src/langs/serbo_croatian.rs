use morphe_train::spec::*;

pub struct SerboCroatian;

const CASES: &str = "Case";

fn has(tags: &[String], t: &str) -> bool {
    tags.iter().any(|x| x == t)
}

const PERSONS: [&str; 3] = ["first-person", "second-person", "third-person"];

/// Tonal accent letters (precomposed) and their plain letters.
const UNMARK: &str = "àa áa âa āa ȁa ȃa èe ée êe ēe ȅe ȇe ìi íi îi īi ȉi ȋi òo óo ôo ōo ȍo ȏo ùu úu ûu ūu ȕu ȗu ȑr ȓr ѐе ѝи ӣи ӯу";

/// A word without tonal accents, for reading endings.
fn plain(w: &str) -> String {
    let map = unmark(UNMARK);
    w.chars()
        .filter(|c| !matches!(c, '\u{300}'..='\u{311}'))
        .map(|c| map.iter().find(|m| m.0 == c).map_or(c, |m| m.1))
        .collect()
}

impl Language for SerboCroatian {
    fn spec(&self) -> Spec {
        let mut s = Spec::new("serbo_croatian", "SerboCroatian", "sh");
        s.marks = vec!['\u{300}', '\u{301}', '\u{302}', '\u{304}', '\u{30F}', '\u{311}'];
        s.unmark = unmark(UNMARK);
        // Headwords carry tonal accents, the tables mostly don't.
        s.canonical_spelling = false;
        s.drop_tags.extend(tags("dated"));
        s.soft_tags.extend(tags("poetic regional"));
        s.features = vec![
            feature(
                "Case",
                &[
                    ("Nominative", "nominative"),
                    ("Genitive", "genitive"),
                    ("Dative", "dative"),
                    ("Accusative", "accusative"),
                    ("Vocative", "vocative"),
                    ("Locative", "locative"),
                    ("Instrumental", "instrumental"),
                ],
            ),
            feature("Number", &[("Singular", "singular"), ("Plural", "plural")]),
            feature("Gender", &[("Masculine", "masculine"), ("Feminine", "feminine"), ("Neuter", "neuter")]),
            feature("Animacy", &[("Animate", "animate"), ("Inanimate", "inanimate")]),
            feature("Definiteness", &[("Indefinite", "indefinite"), ("Definite", "definite")]),
            feature("Aspect", &[("Imperfective", "imperfective"), ("Perfective", "perfective")]),
            feature("Person", &[("First", "first-person"), ("Second", "second-person"), ("Third", "third-person")]),
            feature("Tense", &[("Present", "present"), ("Past", "past")]),
        ];
        s.pos = vec![
            pos("noun", "noun", vec![group("", "", &format!("{CASES} Number"))])
                .inherent("Gender Animacy")
                .citation("NominativeSingular GenitiveSingular")
                .head_last(),
            pos(
                "adj",
                "adj",
                vec![
                    group("", "!comparative !superlative", &format!("Definiteness? {CASES} Number Gender Animacy?")),
                    group("Comparative", "comparative", &format!("{CASES} Number Gender Animacy?")),
                    group("Superlative", "superlative", &format!("{CASES} Number Gender Animacy?")),
                ],
            ),
            pos(
                "verb",
                "verb",
                vec![
                    group("Infinitive", "infinitive", ""),
                    group("Present", "present !imperative", "Person Number"),
                    group("Future", "future-i", "Person Number"),
                    group("Aorist", "aorist", "Person Number"),
                    group("Imperfect", "imperfect", "Person Number"),
                    group("Imperative", "imperative second-person", "Number"),
                    group("ImperativeFirstPlural", "imperative first-person plural", ""),
                    group("Past", "participle active past", "Number Gender"),
                    group("Passive", "participle passive past", "Number Gender"),
                    group("Gerund", "adverbial participle", "Tense"),
                    group("VerbalNoun", "noun-from-verb", ""),
                ],
            )
            .inherent("Aspect")
            .citation("Infinitive PresentFirstSingular")
            // Perfective verbs have no imperfect or present adverbial
            // participle.
            .missing_is_absent(),
            pos("pron", "pron", vec![group("", "", &format!("{CASES} Number Gender? Animacy?"))]),
            pos("det", "det", vec![group("", "", &format!("{CASES} Number Gender? Animacy?"))]),
            pos("num", "num", vec![group("", "", &format!("{CASES} Number? Gender? Animacy?"))]),
        ];
        s
    }

    /// Wiktionary's Serbo-Croatian conjugation tables name only the third
    /// person, and give the participles as "písao m", "písala f" with the
    /// gender in the form string and an unrecognized header. Persons are read
    /// off the endings and auxiliaries (-m/-h/-u/ću, -š/ćeš, -mo, -te); the
    /// participles off the gender letter and the -l- of the active one.
    fn retag(&self, pos: &PosSpec, form: &str, t: &mut Vec<String>) {
        if pos.name != "verb" || form.is_empty() {
            return;
        }
        let words: Vec<String> = form.split_whitespace().map(plain).filter(|w| w != "se").collect();
        if let Some(i) = t.iter().position(|x| x == "error-unrecognized-form") {
            t.remove(i);
            t.retain(|x| x != "third-person");
            let gender = match words.last().map(String::as_str) {
                Some("m") => "masculine",
                Some("f") => "feminine",
                Some("n") => "neuter",
                _ => "",
            };
            let Some(word) = words.first() else { return };
            if gender.is_empty() {
                // Adverbial participles: píšūći, písāvši.
                if word.ends_with("ći") {
                    t.extend(tags("adverbial participle present"));
                } else if word.ends_with("ši") {
                    t.extend(tags("adverbial participle past"));
                }
                return;
            }
            let penult = word.chars().rev().nth(1);
            let active = if gender == "masculine" && has(t, "singular") { word.ends_with('o') } else { penult == Some('l') };
            t.push(gender.into());
            t.extend(tags(if active { "participle active past" } else { "participle passive past" }));
            return;
        }
        if PERSONS.iter().any(|p| has(t, p)) {
            return;
        }
        let finite = ["present", "future-i", "aorist", "imperfect", "imperative"].iter().any(|x| has(t, x));
        let (sg, pl) = (has(t, "singular"), has(t, "plural"));
        if !finite || sg == pl {
            return;
        }
        let any = |f: &dyn Fn(&str) -> bool| words.iter().any(|w| f(w));
        let person = if pl {
            if any(&|w| w.ends_with("mo")) {
                "first-person"
            } else if any(&|w| w.ends_with("te")) {
                "second-person"
            } else {
                return;
            }
        } else if has(t, "imperative") || any(&|w| w.ends_with('š')) {
            "second-person"
        } else if any(&|w| w.ends_with('m') || w.ends_with('h') || w.ends_with('u')) {
            "first-person"
        } else {
            "second-person"
        };
        t.push(person.into());
    }

    /// Drop the gender letter of "písao m".
    fn split_form(&self, raw: &str) -> Vec<String> {
        let raw = raw.trim();
        let stripped = [" m", " f", " n"].iter().find_map(|g| raw.strip_suffix(g)).unwrap_or(raw);
        vec![stripped.to_string()]
    }

    fn samples(&self) -> Vec<&'static str> {
        vec!["žena", "pas, psa", "жена", "pisati", "napisati", "star", "glorpa", "zlimati"]
    }

    fn readme(&self) -> String {
        r#"## Example

```rust
use serbo_croatian::*;
println!("{}", noun("žena", Case::Instrumental, Number::Plural));
println!("{}", noun("grad", Case::Locative, Number::Singular));
println!("{:?}", noun_gender("kost"));
println!("{}", verb_present("pisati", Person::First, Number::Singular));
println!("{}", verb_past("pisati", Number::Singular, Gender::Feminine));
println!("{}", adj("star", Definiteness::Definite, Case::Genitive, Number::Singular, Gender::Masculine, Animacy::Inanimate));
// Citations pin principal parts: nominative and genitive singular.
println!("{}", noun("pas, psa", Case::Dative, Number::Singular));
```

Input matching ignores tonal accents (ȁ, ȃ, à, á, ā…). Output carries them
where Wiktionary's tables have them (`marks(false)` removes them); unknown
words mostly come out without them. č, ć, đ, š and ž are
letters, not accents. Latin and Cyrillic are separate dictionary entries;
both are learned as written. The analytic tenses (perfect, conditional,
future II) are built from the `verb_past` participle plus an auxiliary.
"#
        .into()
    }
}

pub fn language() -> Box<dyn Language> {
    Box::new(SerboCroatian)
}
