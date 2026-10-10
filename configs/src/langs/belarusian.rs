use morphe_train::spec::*;

pub struct Belarusian;

const CASES: &str = "Case";

impl Language for Belarusian {
    fn spec(&self) -> Spec {
        let mut s = Spec::new("belarusian", "Belarusian", "be");
        s.marks = vec!['\u{301}', '\u{300}'];
        s.fold = vec![('’', '\''), ('ʼ', '\'')];
        s.drop_tags.extend(tags("dated"));
        // Polite plural-as-singular (Ви) is an alternative to ти.
        s.soft_tags.extend(tags("poetic formal"));
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
                ],
            ),
            feature("Number", &[("Singular", "singular"), ("Plural", "plural")]),
            feature("Gender", &[("Masculine", "masculine"), ("Feminine", "feminine"), ("Neuter", "neuter")]),
            feature(
                "GenderNumber",
                &[
                    ("Masculine", "masculine !plural"),
                    ("Feminine", "feminine !plural"),
                    ("Neuter", "neuter !plural"),
                    ("Plural", "plural"),
                ],
            ),
            // Nouns are tagged person / animal / inanimate; adjective
            // accusatives only animate / inanimate.
            feature(
                "Animacy",
                &[
                    ("Animate", "animate"),
                    ("Inanimate", "inanimate"),
                    ("Personal", "person"),
                    ("Animal", "animal-not-person"),
                ],
            ),
            feature("Aspect", &[("Imperfective", "imperfective"), ("Perfective", "perfective")]),
            feature("Tense", &[("Present", "present"), ("Future", "future"), ("Past", "past")]),
            feature("Person", &[("First", "first-person"), ("Second", "second-person"), ("Third", "third-person")]),
            feature(
                "PronPerson",
                &[("First", "first-person"), ("Second", "second-person"), ("Third", "third-person"), ("Reflexive", "reflexive")],
            ),
            feature("Voice", &[("Active", "active"), ("Passive", "passive")]),
        ];
        let adjlike = format!("{CASES} GenderNumber Animacy[Animate,Inanimate]?");
        s.pos = vec![
            pos(
                "noun",
                "noun",
                vec![group("", "!vocative !count-form", "Case Number"), group("Count", "count-form plural", "")],
            )
                .inherent("Gender Animacy")
                .citation("NominativeSingular GenitiveSingular")
                .head_last(),
            pos(
                "adj",
                "adj",
                vec![
                    group("", "!comparative !superlative", &adjlike),
                    group("Comparative", "comparative", ""),
                    group("Superlative", "superlative", ""),
                ],
            ),
            pos(
                "verb",
                "verb",
                vec![
                    group("Infinitive", "infinitive", ""),
                    group("", "!imperative !adverbial !infinitive", "Tense[Present,Future] Person Number"),
                    group("Past", "past !adverbial !passive !active", "GenderNumber"),
                    group("Imperative", "imperative second-person", "Number"),
                    group("Participle", "!adverbial", "Voice Tense[Present,Past]").declines("adj", ""),
                    group("Gerund", "adverbial", "Tense[Present,Past]"),
                ],
            )
            .inherent("Aspect")
            .citation("Infinitive PresentFirstSingular|FutureFirstSingular PresentThirdPlural|FutureThirdPlural"),
            pos(
                "pron",
                "pron",
                vec![
                    // Every personal pronoun's entry carries the whole table.
                    group("Personal", "personal", &format!("{CASES} PronPerson Number? Gender?")),
                    group("", "!personal", &format!("{CASES} Number? Gender? Animacy[Animate,Inanimate]?")),
                ],
            ),
            pos("det", "det", vec![group("", "", &adjlike)]),
            pos("num", "num", vec![group("", "", &format!("{CASES} Number? Gender? Animacy[Animate,Inanimate]?"))]),
        ];
        s
    }

    fn readme(&self) -> String {
        r#"## Example

```rust
use belarusian::*;
println!("{}", noun("кніга", Case::Genitive, Number::Plural));
println!("{}", noun("горад", Case::Locative, Number::Singular));
println!("{:?}", noun_gender("ноч"));
println!("{}", verb("пісаць", Tense::Present, Person::First, Number::Singular));
println!("{}", verb_past("быць", GenderNumber::Feminine));
println!("{}", adj("добры", Case::Dative, GenderNumber::Feminine, Animacy::Inanimate));
let plain = Belarusian::default().marks(false);
println!("{}", plain.noun("брат", Case::Instrumental, Number::Plural));
// Citations pin principal parts: nominative and genitive singular.
println!("{}", noun("кот, ката", Case::Instrumental, Number::Singular));
```

Input matching ignores stress marks and treats ’ and ʼ as the apostrophe '.
Output carries stress marks where Wiktionary has them (`marks(false)` removes
them). Unknown words get their stress predicted from similar words, which is
often wrong.
"#
        .into()
    }
}

pub fn language() -> Box<dyn Language> {
    Box::new(Belarusian)
}
