use morphe_train::spec::*;

pub struct Russian;

const CASES: &str = "Case[Nominative,Genitive,Dative,Accusative,Instrumental,Prepositional]";

impl Language for Russian {
    fn spec(&self) -> Spec {
        let mut s = Spec::new("russian", "Russian", "ru");
        s.marks = vec!['\u{301}', '\u{300}'];
        s.fold = vec![('ё', 'е')];
        // Pre-reform spellings come as whole "dated" tables.
        s.drop_tags.extend(tags("dated"));
        s.soft_tags.extend(tags("poetic"));
        s.features = vec![
            feature(
                "Case",
                &[
                    ("Nominative", "nominative"),
                    ("Genitive", "genitive !partitive"),
                    ("Dative", "dative"),
                    ("Accusative", "accusative"),
                    ("Instrumental", "instrumental"),
                    ("Prepositional", "prepositional !locative"),
                    ("Partitive", "partitive"),
                    ("Locative", "locative"),
                    ("Vocative", "vocative"),
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
            feature("Animacy", &[("Animate", "animate"), ("Inanimate", "inanimate")]),
            feature("Aspect", &[("Imperfective", "imperfective"), ("Perfective", "perfective")]),
            feature("Tense", &[("Present", "present"), ("Future", "future"), ("Past", "past")]),
            feature("Person", &[("First", "first-person"), ("Second", "second-person"), ("Third", "third-person")]),
            feature("Voice", &[("Active", "active"), ("Passive", "passive")]),
        ];
        s.pos = vec![
            pos("noun", "noun", vec![group("", "", "Case Number")])
                .inherent("Gender Animacy")
                .citation("NominativeSingular GenitiveSingular")
                .head_last(),
            pos(
                "adj",
                "adj",
                vec![
                    group("", "!short-form !comparative !superlative", &format!("{CASES} GenderNumber Animacy?")),
                    group("Short", "short-form", "GenderNumber"),
                    group("Comparative", "comparative", ""),
                    group("Superlative", "superlative", ""),
                ],
            ),
            pos(
                "verb",
                "verb",
                vec![
                    group("Infinitive", "infinitive", ""),
                    group("", "!participle !imperative !adverbial !infinitive", "Tense[Present,Future] Person Number"),
                    group("Past", "past !participle !adverbial", "GenderNumber"),
                    group("Imperative", "imperative", "Number"),
                    group("Participle", "participle !adverbial", "Voice Tense[Present,Past]"),
                    group("Gerund", "adverbial participle", "Tense[Present,Past]"),
                ],
            )
            .inherent("Aspect")
            .citation("Infinitive PresentFirstSingular|FutureFirstSingular PresentSecondSingular|FutureSecondSingular"),
            pos("pron", "pron", vec![group("", "", &format!("{CASES} GenderNumber? Animacy?"))]),
            pos("num", "num", vec![group("", "", &format!("{CASES} Gender? Animacy?"))]),
        ];
        s
    }

    fn readme(&self) -> String {
        r#"## Example

```rust
use russian::*;
assert_eq!(noun("книга", Case::Genitive, Number::Plural), "книг");
let plain = Russian::default().marks(false);
println!("{}", plain.noun("стол", Case::Instrumental, Number::Singular));
println!("{:?}", noun_gender("тетрадь"));
println!("{}", verb("писать", Tense::Present, Person::First, Number::Singular));
// Citations pin principal parts: nominative and genitive singular.
println!("{}", noun("путь, пути", Case::Instrumental, Number::Singular));
```

Input matching ignores stress marks and treats ё as е. Output carries stress
marks where Wiktionary has them (`marks(false)` removes them). Unknown words
get their stress predicted from similar words, which is often wrong.
"#
        .into()
    }
}

pub fn language() -> Box<dyn Language> {
    Box::new(Russian)
}
