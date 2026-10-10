use morphe_train::kaikki::Entry;
use morphe_train::spec::*;

pub struct Malay;

impl Language for Malay {
    fn spec(&self) -> Spec {
        let mut s = Spec::new("malay", "Malay", "ms");
        s.source = Source::Head;
        // Headwords mark the schwa (ĕ), which ordinary spelling doesn't.
        s.canonical_spelling = false;
        s.unmark = unmark("ĕe ée");
        s.features = vec![feature("Degree", &[("Comparative", "comparative"), ("Superlative", "superlative")])];
        s.pos = vec![
            pos(
                "verb",
                "verb",
                vec![group("Active", "active !passive", ""), group("Passive", "passive", "")],
            )
            .initial(3)
            .head_first(),
            pos("adj", "adj", vec![group("", "", "Degree")]).head_first(),
        ];
        s
    }

    /// Jawi spellings are listed as forms; the passive is tagged as a
    /// third person form; labels leak into some forms.
    fn prepare(&self, e: &mut Entry) {
        e.forms.retain(|f| !f.form.contains('&') && f.form.is_ascii());
        for f in &mut e.forms {
            f.tags.retain(|t| t != "third-person");
        }
    }

    fn extra_code(&self) -> String {
        super::indonesian::REDUPLICATION.into()
    }

    fn samples(&self) -> Vec<&'static str> {
        vec!["lihat", "tulis", "baca", "pukul", "besar", "gombrak", "plintir"]
    }

    fn readme(&self) -> String {
        r#"## Example

```rust
use malay::*;
println!("{}", verb_active("lihat")); // melihat
println!("{}", verb_passive("tulis")); // ditulis
println!("{}", adj("besar", Degree::Comparative)); // lebih besar
println!("{}", noun_plural("buku")); // buku-buku
```

Verbs have the active (meN-, with its nasal assimilation) and the passive
(di-), from the headword lines of Wiktionary, which has no Malay
inflection tables; the tree reads the start of the word, where the
prefixes attach. Adjectives compare with lebih and paling. Noun plurals
are reduplication and are written by hand (`noun_plural`). Words are in
Rumi (Latin) spelling; input matching ignores the ĕ dictionaries use for
the schwa.
"#
        .into()
    }
}

pub fn language() -> Box<dyn Language> {
    Box::new(Malay)
}
