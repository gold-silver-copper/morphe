use morphe_train::kaikki::Entry;
use morphe_train::spec::*;

pub struct Danish;

/// Words a headword line or table puts in front of a form.
const LEADING: &[&str] = &["at", "har", "er", "den", "det", "de"];

impl Language for Danish {
    fn spec(&self) -> Spec {
        let mut s = Spec::new("danish", "Danish", "da");
        s.drop_tags.extend(tags("dated"));
        s.features = vec![
            feature("Definiteness", &[("Indefinite", "indefinite"), ("Definite", "definite")]),
            feature("Case", &[("Nominative", "!genitive"), ("Genitive", "genitive")]),
            feature("Number", &[("Singular", "singular"), ("Plural", "plural")]),
            feature("Gender", &[("Common", "common-gender"), ("Neuter", "neuter")]),
            feature("Voice", &[("Active", "!passive"), ("Passive", "passive")]),
        ];
        let noun = || vec![group("", "", "Definiteness Case Number")];
        let positive = "!comparative !superlative";
        s.pos = vec![
            pos("noun", "noun", noun())
                .inherent("Gender")
                .citation("IndefiniteNominativeSingular DefiniteNominativeSingular IndefiniteNominativePlural")
                .head_last(),
            pos(
                "adj",
                "adj",
                vec![
                    // et stort hus
                    group("Neuter", &format!("neuter {positive}"), ""),
                    // det store hus
                    group("Definite", &format!("definite {positive}"), ""),
                    // store huse
                    group("Plural", &format!("plural !definite {positive}"), ""),
                    group("Comparative", "comparative", ""),
                    // huset er størst
                    group("Superlative", "superlative !definite", ""),
                    // det største hus
                    group("SuperlativeDefinite", "superlative definite", ""),
                ],
            )
            .citation("Lemma Neuter Definite"),
            pos(
                "verb",
                "verb",
                vec![
                    group("Infinitive", "infinitive", "Voice"),
                    group("Present", "present !participle", "Voice"),
                    group("Past", "past !participle", "Voice"),
                    group("Imperative", "imperative", "Voice[Active]"),
                    group("ParticiplePresent", "participle present", ""),
                    // har givet
                    group("ParticiplePast", "participle past !common-gender !plural", ""),
                    // en given sag
                    group("ParticiplePastCommon", "participle past common-gender", ""),
                    // de givne
                    group("ParticiplePastPlural", "participle past plural", ""),
                ],
            )
            .citation("InfinitiveActive PresentActive PastActive ParticiplePast")
            .head_first(),
        ];
        s
    }

    fn prepare(&self, e: &mut Entry) {
        let (mut positive, mut superlative) = (0, 0);
        for f in &mut e.forms {
            // "at give", "har givet", "den ansatte", "(have /havde) givet".
            if let Some((_, rest)) = f.form.split_once(") ") {
                f.form = rest.to_string();
            }
            if let Some((first, rest)) = f.form.split_once(' ') {
                if LEADING.contains(&first) && !rest.is_empty() {
                    f.form = rest.to_string();
                }
            }
            let has = |t: &str| f.tags.iter().any(|x| x == t);
            if has("perfect") {
                f.tags = tags("participle past");
            } else if has("attributive") && has("superlative") {
                f.tags.push("definite".into());
            } else if e.pos == "adj" && has("error-unrecognized-form") {
                // The adjective table repeats one tag set for the common,
                // neuter and definite forms, in that order (and for the
                // indefinite and definite superlative).
                f.tags.retain(|t| t != "error-unrecognized-form");
                let has = |t: &str| f.tags.iter().any(|x| x == t);
                if has("positive") {
                    match positive {
                        1 => f.tags.push("neuter".into()),
                        2 => f.tags.push("definite".into()),
                        _ => {}
                    }
                    positive += 1;
                } else if has("superlative") {
                    if superlative == 1 {
                        f.tags.push("definite".into());
                    }
                    superlative += 1;
                }
            }
        }
    }

    fn samples(&self) -> Vec<&'static str> {
        vec!["hus", "dreng", "give", "stor", "grøn", "tale", "flumpe", "snorkig"]
    }

    fn readme(&self) -> String {
        r#"## Example

```rust
use danish::*;
assert_eq!(noun("dreng", Definiteness::Definite, Case::Nominative, Number::Plural), "drengene");
println!("{:?}", noun_gender("hus"));
assert_eq!(verb_past("give", Voice::Active), "gav");
assert_eq!(verb_participle_past("give"), "givet");
assert_eq!(adj_neuter("stor"), "stort");
assert_eq!(adj_superlative_definite("stor"), "største");
// Citations pin principal parts: singular, definite singular, plural.
println!("{}", noun("mand, manden, mænd", Definiteness::Definite, Case::Nominative, Number::Plural));
```

Forms are given without "at", "har" or articles ("give", "givet", "ansatte").
"#
        .into()
    }
}

pub fn language() -> Box<dyn Language> {
    Box::new(Danish)
}
