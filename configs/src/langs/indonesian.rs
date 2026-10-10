use morphe_train::kaikki::Entry;
use morphe_train::spec::*;

pub struct Indonesian;

/// Plurals are full reduplication, which edit trees can't learn; it is
/// written by hand.
pub const REDUPLICATION: &str = r#"/// A noun's plural: the (first) word reduplicated, as in buku-buku,
/// rumah-rumah sakit.
pub fn noun_plural(word: &str) -> String {
    let word = word.trim();
    match word.split_once(' ') {
        Some((head, rest)) => format!("{head}-{head} {rest}"),
        None => format!("{word}-{word}"),
    }
}
"#;

impl Language for Indonesian {
    fn spec(&self) -> Spec {
        let mut s = Spec::new("indonesian", "Indonesian", "id");
        // The conjugation tables are mostly untagged; the headword lines
        // give the voice forms.
        s.source = Source::Head;
        // Headwords mark the schwa (ê), which ordinary spelling doesn't.
        s.canonical_spelling = false;
        s.unmark = unmark("êe ée èe");
        s.features = vec![];
        s.pos = vec![pos(
            "verb",
            "verb",
            vec![
                group("Active", "active !passive !causative", ""),
                group("Passive", "passive !reflexive !causative", ""),
                group("Stative", "passive reflexive", ""),
                group("Imperative", "imperative !causative !root", ""),
                group("Jussive", "jussive", ""),
            ],
        )
        .initial(3)
        .head_first()];
        s
    }

    /// Some headword lines leak their labels into the forms ("& colloquial").
    fn prepare(&self, e: &mut Entry) {
        e.forms.retain(|f| !f.form.contains('&'));
    }

    fn extra_code(&self) -> String {
        REDUPLICATION.into()
    }

    fn samples(&self) -> Vec<&'static str> {
        vec!["cat", "lihat", "tulis", "pukul", "sapu", "baca", "gombrak", "plintir"]
    }

    fn readme(&self) -> String {
        r#"## Example

```rust
use indonesian::*;
println!("{}", verb_active("tulis")); // menulis
println!("{}", verb_active("pukul")); // memukul
println!("{}", verb_active("cat")); // mengecat
println!("{}", verb_passive("lihat")); // dilihat
println!("{}", verb_stative("balik")); // terbalik
println!("{}", noun_plural("buku")); // buku-buku
```

Verbs have the active (meN-, with its nasal assimilation), the passive
(di-), the stative/accidental ter- form, the imperative and the jussive
(-lah), from the headword lines of Wiktionary; the tree reads the start of
the word, where the prefixes attach. Noun plurals are reduplication and are
written by hand (`noun_plural`). Input matching ignores the ê that
dictionaries use for the schwa.
"#
        .into()
    }
}

pub fn language() -> Box<dyn Language> {
    Box::new(Indonesian)
}
