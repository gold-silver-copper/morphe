use morphe_train::kaikki::Entry;
use morphe_train::spec::*;

pub struct OldDutch;

fn has(tags: &[String], t: &str) -> bool {
    tags.iter().any(|x| x == t)
}

fn only(tags: &[String], want: &[&str]) -> bool {
    tags.len() == want.len() && want.iter().all(|w| has(tags, w))
}

impl Language for OldDutch {
    fn spec(&self) -> Spec {
        let mut s = Spec::new("old_dutch", "OldDutch", "odt");
        s.unmark = unmark("āa ēe īi ōo ūu");
        s.features = vec![
            feature(
                "Case",
                &[("Nominative", "nominative"), ("Accusative", "accusative"), ("Genitive", "genitive"), ("Dative", "dative")],
            ),
            feature("Number", &[("Singular", "singular"), ("Plural", "plural")]),
            feature("Gender", &[("Masculine", "masculine"), ("Feminine", "feminine"), ("Neuter", "neuter")]),
            feature(
                "GenderNumber",
                &[
                    ("Masculine", "masculine !plural"),
                    ("Feminine", "feminine !plural"),
                    ("Neuter", "neuter !plural"),
                    ("Plural", "plural"),
                ],
            ),
            feature("Declension", &[("Strong", "strong"), ("Weak", "weak")]),
            feature("Mood", &[("Indicative", "indicative"), ("Subjunctive", "subjunctive")]),
            feature("Tense", &[("Present", "present"), ("Past", "past")]),
            feature("Person", &[("First", "first-person"), ("Second", "second-person"), ("Third", "third-person")]),
        ];
        s.pos = vec![
            pos("noun", "noun", vec![group("", "", "Case Number")])
                .inherent("Gender")
                .citation("NominativeSingular GenitiveSingular NominativePlural")
                .head_last(),
            pos("adj", "adj", vec![group("", "", "Declension Case GenderNumber")])
                .citation("StrongNominativeMasculine"),
            pos(
                "verb",
                "verb",
                vec![
                    group("Infinitive", "infinitive", ""),
                    group("", "!imperative !participle !infinitive", "Mood Tense Person Number"),
                    group("Imperative", "imperative", "Number"),
                    group("Participle", "participle", "Tense"),
                ],
            )
            .citation("Infinitive IndicativePastThirdSingular IndicativePastThirdPlural ParticiplePast")
            .head_first(),
        ];
        s
    }

    fn prepare(&self, e: &mut Entry) {
        if e.pos != "verb" {
            return;
        }
        // The table names no moods. The indicative comes first and ends
        // with the third person plural past; the subjunctive repeats its
        // tags; then come the imperative and the participles.
        let mut subj = false;
        let mut ind_done = false;
        for f in &mut e.forms {
            if f.source.is_none() {
                continue;
            }
            if has(&f.tags, "table-tags") {
                subj = false;
                ind_done = false;
                continue;
            }
            let t = f.tags.clone();
            let personal = has(&t, "first-person") || has(&t, "second-person") || has(&t, "third-person");
            if personal {
                let last = only(&t, &["past", "plural", "third-person"]);
                if ind_done && !last {
                    subj = true;
                }
                if last && !subj {
                    ind_done = true;
                }
                f.tags.push(if subj { "subjunctive" } else { "indicative" }.into());
            } else if only(&t, &["present", "singular"]) || only(&t, &["plural", "present"]) {
                f.tags.retain(|x| x != "present");
                f.tags.push("imperative".into());
            } else if only(&t, &["present"]) || only(&t, &["past"]) {
                f.tags.push("participle".into());
            } else if f.form == "participle" {
                // A header cell read as a form.
                f.tags = vec!["table-tags".into()];
            }
        }
    }

    fn samples(&self) -> Vec<&'static str> {
        vec!["craft", "at", "win", "teman", "han", "het", "lao", "flomon", "snor"]
    }

    fn readme(&self) -> String {
        r#"## Example

```rust
use old_dutch::*;
println!("{}", noun("craft", Case::Dative, Number::Plural)); // creftin
println!("{:?}", noun_gender("craft"));
println!("{}", verb("teman", Mood::Indicative, Tense::Past, Person::Third, Number::Plural)); // tāmon
println!("{}", verb_participle("teman", Tense::Past)); // gituman
println!("{}", adj("het", Declension::Weak, Case::Nominative, GenderNumber::Masculine)); // hēto
```

Macrons are optional on input; output carries them. Wiktionary's Old Dutch
paradigms are largely reconstructed from the sparse record.
"#
        .into()
    }
}

pub fn language() -> Box<dyn Language> {
    Box::new(OldDutch)
}
