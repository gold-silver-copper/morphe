use morphe_train::kaikki::{Entry, Form};
use morphe_train::spec::*;

pub struct Gujarati;

impl Language for Gujarati {
    fn spec(&self) -> Spec {
        let mut s = Spec::new("gujarati", "Gujarati", "gu");
        // Anusvara and chandrabindu are written interchangeably.
        s.fold = vec![('ઁ', 'ં')];
        s.features = vec![
            feature(
                "Case",
                &[
                    ("Nominative", "nominative"),
                    ("Oblique", "oblique"),
                    ("Vocative", "vocative"),
                    ("Instrumental", "instrumental"),
                    ("Locative", "locative"),
                ],
            ),
            feature("Number", &[("Singular", "singular"), ("Plural", "plural")]),
            feature("Gender", &[("Masculine", "masculine"), ("Feminine", "feminine"), ("Neuter", "neuter")]),
            feature(
                "Form",
                &[
                    ("Nominative", "nominative singular"),
                    ("Oblique", "oblique singular"),
                    ("Plural", "plural"),
                    ("Locative", "locative"),
                ],
            ),
            feature(
                "Person",
                &[
                    ("FirstSingular", "first-person singular"),
                    ("SecondSingular", "second-person singular"),
                    ("Third", "third-person"),
                    ("FirstPlural", "first-person plural"),
                    ("SecondPlural", "second-person plural"),
                ],
            ),
            feature("Number2", &[("Singular", "singular"), ("Plural", "plural")]),
        ];
        let agreeing = || vec![group("", "", "Gender Form")];
        s.pos = vec![
            pos("noun", "noun", vec![group("", "", "Case Number")])
                .inherent("Gender")
                .citation("NominativeSingular NominativePlural")
                .head_last(),
            pos("adj", "adj", agreeing()).citation("MasculineNominative FeminineNominative NeuterNominative").head_last(),
            pos(
                "verb",
                "verb",
                vec![
                    group("VerbalNoun", "noun-from-verb", ""),
                    group("Conjunctive", "conjunctive", ""),
                    group("Consecutive", "consecutive", ""),
                    group("Passive", "passive potential", ""),
                    group("Counterfactual", "counterfactual", ""),
                    group("Past", "past", ""),
                    group("Present", "x-present", "Person"),
                    group("Future", "future !x-imperative", "Person"),
                    group("Imperative", "x-imperative !future", "Person[SecondSingular,FirstPlural,SecondPlural]"),
                    group("FutureImperative", "x-imperative future", "Number2"),
                ],
            )
            .head_last(),
            pos("pron", "pron", agreeing()),
            pos("det", "det", agreeing()),
        ];
        s
    }

    /// Nouns get their gender from the declension template's name. Adjective
    /// tables leave the oblique singular untagged. Verb tables are read for
    /// their one-word forms: the second table has the present (untagged)
    /// and future per person, with the third person untagged; the fourth is
    /// the imperative.
    fn prepare(&self, e: &mut Entry) {
        let spaced = e.word.contains(' ');
        e.forms.retain(|f| f.source.is_none() || spaced || !f.form.trim().contains(' '));
        let template = e.forms.iter().find(|f| f.tags.iter().any(|t| t == "inflection-template")).map(|f| f.form.clone());
        if e.pos == "noun" {
            let gender = match template.as_deref() {
                Some("gu-ndecl-m") => "masculine",
                Some("gu-ndecl-f") => "feminine",
                Some(t) if t.starts_with("gu-ndecl-um") || t == "gu-ndecl-n" => "neuter",
                _ => "",
            };
            if !gender.is_empty() {
                e.forms.push(Form { form: e.word.clone(), tags: tags(&format!("canonical {gender}")), source: None });
            }
            return;
        }
        let mut table = 0;
        for f in e.forms.iter_mut().filter(|f| f.source.is_some()) {
            let has = |t: &str| f.tags.iter().any(|x| x == t);
            if has("table-tags") {
                table += 1;
                continue;
            }
            let error = has("error-unrecognized-form");
            f.tags.retain(|t| t != "error-unrecognized-form");
            if e.pos != "verb" {
                if error && f.tags.iter().any(|t| t == "singular") {
                    f.tags.push("oblique".into());
                }
                continue;
            }
            let has = |t: &str| f.tags.iter().any(|x| x == t);
            let (progressive, future, past) = (has("progressive"), has("future"), has("past"));
            match table {
                2 => {
                    if progressive {
                        f.tags = tags("table-tags");
                        continue;
                    }
                    if !f.tags.iter().any(|t| t.ends_with("-person")) {
                        f.tags.push("third-person".into());
                    }
                    if !future {
                        f.tags.push("x-present".into());
                    }
                }
                3 => {
                    if !past || progressive {
                        f.tags = tags("table-tags");
                    }
                }
                4 => f.tags.push("x-imperative".into()),
                _ => {}
            }
        }
    }

    fn samples(&self) -> Vec<&'static str> {
        vec!["ઘોડો", "પીછું", "છોકરી", "સારું", "કરવું", "જવું", "ઝપટવું", "ગુલબો"]
    }

    fn readme(&self) -> String {
        r#"## Example

```rust
use gujarati::*;
println!("{}", noun("ઘોડો", Case::Nominative, Number::Plural)); // ઘોડા
println!("{}", noun("ઘોડો", Case::Instrumental, Number::Singular)); // ઘોડે
println!("{:?}", noun_gender("પીછું")); // Neuter
println!("{}", adj("સારું", Gender::Masculine, Form::Nominative)); // સારો
println!("{}", verb_future("કરવું", Person::FirstSingular)); // કરીશ
println!("{}", verb_present("કરવું", Person::SecondPlural)); // કરો
println!("{}", verb_past("કરવું")); // કર્યું
```

Nouns have five cases in both numbers and their gender. Adjectives agree
in gender, with a nominative, oblique, plural and locative form. Verbs have
the verbal noun, the conjunctive (કરી, કરીને), the passive (કરાય), the
counterfactual, the past (perfective) participle, the present subjunctive
and future for five persons, and the imperatives. Tenses built with
auxiliaries (કરું છું, કરતું હતું) are left out.
"#
        .into()
    }
}

pub fn language() -> Box<dyn Language> {
    Box::new(Gujarati)
}
