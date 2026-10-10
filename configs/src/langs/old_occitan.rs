use morphe_train::kaikki::Entry;
use morphe_train::spec::*;

pub struct OldOccitan;

fn has(tags: &[String], t: &str) -> bool {
    tags.iter().any(|x| x == t)
}

impl Language for OldOccitan {
    fn spec(&self) -> Spec {
        let mut s = Spec::new("old_occitan", "OldOccitan", "pro");
        s.unmark = unmark("òo óo üu áa çc");
        // The interpunct of editions (de·l) is not typed.
        s.marks = vec!['·'];
        s.features = vec![
            feature("Gender", &[("Masculine", "masculine"), ("Feminine", "feminine")]),
            feature("Case", &[("Nominative", "nominative"), ("Oblique", "oblique")]),
            feature("Number", &[("Singular", "singular"), ("Plural", "plural")]),
            feature(
                "Form",
                &[
                    ("Feminine", "feminine !plural"),
                    ("MasculinePlural", "masculine plural"),
                    ("FemininePlural", "feminine plural"),
                ],
            ),
        ];
        s.pos = vec![
            // The two-case system: the dictionary form is the oblique
            // singular (cavalier); the nominative singular adds -s (cavaliers).
            pos("noun", "noun name", vec![group("", "", "Case Number")])
                .inherent("Gender")
                .lemma_cell("ObliqueSingular")
                .citation("ObliqueSingular NominativeSingular")
                .head_last(),
            pos("adj", "adj", vec![group("", "", "Form")]).citation("Lemma Feminine MasculinePlural"),
        ];
        s
    }

    fn prepare(&self, e: &mut Entry) {
        if e.pos != "adj" {
            return;
        }
        // One plural for both genders.
        let mut extra = Vec::new();
        for f in &mut e.forms {
            if f.tags.len() == 1 && has(&f.tags, "plural") {
                f.tags.push("masculine".into());
                let mut g = f.clone();
                g.tags = tags("feminine plural");
                extra.push(g);
            }
        }
        e.forms.extend(extra);
    }

    fn samples(&self) -> Vec<&'static str> {
        vec!["libre", "dia", "cavalier", "bel", "celestial", "flor", "glorbel"]
    }

    fn readme(&self) -> String {
        r#"## Example

```rust
use old_occitan::*;
println!("{}", noun("libre", Case::Nominative, Number::Singular)); // libres
println!("{}", noun("libre", Case::Oblique, Number::Plural)); // libres
println!("{}", adj("bel", Form::Feminine)); // bela
```

Nouns keep the old two-case system (nominative and oblique). Verbs are
left out: Wiktionary has conjugation tables for only a handful.
"#
        .into()
    }
}

pub fn language() -> Box<dyn Language> {
    Box::new(OldOccitan)
}
