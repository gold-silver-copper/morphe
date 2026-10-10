use morphe_train::spec::*;

pub struct Polish;

fn has(tags: &[String], t: &str) -> bool {
    tags.iter().any(|x| x == t)
}

const PERSONS: [&str; 3] = ["first-person", "second-person", "third-person"];
const CASES: [&str; 7] = ["nominative", "genitive", "dative", "accusative", "instrumental", "locative", "vocative"];

impl Language for Polish {
    fn spec(&self) -> Spec {
        let mut s = Spec::new("polish", "Polish", "pl");
        s.drop_tags.extend(tags("dated Middle"));
        s.soft_tags.extend(tags("poetic literary derogatory regional Łowicz"));
        s.features = vec![
            feature(
                "Case",
                &[
                    ("Nominative", "nominative"),
                    ("Genitive", "genitive"),
                    ("Dative", "dative"),
                    ("Accusative", "accusative"),
                    ("Instrumental", "instrumental"),
                    ("Locative", "locative"),
                    ("Vocative", "vocative"),
                ],
            ),
            feature("Number", &[("Singular", "singular"), ("Plural", "plural")]),
            // A noun's gender: the masculine splits into personal, animal and
            // inanimate.
            feature(
                "Gender",
                &[
                    ("MasculinePersonal", "masculine person"),
                    ("MasculineAnimal", "masculine animal-not-person"),
                    ("MasculineInanimate", "masculine inanimate"),
                    ("Feminine", "feminine"),
                    ("Neuter", "neuter"),
                ],
            ),
            // What adjectives, past forms and participles agree with.
            feature(
                "GenderNumber",
                &[
                    ("Masculine", "masculine singular"),
                    ("Feminine", "feminine singular"),
                    ("Neuter", "neuter singular"),
                    ("Virile", "virile plural"),
                    ("Nonvirile", "nonvirile plural"),
                ],
            ),
            // The same, for pronoun and numeral tables that may name none.
            feature(
                "Agreement",
                &[
                    ("Masculine", "masculine"),
                    ("Feminine", "feminine"),
                    ("Neuter", "neuter"),
                    ("Virile", "virile"),
                    ("Nonvirile", "nonvirile"),
                ],
            ),
            feature("Animacy", &[("Animate", "animate"), ("Inanimate", "inanimate")]),
            feature("Aspect", &[("Imperfective", "imperfective"), ("Perfective", "perfective")]),
            feature("Person", &[("First", "first-person"), ("Second", "second-person"), ("Third", "third-person")]),
            feature("Voice", &[("Active", "active"), ("Passive", "passive")]),
            feature("Tense", &[("Present", "present"), ("Future", "future"), ("Past", "past")]),
        ];
        let adjlike = "Case GenderNumber Animacy?";
        let finite = "!impersonal !masculine !feminine !neuter !virile !nonvirile";
        s.pos = vec![
            pos("noun", "noun", vec![group("", "", "Case Number")])
                .inherent("Gender")
                .citation("NominativeSingular GenitiveSingular")
                .head_last(),
            pos(
                "adj",
                "adj",
                vec![
                    group("", "!comparative !superlative", adjlike),
                    group("Comparative", "comparative", ""),
                    group("Superlative", "superlative", ""),
                ],
            ),
            pos(
                "verb",
                "verb",
                vec![
                    group("Infinitive", "infinitive", ""),
                    group("Present", &format!("present {finite}"), "Person Number"),
                    group("Future", &format!("future {finite}"), "Person Number"),
                    group("Past", "past !impersonal !conditional", "Person GenderNumber"),
                    group("Conditional", "conditional !impersonal", "Person GenderNumber"),
                    group("Imperative", "imperative", "Person Number"),
                    group("Impersonal", "impersonal !conditional", "Tense"),
                    group("ImpersonalConditional", "impersonal conditional", ""),
                    group("Participle", "adjectival participle masculine singular", "Voice").declines("adj", ""),
                    group("ContemporaryParticiple", "contemporary participle", ""),
                    group("AnteriorParticiple", "anterior participle", ""),
                    group("VerbalNoun", "noun-from-verb", ""),
                ],
            )
            .inherent("Aspect")
            .citation("Infinitive PresentFirstSingular|FutureFirstSingular PresentThirdSingular|FutureThirdSingular"),
            pos("pron", "pron", vec![group("", "", "Case Number Agreement? Animacy?")]),
            pos("det", "det", vec![group("", "", adjlike)]),
            pos("num", "num", vec![group("", "", "Case Number Agreement? Animacy?")]),
        ];
        s
    }

    /// Wiktionary's Polish tables lose tags in merged headers: the
    /// "nominative/vocative" row and the "nonvirile" column of adjective
    /// tables come back as `error-unrecognized-form`, and verb tables name
    /// only the third person. The first and second persons are read off the
    /// endings, which are regular (-m/-ę, -sz/-ś, -my, -cie; "niech …" for the
    /// first-person imperative).
    fn retag(&self, pos: &PosSpec, form: &str, t: &mut Vec<String>) {
        if let Some(i) = t.iter().position(|x| x == "error-unrecognized-form") {
            t.remove(i);
            if !CASES.iter().any(|c| has(t, c)) {
                t.push("nominative".into());
            }
            if has(t, "plural") && !has(t, "virile") {
                t.push("nonvirile".into());
            }
        }
        if pos.name != "verb" || form.is_empty() || PERSONS.iter().any(|p| has(t, p)) || has(t, "impersonal") {
            return;
        }
        let finite = ["present", "future", "past", "conditional", "imperative"].iter().any(|x| has(t, x));
        let (sg, pl) = (has(t, "singular"), has(t, "plural"));
        if !finite || sg == pl {
            return;
        }
        let first = form.split_whitespace().next().unwrap_or("");
        let person = if has(t, "imperative") {
            match (first, sg) {
                ("niech", _) => "first-person",
                (w, false) if w.ends_with("my") => "first-person",
                _ => "second-person",
            }
        } else if pl {
            if first.ends_with("my") {
                "first-person"
            } else if first.ends_with("cie") {
                "second-person"
            } else {
                return;
            }
        } else if first.ends_with("sz") || first.ends_with('ś') {
            "second-person"
        } else if first.ends_with('m') || first.ends_with('ę') {
            "first-person"
        } else {
            return;
        };
        t.push(person.into());
    }

    fn readme(&self) -> String {
        r#"## Example

```rust
use polish::*;
println!("{}", noun("kobieta", Case::Instrumental, Number::Plural));
println!("{}", noun("pies", Case::Genitive, Number::Singular));
println!("{:?}", noun_gender("nauczyciel"));
println!("{}", verb_present("pisać", Person::First, Number::Singular));
println!("{}", verb_past("zrobić", Person::Third, GenderNumber::Virile));
println!("{}", adj("dobry", Case::Locative, GenderNumber::Feminine, Animacy::Inanimate));
// Citations pin principal parts: nominative and genitive singular.
println!("{}", noun("kot, kota", Case::Dative, Number::Singular));
```

Input is matched as written: Polish diacritics (ą, ć, ę, ł, ń, ó, ś, ź, ż)
are letters, not optional marks.
"#
        .into()
    }
}

pub fn language() -> Box<dyn Language> {
    Box::new(Polish)
}
