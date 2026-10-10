use morphe_train::kaikki::Entry;
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
                .missing_is_absent()
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
            .citation("NominativeMasculineSingular NominativeFeminineSingular|GenitiveMasculineSingular NominativeNeuterSingular")
            .missing_is_absent(),
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
                    group("Participle", "participle active", "Tense[Present,Future]").declines("adj", ""),
                    group("ParticiplePassive", "participle passive", "Tense[Perfect,Future]").declines("adj", ""),
                    group("Gerund", "gerund", "Case[Genitive,Dative,Accusative,Ablative]"),
                    group("Supine", "supine", "Case[Accusative,Ablative]"),
                ],
            )
            .citation("IndicativePresentFirstSingular InfinitiveActivePresent IndicativePerfectFirstSingular SupineAccusative|ParticiplePassivePerfect")
            // Deponents and defective verbs lack whole voices.
            .missing_is_absent()
            .head_first(),
            pos("pron", "pron", vec![group("", "", &format!("{CASES} Gender? Number"))]),
            pos("det", "det", vec![group("", "", "Case Gender? Number")]),
            pos("num", "num", vec![group("", "", &format!("{CASES} Gender? Number"))]),
        ];
        s
    }

    /// A vowel marked both long and short (egō̆, lī̆mō̆nis) has uncertain
    /// length: it is written plain.
    fn prepare(&self, e: &mut Entry) {
        for f in &mut e.forms {
            f.form = plain_anceps(&f.form);
        }
    }

    /// Periphrastic perfect passives come as "amātus + present active
    /// indicative of sum": a description, not a form. A j-spelling next to
    /// the same form with i (eius, ejus) is the same word.
    fn adjust(&self, _pos: &PosSpec, _lemma: &str, forms: &mut Vec<(usize, String, bool)>) {
        forms.retain(|f| !f.1.contains('+'));
        let all = forms.clone();
        forms.retain(|f| !f.1.contains('j') || !all.iter().any(|g| g.0 == f.0 && g.1 == f.1.replace('j', "i")));
    }

    fn samples(&self) -> Vec<&'static str> {
        vec!["rosa", "rex", "amo", "fero, ferre, tuli, latum", "hortor", "bonus", "blorgus", "zarpo"]
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
// Participles decline like adjectives:
println!("{}", verb_participle_passive_declined("amo", Tense::Perfect, Case::Genitive, Gender::Feminine, Number::Plural));
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

fn plain_anceps(form: &str) -> String {
    let mut out = String::new();
    let mut chars = form.chars().peekable();
    while let Some(c) = chars.next() {
        if chars.peek() == Some(&'\u{306}') {
            if let Some(&(_, plain)) = MACRONS.iter().find(|m| m.0 == c) {
                chars.next();
                out.push(plain);
                continue;
            }
        }
        out.push(c);
    }
    out
}

const MACRONS: [(char, char); 12] = [
    ('ā', 'a'), ('ē', 'e'), ('ī', 'i'), ('ō', 'o'), ('ū', 'u'), ('ȳ', 'y'),
    ('Ā', 'A'), ('Ē', 'E'), ('Ī', 'I'), ('Ō', 'O'), ('Ū', 'U'), ('Ȳ', 'Y'),
];

pub fn language() -> Box<dyn Language> {
    Box::new(Latin)
}
