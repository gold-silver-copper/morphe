use morphe_train::kaikki::Entry;
use morphe_train::spec::*;

pub struct OldChurchSlavonic;

const CASES: &str = "Case[Nominative,Accusative,Genitive,Dative,Instrumental,Locative]";

fn glagolitic(c: char) -> bool {
    matches!(c, '\u{2c00}'..='\u{2c5f}' | '\u{1e000}'..='\u{1e02f}')
}

fn latin(c: char) -> bool {
    c.is_ascii_alphabetic() || ('\u{c0}'..='\u{24f}').contains(&c)
}

impl Language for OldChurchSlavonic {
    fn spec(&self) -> Spec {
        let mut s = Spec::new("old_church_slavonic", "OldChurchSlavonic", "cu");
        // Palatalization hook, titlo, stress.
        s.marks = vec!['\u{484}', '\u{483}', '\u{301}'];
        // Negated forms of бꙑти (нѣсмь) are a separate word.
        s.drop_tags.push("negative".into());
        // Letters written interchangeably in the manuscripts.
        s.fold = vec![('і', 'и'), ('ї', 'и'), ('ѡ', 'о'), ('ꙃ', 'ѕ'), ('ꙁ', 'з')];
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
            feature("Tense", &[("Present", "present"), ("Imperfect", "imperfect"), ("Aorist", "aorist"), ("Past", "past")]),
            feature("Person", &[("First", "first-person"), ("Second", "second-person"), ("Third", "third-person")]),
            feature(
                "PersonNumber",
                &[
                    ("SecondSingular", "second-person singular"),
                    ("ThirdSingular", "third-person singular"),
                    ("FirstDual", "first-person dual"),
                    ("SecondDual", "second-person dual"),
                    ("FirstPlural", "first-person plural"),
                    ("SecondPlural", "second-person plural"),
                ],
            ),
            feature("Voice", &[("Active", "active"), ("Passive", "passive")]),
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
                vec![group("", "!long-form", "Case Gender Number"), group("Long", "long-form", "Case Gender Number")],
            ),
            pos(
                "verb",
                "verb",
                vec![
                    group("", "!imperative !participle !l-participle", "Tense[Present,Imperfect,Aorist] Person Number"),
                    group("Imperative", "imperative", "PersonNumber"),
                    group("Participle", "participle", "Voice Tense[Present,Past]").declines("adj", ""),
                    group("LParticiple", "l-participle", "Gender Number"),
                ],
            )
            .inherent("Aspect")
            .head_first(),
            pos("pron", "pron", vec![group("", "", &format!("{CASES} Gender? Number"))]),
            pos("num", "num", vec![group("", "", &format!("{CASES} Gender? Number?"))]),
        ];
        s
    }

    fn keep_entry(&self, e: &Entry) -> bool {
        !e.word.chars().any(glagolitic)
    }

    /// Kaikki's Old Church Slavonic tables need help:
    /// - finite verb forms come tagged "l-participle" as well, and the aorist
    ///   as "error-unrecognized-form";
    /// - participle stems ("правѧ [active present]") lack "participle";
    /// - the participles' own declension tables don't say which participle
    ///   they decline, so they are dropped (participles decline as
    ///   adjectives instead);
    /// - every pronoun page repeats the personal-pronoun table;
    /// - numeral forms carry their romanization ("ѥдинъ jedinŭ");
    /// - an adjective's second table, its long (definite) forms, is untagged.
    fn prepare(&self, e: &mut Entry) {
        let verb = e.pos == "verb";
        let pron = e.pos == "pron";
        e.forms.retain(|f| {
            let t = |x: &str| f.tags.iter().any(|y| y == x);
            !(verb && (t("long-form") || t("short-form")) || pron && t("personal") || f.form.chars().any(glagolitic))
        });
        let adj = e.pos == "adj";
        let mut table = 0;
        for f in &mut e.forms {
            if f.source.is_none() {
                continue;
            }
            if f.tags.iter().any(|t| t == "table-tags") {
                table += 1;
            }
            if adj && table > 1 && !f.tags.iter().any(|t| t == "long-form" || t == "short-form") {
                f.tags.push("long-form".into());
            }
            // Romanizations, and header labels ("genitive") read as forms.
            if f.form.chars().any(latin) {
                f.form = f.form.split(' ').filter(|w| !w.chars().any(latin)).collect::<Vec<_>>().join(" ");
            }
            if !verb {
                continue;
            }
            let person = f.tags.iter().any(|t| t.ends_with("-person"));
            if person {
                f.tags.retain(|t| t != "l-participle");
                if let Some(t) = f.tags.iter_mut().find(|t| *t == "error-unrecognized-form") {
                    *t = "aorist".into();
                }
            } else if f.tags.iter().any(|t| t == "active" || t == "passive") && f.tags.iter().any(|t| t == "present" || t == "past") {
                f.tags.push("participle".into());
            }
        }
    }

    fn samples(&self) -> Vec<&'static str> {
        vec!["градъ", "рѫка", "правити", "бꙑти", "добръ", "кракъ", "блюпати", "село"]
    }

    fn readme(&self) -> String {
        r#"## Example

```rust
use old_church_slavonic::*;
println!("{}", noun("градъ", Case::Locative, Number::Singular)); // градѣ
println!("{:?}", noun_gender("рѫка"));
println!("{}", verb("правити", Tense::Aorist, Person::Third, Number::Plural)); // правишѧ
println!("{}", verb_imperative("правити", PersonNumber::SecondSingular)); // прави
println!("{}", verb_l_participle("правити", Gender::Feminine, Number::Singular)); // правила
println!("{}", adj("добръ", Case::Dative, Gender::Feminine, Number::Singular)); // добрѣ
println!("{}", adj_long("добръ", Case::Nominative, Gender::Masculine, Number::Singular)); // добрꙑи
println!("{:?}", verb_aspect("правити"));
```

Cyrillic, as Wiktionary normalizes it. Input may leave out the palatalization
hook (҄), titlo and stress marks; і/ї match и, ѡ matches о, and ꙋ/ѹ match оу.
Adjectives have short (indefinite) forms, `adj`, and long (definite) forms,
`adj_long`. Personal pronouns are not covered.
"#
        .into()
    }
}

pub fn language() -> Box<dyn Language> {
    Box::new(OldChurchSlavonic)
}
