use morphe_train::kaikki::Entry;
use morphe_train::spec::*;

pub struct Malayalam;

impl Language for Malayalam {
    fn spec(&self) -> Spec {
        let mut s = Spec::new("malayalam", "Malayalam", "ml");
        // Zero-width non-joiners are optional; chillu letters are also
        // written as consonant + virama + zero-width joiner.
        s.marks = vec!['\u{200C}'];
        s.fold_strs = [("ൻ", "ന്\u{200D}"), ("ൺ", "ണ്\u{200D}"), ("ർ", "ര്\u{200D}"), ("ൽ", "ല്\u{200D}"), ("ൾ", "ള്\u{200D}")]
            .iter()
            .map(|(a, b)| ((*b).to_string(), (*a).to_string()))
            .collect();
        s.features = vec![
            feature(
                "Case",
                &[
                    ("Nominative", "nominative"),
                    ("Accusative", "accusative"),
                    ("Dative", "dative"),
                    ("Sociative", "sociative"),
                    ("Genitive", "genitive"),
                    ("Instrumental", "instrumental"),
                    ("Locative", "locative"),
                    ("Vocative", "vocative"),
                ],
            ),
            feature("Number", &[("Singular", "singular"), ("Plural", "plural")]),
        ];
        let nominal = || vec![group("", "", "Case Number")];
        s.pos = vec![
            pos("noun", "noun", nominal()).citation("NominativeSingular NominativePlural").lemma_cell("NominativeSingular").head_last(),
            pos("verb", "verb", vec![group("Past", "past", "")]).citation("Lemma Past").head_last(),
            pos("pron", "pron", nominal()).lemma_cell("NominativeSingular"),
        ];
        s
    }

    /// The table leaves the sociative (-ഓട്) untagged; a headword-line
    /// plural is the nominative.
    fn prepare(&self, e: &mut Entry) {
        for f in &mut e.forms {
            let has = |t: &str| f.tags.iter().any(|x| x == t);
            if has("error-unrecognized-form") {
                f.tags.retain(|t| t != "error-unrecognized-form");
                f.tags.push("sociative".into());
            } else if f.source.is_none() && f.tags.len() == 1 && has("plural") {
                f.tags.push("nominative".into());
            }
        }
    }

    fn samples(&self) -> Vec<&'static str> {
        vec!["അമ്മ", "അധ്യാപകൻ", "മരം", "കരയുക", "പോകുക", "താൻ", "കുലബം", "ചപടുക"]
    }

    fn readme(&self) -> String {
        r#"## Example

```rust
use malayalam::*;
println!("{}", noun("അമ്മ", Case::Dative, Number::Singular)); // അമ്മയ്ക്ക്
println!("{}", noun("അമ്മ", Case::Nominative, Number::Plural)); // അമ്മമാർ
println!("{}", noun("അമ്മ", Case::Vocative, Number::Singular)); // അമ്മേ
println!("{}", verb_past("കരയുക")); // കരഞ്ഞു
```

Nouns and pronouns have eight cases in both numbers. Verbs have only their
past tense, which is all Wiktionary gives for most of them. Input matching
ignores zero-width non-joiners and accepts chillu letters written with a
joiner.
"#
        .into()
    }
}

pub fn language() -> Box<dyn Language> {
    Box::new(Malayalam)
}
