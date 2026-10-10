use morphe_train::spec::*;

pub struct MiddleFrench;

impl Language for MiddleFrench {
    fn spec(&self) -> Spec {
        let mut s = Spec::new("middle_french", "MiddleFrench", "frm");
        // Compound tenses are avoir/estre + participle.
        s.drop_tags.extend(tags("multiword-construction"));
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
                    ("Imperfect", "imperfect indicative"),
                    ("Preterite", "historic past"),
                    ("Future", "future"),
                    ("Conditional", "conditional"),
                    ("Subjunctive", "present subjunctive"),
                    ("ImperfectSubjunctive", "imperfect subjunctive"),
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
            pos(
                "adj",
                "adj",
                vec![group("", "!comparative !superlative", "Form"), group("Comparative", "comparative", "")],
            )
            .citation("Lemma Feminine"),
            pos(
                "verb",
                "verb",
                vec![
                    group("", "!imperative !participle !infinitive", "Tense Person Number"),
                    group("Imperative", "imperative", "Imperative"),
                    group("ParticiplePresent", "participle present", ""),
                    group("ParticiplePast", "participle past", ""),
                ],
            )
            .citation("Lemma PresentThirdSingular PreteriteThirdSingular ParticiplePast")
            .head_first(),
        ];
        s
    }

    fn retag(&self, pos: &PosSpec, _form: &str, tags: &mut Vec<String>) {
        // An adjective's bare "plural" (livres) serves both genders.
        if pos.name == "adj" && tags.len() == 1 && tags[..] == ["plural".to_string()] {
            tags.extend(["masculine".to_string(), "feminine".to_string()]);
        }
    }

    fn samples(&self) -> Vec<&'static str> {
        vec!["livre", "chevalier", "nu", "bon", "abhorrer", "avoir", "estre", "flunchier", "glorbe"]
    }

    fn readme(&self) -> String {
        r#"## Example

```rust
use middle_french::*;
println!("{}", noun_plural("livre")); // livres
assert_eq!(adj("nu", Form::Feminine), "nue");
assert_eq!(verb("abhorrer", Tense::Imperfect, Person::Third, Number::Singular), "abhorroit");
println!("{}", verb("abhorrer", Tense::Preterite, Person::Third, Number::Plural)); // abhorrerent
```

Compound tenses are left to the caller: avoir or estre plus
`verb_participle_past`.
"#
        .into()
    }
}

pub fn language() -> Box<dyn Language> {
    Box::new(MiddleFrench)
}
