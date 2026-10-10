use morphe_train::kaikki::{Entry, Sense};
use morphe_train::spec::*;

pub struct Pashto;

impl Language for Pashto {
    fn spec(&self) -> Spec {
        let mut s = Spec::new("pashto", "Pashto", "ps");
        // Short-vowel marks (اَسَد) are usually left out.
        s.marks = ('\u{64b}'..='\u{652}').chain(['\u{670}']).collect();
        s.features = vec![
            feature("Case", &[("Direct", "direct"), ("Oblique", "oblique !error-unrecognized-form"), ("Vocative", "vocative")]),
            feature("Number", &[("Singular", "singular"), ("Plural", "plural")]),
            feature("Gender", &[("Masculine", "masculine"), ("Feminine", "feminine")]),
            feature("Aspect", &[("Imperfective", "imperfective"), ("Perfective", "perfective")]),
            feature("Person", &[("First", "first-person"), ("Second", "second-person"), ("Third", "third-person")]),
            feature(
                "PastPerson",
                &[
                    ("First", "first-person"),
                    ("Second", "second-person"),
                    ("ThirdMasculine", "third-person masculine"),
                    ("ThirdFeminine", "third-person feminine"),
                ],
            ),
        ];
        s.pos = vec![
            pos("noun", "noun", vec![group("", "", "Case Number")])
                .inherent("Gender")
                .citation("DirectSingular DirectPlural")
                .head_last(),
            pos("adj", "adj", vec![group("", "", "Gender Case Number")]).head_last(),
            pos(
                "verb",
                "verb",
                vec![
                    group("Present", "present !perfect", "Aspect Person Number"),
                    group("Past", "past !perfect", "Aspect PastPerson Number"),
                    group("Imperative", "imperative !negative !perfect", "Number"),
                    group("NegativeImperative", "imperative negative !perfect", "Number"),
                ],
            )
            .head_last(),
        ];
        s
    }

    /// Noun genders are named only by the declension template
    /// (ps-noun-f-aleph); the imperative table's rows name only the person.
    fn prepare(&self, e: &mut Entry) {
        let template = e.forms.iter().find(|f| f.tags.iter().any(|t| t == "inflection-template")).map(|f| f.form.clone());
        if e.pos == "noun" {
            let gender = template.as_deref().and_then(|t| {
                if t.starts_with("ps-noun-m") {
                    Some("masculine")
                } else if t.starts_with("ps-noun-f") {
                    Some("feminine")
                } else {
                    None
                }
            });
            if let Some(g) = gender {
                e.senses.push(Sense { tags: vec![g.into()] });
            }
            return;
        }
        if e.pos != "verb" {
            return;
        }
        let mut table = 0usize;
        for f in e.forms.iter_mut().filter(|f| f.source.is_some()) {
            if f.tags.iter().any(|t| t == "table-tags") {
                table += 1;
                continue;
            }
            let has = |t: &str| f.tags.iter().any(|x| x == t);
            if table >= 2 && has("second-person") && !has("present") && !has("past") && !has("perfect") && !has("imperative") {
                f.tags.push("imperative".into());
            }
        }
    }

    /// Cells carry a romanization ("کوم (kawә́m)"), list alternatives with
    /// a backslash ("وو\ول"), and sometimes use Arabic letter variants.
    fn split_form(&self, raw: &str) -> Vec<String> {
        let main = raw.split(" (").next().unwrap_or(raw);
        let main: String = main
            .chars()
            .map(|c| match c {
                '\u{fe89}' | '\u{fe8a}' | '\u{fe8b}' | '\u{fe8c}' => 'ئ',
                'ك' => 'ک',
                'ى' => 'ی',
                c => c,
            })
            .collect();
        // Romanized rows ("anāgā́ne anāwe") repeat cells in Latin script.
        if main.chars().any(|c| c.is_alphabetic() && !('\u{600}'..='\u{6ff}').contains(&c)) {
            return Vec::new();
        }
        main.split('\\').map(|s| s.trim().to_string()).filter(|s| !s.is_empty()).collect()
    }

    fn samples(&self) -> Vec<&'static str> {
        vec!["انا", "سړی", "کتاب", "پروت", "کول", "لیکل", "ښار"]
    }

    fn readme(&self) -> String {
        r#"## Example

```rust
use pashto::*;
println!("{}", noun("انا", Case::Direct, Number::Plural)); // اناګانې
println!("{:?}", noun_gender("انا")); // Feminine
println!("{}", adj("پروت", Gender::Feminine, Case::Direct, Number::Singular)); // پرته
println!("{}", verb_present("کول", Aspect::Imperfective, Person::First, Number::Singular)); // کوم
println!("{}", verb_past("کول", Aspect::Perfective, PastPerson::ThirdMasculine, Number::Singular)); // کړ
```

Nouns have the direct, oblique and vocative in both numbers and their
gender; adjectives have them for both genders. Verbs have the present and
the past, imperfective and perfective, for six persons (with the third
person's gender in the past), and the imperative, positive and negative.
Input matching ignores the short-vowel marks.
"#
        .into()
    }
}

pub fn language() -> Box<dyn Language> {
    Box::new(Pashto)
}
