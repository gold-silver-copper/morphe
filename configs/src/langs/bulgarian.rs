use morphe_train::spec::*;

pub struct Bulgarian;

impl Language for Bulgarian {
    fn spec(&self) -> Spec {
        let mut s = Spec::new("bulgarian", "Bulgarian", "bg");
        s.marks = vec!['\u{301}', '\u{300}'];
        s.drop_tags.extend(tags("dated"));
        s.soft_tags.extend(tags("poetic regional"));
        s.sentinels.push("none".into());
        s.features = vec![
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
            // The masculine singular has a full (subject) and a short
            // (object) definite article; elsewhere both are the same form.
            feature(
                "Article",
                &[
                    ("Indefinite", "indefinite"),
                    ("Definite", "definite !objective"),
                    ("DefiniteShort", "definite !subjective"),
                ],
            ),
            feature("Case", &[("Nominative", "nominative"), ("Accusative", "accusative"), ("Dative", "dative")]),
            feature("Aspect", &[("Imperfective", "imperfective"), ("Perfective", "perfective")]),
            feature("Person", &[("First", "first-person"), ("Second", "second-person"), ("Third", "third-person")]),
        ];
        let adjlike = "Article GenderNumber";
        s.pos = vec![
            pos(
                "noun",
                "noun",
                vec![
                    group("", "!count-form !vocative", "Article Number"),
                    group("Count", "count-form plural", ""),
                    group("Vocative", "vocative", "Number"),
                ],
            )
            .inherent("Gender")
            .citation("IndefiniteSingular IndefinitePlural")
            .head_last(),
            pos(
                "adj",
                "adj",
                vec![
                    group("", "!comparative !superlative", adjlike),
                    group("Comparative", "comparative", adjlike),
                    group("Superlative", "superlative", adjlike),
                ],
            ),
            pos(
                "verb",
                "verb",
                vec![
                    group("Present", "indicative present", "Person Number"),
                    group("Imperfect", "indicative imperfect", "Person Number"),
                    group("Aorist", "indicative aorist", "Person Number"),
                    group("Imperative", "imperative second-person", "Number"),
                    group("AoristParticiple", "active aorist participle", adjlike),
                    group("ImperfectParticiple", "active imperfect participle indefinite", "GenderNumber"),
                    group("PresentParticiple", "active participle present", adjlike),
                    group("PassiveParticiple", "participle passive past", adjlike),
                    group("AdverbialParticiple", "adverbial participle indefinite masculine", ""),
                    group("VerbalNoun", "noun-from-verb", "Article Number"),
                ],
            )
            .inherent("Aspect")
            .citation("PresentFirstSingular AoristFirstSingular"),
            pos(
                "pron",
                "pron",
                vec![
                    group("", "!short-form", "Case? Article? GenderNumber"),
                    group("Short", "short-form", ""),
                ],
            ),
            pos("det", "det", vec![group("", "", "Case? Article? GenderNumber")]),
            pos(
                "num",
                "num",
                vec![group("", "!count-form", "Article GenderNumber?"), group("Count", "count-form plural", "")],
            ),
        ];
        s
    }

    /// Possessive pronoun tables (мой, твой) come tagged "comparative"; the
    /// verbal noun's singular is tagged only "neuter".
    fn retag(&self, pos: &PosSpec, _form: &str, t: &mut Vec<String>) {
        if pos.name == "pron" {
            t.retain(|x| x != "comparative");
        }
        if t.iter().any(|x| x == "noun-from-verb") && !t.iter().any(|x| x == "plural") {
            t.push("singular".into());
        }
    }

    fn readme(&self) -> String {
        r#"## Example

```rust
use bulgarian::*;
println!("{}", noun("град", Article::Definite, Number::Singular));
println!("{}", noun("град", Article::DefiniteShort, Number::Singular));
println!("{}", noun("жена", Article::Indefinite, Number::Plural));
println!("{:?}", noun_gender("нощ"));
println!("{}", verb_present("чета", Person::Second, Number::Singular));
println!("{}", verb_aorist("чета", Person::First, Number::Singular));
println!("{}", adj("хубав", Article::Definite, GenderNumber::Feminine));
// Citations pin principal parts: singular and plural.
println!("{}", noun("крак, крака", Article::Definite, Number::Plural));
```

Verbs are cited, as in Bulgarian dictionaries, by the first person singular
present. Input matching ignores stress marks; output carries them where
Wiktionary has them (`marks(false)` removes them).
"#
        .into()
    }
}

pub fn language() -> Box<dyn Language> {
    Box::new(Bulgarian)
}
