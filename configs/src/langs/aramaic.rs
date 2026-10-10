use morphe_train::kaikki::{Entry, Form};
use morphe_train::spec::*;

pub struct Aramaic;

fn is_point(c: char) -> bool {
    matches!(c, '\u{591}'..='\u{5BD}' | '\u{5BF}' | '\u{5C1}' | '\u{5C2}' | '\u{5C4}' | '\u{5C5}' | '\u{5C7}')
}

impl Language for Aramaic {
    fn spec(&self) -> Spec {
        let mut s = Spec::new("aramaic", "Aramaic", "arc");
        // Vowel points and accents: written in the dictionary, rarely typed.
        s.marks = ('\u{591}'..='\u{5C7}').filter(|&c| is_point(c)).collect();
        // Each verb lists several dialects' tables; the first is the main one.
        s.first_table_only = true;
        s.features = vec![
            feature("State", &[("Absolute", "absolute"), ("Construct", "construct"), ("Emphatic", "emphatic")]),
            feature("Number", &[("Singular", "singular"), ("Plural", "plural")]),
            feature("Gender", &[("Masculine", "masculine"), ("Feminine", "feminine")]),
            feature(
                "Person",
                &[
                    ("FirstSingular", "first-person singular"),
                    ("SecondMasculineSingular", "second-person masculine singular"),
                    ("SecondFeminineSingular", "second-person feminine singular"),
                    ("ThirdMasculineSingular", "third-person masculine singular"),
                    ("ThirdFeminineSingular", "third-person feminine singular"),
                    ("FirstPlural", "first-person plural"),
                    ("SecondMasculinePlural", "second-person masculine plural"),
                    ("SecondFemininePlural", "second-person feminine plural"),
                    ("ThirdMasculinePlural", "third-person masculine plural"),
                    ("ThirdFemininePlural", "third-person feminine plural"),
                ],
            ),
            feature(
                "Imperative",
                &[
                    ("MasculineSingular", "masculine singular"),
                    ("FeminineSingular", "feminine singular"),
                    ("MasculinePlural", "masculine plural"),
                    ("FemininePlural", "feminine plural"),
                ],
            ),
            feature("Tense", &[("Past", "past"), ("Future", "future")]),
        ];
        s.pos = vec![
            pos("noun", "noun", vec![group("", "", "State Number")])
                .inherent("Gender")
                .citation("EmphaticSingular EmphaticPlural")
                .head_last(),
            pos("adj", "adj", vec![group("", "", "State Gender Number")])
                .citation("AbsoluteMasculineSingular AbsoluteFeminineSingular AbsoluteMasculinePlural")
                .head_last(),
            pos(
                "verb",
                "verb",
                vec![
                    group("", "!imperative !present", "Tense Person"),
                    group("Imperative", "imperative", "Imperative"),
                    group("Participle", "x-participle", "Imperative"),
                ],
            )
            .citation("PastThirdMasculineSingular FutureThirdMasculineSingular")
            .head_last(),
        ];
        s
    }

    /// Headword-line forms name no state: a noun's or adjective's plural and
    /// feminine are in the lemma's own state (emphatic if it ends in א,
    /// else absolute). The present participle comes untagged.
    fn prepare(&self, e: &mut Entry) {
        // Forms are read unpointed: pointing is inconsistent across the
        // dialects' tables, and stacked points (בְּ) don't survive the
        // runtime's mark check.
        let header = |f: &Form| f.tags.iter().any(|t| t == "table-tags" || t == "inflection-template");
        e.forms.retain(|f| header(f) || !f.form.chars().any(|c| c.is_ascii_alphabetic()));
        for f in e.forms.iter_mut().filter(|f| !header(f)) {
            f.form.retain(|c| !is_point(c));
        }
        if e.pos == "noun" && e.word.ends_with('א') && !e.forms.iter().any(|f| f.tags.iter().any(|t| t == "emphatic")) {
            e.forms.push(Form { form: e.word.clone(), tags: tags("emphatic singular"), source: None });
        }
        let lemma_state = if e.word.ends_with('א') { "emphatic" } else { "absolute" };
        let adj = e.pos == "adj";
        for f in e.forms.iter_mut() {
            let has = |t: &str| f.tags.iter().any(|x| x == t);
            if f.source.is_none() && (e.pos == "noun" || adj) {
                let new = match f.tags.as_slice() {
                    [t] if t == "plural" => Some(if adj { format!("{lemma_state} masculine plural") } else { format!("{lemma_state} plural") }),
                    [t] if t == "feminine" && adj => Some(format!("{lemma_state} feminine singular")),
                    [t] if t == "absolute" || t == "construct" => Some(format!("{t} singular")),
                    _ => None,
                };
                if let Some(n) = new {
                    f.tags = tags(&n);
                }
            } else if has("present") && has("error-unrecognized-form") {
                f.tags.retain(|t| t != "error-unrecognized-form" && t != "present");
                f.tags.push("x-participle".into());
            }
        }
    }

    fn samples(&self) -> Vec<&'static str> {
        vec!["כתב", "ברא", "מלכא", "שמיא", "טב", "גלבא"]
    }

    fn readme(&self) -> String {
        r#"## Example

```rust
use aramaic::*;
println!("{}", noun("ברא", State::Absolute, Number::Plural)); // בנין
println!("{}", noun("ברא", State::Construct, Number::Plural)); // בני
println!("{:?}", noun_gender("מלכא"));
println!("{}", verb("כתב", Tense::Past, Person::FirstSingular)); // כתבת
println!("{}", verb("כתב", Tense::Future, Person::ThirdMasculinePlural)); // יכתבון
println!("{}", verb_imperative("כתב", Imperative::MasculineSingular)); // כתב
```

Nouns have the absolute, construct and emphatic states in both numbers,
and their gender; adjectives agree in gender too. Verbs have the past and
future for ten persons, the imperative and the active participle. Input
matching ignores vowel points, and forms are given unpointed, as Aramaic is
usually written. Words are in Hebrew square script.
"#
        .into()
    }
}

pub fn language() -> Box<dyn Language> {
    Box::new(Aramaic)
}
