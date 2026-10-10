use morphe_train::kaikki::{Entry, Sense};
use morphe_train::spec::*;

pub struct Yiddish;

fn has(tags: &[String], t: &str) -> bool {
    tags.iter().any(|x| x == t)
}

/// Subject pronouns the conjugation table writes before each present form.
const PRONOUNS: &[(&str, &str, &str)] = &[
    ("איך", "first-person", "singular"),
    ("דו", "second-person", "singular"),
    ("ער", "third-person", "singular"),
    ("מיר", "first-person", "plural"),
    ("איר", "second-person", "plural"),
    ("זיי", "third-person", "plural"),
];

impl Language for Yiddish {
    fn spec(&self) -> Spec {
        let mut s = Spec::new("yiddish", "Yiddish", "yi");
        // Vowel points and the rafe/dagesh are often left out.
        s.marks = vec!['\u{5B4}', '\u{5B7}', '\u{5B8}', '\u{5BC}', '\u{5BF}', '\u{5C2}'];
        s.fold_strs = vec![("ײ".into(), "יי".into()), ("װ".into(), "וו".into()), ("ױ".into(), "וי".into())];
        // Compound tenses are auxiliary + participle.
        s.drop_tags.extend(tags("dated Daytshmerish pluperfect future"));
        s.features = vec![
            // The auxiliary of the perfect, read from the conjugation table.
            feature("Auxiliary", &[("Have", "auxiliary-have"), ("Be", "auxiliary-be")]),
            feature("Gender", &[("Masculine", "masculine"), ("Feminine", "feminine"), ("Neuter", "neuter")]),
            feature("Case", &[("Nominative", "nominative"), ("Accusative", "accusative"), ("Dative", "dative")]),
            feature(
                "Form",
                &[
                    ("Masculine", "masculine"),
                    ("Feminine", "feminine"),
                    // אַ גוט קינד
                    ("Neuter", "neuter indefinite"),
                    // דאָס גוטע קינד
                    ("NeuterDefinite", "neuter definite"),
                    // עפּעס גוטס
                    ("NeuterPostpositional", "neuter postpositional"),
                    ("Plural", "plural"),
                ],
            ),
            feature(
                "PersonNumber",
                &[
                    ("FirstSingular", "first-person singular"),
                    ("SecondSingular", "second-person singular"),
                    ("ThirdSingular", "third-person singular"),
                    ("FirstPlural", "first-person plural"),
                    ("SecondPlural", "second-person plural"),
                    ("ThirdPlural", "third-person plural"),
                ],
            ),
            feature("Number", &[("Singular", "singular"), ("Plural", "plural")]),
        ];
        s.pos = vec![
            pos("noun", "noun", vec![group("Plural", "plural", ""), group("Diminutive", "diminutive", "")])
                .inherent("Gender")
                .citation("Lemma Plural")
                .head_last(),
            pos(
                "adj",
                "adj",
                vec![
                    group("", "!comparative !superlative", "Case Form"),
                    group("Comparative", "comparative", ""),
                    group("Superlative", "superlative", ""),
                ],
            )
            .citation("Lemma Comparative Superlative"),
            pos(
                "verb",
                "verb",
                vec![
                    group("Present", "present !participle", "PersonNumber"),
                    group("Imperative", "imperative", "Number"),
                    group("ParticiplePresent", "participle present", ""),
                    group("ParticiplePast", "participle past", ""),
                ],
            )
            .inherent("Auxiliary")
            .citation("Lemma ParticiplePast")
            .head_first(),
        ];
        s
    }

    fn prepare(&self, e: &mut Entry) {
        // The auxiliary is a property of the verb: keep it as a sense tag.
        if let Some(aux) = e.forms.iter().find(|f| f.tags.iter().any(|t| t == "auxiliary")).map(|f| f.form.clone()) {
            let tag = if aux == "האָבן" { "auxiliary-have" } else if aux == "זײַן" { "auxiliary-be" } else { "" };
            if !tag.is_empty() {
                if e.senses.is_empty() {
                    e.senses.push(Sense { tags: Vec::new() });
                }
                for sense in &mut e.senses {
                    sense.tags.push(tag.into());
                }
            }
        }
        // The headword line comes as canonical forms: "plural כלבים",
        // "diminutive הינטל", then further variants of the same label.
        let mut label: Option<&str> = None;
        for (i, f) in e.forms.iter_mut().enumerate() {
            if has(&f.tags, "canonical") {
                if let Some((l, rest)) = f.form.split_once(' ') {
                    if l == "plural" || l == "diminutive" {
                        label = Some(if l == "plural" { "plural" } else { "diminutive" });
                        f.form = rest.to_string();
                    }
                }
                if i > 0 {
                    if let Some(l) = label {
                        f.tags = vec![l.into()];
                    }
                }
                continue;
            }
            if f.source.as_deref() != Some("conjugation") {
                continue;
            }
            if has(&f.tags, "present") {
                // "איך בענטש" → first-person singular "בענטש"
                if let Some((p, rest)) = f.form.split_once(' ') {
                    if let Some((_, person, number)) = PRONOUNS.iter().find(|x| x.0 == p) {
                        f.form = rest.to_string();
                        f.tags.extend([person.to_string(), number.to_string()]);
                    }
                }
            } else if has(&f.tags, "imperative") {
                // "בענטש (דו)"; the romanized rows are dropped.
                if let Some(rest) = f.form.strip_suffix(" (דו)") {
                    f.form = rest.to_string();
                    f.tags.push("singular".into());
                } else if let Some(rest) = f.form.strip_suffix(" (איר)") {
                    f.form = rest.to_string();
                    f.tags.push("plural".into());
                } else {
                    f.tags.clear();
                }
            } else if has(&f.tags, "past") {
                f.tags.clear();
            }
        }
    }

    fn samples(&self) -> Vec<&'static str> {
        vec!["הונט", "כלב", "בוך", "גוט", "גרויס", "בענטשן", "גיין", "שרײַבן", "פּליאָנטש"]
    }

    fn readme(&self) -> String {
        r#"## Example

```rust
use yiddish::*;
println!("{}", noun_plural("הונט")); // הינט
println!("{:?}", noun_gender("הונט"));
println!("{}", verb_present("בענטשן", PersonNumber::SecondSingular)); // בענטשסט
println!("{:?}", verb_auxiliary("גיין")); // Be
println!("{}", adj("גוט", Case::Nominative, Form::Masculine)); // גוטער
```

Vowel points are optional on input, and ײ, װ, ױ match their two-letter
spellings. The past and future are compound tenses: `verb_auxiliary` plus
`verb_participle_past`, or וועלן plus the infinitive.
"#
        .into()
    }
}

pub fn language() -> Box<dyn Language> {
    Box::new(Yiddish)
}
