use morphe_train::kaikki::Entry;
use morphe_train::spec::*;

pub struct Bengali;

impl Language for Bengali {
    fn spec(&self) -> Spec {
        let mut s = Spec::new("bengali", "Bengali", "bn");
        // Khanda ta is also written as ta + hasanta.
        s.fold_strs = vec![("ৎ".into(), "ত্".into())];
        s.drop_tags.extend(tags("dated"));
        s.features = vec![
            feature(
                "Case",
                &[("Nominative", "nominative"), ("Objective", "objective"), ("Genitive", "genitive"), ("Locative", "locative")],
            ),
            feature("Number", &[("Singular", "singular"), ("Plural", "plural")]),
            // তুই, তুমি and আপনি are the intimate, familiar and polite second
            // persons; the polite form also serves the third person (তিনি).
            feature(
                "Person",
                &[
                    ("First", "first-person"),
                    ("SecondIntimate", "second-person x-intimate"),
                    ("SecondFamiliar", "second-person familiar !x-intimate"),
                    ("Third", "third-person familiar"),
                    ("Polite", "polite"),
                ],
            ),
            feature(
                "Tense",
                &[
                    ("Present", "present !continuative !perfect"),
                    ("PresentContinuous", "present continuative"),
                    ("PresentPerfect", "present perfect"),
                    ("Past", "past !continuative !perfect !habitual"),
                    ("PastContinuous", "past continuative"),
                    ("PastPerfect", "past perfect"),
                    ("PastHabitual", "past habitual"),
                    ("Future", "future"),
                ],
            ),
            feature(
                "Participle",
&[("Perfect", "perfect"), ("Conditional", "conditional")],
            ),
        ];
        s.pos = vec![
            pos(
                "noun",
                "noun",
                vec![group("Indefinite", "indefinite", "Case"), group("Definite", "definite", "Case Number")],
            )
            .citation("IndefiniteNominative IndefiniteObjective")
            .lemma_cell("IndefiniteNominative")
            .head_last(),
            pos(
                "verb",
                "verb",
                vec![
                    group("VerbalNoun", "noun-from-verb", ""),
                    group("Infinitive", "infinitive", ""),
                    group("Participle", "participle", "Participle"),
                    group("", "!participle", "Tense Person"),
                ],
            )
            .head_last(),
            pos(
                "pron",
                "pron",
                vec![
                    group("", "!formal !informal !definite", "Case"),
                    group("Definite", "definite", "Case Number"),
                    group("Formal", "formal", "Case Number"),
                    group("Informal", "informal", "Case Number"),
                ],
            )
            .lemma_cell("Nominative"),
        ];
        s
    }

    /// Possessive pronoun forms are genitives.
    fn retag(&self, _p: &PosSpec, _form: &str, t: &mut Vec<String>) {
        for x in t.iter_mut() {
            if x == "possessive" {
                *x = "genitive".into();
            }
        }
    }

    /// Tables in the literary sadhu language are dropped. In each verb row
    /// the second person forms come intimate first, familiar last. Case
    /// forms on the headword line are indefinite.
    fn prepare(&self, e: &mut Entry) {
        let mut sadhu = false;
        let mut keep = Vec::new();
        for f in &e.forms {
            if f.source.is_some() && f.tags.iter().any(|t| t == "inflection-template") {
                sadhu = f.form.ends_with("/sadhu");
            }
            if f.source.is_none() || !sadhu {
                keep.push(f.clone());
            }
        }
        for i in 0..keep.len() {
            let row = |j: usize| keep.get(j).map(|f| (f.source.is_some(), f.tags.clone()));
            let me = row(i);
            let second = me.as_ref().is_some_and(|(t, x)| *t && x.iter().any(|t| t == "second-person") && x.iter().any(|t| t == "familiar"));
            if second && row(i + 1) == me {
                if let Some(f) = keep.get_mut(i) {
                    f.tags.push("x-intimate".into());
                }
            }
        }
        // Headword-line forms split by a stray space are broken.
        let spaced = e.word.contains(' ');
        keep.retain(|f| f.source.is_some() || spaced || !f.form.contains(' '));
        for f in keep.iter_mut().filter(|f| f.source.is_none()) {
            if f.tags.iter().any(|t| ["objective", "genitive", "locative"].contains(&t.as_str())) && e.pos == "noun" {
                f.tags.push("indefinite".into());
            }
        }
        e.forms = keep;
    }

    /// "ছেলেটি (cheleṭa (colloquial), …)" carries a note; "ছেলেদের(কে)" has
    /// an optional ending.
    fn split_form(&self, raw: &str) -> Vec<String> {
        let raw = raw.split(" (").next().unwrap_or(raw).trim();
        match raw.split_once('(') {
            Some((a, rest)) => match rest.split_once(')') {
                Some((opt, b)) => vec![format!("{a}{b}"), format!("{a}{opt}{b}")],
                None => vec![a.to_string()],
            },
            None => vec![raw.to_string()],
        }
    }

    fn samples(&self) -> Vec<&'static str> {
        vec!["ছেলে", "চোখ", "মাস", "করা", "ভালোবাসা", "আমি", "টপলা", "ঝাপানো"]
    }

    fn readme(&self) -> String {
        r#"## Example

```rust
use bengali::*;
println!("{}", noun_indefinite("চোখ", Case::Locative)); // চোখে
println!("{}", noun_definite("ছেলে", Case::Nominative, Number::Plural)); // ছেলেরা
println!("{}", noun_definite("মাস", Case::Genitive, Number::Singular)); // মাসটার
println!("{}", verb("করা", Tense::Past, Person::First)); // করলাম
println!("{}", verb("করা", Tense::Future, Person::Polite)); // করবেন
println!("{}", verb("করা", Tense::Present, Person::SecondIntimate)); // করিস
println!("{}", verb_participle("করা", Participle::Perfect)); // করে
```

Nouns have the indefinite nominative, objective, genitive and locative,
and the definite (with the classifier -টা/-টি, plural -গুলো/-রা) in both
numbers. Verbs are cited by the verbal noun (করা) and have eight tenses
for the five persons (আমি; তুই; তুমি; সে; আপনি/তিনি), the infinitive and
the perfect and conditional participles. The reduplicated participles
(করতে-করতে, করে-করে) are the infinitive and the perfect participle doubled. Forms are the standard colloquial language; the literary
sadhu tables are left out.
"#
        .into()
    }
}

pub fn language() -> Box<dyn Language> {
    Box::new(Bengali)
}
