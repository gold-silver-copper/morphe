use morphe_train::kaikki::{Entry, Form};
use morphe_train::spec::*;

pub struct Welsh;

/// Personal pronouns that colloquial tables write after a verb or preposition.
const PRONOUNS: &[&str] = &["i", "fi", "di", "ti", "o", "fo", "e", "fe", "hi", "ni", "chi", "nhw", "hwy"];

fn strip_pronoun(form: &str) -> Option<&str> {
    let (head, last) = form.rsplit_once(' ')?;
    (PRONOUNS.contains(&last) && !head.contains(' ')).then_some(head)
}

impl Language for Welsh {
    fn spec(&self) -> Spec {
        let mut s = Spec::new("welsh", "Welsh", "cy");
        s.unmark = unmark(
            "âa êe îi ôo ûu ŵw ŷy áa ée íi óo úu ẃw ýy àa èe ìi òo ùu ẁw ỳy äa ëe ïi öo üu ẅw ÿy",
        );
        s.drop_tags.extend(tags("dated literary-only"));
        s.soft_tags.extend(tags("literary North-Wales South-Wales poetic"));
        s.features = vec![
            feature("Gender", &[("Masculine", "masculine"), ("Feminine", "feminine")]),
            feature(
                "Mutation",
                &[("Soft", "soft"), ("Nasal", "nasal"), ("Aspirate", "aspirate"), ("HProthesis", "prothesis-h")],
            ),
            feature("Number", &[("Plural", "plural"), ("Singulative", "singulative")]),
            feature(
                "Form",
                &[
                    ("Feminine", "feminine singular"),
                    ("Plural", "plural"),
                    ("Equative", "equative"),
                    ("Comparative", "comparative"),
                    ("Superlative", "superlative"),
                ],
            ),
            feature(
                "Tense",
                &[
                    // The literary present, which is also the future.
                    ("Present", "present !subjunctive !imperfect"),
                    // The colloquial future (eurith).
                    ("Future", "future !indicative !present"),
                    ("Preterite", "preterite"),
                    ("Imperfect", "imperfect"),
                    ("Conditional", "conditional"),
                    ("Pluperfect", "pluperfect"),
                    ("Subjunctive", "present subjunctive"),
                    ("Imperative", "imperative"),
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
                    ("Impersonal", "impersonal"),
                ],
            ),
            feature(
                "Person",
                &[
                    ("FirstSingular", "first-person singular"),
                    ("SecondSingular", "second-person singular"),
                    ("ThirdSingularMasculine", "third-person singular !feminine"),
                    ("ThirdSingularFeminine", "third-person singular feminine"),
                    ("FirstPlural", "first-person plural"),
                    ("SecondPlural", "second-person plural"),
                    ("ThirdPlural", "third-person plural"),
                ],
            ),
        ];
        let mutated = || group("Mutated", "", "Mutation");
        s.pos = vec![
            pos("noun", "noun", vec![group("", "", "Number"), mutated()])
                .inherent("Gender")
                .citation("Lemma Plural")
                .initial(2)
                .head_first(),
            pos("adj", "adj", vec![group("", "!first-person", "Form"), mutated()])
                .citation("Lemma Feminine Plural")
                .initial(2),
            pos(
                "verb",
                "verb",
                vec![group("", "!participle", "Tense PersonNumber"), group("Participle", "participle", ""), mutated()],
            )
            .citation("Lemma PresentFirstSingular")
            .initial(2)
            .head_first(),
            pos("prep", "prep", vec![group("", "", "Person")]),
            pos("num", "num", vec![mutated()]).initial(2),
            pos("pron", "pron", vec![mutated()]).initial(2),
        ];
        s
    }

    fn prepare(&self, e: &mut Entry) {
        // The mutation table labels only soft mutation and h-prothesis; the
        // radical and the nasal and aspirate forms come unlabeled.
        let word = e.word.to_lowercase();
        for f in e.forms.iter_mut() {
            if f.source.as_deref() != Some("mutation") || !f.tags.iter().any(|t| t == "error-unrecognized-form") {
                continue;
            }
            let form = f.form.to_lowercase();
            let tag = if form == word {
                "radical"
            } else if ["ch", "ph", "th"].iter().any(|p| form.starts_with(p)) {
                "aspirate"
            } else {
                "nasal"
            };
            f.tags = vec![tag.into()];
        }
        // A word with a mutation table but no form for some mutation keeps
        // its radical there (b has no aspirate mutation).
        if e.forms.iter().any(|f| f.source.as_deref() == Some("mutation")) {
            for m in ["soft", "nasal", "aspirate", "prothesis-h"] {
                if !e.forms.iter().any(|f| f.tags.iter().any(|t| t == m)) {
                    e.forms.push(Form { form: e.word.clone(), tags: vec![m.into()], source: Some("mutation".into()) });
                }
            }
        }
    }

    fn retag(&self, pos: &PosSpec, form: &str, tags: &mut Vec<String>) {
        // Prepositions' third-person singular comes as "ohono m", "ohoni f".
        if form.ends_with(" f") {
            tags.push("feminine".into());
        }
        if pos.name == "noun" && tags.iter().any(|t| t == "singular" || t == "singulative") && !tags.iter().any(|t| t == "plural") {
            tags.retain(|t| t != "masculine" && t != "feminine");
            tags.push("singulative".into());
        }
    }

    fn split_form(&self, raw: &str) -> Vec<String> {
        let raw = raw.strip_suffix(" m").or_else(|| raw.strip_suffix(" f")).unwrap_or(raw);
        vec![raw.to_string()]
    }

    fn adjust(&self, pos: &PosSpec, _lemma: &str, forms: &mut Vec<(usize, String, bool)>) {
        match pos.name.as_str() {
            // Colloquial tables add the pronoun: "eurith o" is "eurith".
            // These colloquial forms rank after the literary ones.
            "verb" => {
                forms.retain(|f| !PRONOUNS.contains(&f.1.as_str()));
                for f in forms.iter_mut() {
                    if let Some(h) = strip_pronoun(&f.1) {
                        f.1 = h.to_string();
                        f.2 = true;
                    }
                }
            }
            "prep" => forms.retain(|f| !f.1.contains(' ') && !PRONOUNS.contains(&f.1.as_str())),
            _ => {}
        }
    }

    fn samples(&self) -> Vec<&'static str> {
        vec!["cath", "tad", "ysgol", "canu", "mynd", "mawr", "glas", "trwnc", "pobwl", "ar"]
    }

    fn readme(&self) -> String {
        r#"## Example

```rust
use welsh::*;
assert_eq!(noun("cath", Number::Plural), "cathod");
assert_eq!(noun_mutated("cath", Mutation::Soft), "gath");
assert_eq!(noun_mutated("cath", Mutation::Nasal), "nghath");
println!("{:?}", noun_gender("ysgol"));
assert_eq!(verb("canu", Tense::Preterite, PersonNumber::FirstSingular), "canais");
assert_eq!(adj("gwyn", Form::Feminine), "gwen");
assert_eq!(prep("ar", Person::FirstSingular), "arnaf");
```

Verb forms are the literary standard ones; `Tense::Future` is the colloquial
future (canith). Mutations are given for the base form only; mutate an
inflected form by passing it as the word. The tree reads a word's first two
letters, so mutation follows the initial consonant. Circumflexes are optional
on input.
"#
        .into()
    }
}

pub fn language() -> Box<dyn Language> {
    Box::new(Welsh)
}
