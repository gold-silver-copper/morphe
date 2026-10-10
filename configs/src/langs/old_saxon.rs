use morphe_train::kaikki::Entry;
use morphe_train::spec::*;

pub struct OldSaxon;

fn has(tags: &[String], t: &str) -> bool {
    tags.iter().any(|x| x == t)
}

const PERSONS: &[&str] = &["first-person", "second-person", "third-person"];

impl Language for OldSaxon {
    fn spec(&self) -> Spec {
        let mut s = Spec::new("old_saxon", "OldSaxon", "osx");
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
            feature(
                "PersonNumber",
                &[
                    ("FirstSingular", "first-person singular"),
                    ("SecondSingular", "second-person singular"),
                    ("ThirdSingular", "third-person singular"),
                    ("Plural", "plural"),
                ],
            ),
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
                    group("", "!participle !infinitive !imperative", "Mood Tense PersonNumber"),
                    group("Imperative", "imperative singular", ""),
                    group("ParticiplePresent", "participle present", ""),
                    group("ParticiplePast", "participle past", ""),
                ],
            )
            .citation("Lemma IndicativePastThirdSingular IndicativePastPlural ParticiplePast")
            .head_first(),
        ];
        s
    }

    fn prepare(&self, e: &mut Entry) {
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
            // The finite plural has no person; the imperative singular
            // has none either.
            let person = PERSONS.iter().any(|p| has(&f.tags, p));
            if !person && !has(&f.tags, "plural") {
                if has(&f.tags, "singular") {
                    f.tags = tags("imperative singular");
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
        vec!["dag", "word", "geba", "kritan", "niman", "wesan", "glumpan", "strabal"]
    }

    fn readme(&self) -> String {
        r#"## Example

```rust
use old_saxon::*;
assert_eq!(noun("dag", Case::Dative, Number::Plural), "dagum");
println!("{:?}", noun_gender("word"));
println!("{}", verb("kritan", Mood::Indicative, Tense::Past, PersonNumber::ThirdSingular)); // krēt
println!("{}", verb("kritan", Mood::Subjunctive, Tense::Present, PersonNumber::Plural)); // krīten
```

Spelling follows Wiktionary's normalized Old Saxon. Length marks are
optional on input. The finite plural has one form for all persons.
"#
        .into()
    }
}

pub fn language() -> Box<dyn Language> {
    Box::new(OldSaxon)
}
