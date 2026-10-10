use morphe_train::kaikki::{Entry, Form};
use morphe_train::spec::*;

pub struct Slovene;

/// Tonal and stress letters (precomposed), the schwa and the "ł" of
/// pronunciation spellings, and their plain letters.
const UNMARK: &str = "áa àa âa ȁa ȃa āa ée èe êe ȅe ȇe ēe ẹe íi ìi îi ȉi ȋi īi óo òo ôo ȍo ȏo ōo ọo úu ùu ûu ȕu ȗu ūu ŕr ȑr ȓr əe łl";

impl Language for Slovene {
    fn spec(&self) -> Spec {
        let mut s = Spec::new("slovene", "Slovene", "sl");
        s.marks = vec!['\u{300}', '\u{301}', '\u{302}', '\u{304}', '\u{30F}', '\u{311}', '\u{323}'];
        s.unmark = unmark(UNMARK);
        s.drop_tags.extend(tags("dated"));
        s.soft_tags.extend(tags("poetic literary"));
        s.features = vec![
            feature(
                "Case",
                &[
                    ("Nominative", "nominative"),
                    ("Genitive", "genitive"),
                    ("Dative", "dative"),
                    ("Accusative", "accusative"),
                    ("Locative", "locative"),
                    ("Instrumental", "instrumental"),
                ],
            ),
            feature("Number", &[("Singular", "singular"), ("Dual", "dual"), ("Plural", "plural")]),
            feature("Gender", &[("Masculine", "masculine"), ("Feminine", "feminine"), ("Neuter", "neuter")]),
            feature("Animacy", &[("Animate", "animate"), ("Inanimate", "inanimate")]),
            feature("Aspect", &[("Imperfective", "imperfective"), ("Perfective", "perfective")]),
            feature("Person", &[("First", "first-person"), ("Second", "second-person"), ("Third", "third-person")]),
            feature("Tense", &[("Present", "present"), ("Past", "past")]),
        ];
        let adjlike = "Case Number Gender Animacy?";
        s.pos = vec![
            pos("noun", "noun", vec![group("", "!vocative", "Case Number")])
                .inherent("Gender Animacy")
                .citation("NominativeSingular GenitiveSingular")
                .head_last(),
            pos(
                "adj",
                "adj",
                vec![
                    group("", "!comparative !superlative", adjlike),
                    group("Comparative", "comparative", ""),
                    group("Superlative", "superlative", ""),
                ],
            ),
            pos(
                "verb",
                "verb",
                vec![
                    group("Infinitive", "infinitive", ""),
                    group("Supine", "supine", ""),
                    group("Present", "present !imperative !participle", "Person Number"),
                    group("Imperative", "imperative second-person", "Number"),
                    group("ImperativeFirst", "imperative first-person", "Number[Dual,Plural]"),
                    group("Past", "l-participle", "Number Gender"),
                    group("Participle", "participle !converb !l-participle", "Tense"),
                    group("Converb", "converb participle !l-participle", "Tense"),
                    group("VerbalNoun", "noun-from-verb", ""),
                ],
            )
            .inherent("Aspect")
            .citation("Infinitive PresentFirstSingular"),
            pos("pron", "pron", vec![group("", "", "Case Number? Gender?")]),
            pos("det", "det", vec![group("", "", adjlike)]),
            pos("num", "num", vec![group("", "", "Case Number? Gender?")]),
        ];
        s
    }

    /// Adjective tables give the masculine accusative singular as the words
    /// "nominative (inanimate) or genitive (animate)": put the forms in.
    fn reshape(&self, pos: &PosSpec, e: &Entry) -> Option<Vec<Form>> {
        if pos.name != "adj" && pos.name != "det" {
            return None;
        }
        let pointer = |f: &Form| f.form.starts_with("nominative") || f.form == "genitive";
        if !e.forms.iter().any(pointer) {
            return None;
        }
        let masc_sg = |case: &str| -> Vec<String> {
            e.forms
                .iter()
                .filter(|f| f.source.is_some() && ["masculine", "singular", case].iter().all(|t| f.tags.iter().any(|x| x == t)))
                .map(|f| f.form.clone())
                .collect()
        };
        let mut out: Vec<Form> = e.forms.iter().filter(|f| !pointer(f)).cloned().collect();
        for (case, animacy) in [("nominative", "inanimate"), ("genitive", "animate")] {
            for form in masc_sg(case) {
                out.push(Form { form, tags: tags(&format!("accusative masculine singular {animacy}")), source: Some("declension".into()) });
            }
        }
        Some(out)
    }

    /// Some conjugation templates leave the infinitive row unrecognized.
    fn retag(&self, pos: &PosSpec, form: &str, t: &mut Vec<String>) {
        if pos.name == "verb" && t.len() == 1 && t[0] == "error-unrecognized-form" {
            let plain: String = form.chars().filter(|c| c.is_alphabetic()).collect();
            if ["ti", "či", "t", "č"].iter().any(|e| plain.ends_with(e)) {
                *t = tags("infinitive");
            }
        }
    }

    /// Drop the gender letter of "mídva m".
    fn split_form(&self, raw: &str) -> Vec<String> {
        let raw = raw.trim();
        let stripped = [" m", " f", " n"].iter().find_map(|g| raw.strip_suffix(g)).unwrap_or(raw);
        vec![stripped.to_string()]
    }

    fn readme(&self) -> String {
        r#"## Example

```rust
use slovene::*;
println!("{}", noun("žena", Case::Instrumental, Number::Dual));
println!("{}", noun("mesto", Case::Locative, Number::Plural));
println!("{:?}", noun_gender("miza"));
println!("{}", verb_present("delati", Person::First, Number::Dual));
println!("{}", verb_past("delati", Number::Singular, Gender::Feminine));
println!("{}", adj("raven", Case::Genitive, Number::Singular, Gender::Feminine, Animacy::Inanimate));
let plain = Slovene::default().marks(false);
println!("{}", plain.verb_past("biti", Number::Singular, Gender::Masculine));
// Citations pin principal parts: nominative and genitive singular.
println!("{}", noun("pes, psa", Case::Dative, Number::Singular));
```

Input matching ignores tonal and stress marks (á, ȃ, ẹ́, ọ̑…), the schwa ə and
the pronunciation letter ł. Output carries them as Wiktionary writes them;
`marks(false)` gives the plain spelling (bil, not bȋł). č, š and ž are letters.
"#
        .into()
    }
}

pub fn language() -> Box<dyn Language> {
    Box::new(Slovene)
}
