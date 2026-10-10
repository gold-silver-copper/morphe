use morphe_train::kaikki::{Entry, Form};
use morphe_train::spec::*;

pub struct MiddleIrish;

fn has(tags: &[String], t: &str) -> bool {
    tags.iter().any(|x| x == t)
}

impl Language for MiddleIrish {
    fn spec(&self) -> Spec {
        let mut s = Spec::new("middle_irish", "MiddleIrish", "mga");
        // Length marks are often left out; the raised dot (·) of conjunct
        // forms is not typed.
        s.unmark = unmark("áa ée íi óo úu āa ṡs ḟf");
        s.marks = vec!['·'];
        s.features = vec![
            feature("Gender", &[("Masculine", "masculine"), ("Feminine", "feminine"), ("Neuter", "neuter")]),
            feature(
                "Case",
                &[
                    ("Nominative", "nominative"),
                    ("Vocative", "vocative"),
                    ("Accusative", "accusative"),
                    ("Genitive", "genitive"),
                    ("Dative", "dative"),
                ],
            ),
            feature("Number", &[("Singular", "singular"), ("Dual", "dual"), ("Plural", "plural")]),
            feature("Mutation", &[("Lenition", "lenition"), ("Nasalization", "nasalization")]),
            feature("Degree", &[("Equative", "equative"), ("Comparative", "comparative"), ("Superlative", "superlative")]),
        ];
        let mutated = || group("Mutated", "mutation", "Mutation");
        s.pos = vec![
            pos("noun", "noun name", vec![group("", "!mutation", "Case Number"), mutated()])
                .inherent("Gender")
                .lemma_cell("NominativeSingular")
                .citation("NominativeSingular GenitiveSingular NominativePlural")
                .initial(2)
                .head_first(),
            pos(
                "adj",
                "adj",
                vec![
                    group("", "!mutation !comparative !superlative !equative", "Case Number[Singular,Plural] Gender"),
                    group("Degree", "", "Degree"),
                    mutated(),
                ],
            )
            .initial(2),
            pos(
                "verb",
                "verb",
                vec![
                    group("VerbalNoun", "noun-from-verb", ""),
                    // ·beir: the form after a preverb or particle.
                    group("Conjunct", "conjunct", ""),
                    mutated(),
                ],
            )
            .citation("Lemma VerbalNoun")
            .initial(2),
        ];
        s
    }

    fn prepare(&self, e: &mut Entry) {
        if !e.forms.iter().any(|f| has(&f.tags, "canonical")) {
            e.forms.push(Form { form: e.word.clone(), tags: vec!["canonical".into()], source: None });
        }
        for f in e.forms.iter_mut() {
            if f.source.as_deref() == Some("mutation") {
                // radical, lenition, then nasalization, unlabeled.
                if has(&f.tags, "error-unrecognized-form") {
                    f.tags = tags("mutation nasalization");
                }
                continue;
            }
            if f.source.is_none() && e.pos != "verb" {
                // The headword line: "genitive", "plural".
                let t = f.tags.join(" ");
                match t.as_str() {
                    "genitive" => f.tags = tags("genitive singular"),
                    "plural" => f.tags = tags("nominative plural"),
                    _ => {}
                }
            }
            if e.pos == "adj" && f.source.is_some() && has(&f.tags, "error-unrecognized-form") {
                // The plural column's forms are labeled "feminine neuter
                // singular" with an unrecognized marker.
                f.tags.retain(|x| x != "error-unrecognized-form" && x != "singular");
                f.tags.push("plural".into());
            }
        }
    }

    fn samples(&self) -> Vec<&'static str> {
        vec!["ab", "mac", "cara", "túathach", "lem", "beirid", "glórach", "snorc"]
    }

    fn readme(&self) -> String {
        r#"## Example

```rust
use middle_irish::*;
println!("{}", noun("mac", Case::Nominative, Number::Plural)); // mic
println!("{}", noun_mutated("ab", Mutation::Nasalization)); // n-ab
println!("{}", verb_verbal_noun("beirid")); // breth
```

Most words have only their headword forms (genitive, plural) and initial
mutations; Wiktionary has full declension tables for a few. Length marks are
optional on input.
"#
        .into()
    }
}

pub fn language() -> Box<dyn Language> {
    Box::new(MiddleIrish)
}
