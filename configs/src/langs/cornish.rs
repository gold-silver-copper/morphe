use morphe_train::kaikki::Entry;
use morphe_train::spec::*;

pub struct Cornish;

fn has(tags: &[String], t: &str) -> bool {
    tags.iter().any(|x| x == t)
}

impl Language for Cornish {
    fn spec(&self) -> Spec {
        let mut s = Spec::new("cornish", "Cornish", "kw");
        s.unmark = unmark("âa ûu ÿy ëe êe ùu àa òo ìi èe üu");
        // Wiktionary follows the Standard Written Form; other revived
        // spellings on the headword line are alternatives.
        s.soft_tags.extend(tags("Revived-Late-Cornish Revived-Middle-Cornish"));
        s.features = vec![
            feature("Gender", &[("Masculine", "masculine"), ("Feminine", "feminine")]),
            feature(
                "Mutation",
                &[
                    ("Soft", "mutation-soft"),
                    ("Aspirate", "mutation-aspirate"),
                    ("Hard", "mutation-hard"),
                    ("Mixed", "mutation-mixed"),
                ],
            ),
            feature(
                "Tense",
                &[
                    ("Present", "indicative present"),
                    ("Preterite", "indicative preterite"),
                    ("Imperfect", "indicative imperfect"),
                    ("Pluperfect", "indicative pluperfect"),
                    ("Subjunctive", "subjunctive present"),
                    ("ImperfectSubjunctive", "subjunctive imperfect"),
                ],
            ),
            feature(
                "Person",
                &[
                    ("FirstSingular", "first-person singular"),
                    ("SecondSingular", "second-person singular"),
                    ("ThirdSingular", "third-person singular"),
                    ("FirstPlural", "first-person plural"),
                    ("SecondPlural", "second-person plural"),
                    ("ThirdPlural", "third-person plural"),
                    ("Impersonal", "impersonal"),
                ],
            ),
            feature("Degree", &[("Comparative", "comparative"), ("Superlative", "superlative")]),
        ];
        let mutated = || group("Mutated", "mutation", "Mutation");
        s.pos = vec![
            pos(
                "noun",
                "noun",
                vec![
                    group("Plural", "plural", ""),
                    // piesen: one of the collective pies.
                    group("Singulative", "singulative", ""),
                    group("Dual", "dual", ""),
                    mutated(),
                ],
            )
            .inherent("Gender")
            .citation("Lemma Plural")
            .initial(2)
            .head_first(),
            pos("adj", "adj", vec![group("", "!mutation", "Degree"), mutated()])
                .citation("Lemma Comparative")
                .initial(2),
            pos(
                "verb",
                "verb",
                vec![
                    group("", "!imperative !participle !mutation", "Tense Person"),
                    group("Imperative", "imperative", "Person[SecondSingular,ThirdSingular,FirstPlural,SecondPlural,ThirdPlural]"),
                    group("ParticiplePresent", "participle present", ""),
                    group("ParticiplePast", "participle past", ""),
                    mutated(),
                ],
            )
            .citation("Lemma PresentThirdSingular ParticiplePast")
            .initial(2)
            .head_first(),
        ];
        s
    }

    fn prepare(&self, e: &mut Entry) {
        for f in &mut e.forms {
            if f.source.is_none() {
                // The superlative comes with its article: "an esya".
                if let Some(rest) = f.form.strip_prefix("an ") {
                    f.form = rest.to_string();
                }
                continue;
            }
            if !has(&f.tags, "error-unrecognized-form") {
                continue;
            }
            f.tags.retain(|t| t != "error-unrecognized-form");
            if has(&f.tags, "mutation") {
                // The mutation table's unlabeled row is the hard mutation.
                f.tags.push("mutation-hard".into());
            } else if f.tags.is_empty() {
                // kerys: the past participle.
                f.tags = tags("participle past");
            } else if has(&f.tags, "participle") {
                // "ow kara"
                if let Some(rest) = f.form.strip_prefix("ow ") {
                    f.form = rest.to_string();
                }
            } else if (has(&f.tags, "indicative") || has(&f.tags, "subjunctive"))
                && !["preterite", "imperfect", "pluperfect"].iter().any(|t| has(&f.tags, t))
            {
                // The present-future has no tense label.
                f.tags.push("present".into());
            }
        }
    }

    fn samples(&self) -> Vec<&'static str> {
        vec!["pies", "on", "kara", "mos", "moon", "both", "glorya", "snowk"]
    }

    fn readme(&self) -> String {
        r#"## Example

```rust
use cornish::*;
println!("{}", noun_plural("on")); // eyn
println!("{}", noun_mutated("pies", Mutation::Soft)); // bies
println!("{}", verb("kara", Tense::Preterite, Person::ThirdSingular)); // karas
println!("{}", verb_participle_past("kara")); // kerys
println!("{}", adj("moon", Degree::Comparative)); // monna
```

Spellings follow the Standard Written Form, as in Wiktionary. The present
participle is given without "ow" and the superlative without "an".
"#
        .into()
    }
}

pub fn language() -> Box<dyn Language> {
    Box::new(Cornish)
}
