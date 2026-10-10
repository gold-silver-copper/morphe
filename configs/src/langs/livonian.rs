use morphe_train::spec::*;

pub struct Livonian;

impl Language for Livonian {
    fn spec(&self) -> Spec {
        let mut s = Spec::new("livonian", "Livonian", "liv");
        // The broken-tone apostrophe (a’b) is often left out.
        s.marks = vec!['’'];
        s.drop_tags.extend(tags("negative perfect debitive"));
        s.features = vec![
            feature(
                "Case",
                &[
                    ("Nominative", "nominative"),
                    ("Genitive", "genitive"),
                    ("Partitive", "partitive"),
                    ("Dative", "dative"),
                    ("Instrumental", "instrumental"),
                    ("Illative", "illative"),
                    ("Inessive", "inessive"),
                    ("Elative", "elative"),
                ],
            ),
            feature("Number", &[("Singular", "singular"), ("Plural", "plural")]),
            feature("Person", &[("First", "first-person"), ("Second", "second-person"), ("Third", "third-person")]),
            feature(
                "Mood",
                &[
                    ("Present", "indicative present"),
                    ("Past", "indicative past"),
                    ("Imperative", "imperative"),
                    ("Jussive", "jussive"),
                    ("Conditional", "conditional"),
                    ("Quotative", "quotative"),
                ],
            ),
        ];
        let nominal = || vec![group("", "", "Case Number")];
        s.pos = vec![
            pos("noun", "noun", nominal()).citation("NominativeSingular GenitiveSingular PartitiveSingular").head_last(),
            pos("adj", "adj", nominal()).head_last(),
            pos(
                "verb",
                "verb",
                vec![
                    group("Infinitive", "infinitive", ""),
                    group("Supine", "supine !abessive", ""),
                    group("SupineAbessive", "supine abessive", ""),
                    group("ActiveParticiple", "active participle", ""),
                    group("PassiveParticiple", "passive participle", ""),
                    group("Gerund", "gerund", ""),
                    group("", "", "Mood Person Number"),
                ],
            )
            .citation("Infinitive PresentFirstSingular PastFirstSingular")
            .head_last(),
            pos("pron", "pron", nominal()),
            pos("num", "num", nominal()),
        ];
        s
    }

    /// Imperatives carry an exclamation mark and footnote numbers
    /// (mǟ’dlõgõm!²).
    fn split_form(&self, raw: &str) -> Vec<String> {
        vec![raw.trim_end_matches(|c: char| c == '!' || "¹²³⁴⁵".contains(c)).to_string()]
    }

    fn samples(&self) -> Vec<&'static str> {
        vec!["ab", "kalā", "mǟdlõ", "vȯlda", "jellõ", "kirpāl"]
    }

    fn readme(&self) -> String {
        r#"## Example

```rust
use livonian::*;
println!("{}", noun("ab", Case::Partitive, Number::Plural)); // a’bḑi
println!("{}", verb("mǟdlõ", Mood::Past, Person::First, Number::Singular)); // mǟ’dliz
println!("{}", verb("mǟdlõ", Mood::Conditional, Person::First, Number::Plural)); // mǟ’dlõksmõ
println!("{}", verb_supine("mǟdlõ")); // mǟ’dlõm
```

Nouns have eight cases in both numbers. Verbs have the indicative present
and past, the imperative, jussive, conditional and quotative in all
persons, the infinitive, the supine, the participles and the gerund;
compound tenses, the debitive and negatives are built with auxiliaries
and left out. Input matching ignores the broken-tone apostrophe (’).
"#
        .into()
    }
}

pub fn language() -> Box<dyn Language> {
    Box::new(Livonian)
}
