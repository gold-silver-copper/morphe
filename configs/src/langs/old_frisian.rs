use morphe_train::kaikki::Entry;
use morphe_train::spec::*;

pub struct OldFrisian;

fn has(tags: &[String], t: &str) -> bool {
    tags.iter().any(|x| x == t)
}

fn only(tags: &[String], want: &[&str]) -> bool {
    tags.len() == want.len() && want.iter().all(|w| has(tags, w))
}

impl Language for OldFrisian {
    fn spec(&self) -> Spec {
        let mut s = Spec::new("old_frisian", "OldFrisian", "ofs");
        s.unmark = unmark("āa ēe īi ōo ūu êe");
        s.features = vec![
            feature(
                "Case",
                &[("Nominative", "nominative"), ("Accusative", "accusative"), ("Genitive", "genitive"), ("Dative", "dative")],
            ),
            feature("Number", &[("Singular", "singular"), ("Plural", "plural")]),
            feature("Gender", &[("Masculine", "masculine"), ("Feminine", "feminine"), ("Neuter", "neuter")]),
            feature(
                "PersonNumber",
                &[
                    ("FirstSingular", "first-person singular"),
                    ("SecondSingular", "second-person singular"),
                    ("ThirdSingular", "third-person singular"),
                    ("Plural", "plural"),
                ],
            ),
            feature("Tense", &[("Present", "present"), ("Past", "past")]),
            feature("Declension", &[("Strong", "strong"), ("Weak", "weak")]),
        ];
        s.pos = vec![
            pos("noun", "noun", vec![group("", "", "Case Number")])
                .inherent("Gender")
                .citation("NominativeSingular GenitiveSingular NominativePlural")
                .head_last(),
            pos("adj", "adj", vec![group("", "singular", "Declension Case Gender")])
                .lemma_cell("StrongNominativeMasculine")
                .citation("StrongNominativeMasculine"),
            pos(
                "verb",
                "verb",
                vec![
                    group("Infinitive", "infinitive", ""),
                    group("", "indicative", "Tense PersonNumber"),
                    group("Subjunctive", "subjunctive", "Tense Number"),
                    group("Imperative", "imperative", "Number"),
                    group("Participle", "participle", "Tense"),
                ],
            )
            .citation("Infinitive PastThirdSingular PastPlural ParticiplePast")
            .head_first(),
            pos("det", "det", vec![group("", "singular", "Declension? Case Gender")]),
            pos("num", "num", vec![group("", "", "Case Gender")]),
        ];
        s
    }

    fn prepare(&self, e: &mut Entry) {
        if e.pos == "adj" || e.pos == "det" {
            // Strong and weak tables differ only in their header.
            let mut decl = String::new();
            for f in &mut e.forms {
                if f.source.is_none() {
                    continue;
                }
                if has(&f.tags, "table-tags") {
                    decl = f.form.clone();
                } else if decl == "strong" || decl == "weak" {
                    f.tags.push(decl.clone());
                }
            }
        }
        if e.pos != "verb" {
            return;
        }
        // The table names no moods: after the indicative come the
        // subjunctive (singular, then plural), the imperative and the
        // participles, each told apart only by their place.
        #[derive(PartialEq)]
        enum Mood {
            Indicative,
            Subjunctive,
            Imperative,
        }
        let mut mood = Mood::Indicative;
        let mut subj_past = false;
        for f in &mut e.forms {
            if f.source.is_none() || has(&f.tags, "table-tags") {
                if has(&f.tags, "table-tags") {
                    mood = Mood::Indicative;
                    subj_past = false;
                }
                continue;
            }
            let t = &f.tags;
            let personal = has(t, "first-person") || has(t, "second-person") || has(t, "third-person");
            let tag = if has(t, "infinitive") || personal {
                "indicative"
            } else if only(t, &["present"]) || only(t, &["past"]) {
                "participle"
            } else if only(t, &["present", "singular"]) {
                if mood == Mood::Indicative {
                    mood = Mood::Subjunctive;
                } else if mood == Mood::Subjunctive && subj_past {
                    mood = Mood::Imperative;
                }
                if mood == Mood::Imperative { "imperative" } else { "subjunctive" }
            } else if only(t, &["past", "singular"]) {
                if mood == Mood::Indicative {
                    mood = Mood::Subjunctive;
                }
                subj_past = true;
                "subjunctive"
            } else {
                match mood {
                    Mood::Indicative => "indicative",
                    Mood::Subjunctive => "subjunctive",
                    Mood::Imperative => "imperative",
                }
            };
            if tag == "imperative" {
                f.tags.retain(|x| x != "present");
            }
            f.tags.push(tag.into());
        }
    }

    fn samples(&self) -> Vec<&'static str> {
        vec!["god", "abba", "brother", "luka", "makia", "sia", "al", "thin", "flomia", "snor"]
    }

    fn readme(&self) -> String {
        r#"## Example

```rust
use old_frisian::*;
println!("{}", noun("abba", Case::Dative, Number::Plural)); // abbaum
println!("{:?}", noun_gender("brother"));
println!("{}", verb("luka", Tense::Past, PersonNumber::ThirdSingular)); // lāk
println!("{}", verb_participle("makia", Tense::Past)); // emakad
println!("{}", adj("al", Declension::Strong, Case::Accusative, Gender::Masculine)); // alene
```

Macrons are optional on input; output carries them.
"#
        .into()
    }
}

pub fn language() -> Box<dyn Language> {
    Box::new(OldFrisian)
}
