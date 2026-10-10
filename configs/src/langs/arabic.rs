use morphe_train::spec::*;

pub struct Arabic;

impl Language for Arabic {
    fn spec(&self) -> Spec {
        let mut s = Spec::new("arabic", "Arabic", "ar");
        // Harakat, tanwin, shadda, sukun, dagger alif: written in the
        // dictionary, rarely typed.
        s.marks = ('\u{64B}'..='\u{65F}').chain(['\u{670}', '\u{640}']).collect();
        s.fold = vec![('أ', 'ا'), ('إ', 'ا'), ('آ', 'ا'), ('ٱ', 'ا'), ('ى', 'ي')];
        // "informal" is the pausal column of the tables, not a register.
        s.soft_tags.retain(|t| t != "informal");
        s.features = vec![
            feature("State", &[("Indefinite", "indefinite"), ("Definite", "definite"), ("Construct", "construct")]),
            feature(
                "Case",
                &[("Nominative", "nominative"), ("Accusative", "accusative"), ("Genitive", "genitive"), ("Pausal", "informal")],
            ),
            feature("Number", &[("Singular", "singular !dual"), ("Dual", "dual"), ("Plural", "plural")]),
            feature("Gender", &[("Masculine", "masculine"), ("Feminine", "feminine")]),
            feature("Voice", &[("Active", "active"), ("Passive", "passive")]),
            feature(
                "Mood",
                &[
                    ("Perfect", "past perfective"),
                    ("Indicative", "non-past indicative"),
                    ("Subjunctive", "subjunctive"),
                    ("Jussive", "jussive"),
                ],
            ),
            // The thirteen persons of the conjugation tables.
            feature(
                "Person",
                &[
                    ("FirstSingular", "first-person singular"),
                    ("FirstPlural", "first-person plural"),
                    ("SecondMasculineSingular", "second-person masculine singular"),
                    ("SecondFeminineSingular", "second-person feminine singular"),
                    ("SecondDual", "second-person dual"),
                    ("SecondMasculinePlural", "second-person masculine plural"),
                    ("SecondFemininePlural", "second-person feminine plural"),
                    ("ThirdMasculineSingular", "third-person masculine singular"),
                    ("ThirdFeminineSingular", "third-person feminine singular"),
                    ("ThirdMasculineDual", "third-person masculine dual"),
                    ("ThirdFeminineDual", "third-person feminine dual"),
                    ("ThirdMasculinePlural", "third-person masculine plural"),
                    ("ThirdFemininePlural", "third-person feminine plural"),
                ],
            ),
            feature(
                "VerbForm",
                &[
                    ("I", "form-i"),
                    ("II", "form-ii"),
                    ("III", "form-iii"),
                    ("IV", "form-iv"),
                    ("V", "form-v"),
                    ("VI", "form-vi"),
                    ("VII", "form-vii"),
                    ("VIII", "form-viii"),
                    ("IX", "form-ix"),
                    ("X", "form-x"),
                    ("XI", "form-xi"),
                    ("XII", "form-xii"),
                    ("XIII", "form-xiii"),
                    ("Iq", "form-iq"),
                    ("IIq", "form-iiq"),
                    ("IIIq", "form-iiiq"),
                    ("IVq", "form-ivq"),
                ],
            ),
        ];
        let persons2 = "Person[SecondMasculineSingular,SecondFeminineSingular,SecondDual,SecondMasculinePlural,SecondFemininePlural]";
        s.pos = vec![
            pos("noun", "noun", vec![group("", "", "State Case Number")])
                .inherent("Gender")
                .citation("IndefinitePausalSingular IndefinitePausalPlural")
                .head_last(),
            pos("adj", "adj", vec![group("", "", "State[Indefinite,Definite] Case Gender Number")])
                .citation("IndefinitePausalMasculineSingular IndefinitePausalFeminineSingular IndefinitePausalMasculinePlural")
                .head_last(),
            pos(
                "verb",
                "verb",
                vec![
                    group("", "!imperative !participle", "Voice Mood Person"),
                    group("Imperative", "imperative", persons2),
                    group("VerbalNoun", "noun-from-verb", ""),
                    group("Participle", "participle", "Voice").declines("adj", ""),
                ],
            )
            .inherent("VerbForm")
            .citation("ActivePerfectThirdMasculineSingular ActiveIndicativeThirdMasculineSingular")
            .head_first(),
            pos("pron", "pron det", vec![group("", "!indefinite", "Case Gender? Number")]),
            pos("num", "num", vec![group("", "", "State Case Gender? Number")]),
        ];
        s
    }

    fn readme(&self) -> String {
        r#"## Example

```rust
use arabic::*;
// Unvocalized input works; output is fully vocalized.
println!("{}", noun("كتاب", State::Definite, Case::Nominative, Number::Plural)); // الْكُتُبُ
println!("{:?}", noun_gender("مدينة"));
println!("{}", adj("كبير", State::Indefinite, Case::Accusative, Gender::Feminine, Number::Singular));
println!("{}", verb("كتب", Voice::Active, Mood::Indicative, Person::ThirdMasculinePlural));
println!("{}", verb_imperative("كتب", Person::SecondFeminineSingular));
println!("{:?}", verb_verb_form("استقبل"));
// Without harakat:
let plain = Arabic::default().marks(false);
println!("{}", plain.verb("كتب", Voice::Active, Mood::Perfect, Person::FirstSingular)); // كتبت
// Citations pin principal parts: past and non-past.
println!("{}", verb("كتب, يكتب", Voice::Active, Mood::Jussive, Person::SecondMasculinePlural));
```

`Case::Pausal` is the form without case endings, as words are said at the
end of a phrase and in everyday speech. Input matching ignores harakat and
tatweel and treats the alif forms (أ إ آ ٱ) as ا and ى as ي. An unvocalized
word is often several dictionary words (كتب is kataba "write" and kattaba
"make write"): the `_scored` functions give them all; a citation picks one.
"#
        .into()
    }
}

pub fn language() -> Box<dyn Language> {
    Box::new(Arabic)
}
