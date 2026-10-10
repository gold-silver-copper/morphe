use morphe_train::kaikki::{Entry, Form};
use morphe_train::spec::*;

pub struct NorwegianNynorsk;

/// Words a table puts in front of a form ("å skrive", "har skrevet").
const LEADING: &[&str] = &["å", "har", "ha"];

impl Language for NorwegianNynorsk {
    fn spec(&self) -> Spec {
        let mut s = Spec::new("norwegian_nynorsk", "NorwegianNynorsk", "nn");
        s.drop_tags.extend(tags("dated Bokmål subjunctive"));
        s.soft_tags.extend(tags("Midlandsnormalen Setesdal Telemark"));
        s.features = vec![
            feature("Gender", &[("Masculine", "masculine"), ("Feminine", "feminine"), ("Neuter", "neuter")]),
            feature("Definiteness", &[("Indefinite", "indefinite"), ("Definite", "definite")]),
            feature("Number", &[("Singular", "singular"), ("Plural", "plural")]),
        ];
        let positive = "!comparative !superlative";
        s.pos = vec![
            pos("noun", "noun", vec![group("", "!dative !genitive", "Definiteness Number")])
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
                    group("Present", "present !participle !imperative", ""),
                    group("Past", "past !participle", ""),
                    // har skrive
                    group("Supine", "supine", ""),
                    group("Imperative", "imperative", ""),
                    group("Passive", "passive", ""),
                    group("ParticiplePresent", "participle present", ""),
                    // ein skriven bok
                    group("ParticiplePast", "participle past !definite !plural !supine", ""),
                ],
            )
            .citation("Lemma Present Past Supine")
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
        // Weak verbs have no separate supine: it is the past participle (kasta).
        if e.pos == "verb" && !e.forms.iter().any(|f| f.tags.iter().any(|t| t == "supine")) {
            let supine: Vec<Form> = e
                .forms
                .iter()
                .filter(|f| f.tags.len() == 2 && f.tags.iter().all(|t| t == "participle" || t == "past"))
                .map(|f| Form { form: f.form.clone(), tags: tags("supine"), source: f.source.clone() })
                .collect();
            e.forms.extend(supine);
        }
        for f in &mut e.forms {
            if let Some((first, rest)) = f.form.split_once(' ') {
                if LEADING.contains(&first) && !rest.is_empty() {
                    f.form = rest.to_string();
                }
            }
            if let Some(rest) = f.form.strip_suffix('!') {
                f.form = rest.to_string();
            }
            if e.pos == "verb" {
                f.tags.retain(|t| t != "error-unrecognized-form");
            }
        }
    }

    fn samples(&self) -> Vec<&'static str> {
        vec!["hus", "gut", "bok", "skrive", "kaste", "vere", "stor", "liten", "flumpe", "snorkete"]
    }

    fn readme(&self) -> String {
        r#"## Example

```rust
use norwegian_nynorsk::*;
assert_eq!(noun("gut", Definiteness::Definite, Number::Plural), "gutane");
println!("{:?}", noun_gender("hus"));
assert_eq!(verb_past("skrive"), "skreiv");
assert_eq!(verb_supine("skrive"), "skrive");
assert_eq!(verb_participle_past("skrive"), "skriven");
assert_eq!(adj_neuter("stor"), "stort");
assert_eq!(adj_superlative_definite("stor"), "største");
// Citations pin principal parts: lemma, definite singular, plural.
println!("{}", noun("bok, boka, bøker", Definiteness::Definite, Number::Plural));
```

Nynorsk often allows several forms (kaste/kast); all are given with
confidences. Forms are given without "å" or "har". Dative and subjunctive
forms are left out as dialectal or archaic.
"#
        .into()
    }
}

pub fn language() -> Box<dyn Language> {
    Box::new(NorwegianNynorsk)
}
