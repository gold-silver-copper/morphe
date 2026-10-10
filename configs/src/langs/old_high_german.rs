use morphe_train::kaikki::Entry;
use morphe_train::spec::*;

pub struct OldHighGerman;

fn has(tags: &[String], t: &str) -> bool {
    tags.iter().any(|x| x == t)
}

const PERSONS: &[&str] = &["first-person", "second-person", "third-person"];

impl Language for OldHighGerman {
    fn spec(&self) -> Spec {
        let mut s = Spec::new("old_high_german", "OldHighGerman", "goh");
        // Length marks and the ë of normalized spelling are often left out.
        s.unmark = unmark("āa ēe īi ōo ūu ëe âa êe îi ôo ûu");
        s.fold = vec![('ȥ', 'z')];
        s.features = vec![
            feature("Gender", &[("Masculine", "masculine"), ("Feminine", "feminine"), ("Neuter", "neuter")]),
            feature(
                "Case",
                &[
                    ("Nominative", "nominative"),
                    ("Accusative", "accusative"),
                    ("Genitive", "genitive"),
                    ("Dative", "dative"),
                    ("Instrumental", "instrumental"),
                ],
            ),
            feature("Number", &[("Singular", "singular"), ("Plural", "plural")]),
            feature("Mood", &[("Indicative", "indicative"), ("Subjunctive", "subjunctive")]),
            feature("Tense", &[("Present", "present"), ("Past", "past")]),
            feature("Person", &[("First", "first-person"), ("Second", "second-person"), ("Third", "third-person")]),
        ];
        s.pos = vec![
            pos("noun", "noun", vec![group("", "", "Case Number")])
                .inherent("Gender")
                .citation("Lemma GenitiveSingular NominativePlural")
                .head_last(),
            pos(
                "adj",
                "adj",
                vec![group("", "strong", "Case Number Gender"), group("Weak", "weak", "Case Number Gender")],
            ),
            pos(
                "verb",
                "verb",
                vec![
                    group("", "!participle !infinitive !imperative", "Mood Tense Person Number"),
                    group("Imperative", "imperative", "Number"),
                    group("ParticiplePresent", "participle present", ""),
                    group("ParticiplePast", "participle past", ""),
                ],
            )
            .citation("Lemma IndicativePastThirdSingular IndicativePastFirstPlural ParticiplePast")
            .head_first(),
        ];
        s
    }

    fn prepare(&self, e: &mut Entry) {
        // A few forms mark length with a circumflex instead of the macron
        // used everywhere else.
        for f in &mut e.forms {
            if f.form.contains(['â', 'ê', 'î', 'ô', 'û']) {
                f.form = f.form.chars().map(|c| match c { 'â' => 'ā', 'ê' => 'ē', 'î' => 'ī', 'ô' => 'ō', 'û' => 'ū', c => c }).collect();
            }
        }
        if e.pos != "verb" {
            e.forms.retain(|f| !f.form.starts_with('*'));
            return;
        }
        // The conjugation table gives the indicative rows, then the same
        // rows for the subjunctive (a row seen again, not right after
        // itself, starts it), then the imperative and the participles,
        // which keep only "present"/"past".
        let mut mood = 0;
        let mut seen: Vec<Vec<String>> = Vec::new();
        let mut prev: Vec<String> = Vec::new();
        for f in &mut e.forms {
            if has(&f.tags, "table-tags") {
                mood = 0;
                seen.clear();
                continue;
            }
            if f.source.is_none() || has(&f.tags, "infinitive") || has(&f.tags, "class") || has(&f.tags, "inflection-template") {
                continue;
            }
            if !PERSONS.iter().any(|p| has(&f.tags, p)) {
                if has(&f.tags, "imperative") || has(&f.tags, "singular") || has(&f.tags, "plural") {
                    f.tags.retain(|t| t == "singular" || t == "plural");
                    f.tags.push("imperative".into());
                } else if f.tags.len() == 1 && (has(&f.tags, "present") || has(&f.tags, "past")) {
                    f.tags.push("participle".into());
                }
                continue;
            }
            f.tags.retain(|t| t != "preterite");
            let mut key = f.tags.clone();
            key.sort();
            if key != prev && seen.contains(&key) {
                mood += 1;
                seen.clear();
            }
            seen.push(key.clone());
            prev = key;
            f.tags.push(if mood == 0 { "indicative" } else { "subjunctive" }.into());
        }
        e.forms.retain(|f| !f.form.starts_with('*'));
    }

    fn samples(&self) -> Vec<&'static str> {
        vec!["tag", "geba", "wort", "bitten", "neman", "salbōn", "blint", "glumpen", "strabal"]
    }

    fn readme(&self) -> String {
        r#"## Example

```rust
use old_high_german::*;
assert_eq!(noun("tag", Case::Dative, Number::Plural), "tagum");
println!("{:?}", noun_gender("geba"));
assert_eq!(verb("bitten", Mood::Indicative, Tense::Past, Person::Third, Number::Singular), "bat");
println!("{}", adj_weak("blint", Case::Nominative, Number::Singular, Gender::Masculine));
```

Spelling follows Wiktionary's normalized Old High German. Length marks are
optional on input.
"#
        .into()
    }
}

pub fn language() -> Box<dyn Language> {
    Box::new(OldHighGerman)
}
