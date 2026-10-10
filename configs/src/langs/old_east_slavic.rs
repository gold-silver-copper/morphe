use morphe_train::kaikki::Entry;
use morphe_train::spec::*;

pub struct OldEastSlavic;

fn has(tags: &[String], t: &str) -> bool {
    tags.iter().any(|x| x == t)
}

impl Language for OldEastSlavic {
    fn spec(&self) -> Spec {
        let mut s = Spec::new("old_east_slavic", "OldEastSlavic", "orv");
        // Titlo, stress.
        s.marks = vec!['\u{483}', '\u{301}'];
        // Letters written interchangeably in the manuscripts.
        s.fold = vec![('і', 'и'), ('ї', 'и'), ('ѡ', 'о'), ('ѳ', 'ф'), ('ѵ', 'и')];
        s.fold_strs = vec![("ꙋ".into(), "оу".into()), ("ѹ".into(), "оу".into())];
        s.features = vec![
            feature(
                "Case",
                &[
                    ("Nominative", "nominative"),
                    ("Accusative", "accusative"),
                    ("Genitive", "genitive"),
                    ("Dative", "dative"),
                    ("Instrumental", "instrumental"),
                    ("Locative", "locative"),
                    ("Vocative", "vocative"),
                ],
            ),
            feature("Number", &[("Singular", "singular"), ("Dual", "dual"), ("Plural", "plural")]),
            feature("Gender", &[("Masculine", "masculine"), ("Feminine", "feminine"), ("Neuter", "neuter")]),
            feature("Tense", &[("Present", "present"), ("Imperfect", "imperfect"), ("Aorist", "aorist")]),
            feature("Person", &[("First", "first-person"), ("Second", "second-person"), ("Third", "third-person")]),
            feature(
                "Participle",
                &[
                    ("ActivePresent", "active present"),
                    ("ActivePast", "active past"),
                    ("PassivePresent", "passive present"),
                    ("PassivePast", "passive past"),
                ],
            ),
            feature("Aspect", &[("Imperfective", "imperfective"), ("Perfective", "perfective")]),
        ];
        s.pos = vec![
            pos("noun", "noun", vec![group("", "", "Case Number")])
                .inherent("Gender")
                .citation("NominativeSingular GenitiveSingular")
                .head_last(),
            pos(
                "adj",
                "adj",
                vec![
                    group("", "!long-form", "Case[Nominative,Accusative,Genitive,Dative,Instrumental,Locative] Gender Number"),
                    // The long (definite) forms: свѧтꙑи.
                    group("Long", "long-form", "Case[Nominative,Accusative,Genitive,Dative,Instrumental,Locative] Gender Number"),
                ],
            )
            .citation("NominativeMasculineSingular NominativeFeminineSingular NominativeNeuterSingular"),
            pos(
                "verb",
                "verb",
                vec![
                    group("Infinitive", "infinitive", ""),
                    group("Supine", "supine", ""),
                    group("", "!imperative !cohortative !participle", "Tense Person Number"),
                    group("Imperative", "imperative", "Number"),
                    // правимъ: "let us …"
                    group("Cohortative", "cohortative", "Number[Dual,Plural]"),
                    group("Participle", "participle", "Participle"),
                    // правилъ: the l-participle of the compound past.
                    group("LParticiple", "l-participle", ""),
                ],
            )
            .inherent("Aspect")
            .citation("Infinitive PresentFirstSingular AoristThirdSingular")
            .head_first(),
        ];
        s
    }

    fn prepare(&self, e: &mut Entry) {
        // An adjective's second table, its long forms, is untagged.
        let mut table = 0;
        for f in &mut e.forms {
            if e.pos != "adj" || f.source.is_none() {
                continue;
            }
            if has(&f.tags, "table-tags") {
                table += 1;
            } else if table > 1 {
                f.tags.push("long-form".into());
            }
        }
        for f in &mut e.forms {
            if f.source.is_none() || e.pos != "verb" {
                continue;
            }
            // правивъ: the past active participle comes unlabeled.
            if has(&f.tags, "error-unrecognized-form") && f.tags.len() == 1 {
                f.tags = tags("participle active past");
            }
        }
    }

    fn samples(&self) -> Vec<&'static str> {
        vec!["собака", "озеро", "правити", "красити", "свѧтъ", "добръ", "глорити", "снорокъ"]
    }

    fn readme(&self) -> String {
        r#"## Example

```rust
use old_east_slavic::*;
println!("{}", noun("собака", Case::Dative, Number::Singular)); // собацѣ
println!("{:?}", noun_gender("собака"));
println!("{}", verb("правити", Tense::Aorist, Person::Third, Number::Plural)); // правиша
println!("{}", verb_l_participle("правити")); // правилъ
println!("{}", adj("свѧтъ", Case::Genitive, Gender::Feminine, Number::Singular)); // свѧтꙑ
println!("{}", adj_long("свѧтъ", Case::Nominative, Gender::Masculine, Number::Singular)); // свѧтꙑи
```

Cyrillic, as Wiktionary normalizes it. Input may leave out the titlo and
stress marks; і/ї match и, ѡ matches о, and ꙋ/ѹ match оу.
"#
        .into()
    }
}

pub fn language() -> Box<dyn Language> {
    Box::new(OldEastSlavic)
}
