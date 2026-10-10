use morphe_train::kaikki::Entry;
use morphe_train::spec::*;

pub struct OldArmenian;

impl Language for OldArmenian {
    fn spec(&self) -> Spec {
        let mut s = Spec::new("old_armenian", "OldArmenian", "xcl");
        // Stress, emphasis and question marks written over a vowel (տե՛ս).
        s.marks = vec!['\u{55B}', '\u{55C}', '\u{55E}'];
        // The ligature և is եւ.
        s.fold_strs = vec![("և".into(), "եւ".into())];
        s.features = vec![
            feature(
                "Case",
                &[
                    ("Nominative", "nominative"),
                    ("Accusative", "accusative"),
                    ("Genitive", "genitive"),
                    ("Dative", "dative"),
                    ("Locative", "locative"),
                    ("Ablative", "ablative"),
                    ("Instrumental", "instrumental"),
                ],
            ),
            feature("Number", &[("Singular", "singular"), ("Plural", "plural")]),
            feature("Person", &[("First", "first-person"), ("Second", "second-person"), ("Third", "third-person")]),
            feature("Voice", &[("Active", "!mediopassive"), ("Mediopassive", "mediopassive")]),
            feature(
                "Tense",
                &[
                    ("Present", "indicative present"),
                    ("Imperfect", "indicative imperfect"),
                    ("Aorist", "indicative aorist"),
                    ("PresentSubjunctive", "subjunctive present"),
                    ("AoristSubjunctive", "subjunctive aorist"),
                ],
            ),
            feature(
                "Command",
                &[
                    ("Plain", "!cohortative !prohibitive"),
                    ("Cohortative", "cohortative"),
                    ("Prohibitive", "prohibitive"),
                ],
            ),
        ];
        let nominal = || vec![group("", "", "Case Number")];
        s.pos = vec![
            pos("noun", "noun", nominal()).citation("NominativeSingular GenitiveSingular").head_last(),
            pos("adj", "adj", nominal()),
            pos(
                "verb",
                "verb",
                vec![
                    group("Infinitive", "infinitive", "Voice"),
                    group("Participle", "participle !mediopassive", ""),
                    group("", "!imperative", "Voice Tense Person Number"),
                    group("Imperative", "imperative second-person", "Voice Command Number"),
                ],
            )
            .citation("Lemma ActiveAoristFirstSingular|MediopassiveAoristFirstSingular")
            .head_first(),
            pos("pron", "pron", nominal()),
            pos("num", "num", nominal()),
        ];
        s
    }

    /// A verb's second table is its mediopassive, and so is a lone table in
    /// -իմ; neither is tagged.
    fn prepare(&self, e: &mut Entry) {
        if e.pos != "verb" {
            return;
        }
        let deponent = e.word.ends_with("իմ");
        let mut table = 0usize;
        let mut passive = deponent;
        for f in e.forms.iter_mut() {
            if f.source.is_none() {
                continue;
            }
            if f.tags.iter().any(|t| t == "table-tags") {
                table += 1;
                passive = deponent || table > 1;
            }
            if passive {
                f.tags.push("mediopassive".into());
            }
        }
    }

    /// Instrumental plurals list the classical and later spelling:
    /// "գրաւք = գրօք (grawkʻ = grōkʻ)".
    fn split_form(&self, raw: &str) -> Vec<String> {
        let main = raw.split(" (").next().unwrap_or(raw);
        main.split(" = ").map(|s| s.trim().to_string()).collect()
    }

    fn samples(&self) -> Vec<&'static str> {
        vec!["գիր", "մարդ", "քաղաք", "խնդրեմ", "տեսանեմ", "գործեմ", "բարի", "կրպան", "շմփեմ"]
    }

    fn readme(&self) -> String {
        r#"## Example

```rust
use old_armenian::*;
println!("{}", noun("գիր", Case::Genitive, Number::Singular)); // գրոյ
println!("{}", noun("գիր", Case::Instrumental, Number::Plural)); // գրովք
println!("{}", verb("խնդրեմ", Voice::Active, Tense::Aorist, Person::Third, Number::Singular)); // խնդրեաց
println!("{}", verb("տեսանեմ", Voice::Mediopassive, Tense::Present, Person::First, Number::Singular)); // տեսանիմ
// The aorist stem is lexical; a citation can give it.
println!("{}", verb("տեսանեմ, տեսի", Voice::Active, Tense::AoristSubjunctive, Person::Third, Number::Singular));
```

Nouns, adjectives, pronouns and numerals have seven cases in both
numbers. Verbs are looked up by the first person present (խնդրեմ) and
have the present, imperfect and aorist indicative and the present and
aorist subjunctive, active and mediopassive, for six persons, plus the
imperative, cohortative and prohibitive, the infinitives and the
participle. Input matching
ignores the stress and question marks (՛ ՜ ՞) and reads և as եւ.
"#
        .into()
    }
}

pub fn language() -> Box<dyn Language> {
    Box::new(OldArmenian)
}
