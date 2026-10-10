use morphe_train::kaikki::Entry;
use morphe_train::spec::*;

pub struct Esperanto;

const UNMARK: &str = "ĉc ĝg ĥh ĵj ŝs ŭu";

impl Language for Esperanto {
    fn spec(&self) -> Spec {
        let mut s = Spec::new("esperanto", "Esperanto", "eo");
        // Matching only: typed ĉ, ŭ… are kept, so unknown words keep them.
        s.fold = unmark(UNMARK);
        // The x-system: cx = ĉ, ux = ŭ…
        s.fold_strs = ["cx", "gx", "hx", "jx", "sx", "ux"].iter().map(|x| (x.to_string(), x[..1].to_string())).collect();
        s.features = vec![
            feature("Case", &[("Nominative", "!accusative"), ("Accusative", "accusative")]),
            feature("Number", &[("Singular", "!plural"), ("Plural", "plural")]),
            feature(
                "Tense",
                &[
                    ("Present", "present !infinitive"),
                    ("Past", "past !imperative"),
                    ("Future", "future !conditional"),
                    ("Conditional", "conditional"),
                    ("Volitive", "imperative"),
                ],
            ),
            feature("Voice", &[("Active", "active"), ("Passive", "passive")]),
        ];
        let tenses = "Tense[Present,Past,Future]";
        s.pos = vec![
            pos("noun", "noun", vec![group("", "!alternative !abbreviation", "Case Number")]).head_last(),
            pos("adj", "adj det", vec![group("", "!alternative", "Case Number")]).head_last(),
            pos("pron", "pron", vec![group("", "!alternative !possessive !plural", "Case"), group("Possessive", "possessive", "")]),
            pos(
                "verb",
                "verb",
                vec![
                    group("", "!participle !infinitive", "Tense"),
                    group("Participle", "participle !noun-from-verb !adverbial !accusative !plural", &format!("Voice {tenses}"))
                        .declines("adj", ""),
                    group("ParticipleNoun", "participle noun-from-verb !accusative !plural", &format!("Voice {tenses}"))
                        .declines("noun", ""),
                    group("ParticipleAdverb", "participle adverbial", &format!("Voice {tenses}")),
                ],
            )
            .head_first(),
        ];
        s
    }

    fn keep_entry(&self, e: &Entry) -> bool {
        // x-system spellings (cxevalo) are entries of their own.
        !e.word.contains(['x', 'X'])
    }

    fn adjust(&self, pos: &PosSpec, lemma: &str, forms: &mut Vec<(usize, String, bool)>) {
        // The nominative singular is the headword itself; tables leave it out.
        // Entries without forms (ĉevaloj, form-of entries) stay out.
        let spec = self.spec();
        let cells = spec.cells(pos);
        let Some(nom) = cells.iter().position(|c| c.name == "NominativeSingular" || c.name == "Nominative") else { return };
        if !forms.is_empty() && !forms.iter().any(|f| f.0 == nom) {
            forms.push((nom, lemma.to_string(), false));
        }
    }

    fn samples(&self) -> Vec<&'static str> {
        vec!["hundo", "fari", "esti", "bona", "mi", "blorko", "ŝpluki"]
    }

    fn readme(&self) -> String {
        r#"## Example

```rust
use esperanto::*;
assert_eq!(noun("ĉevalo", Case::Accusative, Number::Plural), "ĉevalojn");
assert_eq!(adj("bela", Case::Nominative, Number::Plural), "belaj");
assert_eq!(verb("ami", Tense::Past), "amis");
assert_eq!(verb_participle("ami", Voice::Passive, Tense::Present), "amata");
println!("{}", verb_participle_noun("lerni", Voice::Active, Tense::Present)); // lernanto
// Input without diacritics or in the x-system finds known words:
println!("{}", noun("cxevalo", Case::Accusative, Number::Singular));
```

Esperanto is regular, so unknown words inflect as reliably as known ones.
"#
        .into()
    }
}

pub fn language() -> Box<dyn Language> {
    Box::new(Esperanto)
}
