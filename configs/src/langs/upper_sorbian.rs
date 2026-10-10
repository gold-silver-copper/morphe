use morphe_train::kaikki::{Entry, Form};
use morphe_train::spec::*;

pub struct UpperSorbian;

fn has(tags: &[String], t: &str) -> bool {
    tags.iter().any(|x| x == t)
}

const ADJ_CASES: &str = "Case[Nominative,Genitive,Dative,Accusative,Instrumental,Locative]";

impl Language for UpperSorbian {
    fn spec(&self) -> Spec {
        let mut s = Spec::new("upper_sorbian", "UpperSorbian", "hsb");
        s.unmark = unmark("łl ěe šs ćc čc źz žz óo řr ńn");
        s.soft_tags.extend(tags("poetic"));
        s.features = vec![
            feature(
                "Case",
                &[
                    ("Nominative", "nominative"),
                    ("Genitive", "genitive"),
                    ("Dative", "dative"),
                    ("Accusative", "accusative"),
                    ("Instrumental", "instrumental"),
                    ("Locative", "locative"),
                    ("Vocative", "vocative !nominative"),
                ],
            ),
            feature("Number", &[("Singular", "singular"), ("Dual", "dual"), ("Plural", "plural")]),
            feature("Gender", &[("Masculine", "masculine"), ("Feminine", "feminine"), ("Neuter", "neuter")]),
            feature("Animacy", &[("Animate", "animate"), ("Inanimate", "inanimate")]),
            feature(
                "SingularGender",
                &[
                    ("MasculineAnimate", "animate"),
                    ("MasculineInanimate", "inanimate"),
                    ("Neuter", "neuter"),
                    ("Feminine", "feminine"),
                ],
            ),
            feature("Virility", &[("Virile", "virile"), ("Nonvirile", "nonvirile")]),
            feature("Aspect", &[("Imperfective", "imperfective"), ("Perfective", "perfective")]),
            feature(
                "Tense",
                &[("Present", "present"), ("Past", "past"), ("Future", "future"), ("Conditional", "conditional")],
            ),
            feature(
                "PersonNumber",
                &[
                    ("FirstSingular", "first-person singular"),
                    ("SecondSingular", "second-person singular"),
                    ("ThirdSingular", "third-person singular"),
                    ("FirstDual", "first-person dual"),
                    ("SecondDual", "second-person dual !nonvirile"),
                    ("SecondDualNonvirile", "second-person dual nonvirile"),
                    ("ThirdDual", "third-person dual !nonvirile"),
                    ("ThirdDualNonvirile", "third-person dual nonvirile"),
                    ("FirstPlural", "first-person plural"),
                    ("SecondPlural", "second-person plural"),
                    ("ThirdPlural", "third-person plural"),
                ],
            ),
        ];
        s.pos = vec![
            pos("noun", "noun", vec![group("", "", "Case Number"), group("Diminutive", "diminutive", "")])
                .inherent("Gender Animacy")
                .citation("NominativeSingular GenitiveSingular")
                .head_last(),
            pos(
                "adj",
                "adj",
                vec![
                    group("", "singular", &format!("{ADJ_CASES} SingularGender")),
                    group("Dual", "dual", &format!("{ADJ_CASES} Virility?")),
                    group("Plural", "plural", &format!("{ADJ_CASES} Virility?")),
                ],
            )
            .citation("NominativeMasculineInanimate NominativeFeminine NominativeNeuter"),
            pos(
                "verb",
                "verb",
                vec![
                    group("Infinitive", "infinitive", ""),
                    group("", "!imperative !passive", "Tense PersonNumber"),
                    group("Imperative", "imperative", "PersonNumber"),
                    group("Transgressive", "transgressive", ""),
                    group("ActiveParticiple", "active participle", ""),
                    group("PassiveParticiple", "passive participle", ""),
                    // pił: the l-participle of the compound past.
                    group("LParticiple", "l-participle", ""),
                    group("VerbalNoun", "noun-from-verb", ""),
                ],
            )
            .inherent("Aspect")
            .citation("Infinitive PresentFirstSingular PresentThirdSingular")
            .head_first(),
        ];
        s
    }

    fn prepare(&self, e: &mut Entry) {
        if !e.forms.iter().any(|f| has(&f.tags, "canonical")) {
            e.forms.push(Form { form: e.word.clone(), tags: vec!["canonical".into()], source: None });
        }
        for f in &mut e.forms {
            if f.source.is_none() || !has(&f.tags, "error-unrecognized-form") {
                continue;
            }
            f.tags.retain(|t| t != "error-unrecognized-form");
            match e.pos.as_str() {
                // The adjective's masculine animate column has no header.
                "adj" => f.tags.push("animate".into()),
                "verb" if f.tags.is_empty() => f.tags.push("l-participle".into()),
                // Unlabeled dual rows (the conditional's, the future's) are
                // the nonvirile ones.
                "verb" if has(&f.tags, "dual") && !has(&f.tags, "first-person") => f.tags.push("nonvirile".into()),
                _ => {}
            }
        }
    }

    fn samples(&self) -> Vec<&'static str> {
        vec!["woman", "dub", "žona", "pić", "być", "pisać", "daty", "dobry", "flompać", "snorak"]
    }

    fn readme(&self) -> String {
        r#"## Example

```rust
use upper_sorbian::*;
println!("{}", noun("woman", Case::Instrumental, Number::Dual)); // womanomaj
println!("{:?}", noun_gender("woman"));
println!("{}", verb("pić", Tense::Past, PersonNumber::ThirdPlural)); // pijachu
println!("{}", verb("pić", Tense::Present, PersonNumber::SecondDualNonvirile)); // pijetej
println!("{}", adj("daty", Case::Genitive, SingularGender::Feminine)); // dateje
```

Upper Sorbian keeps the dual; its second and third persons have virile and
nonvirile forms. Diacritics are optional on input.
"#
        .into()
    }
}

pub fn language() -> Box<dyn Language> {
    Box::new(UpperSorbian)
}
