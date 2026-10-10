use morphe_train::kaikki::{Entry, Form};
use morphe_train::spec::*;

pub struct Sardinian;

fn has(tags: &[String], t: &str) -> bool {
    tags.iter().any(|x| x == t)
}

impl Language for Sardinian {
    fn spec(&self) -> Spec {
        let mut s = Spec::new("sardinian", "Sardinian", "sc");
        s.unmark = unmark("àa èe ée ìi íi òo óo ùu úu áa");
        // The future (apo a cantare) and conditional (dia cantare) are
        // periphrastic, like the compound tenses.
        s.drop_tags.extend(tags("multiword-construction future conditional"));
        // The tables follow the Logudorese-based standard; forms of the
        // other varieties on the headword line are alternatives.
        s.soft_tags.extend(tags("Campidanese Nuorese Logudorese"));
        s.features = vec![
            feature("Gender", &[("Masculine", "masculine"), ("Feminine", "feminine")]),
            feature(
                "Form",
                &[
                    ("Feminine", "feminine singular"),
                    ("MasculinePlural", "masculine plural"),
                    ("FemininePlural", "feminine plural"),
                ],
            ),
            feature(
                "Tense",
                &[
                    ("Present", "indicative present"),
                    ("Imperfect", "indicative imperfect"),
                    ("Subjunctive", "subjunctive present"),
                    ("ImperfectSubjunctive", "subjunctive imperfect"),
                ],
            ),
            feature("Person", &[("First", "first-person"), ("Second", "second-person"), ("Third", "third-person")]),
            feature("Number", &[("Singular", "singular"), ("Plural", "plural")]),
        ];
        s.pos = vec![
            pos("noun", "noun", vec![group("Plural", "plural !feminine", "")])
                .inherent("Gender")
                .citation("Lemma Plural")
                .head_last(),
            pos("adj", "adj", vec![group("", "", "Form")]).citation("Lemma Feminine MasculinePlural"),
            pos(
                "verb",
                "verb",
                vec![
                    group("Infinitive", "infinitive", ""),
                    group("Gerund", "gerund", ""),
                    group("ParticiplePast", "participle past", ""),
                    group("", "!imperative", "Tense Person Number"),
                    group("Imperative", "imperative", "Person Number"),
                ],
            )
            .citation("Infinitive PresentFirstSingular ParticiplePast")
            .head_first(),
        ];
        s
    }

    fn prepare(&self, e: &mut Entry) {
        if !e.forms.iter().any(|f| has(&f.tags, "canonical")) {
            e.forms.push(Form { form: e.word.clone(), tags: vec!["canonical".into()], source: None });
        }
        if e.pos == "adj" {
            // A bare plural serves both genders.
            let mut extra = Vec::new();
            for f in &mut e.forms {
                if f.tags.len() == 1 && has(&f.tags, "plural") {
                    f.tags.push("masculine".into());
                    let mut g = f.clone();
                    g.tags = vec!["feminine".into(), "plural".into()];
                    extra.push(g);
                }
            }
            e.forms.extend(extra);
        }
    }

    fn samples(&self) -> Vec<&'static str> {
        vec!["cane", "abate", "cantare", "tìmere", "malu", "rosa", "flompare", "snorcu"]
    }

    fn readme(&self) -> String {
        r#"## Example

```rust
use sardinian::*;
println!("{}", noun_plural("abate")); // abates
println!("{}", adj("malu", Form::FemininePlural)); // malas
println!("{}", verb("cantare", Tense::Imperfect, Person::Third, Number::Singular)); // cantaiat
println!("{}", verb_participle_past("cantare")); // cantadu
```

Forms follow the Logudorese-based standard of Wiktionary's tables. The
future and conditional are periphrastic (apo a cantare, dia cantare) and are
left out. Accents are optional on input.
"#
        .into()
    }
}

pub fn language() -> Box<dyn Language> {
    Box::new(Sardinian)
}
