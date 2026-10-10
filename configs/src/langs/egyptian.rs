use morphe_train::kaikki::Entry;
use morphe_train::spec::*;

pub struct Egyptian;

impl Language for Egyptian {
    fn spec(&self) -> Spec {
        let mut s = Spec::new("egyptian", "Egyptian", "egy");
        // Lemmas are transliterations; the canonical form is a hieroglyphic
        // spelling code, not a spelling of the word.
        s.canonical_spelling = false;
        s.features = vec![
            feature("Gender", &[("Masculine", "masculine"), ("Feminine", "feminine")]),
            feature("Number", &[("Singular", "singular"), ("Dual", "dual"), ("Plural", "plural")]),
            feature("ImperativeNumber", &[("Singular", "singular"), ("Plural", "plural")]),
            feature(
                "Tense",
                &[
                    ("Perfect", "perfect"),
                    ("Perfective", "perfective"),
                    ("Imperfective", "imperfective"),
                    ("Prospective", "prospective"),
                    ("Subjunctive", "subjunctive"),
                    ("Consecutive", "consecutive"),
                    ("Terminative", "terminative"),
                    ("Obligative", "obligative"),
                    ("Potential", "potential"),
                ],
            ),
            feature(
                "Aspect",
                &[("Perfect", "perfect"), ("Perfective", "perfective"), ("Imperfective", "imperfective"), ("Prospective", "prospective")],
            ),
            feature("Voice", &[("Active", "active"), ("Passive", "passive")]),
        ];
        s.pos = vec![
            pos("noun", "noun", vec![group("", "", "Number")])
                .inherent("Gender")
                .citation("Singular Plural")
                .lemma_cell("Singular")
                .head_last(),
            pos("adj", "adj", vec![group("", "", "Gender Number")])
                .citation("MasculineSingular FeminineSingular")
                .head_last(),
            pos(
                "verb",
                "verb",
                vec![
                    group("Infinitive", "infinitive !negative !imperative", ""),
                    group("NegativeInfinitive", "infinitive negative", ""),
                    group("Imperative", "imperative", "ImperativeNumber"),
                    group("Stative", "stative", ""),
                    group("", "active !participle", "Tense"),
                    group("Passive", "passive !participle", "Tense"),
                    group("Participle", "participle !relative", "Voice? Aspect"),
                    group("Relative", "participle relative", "Voice? Aspect"),
                ],
            )
            .citation("Infinitive Perfect")
            .head_last(),
        ];
        s
    }

    /// Periphrastic forms (ḥr sḏm) and passive formulas ("active + .tj")
    /// are left out.
    fn prepare(&self, e: &mut Entry) {
        let spaced = e.word.contains(' ');
        e.forms.retain(|f| f.source.is_none() || f.tags.iter().any(|t| t == "table-tags" || t == "inflection-template") || !(f.form.contains('+') || (!spaced && f.form.contains(' '))));
    }

    fn samples(&self) -> Vec<&'static str> {
        vec!["gp", "km", "sḏm", "h", "mt", "nfr", "ḏd", "wzf"]
    }

    fn readme(&self) -> String {
        r#"## Example

```rust
use egyptian::*;
println!("{}", noun("h", Number::Plural)); // hw
println!("{}", noun("h", Number::Dual)); // hwj
println!("{}", adj("km", Gender::Feminine, Number::Singular)); // kmt
println!("{}", verb("gp", Tense::Perfect)); // gp.n
println!("{}", verb_participle("gp", Voice::Passive, Aspect::Perfective)); // gpp
```

Words are in the standard Egyptological transliteration (ꜣ, ꜥ, ḥ, ḫ, ẖ, š,
ṯ, ḏ). Nouns have the singular, dual and plural and their gender;
adjectives agree in gender and number. Verbs have the infinitive and its
negative, the imperative, the stative, the suffix-conjugation forms (sḏm.n,
sḏm.jn, sḏm.ḫr, sḏm.kꜣ, …) active and passive, and the participles and
relative forms. Forms built with auxiliaries (ḥr sḏm) are left out.
"#
        .into()
    }
}

pub fn language() -> Box<dyn Language> {
    Box::new(Egyptian)
}
