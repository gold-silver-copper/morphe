use morphe_train::kaikki::Entry;
use morphe_train::spec::*;

pub struct Ingrian;

/// Cases whose table rows the parser tags right; in the rows after them the
/// singular comes tagged `plural` and the plural untagged.
const UPPER: [&str; 4] = ["nominative", "genitive", "partitive", "illative"];
/// The conjugation's present-tense blocks, in table order; each ends with
/// its impersonal form.
const MOODS: [&str; 4] = ["indicative", "conditional", "imperative", "potential"];

impl Language for Ingrian {
    fn spec(&self) -> Spec {
        let mut s = Spec::new("ingrian", "Ingrian", "izh");
        s.drop_tags.extend(tags("negative perfect pluperfect"));
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
                    ("Essive", "essive"),
                    ("Exessive", "exessive"),
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
                    ("Potential", "potential"),
                ],
            ),
            feature("Voice", &[("Active", "active"), ("Passive", "passive")]),
            feature("Tense", &[("Present", "present"), ("Past", "past")]),
            feature(
                "Infinitive",
                &[
                    ("IiInessive", "infinitive-ii inessive"),
                    ("IiInstructive", "infinitive-ii instructive"),
                    ("IiiIllative", "infinitive-iii illative"),
                    ("IiiInessive", "infinitive-iii inessive"),
                    ("IiiElative", "infinitive-iii elative"),
                    ("IiiAbessive", "infinitive-iii abessive"),
                    ("IvNominative", "infinitive-iv nominative"),
                    ("IvPartitive", "infinitive-iv partitive"),
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
                    group("Infinitive", "infinitive-i", ""),
                    group("InfinitiveCase", "infinitive !infinitive-i", "Infinitive"),
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

    /// Only the first declension table is read (the second repeats it with
    /// worse tags). In it, rows below the illative tag their singular
    /// `plural` and leave the plural untagged. The conjugation tags its
    /// present, conditional, imperative and potential blocks all `present`.
    fn prepare(&self, e: &mut Entry) {
        let mut table = 0;
        let mut mood = 0usize;
        e.forms.retain_mut(|f| {
            if f.source.is_none() {
                return true;
            }
            if f.tags.iter().any(|t| t == "table-tags") {
                table += 1;
                return false;
            }
            let has = |tags: &[String], t: &str| tags.iter().any(|x| x == t);
            if f.source.as_deref() == Some("declension") {
                if table > 1 {
                    return false;
                }
                let upper = UPPER.iter().any(|c| has(&f.tags, c));
                if !upper && !has(&f.tags, "singular") {
                    let plural = has(&f.tags, "plural");
                    f.tags.retain(|t| t != "plural");
                    f.tags.push(if plural { "singular" } else { "plural" }.into());
                }
                return true;
            }
            if has(&f.tags, "present") && !has(&f.tags, "participle") && !has(&f.tags, "negative") {
                let Some(m) = MOODS.get(mood) else { return false };
                f.tags.push((*m).into());
                if has(&f.tags, "impersonal") {
                    mood += 1;
                }
            }
            true
        });
    }

    fn samples(&self) -> Vec<&'static str> {
        vec!["kana", "lens", "tulla", "olla", "suuri", "kirpakka", "plotkia"]
    }

    fn readme(&self) -> String {
        r#"## Example

```rust
use ingrian::*;
println!("{}", noun("kana", Case::Inessive, Number::Singular)); // kanas
println!("{}", noun("kana", Case::Partitive, Number::Plural)); // kanoja
println!("{}", verb("tulla", Mood::Present, Person::Third, Number::Singular)); // tulloo
println!("{}", verb("tulla", Mood::Conditional, Person::First, Number::Singular)); // tulisin
println!("{}", verb_participle("tulla", Voice::Passive, Tense::Past)); // tultu
```

Nouns have twelve cases in both numbers. Verbs have the present, past,
conditional, imperative and potential in all persons and the impersonal,
the infinitives and the participles; compound tenses and negatives (en
tule, oon tult) are built with auxiliaries and left out.
"#
        .into()
    }
}

pub fn language() -> Box<dyn Language> {
    Box::new(Ingrian)
}
