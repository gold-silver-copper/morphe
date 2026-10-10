use morphe_train::kaikki::Entry;
use morphe_train::spec::*;

pub struct ClassicalSyriac;

impl Language for ClassicalSyriac {
    fn spec(&self) -> Spec {
        let mut s = Spec::new("classical_syriac", "ClassicalSyriac", "syc");
        // Vowel points and other diacritics above and below the line: the
        // tables are unpointed, headword lines pointed.
        s.marks = ('\u{730}'..='\u{74A}').collect();
        s.features = vec![
            feature("State", &[("Absolute", "absolute"), ("Construct", "construct"), ("Emphatic", "emphatic")]),
            feature("Number", &[("Singular", "singular"), ("Plural", "plural")]),
            feature("Gender", &[("Masculine", "masculine"), ("Feminine", "feminine")]),
            feature(
                "Possessor",
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
        ];
        s.pos = vec![
            pos(
                "noun",
                "noun",
                vec![group("", "!possessed-form", "State Number"), group("Possessed", "possessed-form", "Possessor")],
            )
            .inherent("Gender")
            .citation("EmphaticSingular EmphaticPlural")
            .head_last(),
            pos("adj", "adj", vec![group("", "", "State Gender Number")]).head_last(),
        ];
        s
    }

    /// Forms are read unpointed, as the tables give them. Headword lines
    /// give the emphatic plural (and for adjectives the
    /// emphatic feminine) without naming the state. First-person possessed
    /// forms are tagged both masculine and feminine; the second and third
    /// persons need the gender to tell them apart, the first doesn't.
    fn prepare(&self, e: &mut Entry) {
        for f in &mut e.forms {
            f.form.retain(|c| !('\u{730}'..='\u{74A}').contains(&c));
            let has = |t: &str| f.tags.iter().any(|x| x == t);
            if f.source.is_none() {
                if e.pos == "noun" && f.tags[..] == ["plural"] {
                    f.tags = tags("emphatic plural");
                } else if e.pos == "adj" && (f.tags[..] == ["feminine"] || f.tags[..] == ["masculine", "plural"]) {
                    let num = if has("plural") { "" } else { " singular" };
                    f.tags = tags(&format!("{} emphatic{num}", f.tags.join(" ")));
                }
            } else if has("possessed-form") && has("first-person") {
                f.tags.retain(|t| t != "masculine" && t != "feminine");
            }
        }
    }

    fn samples(&self) -> Vec<&'static str> {
        vec!["ܟܠܒܐ", "ܡܠܟܐ", "ܒܝܬܐ", "ܫܦܝܪܐ", "ܓܪܒܢܐ"]
    }

    fn readme(&self) -> String {
        r#"## Example

```rust
use classical_syriac::*;
println!("{}", noun("ܟܠܒܐ", State::Absolute, Number::Singular)); // ܟܠܒ
println!("{}", noun("ܟܠܒܐ", State::Emphatic, Number::Plural));
println!("{}", noun_possessed("ܟܠܒܐ", Possessor::FirstSingular)); // ܟܠܒܝ
println!("{:?}", noun_gender("ܟܠܒܐ"));
```

Nouns have the absolute, construct and emphatic states in both numbers,
and the possessed forms for ten possessors; adjectives decline for state,
gender and number. Input matching ignores vowel points; output is
unpointed.
"#
        .into()
    }
}

pub fn language() -> Box<dyn Language> {
    Box::new(ClassicalSyriac)
}
