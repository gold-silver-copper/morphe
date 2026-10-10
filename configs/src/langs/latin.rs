use morphe_train::spec::*;

pub struct Latin;

const CASES: &str = "Case[Nominative,Genitive,Dative,Accusative,Ablative,Vocative]";

impl Language for Latin {
    fn spec(&self) -> Spec {
        let mut s = Spec::new("latin", "Latin", "la");
        // Macrons and breves are optional in input; j is written as i.
        s.unmark = unmark("āa ēe īi ōo ūu ȳy ăa ĕe ĭi ŏo ŭu");
        s.marks = vec!['\u{304}', '\u{306}', '\u{361}'];
        s.fold = vec![('j', 'i')];
        // Old Latin sigmatic futures/aorists and "potential" perfect infinitives.
        s.drop_tags.extend(tags("sigmatic aorist potential"));
        s.soft_tags.extend(tags("poetic"));
        s.features = vec![
            feature(
                "Case",
                &[
                    ("Nominative", "nominative"),
                    ("Genitive", "genitive"),
                    ("Dative", "dative"),
                    ("Accusative", "accusative"),
                    ("Ablative", "ablative"),
                    ("Vocative", "vocative"),
                    ("Locative", "locative"),
                ],
            ),
            feature("Number", &[("Singular", "singular"), ("Plural", "plural")]),
            feature("Gender", &[("Masculine", "masculine"), ("Feminine", "feminine"), ("Neuter", "neuter")]),
            feature(
                "Tense",
                &[
                    ("Present", "present"),
                    ("Imperfect", "imperfect"),
                    ("Future", "future !perfect"),
                    ("Perfect", "perfect !future"),
                    ("Pluperfect", "pluperfect"),
                    ("FuturePerfect", "future perfect"),
                ],
            ),
            feature("Person", &[("First", "first-person"), ("Second", "second-person"), ("Third", "third-person")]),
            feature("Voice", &[("Active", "active"), ("Passive", "passive")]),
        ];
        let finite = "!imperative !infinitive !participle";
        s.pos = vec![
            pos("noun", "noun", vec![group("", "", "Case Number")])
                .inherent("Gender")
                .citation("NominativeSingular GenitiveSingular")
                .head_last(),
            pos(
                "adj",
                "adj",
                vec![
                    group("", "!comparative !superlative !adverb", &format!("{CASES} Gender Number")),
                    group("Comparative", "comparative", ""),
                    group("Superlative", "superlative", ""),
                    group("Adverb", "adverb", ""),
                ],
            )
            .citation("NominativeMasculineSingular NominativeFeminineSingular|GenitiveMasculineSingular NominativeNeuterSingular"),
            pos(
                "verb",
                "verb",
                vec![
                    group("Indicative", &format!("indicative active {finite}"), "Tense Person Number"),
                    group("Subjunctive", &format!("subjunctive active {finite}"), "Tense[Present,Imperfect,Perfect,Pluperfect] Person Number"),
                    group("IndicativePassive", &format!("indicative passive {finite}"), "Tense[Present,Imperfect,Future] Person Number"),
                    group("SubjunctivePassive", &format!("subjunctive passive {finite}"), "Tense[Present,Imperfect] Person Number"),
                    group("Imperative", "imperative active present second-person", "Number"),
                    group("FutureImperative", "imperative active future", "Person[Second,Third] Number"),
                    group("ImperativePassive", "imperative passive present second-person", "Number"),
                    group("FutureImperativePassive", "imperative passive future", "Person[Second,Third] Number"),
                    group("Infinitive", "infinitive", "Voice Tense[Present,Perfect,Future]"),
                    group("Participle", "participle active", "Tense[Present,Future]"),
                    group("ParticiplePassive", "participle passive", "Tense[Perfect,Future]"),
                    group("Gerund", "gerund", "Case[Genitive,Dative,Accusative,Ablative]"),
                    group("Supine", "supine", "Case[Accusative,Ablative]"),
                ],
            )
            .citation("IndicativePresentFirstSingular InfinitiveActivePresent IndicativePerfectFirstSingular SupineAccusative|ParticiplePassivePerfect")
            .head_first(),
            pos("pron", "pron", vec![group("", "", &format!("{CASES} Gender? Number"))]),
            pos("det", "det", vec![group("", "", "Case Gender? Number")]),
            pos("num", "num", vec![group("", "", &format!("{CASES} Gender? Number"))]),
        ];
        s
    }

    /// Periphrastic perfect passives come as "amātus + present active
    /// indicative of sum": a description, not a form.
    fn adjust(&self, _pos: &PosSpec, _lemma: &str, forms: &mut Vec<(usize, String, bool)>) {
        forms.retain(|f| !f.1.contains('+'));
    }

    fn readme(&self) -> String {
        r#"## Example

```rust
use latin::*;
assert_eq!(noun("rosa", Case::Genitive, Number::Plural), "rosārum");
println!("{}", noun("rex", Case::Ablative, Number::Singular)); // rēge
println!("{:?}", noun_gender("nox"));
println!("{}", verb_indicative("amo", Tense::Perfect, Person::Third, Number::Plural)); // amāvērunt
println!("{}", verb_subjunctive_passive("capio", Tense::Imperfect, Person::First, Number::Singular));
println!("{}", adj("bonus", Case::Accusative, Gender::Feminine, Number::Singular)); // bonam
// Citations pin principal parts: amō, amāre, amāvī, amātum.
println!("{}", verb_indicative("fero, ferre, tuli, latum", Tense::Perfect, Person::First, Number::Plural));
// Without macrons in the output:
println!("{}", Latin::default().marks(false).noun("puella", Case::Dative, Number::Plural));
```

Input may omit macrons and breves, and may write j for i. Output carries
macrons as Wiktionary gives them (`marks(false)` removes them). Perfect
passive and other periphrastic tenses (amātus sum) are left to the caller:
combine `verb_participle_passive` with a form of *sum*.
"#
        .into()
    }
}

pub fn language() -> Box<dyn Language> {
    Box::new(Latin)
}
