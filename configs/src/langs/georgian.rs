use morphe_train::kaikki::Entry;
use morphe_train::spec::*;

pub struct Georgian;

/// Postpositional forms carry a case tag too; the plain case cells leave
/// them out.
const PLAIN: &str = "!on-position !near-position !in-position !like-position !for-position !towards-position \
                     !from-position !in-relation-to-position !since-position !together-with-position !up-to-position";

const PERSONS: &str = "Person Number";

impl Language for Georgian {
    fn spec(&self) -> Spec {
        let mut s = Spec::new("georgian", "Georgian", "ka");
        s.features = vec![
            feature(
                "Case",
                &[
                    ("Nominative", "nominative"),
                    ("Ergative", "ergative"),
                    ("Dative", "dative"),
                    ("Genitive", "genitive"),
                    ("Instrumental", "instrumental"),
                    ("Adverbial", "adverbial"),
                    ("Vocative", "vocative"),
                ],
            ),
            feature(
                "Postposition",
                &[
                    ("On", "on-position"),
                    ("Near", "near-position"),
                    ("In", "in-position"),
                    ("Like", "dative like-position"),
                    ("For", "for-position"),
                    ("From", "from-position"),
                    ("Towards", "towards-position"),
                    ("Regarding", "in-relation-to-position"),
                    ("Similar", "genitive like-position"),
                    ("Since", "since-position"),
                    ("With", "together-with-position"),
                    ("UpTo", "up-to-position"),
                ],
            ),
            feature("Number", &[("Singular", "singular"), ("Plural", "plural")]),
            feature("Person", &[("First", "first-person"), ("Second", "second-person"), ("Third", "third-person")]),
            feature(
                "Screeve",
                &[
                    ("Present", "present !imperfect !subjunctive !perfect"),
                    ("Imperfect", "imperfect"),
                    ("PresentSubjunctive", "present subjunctive"),
                    ("Future", "future !conditional !subjunctive"),
                    ("Conditional", "conditional"),
                    ("FutureSubjunctive", "future subjunctive"),
                    ("Aorist", "aorist !optative"),
                    ("Optative", "optative"),
                    ("Perfect", "perfect !pluperfect !subjunctive"),
                    ("Pluperfect", "pluperfect"),
                    ("PerfectSubjunctive", "perfect subjunctive"),
                ],
            ),
            feature("Aspect", &[("Imperfective", "imperfective"), ("Perfective", "perfective")]),
            feature(
                "Participle",
                &[("Present", "present"), ("Future", "future"), ("Past", "perfect")],
            ),
        ];
        s.pos = vec![
            pos(
                "noun",
                "noun",
                vec![group("", PLAIN, "Case Number"), group("Postposition", "", "Postposition Number")],
            )
            .citation("NominativeSingular GenitiveSingular")
            .head_last(),
            pos(
                "adj",
                "adj",
                vec![
                    group("Attributive", "adjectival", "Case[Nominative,Ergative,Dative,Genitive,Instrumental,Adverbial,Vocative]"),
                    group("", &format!("!adjectival {PLAIN}"), "Case Number"),
                ],
            )
            .head_last(),
            pos(
                "verb",
                "verb",
                vec![
                    group("", "!participle !imperative !noun-from-verb", &format!("Screeve {PERSONS}")),
                    group("Imperative", "imperative !negative", "Person[First,Second] Number"),
                    group("VerbalNoun", "noun-from-verb", "Aspect"),
                    group("Participle", "participle", "Participle Aspect"),
                ],
            )
            .citation("PresentThirdSingular FutureThirdSingular AoristThirdSingular")
            .initial(2)
            .head_last(),
        ];
        s
    }

    /// The conjugation tables come out tagged `error-unrecognized-form`
    /// though their person, number and screeve tags are right.
    fn prepare(&self, e: &mut Entry) {
        if e.pos != "verb" {
            return;
        }
        for f in e.forms.iter_mut().filter(|f| f.source.is_some()) {
            f.tags.retain(|t| t != "error-unrecognized-form");
        }
    }

    fn samples(&self) -> Vec<&'static str> {
        vec!["მეგობარი", "კაცი", "წყალი", "ქართული", "ნიშნავს", "წერს", "ბლიკვანი", "გლორპავს"]
    }

    fn readme(&self) -> String {
        r#"## Example

```rust
use georgian::*;
assert_eq!(noun("მეგობარი", Case::Genitive, Number::Singular), "მეგობრის");
println!("{}", noun("კაცი", Case::Ergative, Number::Plural)); // კაცებმა
println!("{}", noun_postposition("სახლი", Postposition::In, Number::Singular)); // სახლში
println!("{}", adj_attributive("ქართული", Case::Ergative)); // ქართულმა
println!("{}", verb("ნიშნავს", Screeve::Aorist, Person::First, Number::Singular)); // დავნიშნე
println!("{}", verb("ნიშნავს", Screeve::Future, Person::Third, Number::Plural)); // დანიშნავენ
```

Nouns have the seven cases and the twelve postpositional forms (-ში, -ზე,
-თან…) in both numbers; archaic plurals in -ნი/-თა are left out. Adjectives
have their attributive forms, and the full declension where Wiktionary
gives one. Verbs have the eleven screeves for the subject's person and
number (the tables give forms with a third-person object), the imperative,
the verbal nouns and the participles. Verbs are looked up by their
third-person singular present (ნიშნავს), as in Wiktionary. A verb's preverb
(და-, გა-, მო-…) can't be told from its present form, so the future and
aorist of unknown verbs are often wrong.
"#
        .into()
    }
}

pub fn language() -> Box<dyn Language> {
    Box::new(Georgian)
}
