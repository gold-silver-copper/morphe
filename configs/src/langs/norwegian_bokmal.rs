use morphe_train::kaikki::{Entry, Form};
use morphe_train::spec::*;

pub struct NorwegianBokmal;

/// Words a table puts in front of a form ("å skrive", "har skrevet").
const LEADING: &[&str] = &["å", "har"];

impl Language for NorwegianBokmal {
    fn spec(&self) -> Spec {
        let mut s = Spec::new("norwegian_bokmal", "NorwegianBokmål", "nb");
        s.drop_tags.extend(tags("dated Riksmål Nynorsk"));
        s.features = vec![
            feature("Gender", &[("Masculine", "masculine"), ("Feminine", "feminine"), ("Neuter", "neuter")]),
            feature("Definiteness", &[("Indefinite", "indefinite"), ("Definite", "definite")]),
            feature("Number", &[("Singular", "singular"), ("Plural", "plural")]),
        ];
        let positive = "!comparative !superlative";
        s.pos = vec![
            pos("noun", "noun", vec![group("", "", "Definiteness Number")])
                .inherent("Gender")
                .citation("Lemma DefiniteSingular IndefinitePlural")
                .head_last(),
            pos(
                "adj",
                "adj",
                vec![
                    // et stort hus
                    group("Neuter", &format!("neuter {positive}"), ""),
                    // det store huset
                    group("Definite", &format!("definite singular {positive}"), ""),
                    // store hus
                    group("Plural", &format!("plural {positive}"), ""),
                    group("Comparative", "comparative", ""),
                    // huset er størst
                    group("Superlative", "superlative !definite", ""),
                    // det største huset
                    group("SuperlativeDefinite", "superlative definite", ""),
                ],
            )
            .citation("Lemma Neuter Plural"),
            pos(
                "verb",
                "verb",
                vec![
                    group("Present", "present !participle !perfect", ""),
                    group("Past", "past !participle", ""),
                    group("Imperative", "imperative", ""),
                    group("Passive", "passive", ""),
                    group("ParticiplePresent", "participle present", ""),
                    // har skrevet
                    group("ParticiplePast", "participle past !definite !plural", ""),
                    // den skrevne boka
                    group("ParticiplePastDefinite", "participle past definite", ""),
                ],
            )
            .citation("Lemma Present Past ParticiplePast")
            .head_first(),
        ];
        s
    }

    fn prepare(&self, e: &mut Entry) {
        // "store": one form for the definite singular and the plural.
        let mut plural = Vec::new();
        for f in &mut e.forms {
            let has = |t: &str| f.tags.iter().any(|x| x == t);
            if e.pos == "adj" && has("definite") && has("plural") && has("singular") {
                f.tags.retain(|t| t != "plural");
                plural.push(Form { form: f.form.clone(), tags: tags("plural"), source: f.source.clone() });
            }
        }
        e.forms.extend(plural);
        for f in &mut e.forms {
            if let Some((first, rest)) = f.form.split_once(' ') {
                if LEADING.contains(&first) && !rest.is_empty() {
                    f.form = rest.to_string();
                }
            }
            if let Some(rest) = f.form.strip_suffix('!') {
                f.form = rest.to_string();
            }
            let has = |t: &str| f.tags.iter().any(|x| x == t);
            if has("perfect") {
                f.tags = tags("participle past");
            } else if has("supine") {
                f.tags = tags("participle past");
            } else if has("plural") && has("participle") {
                f.tags.push("definite".into());
            } else if has("error-unrecognized-form") && has("participle") {
                f.tags.retain(|t| t != "error-unrecognized-form");
            }
        }
    }

    fn samples(&self) -> Vec<&'static str> {
        vec!["hus", "gutt", "bok", "skrive", "kaste", "stor", "liten", "flumpe", "snorkete"]
    }

    fn readme(&self) -> String {
        r#"## Example

```rust
use norwegian_bokmal::*;
assert_eq!(noun("gutt", Definiteness::Definite, Number::Plural), "guttene");
println!("{:?}", noun_gender("hus"));
assert_eq!(verb_past("skrive"), "skrev");
assert_eq!(verb_participle_past("skrive"), "skrevet");
assert_eq!(adj_neuter("stor"), "stort");
assert_eq!(adj_plural("liten"), "små");
assert_eq!(adj_superlative_definite("stor"), "største");
// Citations pin principal parts: lemma, definite singular, plural.
println!("{}", noun("bok, boka, bøker", Definiteness::Definite, Number::Plural));
```

Bokmål often allows several forms (kasta/kastet, boka/boken); all are
given with confidences. Forms are given without "å" or "har".
"#
        .into()
    }
}

pub fn language() -> Box<dyn Language> {
    Box::new(NorwegianBokmal)
}
