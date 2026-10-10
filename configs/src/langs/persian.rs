use morphe_train::kaikki::Entry;
use morphe_train::spec::*;

pub struct Persian;

impl Language for Persian {
    fn spec(&self) -> Spec {
        let mut s = Spec::new("persian", "Persian", "fa");
        // Harakat, dagger alif, tatweel and the zero-width non-joiner of
        // می‌خورم: written in the dictionary, often left out when typing.
        s.marks = ('\u{64B}'..='\u{65F}').chain(['\u{670}', '\u{640}', '\u{200C}']).collect();
        // Arabic yeh and kaf are typed for the Persian letters.
        s.fold = vec![('ي', 'ی'), ('ك', 'ک'), ('ى', 'ی')];
        s.features = vec![
            feature("Number", &[("Singular", "singular"), ("Plural", "plural")]),
            feature("Person", &[("First", "first-person"), ("Second", "second-person"), ("Third", "third-person")]),
            feature(
                "Tense",
                &[
                    ("Present", "indicative present !progressive !perfect"),
                    ("PresentProgressive", "present progressive"),
                    ("Aorist", "aorist"),
                    ("Past", "indicative past !progressive !imperfect"),
                    ("Imperfect", "imperfect"),
                    ("PastProgressive", "past progressive"),
                    ("Perfect", "perfect"),
                    ("Pluperfect", "pluperfect"),
                    ("Future", "future"),
                    ("PresentSubjunctive", "present subjunctive"),
                    ("PastSubjunctive", "past subjunctive"),
                ],
            ),
            feature("Stem", &[("Present", "present"), ("Past", "past")]),
            feature("Degree", &[("Comparative", "comparative !superlative"), ("Superlative", "superlative")]),
        ];
        s.pos = vec![
            pos("noun", "noun", vec![group("", "", "Number[Plural]")]).citation("Lemma Plural").head_last(),
            pos("adj", "adj", vec![group("", "", "Degree")]).citation("Lemma Comparative Superlative").head_last(),
            pos(
                "verb",
                "verb",
                vec![
                    group("Infinitive", "infinitive", ""),
                    group("Stem", "stem", "Stem"),
                    group("Participle", "participle", "Stem"),
                    group("", "!imperative !participle !stem", "Tense Person Number"),
                    group("Imperative", "imperative second-person", "Number"),
                ],
            )
            .citation("Infinitive StemPresent")
            .head_last(),
        ];
        s
    }

    /// The conjugation template prints a formal table, then a colloquial
    /// one (خورم, خوره…); only the formal one is read. Its imperfect rows
    /// (می‌خوردم) carry the same tags as the simple past, and come second.
    ///
    /// Noun and adjective tables (possessive, predicative, colloquial
    /// forms) exist for a handful of words only; the headword line is read.
    fn prepare(&self, e: &mut Entry) {
        if e.pos != "verb" {
            e.forms.retain(|f| f.source.is_none());
            return;
        }
        let mut table = 0usize;
        let mut seen: Vec<Vec<String>> = Vec::new();
        e.forms.retain_mut(|f| {
            if f.source.is_none() {
                return true;
            }
            if f.tags.iter().any(|t| t == "table-tags") {
                table += 1;
                seen.clear();
                return false;
            }
            if table > 1 {
                return false;
            }
            let mut key = f.tags.clone();
            key.sort();
            if key.iter().any(|t| t == "past") && key.iter().any(|t| t == "indicative") && seen.contains(&key) {
                f.tags.push("imperfect".into());
            }
            seen.push(key);
            true
        });
    }

    fn samples(&self) -> Vec<&'static str> {
        vec!["خوردن", "رفتن", "کتاب", "زن", "بزرگ", "خوب", "پرسیدن", "گلبیدن"]
    }

    fn readme(&self) -> String {
        r#"## Example

```rust
use persian::*;
println!("{}", verb("خوردن", Tense::Present, Person::First, Number::Singular)); // میخورم
println!("{}", verb("رفتن", Tense::Past, Person::Third, Number::Plural)); // رفتند
println!("{}", verb("کردن", Tense::PresentSubjunctive, Person::Second, Number::Singular)); // بکنی
println!("{}", verb_stem("آمدن", Stem::Present)); // آ
println!("{}", noun("کتاب", Number::Plural));
println!("{}", adj("بزرگ", Degree::Comparative)); // بزرگتر
```

Cells cover the infinitive, both stems and participles, eleven indicative
and subjunctive tenses (including the periphrastic progressive, perfect,
pluperfect and future) in all persons, and the imperative. Wiktionary's
colloquial conjugation table is left out. Nouns have the plural and
adjectives the comparative and superlative, from the headword line.

Input matching ignores harakat, tatweel and the zero-width non-joiner, and
treats Arabic ي and ك as ی and ک. Wiktionary writes most table forms without
the non-joiner (میخورم for می‌خورم).
"#
        .into()
    }
}

pub fn language() -> Box<dyn Language> {
    Box::new(Persian)
}
