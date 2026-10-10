use morphe_train::kaikki::Entry;
use morphe_train::spec::*;

pub struct Coptic;

impl Language for Coptic {
    fn spec(&self) -> Spec {
        let mut s = Spec::new("coptic", "Coptic", "cop");
        // Supralinear strokes, jinkim and diaeresis: written in editions,
        // often left out when typing.
        s.marks = vec!['\u{300}', '\u{304}', '\u{305}', '\u{307}', '\u{308}', '\u{FE24}', '\u{FE25}', '\u{FE26}'];
        s.features = vec![
            feature("Gender", &[("Masculine", "masculine"), ("Feminine", "feminine")]),
            feature(
                "Person",
                &[
                    ("FirstSingular", "first-person singular"),
                    ("SecondMasculine", "second-person masculine singular"),
                    ("SecondFeminine", "second-person feminine singular"),
                    ("ThirdMasculine", "third-person masculine singular"),
                    ("ThirdFeminine", "third-person feminine singular"),
                    ("FirstPlural", "first-person plural"),
                    ("SecondPlural", "second-person plural"),
                    ("ThirdPlural", "third-person plural"),
                ],
            ),
            feature(
                "Tense",
                &[
                    ("Present", "durative present"),
                    ("Future", "durative future"),
                    ("Past", "non-durative past"),
                    ("Aorist", "aorist"),
                    ("Expectative", "expectative"),
                    ("Optative", "optative"),
                    ("Jussive", "jussive"),
                    ("Conjunctive", "conjunctive !future"),
                    ("FutureConjunctive", "conjunctive future"),
                    ("Conditional", "conditional long-form"),
                    ("Limitative", "limitative"),
                    ("Precursive", "precursive"),
                ],
            ),
        ];
        s.pos = vec![
            pos(
                "noun",
                "noun",
                vec![group("Plural", "plural", ""), group("Construct", "nominal", ""), group("Pronominal", "pronominal", "")],
            )
            .inherent("Gender")
            .citation("Lemma Plural")
            .head_last(),
            pos(
                "verb",
                "verb",
                vec![
                    group("Stative", "stative", ""),
                    group("Construct", "nominal !x-finite", ""),
                    group("Pronominal", "pronominal !x-finite", ""),
                    group("NegativeInfinitive", "infinitive negative", ""),
                    group("NegativeImperative", "negative !infinitive !x-finite", ""),
                    group("", "x-finite", "Tense Person"),
                ],
            )
            .citation("Lemma Stative")
            .head_last(),
        ];
        s
    }

    /// Headword-line states are written with their dialect labels
    /// ("form (S, A) ⲣⲛ̄ⲧ⸗") and with unknown-tag flags. In conjugation
    /// tables, the main (not relative, circumstantial, focalising or
    /// preterite) personal forms are read; forms with a nominal subject
    /// slot are left out, as are the untagged variant rows (negative,
    /// circumstantial) that repeat a row's tags.
    fn prepare(&self, e: &mut Entry) {
        e.forms.retain(|f| !f.form.starts_with("of "));
        // Untagged variant rows (negative, circumstantial…) repeat the
        // affirmative row's tags; only the first row of each is read.
        let mut seen: Vec<Vec<String>> = Vec::new();
        for f in &mut e.forms {
            f.tags.retain(|t| t != "error-unknown-tag" && t != "error-unrecognized-form");
            if f.source.is_none() && f.form.contains('(') {
                let before = f.form.split(" (").next().unwrap_or("").to_string();
                f.form = if f.form.starts_with("form") || before.is_empty() {
                    f.form.split_whitespace().last().unwrap_or("").to_string()
                } else {
                    before
                };
            }
            if f.source.is_some() {
                let has = |t: &str| f.tags.iter().any(|x| x == t);
                let finite = f.tags.iter().any(|t| t.ends_with("-person"))
                    && !["relative", "circumstantial", "focalising", "preterite", "nominal"].iter().any(|t| has(t));
                let mut key = f.tags.clone();
                key.sort();
                let repeat = seen.contains(&key);
                seen.push(key);
                if finite && !repeat {
                    f.tags.push("x-finite".into());
                } else if finite {
                    f.tags = tags("table-tags");
                } else if f.tags.iter().any(|t| t.ends_with("-person")) || has("nominal") {
                    f.tags = tags("table-tags");
                }
            }
        }
    }

    fn samples(&self) -> Vec<&'static str> {
        vec!["ϣⲱⲡⲓ", "ϣⲱⲡⲉ", "ⲥⲁϫⲓ", "ⲣⲱⲙⲉ", "ϩⲧⲟ", "ⲕⲱⲧ"]
    }

    fn readme(&self) -> String {
        r#"## Example

```rust
use coptic::*;
println!("{}", verb("ⲥⲁϫⲓ", Tense::Past, Person::ThirdMasculine)); // ⲁϥⲥⲁϫⲓ
println!("{}", verb("ⲥⲁϫⲓ", Tense::Aorist, Person::FirstPlural)); // ϣⲁⲛⲥⲁϫⲓ
println!("{}", verb("ϣⲱⲡⲉ", Tense::Future, Person::SecondFeminine));
println!("{}", verb_stative("ϣⲱⲡⲉ")); // ϣⲟⲟⲡ
println!("{}", noun_plural("ϩⲧⲟ"));
```

Verbs have their stative and construct states where Wiktionary gives them,
the negative infinitive and imperative, and the main conjugations (present,
future, past, aorist, expectative, optative, jussive, conjunctives,
conditional, limitative, precursive) for eight persons; Coptic builds
these with prefixes on an unchanging infinitive. Each dialect (Sahidic,
Bohairic, …) spells its own entries. Nouns have the plural, the construct
and pronominal states and their gender. Input matching ignores
supralinear strokes and jinkim.
"#
        .into()
    }
}

pub fn language() -> Box<dyn Language> {
    Box::new(Coptic)
}
