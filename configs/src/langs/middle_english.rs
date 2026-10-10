use morphe_train::kaikki::{Entry, Form};
use morphe_train::spec::*;

pub struct MiddleEnglish;

fn has(tags: &[String], t: &str) -> bool {
    tags.iter().any(|x| x == t)
}

impl Language for MiddleEnglish {
    fn spec(&self) -> Spec {
        let mut s = Spec::new("middle_english", "MiddleEnglish", "enm");
        s.unmark = unmark("āa ēe īi ōo ūu ȳy ëe äa");
        s.fold_strs = vec![("þ".into(), "th".into()), ("ð".into(), "th".into())];
        s.features = vec![
            feature(
                "PersonNumber",
                &[
                    ("FirstSingular", "first-person singular"),
                    ("SecondSingular", "second-person singular"),
                    ("ThirdSingular", "third-person singular"),
                    ("Plural", "plural"),
                ],
            ),
            feature("Number", &[("Singular", "singular"), ("Plural", "plural")]),
            feature("Case", &[("Genitive", "genitive"), ("Dative", "dative")]),
            feature("Tense", &[("Present", "present"), ("Past", "past")]),
        ];
        s.pos = vec![
            pos(
                "noun",
                "noun",
                vec![group("Plural", "plural !genitive !dative", ""), group("", "", "Case Number")],
            )
            .citation("Lemma Plural")
            .head_last(),
            pos(
                "adj",
                "adj",
                vec![
                    // weak and plural: "the olde man", "olde men"
                    group("Weak", "!comparative !superlative", "Number"),
                    group("Comparative", "comparative", ""),
                    group("Superlative", "superlative", ""),
                ],
            ),
            pos(
                "verb",
                "verb",
                vec![
                    group("Present", "present !subjunctive !imperative !participle", "PersonNumber"),
                    group("Past", "past !subjunctive !imperative !participle", "PersonNumber"),
                    group("Subjunctive", "subjunctive", "Tense Number"),
                    group("Imperative", "imperative present", "Number"),
                    group("ParticiplePresent", "participle present", ""),
                    group("ParticiplePast", "participle past", ""),
                ],
            )
            .citation("Lemma PastThirdSingular ParticiplePast")
            .head_first(),
        ];
        s
    }

    fn prepare(&self, e: &mut Entry) {
        let mut extra = Vec::new();
        for f in &mut e.forms {
            // The noun table's plain singular and plural.
            if f.source.is_some() && e.pos == "noun" && has(&f.tags, "error-unrecognized-form") {
                f.tags.retain(|t| t != "error-unrecognized-form");
            }
            // The headword's "abregged" is both the past and the participle.
            if e.pos == "verb" && has(&f.tags, "participle") && has(&f.tags, "indicative") {
                f.tags = tags("participle past");
                for p in ["first-person", "third-person"] {
                    extra.push(Form { form: f.form.clone(), tags: vec!["past".into(), p.into(), "singular".into()], source: None });
                }
            }
            if e.pos == "verb" && has(&f.tags, "indicative") {
                f.tags.retain(|t| t != "indicative");
            }
            if e.pos == "verb" && has(&f.tags, "preterite") {
                f.tags.push("past".into());
            }
            // "singular weak" is the weak form; the plural has the same.
            if e.pos == "adj" && has(&f.tags, "weak") {
                f.tags.retain(|t| t != "weak");
            }
        }
        e.forms.extend(extra);
        // Reconstructed forms (*forthsenden).
        e.forms.retain(|f| !f.form.starts_with('*'));
    }

    fn samples(&self) -> Vec<&'static str> {
        vec!["cat", "child", "day", "open", "olde", "abreggen", "loven", "been", "flumpen", "snerkle"]
    }

    fn readme(&self) -> String {
        r#"## Example

```rust
use middle_english::*;
println!("{}", noun_plural("child")); // children
println!("{}", verb_present("loven", PersonNumber::ThirdSingular)); // loveth
println!("{}", verb_past("loven", PersonNumber::Plural)); // loveden
println!("{}", verb_participle_past("abreggen")); // abregged
```

Spelling varied widely; forms follow Wiktionary's headword spellings.
Macrons are optional on input, and th matches þ and ð.
"#
        .into()
    }
}

pub fn language() -> Box<dyn Language> {
    Box::new(MiddleEnglish)
}
