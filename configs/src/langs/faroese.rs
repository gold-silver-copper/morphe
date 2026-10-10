use morphe_train::kaikki::Entry;
use morphe_train::spec::*;

pub struct Faroese;

fn has(tags: &[String], t: &str) -> bool {
    tags.iter().any(|x| x == t)
}

impl Language for Faroese {
    fn spec(&self) -> Spec {
        let mut s = Spec::new("faroese", "Faroese", "fo");
        s.drop_tags.extend(tags("dated"));
        s.features = vec![
            feature("Definiteness", &[("Indefinite", "indefinite"), ("Definite", "definite")]),
            feature(
                "Case",
                &[("Nominative", "nominative"), ("Accusative", "accusative"), ("Dative", "dative"), ("Genitive", "genitive")],
            ),
            feature("Number", &[("Singular", "singular"), ("Plural", "plural")]),
            feature("Gender", &[("Masculine", "masculine"), ("Feminine", "feminine"), ("Neuter", "neuter")]),
            feature(
                "PersonNumber",
                &[
                    ("FirstSingular", "first-person singular"),
                    ("SecondSingular", "second-person singular"),
                    ("ThirdSingular", "third-person singular"),
                    ("Plural", "plural"),
                ],
            ),
        ];
        s.pos = vec![
            pos("noun", "noun", vec![group("", "", "Definiteness Case Number")])
                .inherent("Gender")
                .citation("Lemma IndefiniteGenitiveSingular IndefiniteNominativePlural")
                .head_last(),
            pos(
                "adj",
                "adj",
                vec![
                    group("", "!weak !comparative !superlative", "Case Number Gender"),
                    group("Weak", "weak singular", "Case Gender"),
                    group("WeakPlural", "weak plural", ""),
                    group("Comparative", "comparative", ""),
                    group("Superlative", "superlative", ""),
                ],
            )
            .citation("Lemma Comparative Superlative"),
            pos(
                "verb",
                "verb",
                vec![
                    group("Present", "present !participle", "PersonNumber"),
                    group("Past", "past !participle", "PersonNumber"),
                    group("Imperative", "imperative", "Number"),
                    group("Supine", "supine", ""),
                    group("ParticiplePresent", "participle present", ""),
                    group("ParticiplePast", "participle past", ""),
                ],
            )
            .citation("Lemma PastThirdSingular Supine")
            .head_first(),
        ];
        s
    }

    fn prepare(&self, e: &mut Entry) {
        // An adjective's weak table follows a "weak" header; its plural has
        // one form for every case and gender.
        let mut weak = false;
        for f in &mut e.forms {
            if let Some(rest) = f.form.strip_suffix('!') {
                f.form = rest.to_string();
            }
            if has(&f.tags, "table-tags") {
                weak = f.form == "weak";
                continue;
            }
            if f.source.is_some() && e.pos == "adj" && weak {
                f.tags.retain(|t| t != "error-unrecognized-form");
                f.tags.push("weak".into());
            }
            // The headword line's plural is the indefinite nominative.
            if f.source.is_none() && e.pos == "noun" && !has(&f.tags, "definite") {
                f.tags.push("indefinite".into());
                if has(&f.tags, "plural") && !has(&f.tags, "genitive") {
                    f.tags.push("nominative".into());
                }
            }
            if e.pos == "verb" && has(&f.tags, "indicative") {
                // "feldu": the headword's past plural.
                let plural = has(&f.tags, "plural");
                f.tags.retain(|t| t != "indicative" && !(plural && t == "third-person"));
            }
        }
    }

    fn samples(&self) -> Vec<&'static str> {
        vec!["hestur", "bók", "hús", "fella", "vera", "stórur", "djúpur", "skrumpa", "glintur"]
    }

    fn readme(&self) -> String {
        r#"## Example

```rust
use faroese::*;
assert_eq!(noun("hestur", Definiteness::Definite, Case::Dative, Number::Plural), "hestunum");
println!("{:?}", noun_gender("bók"));
assert_eq!(verb_past("fella", PersonNumber::ThirdSingular), "feldi");
assert_eq!(verb_supine("fella"), "felt");
assert_eq!(adj("stórur", Case::Accusative, Number::Singular, Gender::Masculine), "stóran");
assert_eq!(adj_weak("stórur", Case::Nominative, Gender::Masculine), "stóri");
// Citations pin principal parts: lemma, genitive singular, plural.
println!("{}", noun("hestur, hests, hestar", Definiteness::Indefinite, Case::Dative, Number::Plural));
```
"#
        .into()
    }
}

pub fn language() -> Box<dyn Language> {
    Box::new(Faroese)
}
