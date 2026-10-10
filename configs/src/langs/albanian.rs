use morphe_train::kaikki::Entry;
use morphe_train::spec::*;

pub struct Albanian;

impl Language for Albanian {
    fn spec(&self) -> Spec {
        let mut s = Spec::new("albanian", "Albanian", "sq");
        s.features = vec![
            feature(
                "Case",
                &[("Nominative", "nominative"), ("Accusative", "accusative"), ("Dative", "dative"), ("Ablative", "ablative")],
            ),
            feature("Definiteness", &[("Indefinite", "indefinite"), ("Definite", "definite")]),
            feature("Number", &[("Singular", "singular"), ("Plural", "plural")]),
            feature("Gender", &[("Masculine", "masculine"), ("Feminine", "feminine"), ("Neuter", "neuter")]),
            feature(
                "Agreement",
                &[("Feminine", "feminine !plural"), ("MasculinePlural", "masculine plural"), ("FemininePlural", "feminine plural")],
            ),
            feature("Person", &[("First", "first-person"), ("Second", "second-person"), ("Third", "third-person")]),
            feature(
                "Tense",
                &[
                    ("Present", "indicative present"),
                    ("Imperfect", "indicative imperfect"),
                    ("Aorist", "indicative aorist"),
                    ("Perfect", "indicative perfect !past !future"),
                    ("Pluperfect", "indicative past perfect"),
                    ("Future", "indicative future !perfect"),
                    ("FuturePerfect", "indicative future perfect"),
                    ("PresentSubjunctive", "subjunctive present"),
                    ("ImperfectSubjunctive", "subjunctive imperfect"),
                    ("PerfectSubjunctive", "subjunctive perfect !past"),
                    ("PluperfectSubjunctive", "subjunctive past perfect"),
                    ("Conditional", "conditional imperfect"),
                    ("ConditionalPerfect", "conditional past perfect"),
                    ("Optative", "optative present"),
                    ("OptativePerfect", "optative perfect"),
                    ("AdmirativePresent", "admirative present"),
                    ("AdmirativeImperfect", "admirative imperfect"),
                    ("AdmirativePerfect", "admirative perfect !past"),
                    ("AdmirativePluperfect", "admirative past perfect"),
                ],
            ),
        ];
        s.pos = vec![
            pos("noun", "noun", vec![group("", "", "Case Definiteness Number")])
                .inherent("Gender")
                .citation("NominativeIndefiniteSingular NominativeDefiniteSingular NominativeIndefinitePlural")
                .head_last(),
            pos("adj", "adj", vec![group("", "", "Agreement")]).citation("Lemma Feminine").head_last(),
            pos(
                "verb",
                "verb",
                vec![
                    group("Participle", "participle", ""),
                    group("Gerund", "gerund", ""),
                    group("Infinitive", "infinitive", ""),
                    group("", "!imperative", "Tense Person Number"),
                    group("Imperative", "imperative second-person", "Number"),
                ],
            )
            .citation("PresentFirstSingular AoristFirstSingular Participle")
            .head_first(),
        ];
        s
    }

    /// Headword lines give the definite and plural forms of nouns without
    /// their case, and the aorist of verbs without its person: fill them in
    /// so they back up words without a table. A verb's second table is its
    /// mediopassive (flitet, u fol); only the active one is read.
    /// Adjective forms are read without their linking article (i, e, të),
    /// which Wiktionary gives for some and not others.
    fn prepare(&self, e: &mut Entry) {
        let mut table = 0;
        e.forms.retain(|f| {
            if f.tags.iter().any(|t| t == "table-tags") {
                table += 1;
            }
            e.pos != "verb" || table < 2
        });
        if e.pos == "adj" {
            for f in &mut e.forms {
                for art in ["i ", "e ", "të "] {
                    if let Some(rest) = f.form.strip_prefix(art) {
                        f.form = rest.to_string();
                    }
                }
            }
        }
        for f in e.forms.iter_mut().filter(|f| f.source.is_none()) {
            let has = |t: &str| f.tags.iter().any(|x| x == t);
            let only = |ok: &[&str]| f.tags.iter().all(|t| ok.contains(&t.as_str()));
            if e.pos == "noun" && !f.tags.is_empty() && only(&["definite", "plural"]) {
                let def = if has("definite") { "definite" } else { "indefinite" };
                let num = if has("plural") { "plural" } else { "singular" };
                f.tags = tags(&format!("nominative {def} {num}"));
            } else if e.pos == "verb" && only(&["aorist"]) && has("aorist") {
                f.tags = tags("indicative aorist first-person singular");
            }
        }
    }

    fn samples(&self) -> Vec<&'static str> {
        vec!["mal", "vajzë", "qytet", "punoj", "jam", "flas", "i mirë", "brakoj"]
    }

    fn readme(&self) -> String {
        r#"## Example

```rust
use albanian::*;
assert_eq!(noun("mal", Case::Nominative, Definiteness::Definite, Number::Plural), "malet");
println!("{}", noun("vajzë", Case::Dative, Definiteness::Definite, Number::Singular)); // vajzës
println!("{:?}", noun_gender("qytet"));
println!("{}", verb("punoj", Tense::Aorist, Person::Third, Number::Singular)); // punoi
println!("{}", verb("punoj", Tense::AdmirativePresent, Person::First, Number::Singular)); // punuakam
println!("{}", verb_participle("punoj")); // punuar
// Citations pin principal parts: present, aorist, participle.
println!("{}", verb("flas, fola, folur", Tense::Imperfect, Person::First, Number::Plural));
```

Nouns have four cases, definite and indefinite, in both numbers (the
genitive is the dative with a linking article). Verbs have every tense and
mood of the tables, including the periphrastic ones (kam punuar, do të
punoj), the admirative and the optative. Adjectives have their feminine and
plural forms from the headword line.
"#
        .into()
    }
}

pub fn language() -> Box<dyn Language> {
    Box::new(Albanian)
}
