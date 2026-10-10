use morphe_train::kaikki::Entry;
use morphe_train::spec::*;

pub struct Xhosa;

/// Tone-marked vowels and syllabic nasals → plain letters.
const TONES: &str = "áa ée íi óo úu âa êe îi ôo ûu àa èe ìi òo ùu ḿm ńn";

impl Language for Xhosa {
    fn spec(&self) -> Spec {
        let mut s = Spec::new("xhosa", "Xhosa", "xh");
        s.unmark = unmark(TONES);
        s.canonical_spelling = false;
        s.features = vec![feature(
            "Class",
            &[
                ("C1", "class-1"),
                ("C1a", "class-1a"),
                ("C2", "class-2"),
                ("C3", "class-3"),
                ("C4", "class-4"),
                ("C5", "class-5"),
                ("C6", "class-6"),
                ("C7", "class-7"),
                ("C8", "class-8"),
                ("C9", "class-9"),
                ("C11", "class-11"),
                ("C14", "class-14"),
                ("C15", "class-15"),
                ("C17", "class-17"),
            ],
        )];
        s.pos = vec![pos("noun", "noun", vec![group("Plural", "plural", "")])
            .inherent("Class")
            .citation("Lemma Plural")
            .initial(3)
            .head_last()];
        s
    }

    /// Plurals are written with tone marks on the headword line; they are
    /// read in plain spelling. The one inflection table is left out.
    fn prepare(&self, e: &mut Entry) {
        let pairs = unmark(TONES);
        e.forms.retain(|f| f.source.is_none());
        for f in &mut e.forms {
            f.form = f.form.chars().map(|c| pairs.iter().find(|p| p.0 == c).map_or(c, |p| p.1)).collect();
        }
    }

    fn samples(&self) -> Vec<&'static str> {
        vec!["impala", "igazi", "isilo", "umpu", "umntu", "isikhwepha"]
    }

    fn readme(&self) -> String {
        r#"## Example

```rust
use xhosa::*;
println!("{}", noun_plural("impala")); // iimpala
println!("{}", noun_plural("isilo")); // izilo
println!("{:?}", noun_class("igazi")); // C5
```

Nouns have their plural and noun class; the tree reads the start of the
word as well as its end, since Xhosa marks class and number with prefixes.
Wiktionary has no Xhosa verb tables (verbs are listed by stem only).
Plurals are given in plain spelling, and input matching ignores tone marks.
"#
        .into()
    }
}

pub fn language() -> Box<dyn Language> {
    Box::new(Xhosa)
}
