use morphe_train::kaikki::{Entry, Form};
use morphe_train::spec::*;

pub struct Venetan;

fn has(tags: &[String], t: &str) -> bool {
    tags.iter().any(|x| x == t)
}

impl Language for Venetan {
    fn spec(&self) -> Spec {
        let mut s = Spec::new("venetan", "Venetan", "vec");
        s.unmark = unmark("ée ìi àa èe òo óo ùu íi áa úu");
        // ł is the "evanescent l" of the standard spelling; many write l.
        s.fold = vec![('ł', 'l')];
        s.inherent_form_tags = tags("auxiliary");
        // Subject clitics written into the table cells: "el sta", "te sti".
        s.strip_words = tags("el ła la i łe le te");
        s.soft_tags.extend(tags("Central Northern"));
        s.features = vec![
            feature("Gender", &[("Masculine", "masculine"), ("Feminine", "feminine")]),
            feature("Auxiliary", &[("Aver", "aver"), ("Eser", "èser")]),
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
                    ("Present", "present !subjunctive"),
                    ("Imperfect", "imperfect !subjunctive"),
                    ("Future", "future"),
                    ("Conditional", "conditional"),
                    ("Subjunctive", "present subjunctive"),
                    ("ImperfectSubjunctive", "imperfect subjunctive"),
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
                    group("", "!imperative !participle", "Tense Person Number"),
                    group("Imperative", "imperative", "Person Number"),
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
        // The singular rows lose their person: in each tense the first
        // form is the first person, the rest ("sti", "te sti") the second;
        // the imperative's is the second.
        let mut seen: Vec<Vec<String>> = Vec::new();
        for f in &mut e.forms {
            if f.source.is_none() || !has(&f.tags, "error-unrecognized-form") {
                continue;
            }
            f.tags.retain(|t| t != "error-unrecognized-form");
            let key = f.tags.clone();
            let person = if has(&f.tags, "imperative") || seen.contains(&key) { "second-person" } else { "first-person" };
            if !seen.contains(&key) {
                seen.push(key);
            }
            f.tags.push(person.into());
        }
        if e.pos == "adj" {
            // "feminine masculine plural": one plural for both genders.
            let mut extra = Vec::new();
            for f in &e.forms {
                if f.tags.len() == 1 && has(&f.tags, "plural") {
                    for g in ["masculine", "feminine"] {
                        let mut c = f.clone();
                        c.tags.push(g.into());
                        extra.push(c);
                    }
                }
            }
            e.forms.extend(extra);
        }
    }

    fn samples(&self) -> Vec<&'static str> {
        vec!["can", "pie", "star", "parlar", "moro", "rente", "flumpar", "snorco"]
    }

    fn readme(&self) -> String {
        r#"## Example

```rust
use venetan::*;
println!("{}", noun_plural("can")); // cani
println!("{}", adj("moro", Form::Feminine)); // mora
println!("{}", verb("star", Tense::Imperfect, Person::Third, Number::Plural)); // stava
println!("{:?}", verb_auxiliary("star")); // Eser
```

Finite forms are given without their subject clitic ("el sta" → "sta").
Accents are optional on input, and l matches the standard spelling's ł.
"#
        .into()
    }
}

pub fn language() -> Box<dyn Language> {
    Box::new(Venetan)
}
