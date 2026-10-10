use morphe_train::kaikki::Entry;
use morphe_train::spec::*;

pub struct Gagauz;

impl Language for Gagauz {
    fn spec(&self) -> Spec {
        let mut s = Spec::new("gagauz", "Gagauz", "gag");
        // Stress and length marks on some headwords (mózak, ildâz).
        s.unmark = unmark("áa ée íi óo úu âa");
        s.features = vec![
            feature(
                "Case",
                &[
                    ("Nominative", "nominative"),
                    ("Genitive", "genitive"),
                    ("Dative", "dative"),
                    ("Accusative", "accusative"),
                    ("Locative", "locative"),
                    ("Ablative", "ablative"),
                ],
            ),
            feature("Number", &[("Singular", "singular"), ("Plural", "plural")]),
        ];
        s.pos = vec![
            pos("noun", "noun", vec![group("", "!possessive", "Case Number")])
                .citation("NominativeSingular AccusativeSingular NominativePlural")
                .head_last(),
            pos(
                "verb",
                "verb",
                vec![group("Infinitive", "infinitive", ""), group("PresentThirdSingular", "present singular third-person", "")],
            )
            .lemma_cell("Infinitive")
            .head_first(),
        ];
        s
    }

    /// Nouns are read from their tables only: a headword line's two forms
    /// would make a known word lacking the other cells. A table with
    /// unfilled template slots ({{{11}}}) is misaligned throughout.
    fn prepare(&self, e: &mut Entry) {
        if e.pos == "noun" && e.forms.iter().any(|f| f.source.is_some() && f.form.contains("{{{")) {
            e.forms.clear();
        }
    }

    fn samples(&self) -> Vec<&'static str> {
        vec!["en", "ev", "kitap", "göz", "üürenmää", "şırpıl", "zıbırmaa"]
    }

    fn readme(&self) -> String {
        r#"## Example

```rust
use gagauz::*;
println!("{}", noun("en", Case::Ablative, Number::Plural)); // ennerdän
println!("{}", noun("ev", Case::Dative, Number::Singular));
println!("{}", verb_present_third_singular("üürenmää")); // üürenär
```

Nouns have six cases in both numbers. Wiktionary has no Gagauz
conjugation tables; verbs have only the third person present the headword
line gives.
"#
        .into()
    }
}

pub fn language() -> Box<dyn Language> {
    Box::new(Gagauz)
}
