use morphe_train::kaikki::Entry;
use morphe_train::spec::*;

pub struct Macedonian;

impl Language for Macedonian {
    fn spec(&self) -> Spec {
        let mut s = Spec::new("macedonian", "Macedonian", "mk");
        s.marks = vec!['\u{301}', '\u{300}'];
        // Compound tenses come as descriptions ("ќе + present").
        s.drop_tags.extend(tags("multiword-construction"));
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
            // The three definite articles: unmarked (-от), proximal (-ов),
            // distal (-он).
            feature(
                "Article",
                &[
                    ("Indefinite", "indefinite"),
                    ("Definite", "definite unspecified"),
                    ("Proximal", "definite proximal"),
                    ("Distal", "definite distal"),
                ],
            ),
            feature("Aspect", &[("Imperfective", "imperfective"), ("Perfective", "perfective")]),
            feature("Person", &[("First", "first-person"), ("Second", "second-person"), ("Third", "third-person")]),
            feature(
                "Tense",
                &[("Present", "present !imperfect !aorist"), ("Imperfect", "imperfect"), ("Aorist", "aorist")],
            ),
        ];
        let adjlike = "Article GenderNumber";
        s.pos = vec![
            pos(
                "noun",
                "noun",
                vec![
                    group("", "!count-form !vocative", "Article Number"),
                    group("Vocative", "vocative", "Number"),
                    group("Count", "count-form plural", ""),
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
                    group("Comparative", "comparative", ""),
                    group("Superlative", "superlative", ""),
                ],
            )
            .head_last(),
            pos(
                "verb",
                "verb",
                vec![
                    group("", "!imperative !participle !noun-from-verb", "Tense Person Number"),
                    group("Imperative", "imperative second-person", "Number"),
                    group("LParticiple", "!participle", "Tense[Imperfect,Aorist] GenderNumber"),
                    group("PassiveParticiple", "adjectival participle", ""),
                    group("AdverbialParticiple", "adverbial participle", ""),
                    group("PerfectParticiple", "participle perfect", ""),
                    group("VerbalNoun", "noun-from-verb", ""),
                ],
            )
            .inherent("Aspect")
            .citation("PresentThirdSingular PresentFirstSingular")
            .head_first(),
        ];
        s
    }

    /// Adjective tables repeat the declension for the comparative (по-) and
    /// superlative (нај-) under the same tags; only the first is read, and
    /// the headword line gives the comparative and superlative.
    fn prepare(&self, e: &mut Entry) {
        if e.pos != "adj" {
            return;
        }
        let mut seen: Vec<Vec<String>> = Vec::new();
        e.forms.retain(|f| {
            if f.source.is_none() || f.tags.iter().any(|t| t == "table-tags" || t == "inflection-template") {
                return true;
            }
            let mut key = f.tags.clone();
            key.sort();
            if seen.contains(&key) {
                return false;
            }
            seen.push(key);
            true
        });
    }

    fn samples(&self) -> Vec<&'static str> {
        vec!["град", "жена", "дете", "пишува", "напише", "стар", "убав", "кродува"]
    }

    fn readme(&self) -> String {
        r#"## Example

```rust
use macedonian::*;
assert_eq!(noun("град", Article::Definite, Number::Singular), "градот");
println!("{}", noun("жена", Article::Proximal, Number::Plural)); // жениве
println!("{:?}", noun_gender("дете"));
println!("{}", adj("стар", Article::Indefinite, GenderNumber::Feminine)); // стара
println!("{}", verb("пишува", Tense::Imperfect, Person::First, Number::Plural)); // пишувавме
println!("{}", verb("напише", Tense::Aorist, Person::Third, Number::Singular)); // напиша
println!("{}", verb_l_participle("напише", Tense::Aorist, GenderNumber::Feminine)); // напишала
println!("{:?}", verb_aspect("пишува"));
```

Verbs are cited, as in Macedonian dictionaries, by the third person
singular present. They have the present, imperfect and aorist, the
imperative, the l-participles, the passive and adverbial participles, the
perfect participle and the verbal noun; compound tenses (ќе пишувам, сум
пишувал) are built from these. Nouns have the indefinite form and the three
definite articles in both numbers, the vocative and the count form. Input
matching ignores stress marks.
"#
        .into()
    }
}

pub fn language() -> Box<dyn Language> {
    Box::new(Macedonian)
}
