use morphe_train::kaikki::Entry;
use morphe_train::spec::*;

pub struct Afrikaans;

fn has(tags: &[String], t: &str) -> bool {
    tags.iter().any(|x| x == t)
}

impl Language for Afrikaans {
    fn spec(&self) -> Spec {
        let mut s = Spec::new("afrikaans", "Afrikaans", "af");
        s.drop_tags.extend(tags("dated"));
        s.soft_tags.extend(tags("Cape-Afrikaans"));
        s.pos = vec![
            pos("noun", "noun", vec![group("Plural", "plural", ""), group("Diminutive", "diminutive", "")])
                .citation("Lemma Plural")
                .head_last(),
            pos(
                "adj",
                "adj",
                vec![
                    // 'n arme man
                    group("Attributive", "attributive !comparative !superlative !plural !partitive !predicative", ""),
                    // die man is armer
                    group("Comparative", "comparative !independent !partitive", ""),
                    // 'n armere man
                    group("ComparativeAttributive", "comparative independent singular", ""),
                    group("Superlative", "superlative !independent !partitive", ""),
                    // iets arms
                    group("Partitive", "partitive positive", ""),
                ],
            )
            .citation("Lemma Attributive Comparative Superlative"),
            pos(
                "verb",
                "verb",
                vec![
                    group("Present", "present !participle !subjunctive", ""),
                    group("Past", "past !participle !subjunctive", ""),
                    group("ParticiplePresent", "participle present", ""),
                    group("ParticiplePast", "participle past", ""),
                ],
            )
            .citation("Lemma ParticiplePast")
            .head_first(),
        ];
        s
    }

    fn prepare(&self, e: &mut Entry) {
        // The table's plain "attributive positive" form (arm) sits beside
        // the inflected one (arme); where they are one form (mal), it is
        // also the predicative.
        let inflected = e.forms.iter().any(|f| has(&f.tags, "independent") && has(&f.tags, "positive"));
        for f in &mut e.forms {
            if f.source.is_none() || !has(&f.tags, "attributive") || !has(&f.tags, "positive") {
                continue;
            }
            if has(&f.tags, "predicative") {
                f.tags.retain(|t| t != "predicative");
            } else if inflected && !has(&f.tags, "independent") {
                f.tags.clear();
            }
        }
    }

    fn split_form(&self, raw: &str) -> Vec<String> {
        // "nabyer|naaste"
        raw.split('|').map(String::from).collect()
    }

    fn samples(&self) -> Vec<&'static str> {
        vec!["hond", "kind", "stad", "arm", "goed", "skryf", "flompie", "glimmerig"]
    }

    fn readme(&self) -> String {
        r#"## Example

```rust
use afrikaans::*;
assert_eq!(noun_plural("hond"), "honde");
assert_eq!(noun_diminutive("hond"), "hondjie");
assert_eq!(adj_attributive("arm"), "arme");
assert_eq!(adj_comparative("arm"), "armer");
println!("{}", verb_participle_past("skryf"));
```

Afrikaans verbs barely inflect: the past is formed with "het" and the past
participle. Nouns have no grammatical gender.
"#
        .into()
    }
}

pub fn language() -> Box<dyn Language> {
    Box::new(Afrikaans)
}
