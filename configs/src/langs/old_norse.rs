use morphe_train::kaikki::{Entry, Form};
use morphe_train::spec::*;

pub struct OldNorse;

fn has(tags: &[String], t: &str) -> bool {
    tags.iter().any(|x| x == t)
}

impl Language for OldNorse {
    fn spec(&self) -> Spec {
        let mut s = Spec::new("old_norse", "OldNorse", "non");
        // Length marks are often left out; ö is a common stand-in for ǫ.
        s.unmark = unmark("áa ée íi óo úu ýy ǿø");
        s.fold = vec![('ö', 'ǫ')];
        s.features = vec![
            feature("Definiteness", &[("Indefinite", "indefinite"), ("Definite", "definite")]),
            feature(
                "Case",
                &[("Nominative", "nominative"), ("Accusative", "accusative"), ("Dative", "dative"), ("Genitive", "genitive")],
            ),
            feature("Number", &[("Singular", "singular"), ("Plural", "plural")]),
            feature("Gender", &[("Masculine", "masculine"), ("Feminine", "feminine"), ("Neuter", "neuter")]),
            feature("Mood", &[("Indicative", "indicative"), ("Subjunctive", "subjunctive")]),
            feature("Tense", &[("Present", "present"), ("Past", "past")]),
            feature("Person", &[("First", "first-person"), ("Second", "second-person"), ("Third", "third-person")]),
            feature(
                "Imperative",
                &[
                    ("SecondSingular", "second-person singular"),
                    ("FirstPlural", "first-person plural"),
                    ("SecondPlural", "second-person plural"),
                ],
            ),
            feature("Voice", &[("Active", "!mediopassive"), ("Mediopassive", "mediopassive")]),
        ];
        let lemma_only = "!feminine !neuter !plural !accusative !dative !genitive";
        s.pos = vec![
            pos("noun", "noun", vec![group("", "", "Definiteness Case Number")])
                .inherent("Gender")
                .citation("Lemma IndefiniteGenitiveSingular IndefiniteNominativePlural")
                .head_last(),
            pos(
                "adj",
                "adj",
                vec![
                    group("", "!weak !comparative !superlative", "Case Number Gender"),
                    group("Weak", "weak !comparative !superlative", "Case Number Gender"),
                    group("Comparative", &format!("comparative {lemma_only}"), ""),
                    group("Superlative", &format!("superlative {lemma_only}"), ""),
                ],
            )
            .citation("Lemma Comparative Superlative"),
            pos(
                "verb",
                "verb",
                vec![
                    group("Infinitive", "infinitive", "Voice"),
                    group("", "!participle !infinitive !imperative", "Mood Tense Person Number Voice"),
                    group("Imperative", "imperative", "Imperative Voice"),
                    group("ParticiplePresent", "participle present", "Voice"),
                    group("ParticiplePast", "participle past", "Voice"),
                ],
            )
            .citation("InfinitiveActive IndicativePresentThirdSingularActive IndicativePastThirdSingularActive ParticiplePastActive")
            .head_first(),
        ];
        s
    }

    fn prepare(&self, e: &mut Entry) {
        // Headword lines carry no canonical form; the title keeps its length
        // marks (bók, stórr), which the plain key drops.
        if !e.forms.iter().any(|f| has(&f.tags, "canonical")) {
            e.forms.push(Form { form: e.word.clone(), tags: vec!["canonical".into()], source: None });
        }
        // A verb's second table is the mediopassive (takask); an adjective's
        // table marked "weak" is the weak declension.
        let mut tables = 0;
        let mut weak = false;
        for f in &mut e.forms {
            if has(&f.tags, "table-tags") {
                tables += 1;
                weak = f.form == "weak";
                continue;
            }
            let Some(src) = f.source.as_deref() else {
                // The headword line: genitive singular and nominative plural.
                if e.pos == "noun" && !has(&f.tags, "definite") {
                    f.tags.push("indefinite".into());
                    if has(&f.tags, "genitive") && !has(&f.tags, "plural") {
                        f.tags.push("singular".into());
                    }
                    if has(&f.tags, "plural") && !["accusative", "dative", "genitive"].iter().any(|c| has(&f.tags, c)) {
                        f.tags.push("nominative".into());
                    }
                }
                continue;
            };
            if e.pos == "verb" && src == "conjugation" {
                if tables > 1 {
                    f.tags.push("mediopassive".into());
                }
                // The imperative rows carry neither mood.
                if has(&f.tags, "present") && !has(&f.tags, "indicative") && !has(&f.tags, "subjunctive") && !has(&f.tags, "participle") {
                    f.tags.push("imperative".into());
                }
            }
            if e.pos == "adj" && weak {
                f.tags.push("weak".into());
            }
        }
    }

    fn samples(&self) -> Vec<&'static str> {
        vec!["hestr", "maðr", "kona", "taka", "kalla", "vera", "blár", "góðr", "skrumpa", "glindr"]
    }

    fn readme(&self) -> String {
        r#"## Example

```rust
use old_norse::*;
assert_eq!(noun("hestr", Definiteness::Definite, Case::Dative, Number::Plural), "hestunum");
println!("{:?}", noun_gender("kona"));
assert_eq!(verb("taka", Mood::Indicative, Tense::Past, Person::Third, Number::Singular, Voice::Active), "tók");
assert_eq!(verb_infinitive("taka", Voice::Mediopassive), "takask");
assert_eq!(adj("blár", Case::Accusative, Number::Singular, Gender::Masculine), "bláan");
// Citations pin principal parts: lemma, genitive singular, plural.
println!("{}", noun("hestr, hests, hestar", Definiteness::Indefinite, Case::Dative, Number::Plural));
```

Spelling follows Wiktionary's normalized Old Norse. Length marks are
optional on input, and ö may be typed for ǫ.
"#
        .into()
    }
}

pub fn language() -> Box<dyn Language> {
    Box::new(OldNorse)
}
