use morphe_train::kaikki::{Entry, Form};
use morphe_train::spec::*;

pub struct NorthFrisian;

fn has(tags: &[String], t: &str) -> bool {
    tags.iter().any(|x| x == t)
}

const DIALECTS: &str = "Föhr-Amrum Föhr Amrum Mooring Sylt Heligoland Halligen Goesharde Wiedingharde Karrharde";

impl Language for NorthFrisian {
    fn spec(&self) -> Spec {
        let mut s = Spec::new("north_frisian", "NorthFrisian", "frr");
        s.unmark = unmark("äa öo üu åa");
        s.fold = vec![('’', '\'')];
        // Each table is one dialect; the first is the headword's own.
        s.first_table_only = true;
        // Compound tenses are auxiliary + participle or infinitive;
        // infinitive III is "än" + infinitive.
        s.drop_tags.extend(tags("perfect pluperfect future infinitive-iii"));
        // Headword-line forms of other dialects.
        s.soft_tags.extend(tags(DIALECTS));
        s.features = vec![
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
            feature("Number", &[("Singular", "singular"), ("Plural", "plural")]),
            feature("Degree", &[("Positive", "positive"), ("Comparative", "comparative"), ("Superlative", "superlative")]),
            feature(
                "Form",
                &[
                    ("Predicative", "!attributive !independent !partitive"),
                    ("AttributiveMasculine", "attributive indefinite masculine"),
                    ("AttributiveNeuter", "attributive unmarked"),
                    ("AttributiveDefinite", "attributive definite"),
                    ("AttributivePlural", "attributive plural"),
                    ("IndependentIndefinite", "independent indefinite"),
                    ("IndependentNeuter", "independent unmarked"),
                    ("IndependentDefinite", "independent definite"),
                    ("IndependentPlural", "independent plural"),
                    ("Partitive", "partitive !plural"),
                ],
            ),
        ];
        s.pos = vec![
            pos("noun", "noun", vec![group("Plural", "plural", "")])
                .inherent("Gender")
                .citation("Lemma Plural")
                .head_last(),
            pos("adj", "adj", vec![group("", "", "Degree Form")])
                .citation("Lemma ComparativePredicative SuperlativePredicative"),
            pos(
                "verb",
                "verb",
                vec![
                    group("Infinitive", "infinitive infinitive-i", ""),
                    // springen (tu springen)
                    group("LongInfinitive", "infinitive infinitive-ii", ""),
                    group("ParticiplePast", "participle past", ""),
                    group("Imperative", "imperative", "Number?"),
                    group("Present", "present !imperative", "PersonNumber"),
                    group("Past", "past !participle", "PersonNumber"),
                ],
            )
            .citation("Infinitive PastThirdSingular ParticiplePast")
            .head_first(),
        ];
        s
    }

    fn prepare(&self, e: &mut Entry) {
        if !e.forms.iter().any(|f| has(&f.tags, "canonical")) {
            e.forms.push(Form { form: e.word.clone(), tags: vec!["canonical".into()], source: None });
        }
        for f in &mut e.forms {
            if f.source.is_none() {
                f.form = f.form.trim_end_matches('|').to_string();
                continue;
            }
            // "tu springen" repeats the long infinitive.
            if has(&f.tags, "infinitive-ii") && f.form.contains(' ') {
                f.form = f.form.rsplit(' ').next().unwrap_or_default().to_string();
            }
            if has(&f.tags, "error-unrecognized-form") {
                f.tags.retain(|t| t != "error-unrecognized-form");
                if e.pos == "verb" {
                    // Sylt tables leave the plural row unlabeled.
                    f.tags.push("plural".into());
                } else {
                    f.tags.push("unmarked".into());
                }
            }
        }
    }

    fn samples(&self) -> Vec<&'static str> {
        vec!["sun", "dai", "spring", "gripe", "green", "leet", "flomp", "snoorig"]
    }

    fn readme(&self) -> String {
        r#"## Example

```rust
use north_frisian::*;
println!("{}", noun_plural("sun")); // sunen
println!("{}", verb_present("spring", PersonNumber::ThirdSingular)); // sprangt
println!("{}", verb_past("spring", PersonNumber::ThirdSingular)); // sproong
println!("{}", adj("green", Degree::Comparative, Form::Predicative)); // greener
```

North Frisian is a group of dialects. Each word follows the first table
Wiktionary gives, which is the dialect of its headword (mostly Föhr-Amrum
or Mooring). Compound tenses are left out. Umlauts are optional on input.
"#
        .into()
    }
}

pub fn language() -> Box<dyn Language> {
    Box::new(NorthFrisian)
}
