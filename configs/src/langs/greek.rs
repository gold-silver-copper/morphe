use morphe_train::kaikki::Entry;
use morphe_train::spec::*;

pub struct Greek;

const CASES: &str = "Case[Nominative,Genitive,Accusative,Vocative]";

impl Language for Greek {
    fn spec(&self) -> Spec {
        let mut s = Spec::new("greek", "Greek", "el");
        s.unmark = unmark("άα έε ήη ίι όο ύυ ώω ΐϊ ΰϋ");
        // Katharevousa and other learned forms come tagged archaic; weak
        // pronoun forms are clitics, kept behind the strong ones.
        s.soft_tags.extend(tags("weak formal literary Katharevousa"));
        s.features = vec![
            feature(
                "Case",
                &[("Nominative", "nominative"), ("Genitive", "genitive"), ("Accusative", "accusative"), ("Vocative", "vocative")],
            ),
            feature("Number", &[("Singular", "singular"), ("Plural", "plural")]),
            feature("Gender", &[("Masculine", "masculine"), ("Feminine", "feminine"), ("Neuter", "neuter")]),
            feature("Person", &[("First", "first-person"), ("Second", "second-person"), ("Third", "third-person")]),
            // Verbs without a passive leave the voice unnamed.
            feature("Voice", &[("Active", "!passive"), ("Passive", "passive")]),
            feature(
                "Tense",
                &[
                    ("Present", "present !imperfect"),
                    ("Imperfect", "imperfect"),
                    ("Aorist", "past perfective"),
                    ("Dependent", "dependent"),
                ],
            ),
            feature("Aspect", &[("Imperfective", "imperfective"), ("Perfective", "perfective")]),
        ];
        s.pos = vec![
            pos("noun", "noun", vec![group("", "", &format!("{CASES} Number"))])
                .inherent("Gender")
                .citation("NominativeSingular GenitiveSingular")
                .head_last(),
            pos(
                "adj",
                "adj",
                vec![
                    group("", "!comparative !superlative", &format!("{CASES} Gender Number")),
                    group("Comparative", "comparative", &format!("{CASES} Gender Number")),
                ],
            )
            .citation("NominativeMasculineSingular NominativeFeminineSingular NominativeNeuterSingular")
            .head_last(),
            pos(
                "verb",
                "verb",
                vec![
                    // The future and subjunctive are θα/να + these forms.
                    group("", "indicative !future !participle", "Voice Tense Person Number"),
                    group("Imperative", "imperative", "Voice Aspect Number"),
                    group("Infinitive", "infinitive-aorist", "Voice"),
                    group("Gerund", "participle present !passive", ""),
                    group("ParticiplePresent", "participle passive present", "").declines("adj", ""),
                    group("ParticiplePerfect", "participle passive !present", "").declines("adj", ""),
                ],
            )
            .citation("ActivePresentFirstSingular|PassivePresentFirstSingular ActiveAoristFirstSingular|PassiveAoristFirstSingular")
            .head_first(),
            pos("pron", "pron det", vec![group("", "", &format!("{CASES} Gender? Number?"))]),
            pos("num", "num", vec![group("", "", &format!("{CASES} Gender? Number?"))]),
            pos("article", "article", vec![group("", "", &format!("{CASES} Gender Number"))]),
        ];
        s
    }

    fn keep_entry(&self, e: &Entry) -> bool {
        // A few Ancient Greek tables slipped into the Modern Greek dump.
        !e.forms.iter().any(|f| f.tags.iter().any(|t| t == "inflection-template") && f.form.starts_with("grc"))
    }

    fn split_form(&self, raw: &str) -> Vec<String> {
        // Verb tables carry editorial notes: "μιλάς (μιλείς → ομιλώ)",
        // "{γραμμένος", "θέλουν²", "a - [b]", "‑η" suffix stubs. Keep the
        // plain words; bracketed forms are learned variants, left out.
        let head = raw.split('(').next().unwrap_or("");
        head.split([',', '-'])
            .map(|p| p.chars().filter(|c| !"{}|^➤¹²³⁴".contains(*c)).collect::<String>().trim().to_string())
            .filter(|w| {
                !w.is_empty()
                    && w.chars().all(|c| (c.is_alphabetic() && !c.is_ascii()) || c == ' ' || ('\u{300}'..='\u{36f}').contains(&c))
            })
            .collect()
    }

    fn readme(&self) -> String {
        r#"## Example

```rust
use greek::*;
assert_eq!(noun("δρόμος", Case::Genitive, Number::Plural), "δρόμων");
println!("{:?}", noun_gender("θάλασσα"));
println!("{}", adj("καλός", Case::Accusative, Gender::Feminine, Number::Plural));
println!("{}", verb("γράφω", Voice::Active, Tense::Aorist, Person::Third, Number::Plural));
println!("{}", verb_imperative("γράφω", Voice::Active, Aspect::Perfective, Number::Plural));
// Accents are optional on input:
println!("{}", noun("θαλασσα", Case::Genitive, Number::Singular));
// Citations pin principal parts: nominative and genitive singular.
println!("{}", noun("γάλα, γάλακτος", Case::Genitive, Number::Plural));
```

Verbs give the synthetic forms; the future (θα + present or dependent), the
subjunctive (να + the same) and the perfect tenses (έχω + infinitive) are
built from them. Input matching ignores the tonos accent.
"#
        .into()
    }
}

pub fn language() -> Box<dyn Language> {
    Box::new(Greek)
}
