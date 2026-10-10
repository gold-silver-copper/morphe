use morphe_train::kaikki::{Entry, Sense};
use morphe_train::spec::*;

pub struct MiddleHighGerman;

fn has(tags: &[String], t: &str) -> bool {
    tags.iter().any(|x| x == t)
}

/// Subject pronouns the conjugation table writes before each form.
const PRONOUNS: &[(&str, &str)] =
    &[("ich", "first-person"), ("du", "second-person"), ("ër", "third-person"), ("wir", "first-person"), ("ir", "second-person"), ("sie", "third-person")];

/// Articles the weak adjective forms come with.
const ARTICLES: &[&str] = &["dër", "diu", "daȥ", "die", "dës", "dëm", "dën"];

impl Language for MiddleHighGerman {
    fn spec(&self) -> Spec {
        let mut s = Spec::new("middle_high_german", "MiddleHighGerman", "gmh");
        // Length marks and the ë of normalized spelling are often left out.
        s.unmark = unmark("āa ēe īi ōo ūu ëe âa êe îi ôo ûu");
        s.fold = vec![('ȥ', 'z')];
        // Compound tenses are auxiliary + participle.
        s.drop_tags.extend(tags("multiword-construction"));
        s.features = vec![
            // The auxiliary of the perfect, read from the conjugation table.
            feature("Auxiliary", &[("Have", "auxiliary-have"), ("Be", "auxiliary-be")]),
            feature("Gender", &[("Masculine", "masculine"), ("Feminine", "feminine"), ("Neuter", "neuter")]),
            feature(
                "Case",
                &[("Nominative", "nominative"), ("Genitive", "genitive"), ("Dative", "dative"), ("Accusative", "accusative")],
            ),
            feature("Number", &[("Singular", "singular"), ("Plural", "plural")]),
            feature("Mood", &[("Indicative", "indicative"), ("Subjunctive", "subjunctive")]),
            feature("Tense", &[("Present", "present"), ("Past", "preterite")]),
            feature("Person", &[("First", "first-person"), ("Second", "second-person"), ("Third", "third-person")]),
        ];
        let adj_cells = "Case Number Gender";
        s.pos = vec![
            pos("noun", "noun", vec![group("", "noun", "Case Number")])
                .inherent("Gender")
                .citation("Lemma GenitiveSingular NominativePlural")
                .head_last(),
            pos(
                "adj",
                "adj",
                vec![
                    group("", "strong !comparative !superlative", adj_cells),
                    group("Weak", "weak !comparative !superlative", adj_cells),
                    group("Comparative", "comparative !nominative !genitive !dative !accusative", ""),
                    group("Superlative", "superlative !nominative !genitive !dative !accusative", ""),
                ],
            )
            .citation("Lemma Comparative Superlative"),
            pos(
                "verb",
                "verb",
                vec![
                    group("", "!participle !infinitive !imperative !auxiliary", "Mood Tense Person Number"),
                    group("Imperative", "imperative", "Number"),
                    group("ParticiplePresent", "participle present", ""),
                    group("ParticiplePast", "participle past", ""),
                ],
            )
            .inherent("Auxiliary")
            .citation("Lemma IndicativePastThirdSingular IndicativePastFirstPlural ParticiplePast")
            .head_first(),
        ];
        s
    }

    fn prepare(&self, e: &mut Entry) {
        // The auxiliary is a property of the verb: keep it as a sense tag.
        if let Some(aux) = e.forms.iter().find(|f| f.tags.iter().any(|t| t == "auxiliary")).map(|f| f.form.clone()) {
            let tag = if aux == "hān" { "auxiliary-have" } else if aux == "sīn" { "auxiliary-be" } else { "" };
            if !tag.is_empty() {
                if e.senses.is_empty() {
                    e.senses.push(Sense { tags: Vec::new() });
                }
                for sense in &mut e.senses {
                    sense.tags.push(tag.into());
                }
            }
        }
        let mut tense = "present";
        for f in &mut e.forms {
            let Some(src) = f.source.as_deref() else {
                // The headword line's genitive and plural are noun forms.
                if e.pos == "noun" && (has(&f.tags, "genitive") || has(&f.tags, "plural")) {
                    f.tags.push("noun".into());
                    if !has(&f.tags, "genitive") {
                        f.tags.push("nominative".into());
                    }
                    if !has(&f.tags, "plural") {
                        f.tags.push("singular".into());
                    }
                }
                if e.pos == "verb" && has(&f.tags, "past") && !has(&f.tags, "participle") {
                    f.tags.retain(|t| t != "past");
                    f.tags.extend(["preterite".to_string(), "third-person".to_string(), "singular".to_string()]);
                    if !has(&f.tags, "subjunctive") {
                        f.tags.push("indicative".into());
                    }
                }
                if e.pos == "verb" && has(&f.tags, "present") && has(&f.tags, "third-person") {
                    f.tags.push("indicative".into());
                }
                continue;
            };
            if e.pos == "adj" && src == "declension" {
                if let Some((first, rest)) = f.form.split_once(' ') {
                    if ARTICLES.contains(&first) {
                        f.form = rest.to_string();
                    }
                }
                // The masculine and feminine plural come unlabeled.
                if has(&f.tags, "error-unrecognized-form") {
                    f.tags.retain(|t| t != "error-unrecognized-form");
                    f.tags.extend(["masculine".to_string(), "feminine".to_string()]);
                }
                continue;
            }
            if e.pos != "verb" || src != "conjugation" {
                continue;
            }
            if has(&f.tags, "imperative") {
                // "gip (du)"
                f.form = f.form.split(" (").next().unwrap_or(&f.form).to_string();
                continue;
            }
            // "ich gibe": the pronoun gives the person; a subjunctive row
            // follows the indicative row of its tense.
            let Some((p, rest)) = f.form.split_once(' ') else {
                if has(&f.tags, "error-unrecognized-form") {
                    // Inflected infinitives (gëbennes).
                    f.tags.clear();
                }
                continue;
            };
            let Some((_, person)) = PRONOUNS.iter().find(|x| x.0 == p) else { continue };
            f.form = rest.to_string();
            f.tags.push((*person).into());
            if has(&f.tags, "indicative") {
                tense = if has(&f.tags, "preterite") { "preterite" } else { "present" };
            } else if has(&f.tags, "subjunctive") {
                f.tags.retain(|t| t != "error-unrecognized-form");
                f.tags.push(tense.into());
            }
        }
    }

    fn samples(&self) -> Vec<&'static str> {
        vec!["tac", "name", "geben", "sīn", "guot", "blint", "glumpen", "strabel"]
    }

    fn readme(&self) -> String {
        r#"## Example

```rust
use middle_high_german::*;
assert_eq!(noun("tac", Case::Dative, Number::Plural), "tagen");
println!("{:?}", noun_gender("name"));
assert_eq!(verb("geben", Mood::Indicative, Tense::Past, Person::Third, Number::Singular), "gap");
println!("{}", verb("geben", Mood::Subjunctive, Tense::Past, Person::First, Number::Singular)); // gæbe
println!("{}", adj_weak("guot", Case::Nominative, Number::Singular, Gender::Masculine)); // guote
```

Spelling follows Wiktionary's normalized Middle High German. Length marks
and ë are optional on input, and z matches ȥ. Weak adjective forms are given
without their article. Compound tenses are left to the caller.
"#
        .into()
    }
}

pub fn language() -> Box<dyn Language> {
    Box::new(MiddleHighGerman)
}
