use morphe_train::kaikki::Entry;
use morphe_train::spec::*;

pub struct Votic;

/// Cases whose table rows the parser tags right; in the rows after them the
/// singular comes tagged `plural` and the plural untagged.
const UPPER: [&str; 4] = ["nominative", "genitive", "partitive", "illative"];

impl Language for Votic {
    fn spec(&self) -> Spec {
        let mut s = Spec::new("votic", "Votic", "vot");
        s.drop_tags.extend(tags("negative perfect pluperfect"));
        // The palatalization mark ʹ is often typed as an apostrophe.
        s.fold = vec![('\'', 'ʹ'), ('’', 'ʹ')];
        s.features = vec![
            feature(
                "Case",
                &[
                    ("Nominative", "nominative"),
                    ("Genitive", "genitive"),
                    ("Partitive", "partitive"),
                    ("Illative", "illative"),
                    ("Inessive", "inessive"),
                    ("Elative", "elative"),
                    ("Allative", "allative"),
                    ("Adessive", "adessive"),
                    ("Ablative", "ablative"),
                    ("Translative", "translative"),
                ],
            ),
            feature("Number", &[("Singular", "singular"), ("Plural", "plural")]),
            feature("Person", &[("First", "first-person"), ("Second", "second-person"), ("Third", "third-person")]),
            feature(
                "Mood",
                &[
                    ("Present", "indicative present"),
                    ("Past", "imperfect"),
                    ("Conditional", "conditional"),
                    ("Imperative", "imperative"),
                ],
            ),
            feature("Voice", &[("Active", "active"), ("Passive", "passive")]),
            feature("Tense", &[("Present", "present"), ("Past", "past")]),
            feature(
                "Supine",
                &[("Illative", "illative"), ("Inessive", "inessive"), ("Elative", "elative"), ("Abessive", "abessive")],
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
                    group("Infinitive", "infinitive first-person", ""),
                    group("Supine", "infinitive second-person", "Supine"),
                    group("", "!impersonal !participle !infinitive", "Mood Person Number"),
                    group("Impersonal", "impersonal", "Mood"),
                    group("Participle", "participle", "Voice Tense"),
                ],
            )
            .citation("Infinitive PresentFirstSingular PastFirstSingular")
            .head_last(),
            pos("pron", "pron", nominal()),
            pos("num", "num", nominal()),
        ];
        s
    }

    /// In the declension, rows below the illative tag their singular
    /// `plural` and leave the plural untagged. The conjugation leaves the
    /// imperative untagged, and tags the infinitive `first-person` and the
    /// supine `second-person`.
    fn prepare(&self, e: &mut Entry) {
        let verb = e.pos == "verb";
        for f in e.forms.iter_mut().filter(|f| f.source.is_some()) {
            let has = |tags: &[String], t: &str| tags.iter().any(|x| x == t);
            if verb {
                if has(&f.tags, "present") && !has(&f.tags, "indicative") && !has(&f.tags, "conditional") && !has(&f.tags, "participle") {
                    f.tags.push("imperative".into());
                }
                continue;
            }
            let upper = UPPER.iter().any(|c| has(&f.tags, c));
            let case = f.tags.iter().any(|t| t.ends_with("ive"));
            if case && !upper && !has(&f.tags, "singular") {
                let plural = has(&f.tags, "plural");
                f.tags.retain(|t| t != "plural");
                f.tags.push(if plural { "singular" } else { "plural" }.into());
            }
        }
    }

    fn samples(&self) -> Vec<&'static str> {
        vec!["kana", "jalkõ", "liikkua", "kuttsua", "kirpakka", "plotkia"]
    }

    fn readme(&self) -> String {
        r#"## Example

```rust
use votic::*;
println!("{}", noun("kana", Case::Inessive, Number::Singular)); // kanaz
println!("{}", noun("kana", Case::Partitive, Number::Plural)); // kanoit
println!("{}", verb("liikkua", Mood::Present, Person::Third, Number::Singular)); // liikub
println!("{}", verb("liikkua", Mood::Past, Person::First, Number::Singular)); // liikkuzin
println!("{}", verb_supine("liikkua", Supine::Illative)); // liikkuma
```

Nouns have ten cases in both numbers (the standardized Vaipooli forms).
Verbs have the present, past, conditional and imperative in all persons
and the impersonal, the infinitive, the supine and the participles;
compound tenses and negatives are built with auxiliaries and left out.
"#
        .into()
    }
}

pub fn language() -> Box<dyn Language> {
    Box::new(Votic)
}
