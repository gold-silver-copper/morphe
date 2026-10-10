use morphe_train::kaikki::{Entry, Form};
use morphe_train::spec::*;

pub struct MiddleWelsh;

fn has(tags: &[String], t: &str) -> bool {
    tags.iter().any(|x| x == t)
}

impl Language for MiddleWelsh {
    fn spec(&self) -> Spec {
        let mut s = Spec::new("middle_welsh", "MiddleWelsh", "wlm");
        // Editions write ð for dd; many type d.
        s.unmark = unmark("ðd äa ỻl ỿy ỽw ẟd");
        s.features = vec![
            feature("Gender", &[("Masculine", "masculine"), ("Feminine", "feminine")]),
            feature(
                "Mutation",
                &[("Soft", "soft"), ("Nasal", "nasal"), ("Aspirate", "aspirate"), ("HProthesis", "prothesis-h")],
            ),
            feature("Degree", &[("Equative", "equative"), ("Comparative", "comparative"), ("Superlative", "superlative")]),
            feature(
                "Tense",
                &[
                    ("Present", "indicative present"),
                    // byð: the consuetudinal future of bot.
                    ("Future", "future"),
                    ("Imperfect", "imperfect !subjunctive"),
                    ("Preterite", "preterite"),
                    ("Perfect", "perfect"),
                    ("Pluperfect", "pluperfect"),
                    ("Subjunctive", "present subjunctive"),
                    ("ImperfectSubjunctive", "imperfect subjunctive"),
                ],
            ),
            feature(
                "Person",
                &[
                    ("FirstSingular", "first-person singular"),
                    ("SecondSingular", "second-person singular"),
                    ("ThirdSingular", "third-person singular"),
                    ("FirstPlural", "first-person plural"),
                    ("SecondPlural", "second-person plural"),
                    ("ThirdPlural", "third-person plural"),
                    ("Impersonal", "impersonal"),
                ],
            ),
        ];
        let mutated = || group("Mutated", "", "Mutation");
        s.pos = vec![
            pos("noun", "noun", vec![group("Plural", "plural", ""), mutated()])
                .inherent("Gender")
                .citation("Lemma Plural")
                .initial(2)
                .head_first(),
            pos("adj", "adj", vec![group("Plural", "plural", ""), group("", "", "Degree"), mutated()])
                .citation("Lemma Plural")
                .initial(2),
            pos(
                "verb",
                "verb",
                vec![
                    group("", "!imperative", "Tense Person"),
                    group("Imperative", "imperative", "Person[SecondSingular,ThirdSingular,FirstPlural,SecondPlural,ThirdPlural,Impersonal]"),
                    group("VerbalNoun", "noun-from-verb", ""),
                    group("Participle", "participle", ""),
                    mutated(),
                ],
            )
            .citation("VerbalNoun PresentThirdSingular PreteriteThirdSingular")
            .initial(2),
        ];
        s
    }

    fn prepare(&self, e: &mut Entry) {
        // The mutation table labels only soft mutation and h-prothesis; the
        // radical and the nasal and aspirate forms come unlabeled.
        let word = e.word.to_lowercase();
        for f in e.forms.iter_mut() {
            if f.source.as_deref() != Some("mutation") || !has(&f.tags, "error-unrecognized-form") {
                continue;
            }
            let form = f.form.to_lowercase();
            let tag = if form == word {
                "radical"
            } else if ["ch", "ph", "th"].iter().any(|p| form.starts_with(p)) {
                "aspirate"
            } else {
                "nasal"
            };
            f.tags = vec![tag.into()];
        }
        // A word with a mutation table but no form for some mutation keeps
        // its radical there (vowels have no soft mutation).
        if e.forms.iter().any(|f| f.source.as_deref() == Some("mutation")) {
            for m in ["soft", "nasal", "aspirate", "prothesis-h"] {
                if !e.forms.iter().any(|f| has(&f.tags, m)) {
                    e.forms.push(Form { form: e.word.clone(), tags: vec![m.into()], source: Some("mutation".into()) });
                }
            }
        }
        // Conjugation rows without a tense: the imperfect, and for bot a
        // future (byð) before it. A row starts with its first person
        // singular.
        let unlabeled = |f: &Form| f.source.as_deref() == Some("conjugation") && has(&f.tags, "error-unrecognized-form");
        let mut blocks = Vec::new();
        let mut block = 0usize;
        let mut prev_first = false;
        for f in &e.forms {
            if !unlabeled(f) {
                prev_first = false;
                blocks.push(None);
                continue;
            }
            let first = has(&f.tags, "first-person") && has(&f.tags, "singular");
            if first && !prev_first && blocks.iter().flatten().count() > 0 {
                block += 1;
            }
            prev_first = first;
            blocks.push(Some(block));
        }
        let total = block + 1;
        for (f, b) in e.forms.iter_mut().zip(blocks) {
            let Some(b) = b else { continue };
            f.tags.retain(|t| t != "error-unrecognized-form");
            let tense = if total > 1 && b == 0 { "future" } else { "imperfect" };
            f.tags.push(tense.into());
        }
    }

    fn split_form(&self, raw: &str) -> Vec<String> {
        // Notes and phrases in the tables: "relative: yssyð", "y mae".
        if raw.contains([' ', ':']) {
            return Vec::new();
        }
        vec![raw.to_string()]
    }

    fn samples(&self) -> Vec<&'static str> {
        vec!["ber", "arch", "bwrw", "bot", "mynet", "coch", "da", "glorya", "snorch"]
    }

    fn readme(&self) -> String {
        r#"## Example

```rust
use middle_welsh::*;
println!("{}", noun_plural("ber")); // bereu
println!("{}", noun_mutated("arch", Mutation::HProthesis)); // harch
println!("{}", verb("bwrw", Tense::Preterite, Person::ThirdSingular)); // byriawð
println!("{}", adj("da", Degree::Superlative)); // goreu
```

Spellings follow Wiktionary's normalized Middle Welsh, with ð for dd; d
matches ð on input.
"#
        .into()
    }
}

pub fn language() -> Box<dyn Language> {
    Box::new(MiddleWelsh)
}
