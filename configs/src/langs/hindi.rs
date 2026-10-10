use morphe_train::kaikki::Entry;
use morphe_train::spec::*;

pub struct Hindi;

const AGREE: &str = "Gender Number";

impl Language for Hindi {
    fn spec(&self) -> Spec {
        let mut s = Spec::new("hindi", "Hindi", "hi");
        // Chandrabindu and anusvara are written interchangeably.
        s.fold = vec![('ँ', 'ं')];
        s.features = vec![
            feature("Case", &[("Direct", "direct"), ("Oblique", "oblique"), ("Vocative", "vocative")]),
            feature(
                "PronCase",
                &[
                    ("Direct", "direct"),
                    ("Indirect", "indirect"),
                    ("Ergative", "ergative"),
                    ("Dative", "dative"),
                    ("Genitive", "genitive"),
                    ("Inessive", "inessive"),
                    ("Adessive", "adessive"),
                    ("Terminative", "terminative"),
                ],
            ),
            feature("Number", &[("Singular", "singular"), ("Plural", "plural")]),
            feature("Gender", &[("Masculine", "masculine !feminine"), ("Feminine", "feminine !masculine")]),
            // Participle agreement: the masculine singular has a direct and an
            // oblique form; the rest have one.
            feature(
                "Agreement",
                &[
                    ("MasculineSingular", "masculine singular !oblique"),
                    ("MasculineOblique", "masculine singular oblique"),
                    ("MasculinePlural", "masculine plural"),
                    ("FeminineSingular", "feminine singular"),
                    ("FemininePlural", "feminine plural"),
                ],
            ),
            // The six persons of the finite forms; तू, तुम and आप are the
            // intimate, familiar and formal second persons.
            feature(
                "Person",
                &[
                    ("FirstSingular", "first-person singular"),
                    ("SecondIntimate", "second-person singular"),
                    ("ThirdSingular", "third-person singular"),
                    ("FirstPlural", "first-person plural"),
                    ("SecondFamiliar", "second-person plural !formal"),
                    ("SecondFormal", "second-person plural formal"),
                    ("ThirdPlural", "third-person plural"),
                ],
            ),
            feature(
                "Address",
                &[
                    ("Intimate", "second-person singular"),
                    ("Familiar", "second-person plural !formal"),
                    ("Formal", "second-person plural formal"),
                ],
            ),
            feature(
                "Participle",
                &[("Habitual", "habitual"), ("Perfective", "perfective"), ("Agentive", "agentive prospective")],
            ),
        ];
        s.pos = vec![
            pos("noun", "noun", vec![group("", "", "Case Number")])
                .inherent("Gender")
                .citation("DirectSingular DirectPlural")
                .head_last(),
            pos("adj", "adj", vec![group("", "", "Case Gender? Number")])
                .citation("DirectMasculineSingular DirectFeminineSingular DirectMasculinePlural")
                .head_last(),
            pos(
                "verb",
                "verb",
                vec![
                    group("Stem", "stem", ""),
                    group("Infinitive", "infinitive !oblique !feminine !plural", ""),
                    group("InfinitiveOblique", "infinitive oblique", ""),
                    group("Conjunctive", "conjunctive", ""),
                    group("Participle", "!infinitive !indicative !counterfactual", "Participle Agreement"),
                    group("Future", "future indicative", "Person Gender"),
                    group("Subjunctive", "future subjunctive", "Person"),
                    group("Imperative", "imperative present", "Address"),
                    group("FutureImperative", "imperative future", "Address"),
                ],
            )
            .head_last(),
            pos(
                "pron",
                "pron",
                vec![group("", "!masculine !feminine", "PronCase Number?"), group("Possessive", "", &format!("Case {AGREE}"))],
            ),
            pos("num", "num", vec![group("", "", "Case Number")]),
        ];
        s
    }

    /// Verb tables list periphrastic forms (करता हूँ, किया होगा) built from
    /// auxiliaries; only the verb's own one-word forms are read. The
    /// agentive participle's oblique masculine singular (करनेवाले) comes
    /// without its `oblique` tag.
    fn prepare(&self, e: &mut Entry) {
        if e.pos != "verb" {
            return;
        }
        let words = e.word.split_whitespace().count();
        e.forms.retain(|f| f.source.is_none() || f.form.split_whitespace().count() <= words);
        for f in &mut e.forms {
            let has = |t: &str| f.tags.iter().any(|x| x == t);
            if has("agentive") && has("masculine") && has("singular") && !has("direct") {
                f.tags.push("oblique".into());
            }
        }
    }

    fn samples(&self) -> Vec<&'static str> {
        vec!["करना", "जाना", "लड़का", "किताब", "नीला", "मैं", "झपटाना", "गुलबा"]
    }

    fn readme(&self) -> String {
        r#"## Example

```rust
use hindi::*;
assert_eq!(noun("लड़का", Case::Oblique, Number::Plural), "लड़कों");
println!("{}", noun("किताब", Case::Direct, Number::Plural)); // किताबें
println!("{:?}", noun_gender("किताब")); // Feminine
println!("{}", adj("नीला", Case::Direct, Gender::Feminine, Number::Singular)); // नीली
println!("{}", verb_future("करना", Person::FirstSingular, Gender::Masculine)); // करूँगा
println!("{}", verb_subjunctive("जाना", Person::ThirdPlural)); // जाएँ
println!("{}", verb_participle("करना", Participle::Perfective, Agreement::FeminineSingular)); // की
println!("{}", verb_imperative("करना", Address::Formal)); // कीजिये
println!("{}", pron("मैं", PronCase::Ergative, Number::Singular)); // मैंने
```

Verbs have their one-word forms: the stem, the infinitive and its oblique,
the conjunctive participle, the habitual, perfective and agentive
participles in their five agreement forms, the future, the subjunctive and
the imperatives. Tenses built with auxiliaries (करता हूँ, कर रहा था) are the
participle plus a form of होना and are left out. Input matching treats
chandrabindu as anusvara.
"#
        .into()
    }
}

pub fn language() -> Box<dyn Language> {
    Box::new(Hindi)
}
