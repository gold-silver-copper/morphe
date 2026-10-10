use morphe_train::kaikki::{Entry, Form};
use morphe_train::spec::*;

pub struct Ladin;

fn has(tags: &[String], t: &str) -> bool {
    tags.iter().any(|x| x == t)
}

impl Language for Ladin {
    fn spec(&self) -> Spec {
        let mut s = Spec::new("ladin", "Ladin", "lld");
        s.unmark = unmark("ée àa ëe ìi ùu òo èe óo üu öo êe áa îi íi úu");
        s.inherent_form_tags = tags("auxiliary");
        // Forms of the other valleys' idioms on the headword line.
        s.soft_tags.extend(tags("Badiot Fascian Gherdëina Anpezan Fodom"));
        s.features = vec![
            feature("Gender", &[("Masculine", "masculine"), ("Feminine", "feminine")]),
            feature("Auxiliary", &[("Avei", "avei"), ("Ester", "ester")]),
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
                    ("Future", "indicative future"),
                    ("Subjunctive", "subjunctive present"),
                    ("ImperfectSubjunctive", "subjunctive imperfect"),
                ],
            ),
            feature("Person", &[("First", "first-person"), ("Second", "second-person"), ("Third", "third-person")]),
            feature("Number", &[("Singular", "singular"), ("Plural", "plural")]),
            feature(
                "Imperative",
                &[
                    ("SecondSingular", "second-person singular"),
                    ("FirstPlural", "first-person plural"),
                    ("SecondPlural", "second-person plural"),
                ],
            ),
        ];
        s.pos = vec![
            pos("noun", "noun", vec![group("Plural", "plural", "")])
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
                    group("Imperative", "imperative", "Imperative"),
                ],
            )
            .inherent("Auxiliary")
            .citation("Infinitive PresentFirstSingular ParticiplePast")
            .head_first(),
        ];
        s
    }

    fn prepare(&self, e: &mut Entry) {
        if !e.forms.iter().any(|f| has(&f.tags, "canonical")) {
            e.forms.push(Form { form: e.word.clone(), tags: vec!["canonical".into()], source: None });
        }
        // The imperfect subjunctive's second person singular is labeled
        // indicative; it comes after the subjunctive has begun.
        let mut subj = false;
        for f in &mut e.forms {
            if has(&f.tags, "table-tags") {
                subj = false;
            }
            if has(&f.tags, "subjunctive") {
                subj = true;
            } else if subj && has(&f.tags, "indicative") {
                f.tags.retain(|t| t != "indicative");
                f.tags.push("subjunctive".into());
            }
            if has(&f.tags, "auxiliary") && matches!(f.form.as_str(), "avai" | "avea" | "avëi") {
                f.form = "avei".into();
            }
        }
    }

    fn samples(&self) -> Vec<&'static str> {
        vec!["dat", "tera", "meter", "dormir", "general", "passé", "flomper", "snorich"]
    }

    fn readme(&self) -> String {
        r#"## Example

```rust
use ladin::*;
println!("{}", noun_plural("dat")); // dac
println!("{}", adj("general", Form::MasculinePlural)); // generai
println!("{}", verb("meter", Tense::Imperfect, Person::Second, Number::Singular)); // metoves
println!("{}", verb_participle_past("meter")); // metù
println!("{:?}", verb_auxiliary("meter")); // Avei
```

Forms follow Wiktionary's tables, which mostly give the Ladin Dolomitan
standard. Forms of the valley idioms are kept only as alternatives. Accents
are optional on input.
"#
        .into()
    }
}

pub fn language() -> Box<dyn Language> {
    Box::new(Ladin)
}
