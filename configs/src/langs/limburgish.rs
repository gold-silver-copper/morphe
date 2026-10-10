use morphe_train::kaikki::{Entry, Form};
use morphe_train::spec::*;

pub struct Limburgish;

fn has(tags: &[String], t: &str) -> bool {
    tags.iter().any(|x| x == t)
}

impl Language for Limburgish {
    fn spec(&self) -> Spec {
        let mut s = Spec::new("limburgish", "Limburgish", "li");
        s.unmark = unmark("èe ëe ée êe öo óo ôo äa áa âa üu ïi");
        s.fold = vec![('’', '\'')];
        s.inherent_form_tags = tags("auxiliary");
        // The verbal noun, adverb and declined participle are separate words.
        s.drop_tags.extend(tags("gerund adverb noun adjectival"));
        s.soft_tags.extend(tags("Maastrichtian"));
        s.features = vec![
            feature("Gender", &[("Masculine", "masculine"), ("Feminine", "feminine"), ("Neuter", "neuter")]),
            feature("Auxiliary", &[("Haane", "haane"), ("Seene", "séëne")]),
            feature(
                "Mood",
                &[("Present", "present !subjunctive"), ("Past", "past !subjunctive"), ("Subjunctive", "subjunctive")],
            ),
            feature("Person", &[("First", "first-person"), ("Second", "second-person"), ("Third", "third-person")]),
            feature("Number", &[("Singular", "singular"), ("Plural", "plural")]),
            feature("Tense", &[("Present", "present"), ("Past", "past")]),
            feature(
                "Imperative",
                &[
                    ("Singular", "singular !polite"),
                    ("Polite", "polite singular"),
                    ("Plural", "plural !dual"),
                    ("Dual", "dual"),
                ],
            ),
            feature(
                "Case",
                &[
                    ("Nominative", "nominative"),
                    ("Accusative", "accusative"),
                    ("Genitive", "genitive"),
                    ("Dative", "dative"),
                    ("Locative", "locative"),
                ],
            ),
            feature(
                "Form",
                &[
                    ("Masculine", "masculine"),
                    ("Feminine", "feminine"),
                    ("Comparative", "comparative"),
                    ("Superlative", "superlative !predicative"),
                    // 't bès
                    ("PredicativeSuperlative", "predicative superlative"),
                ],
            ),
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
                    group("", "", "Case Gender Number"),
                    // The headword line's forms.
                    group("Simple", "", "Form"),
                ],
            )
            .citation("Lemma SimpleComparative SimpleSuperlative"),
            pos(
                "verb",
                "verb",
                vec![
                    group("Infinitive", "infinitive", ""),
                    group("Participle", "participle", "Tense"),
                    // Verb-second order: "ich male".
                    group("", "!v1 !imperative !participle", "Mood Person Number"),
                    group("Imperative", "imperative !v1", "Imperative"),
                    // Verb-first order, with the clitic subject: "maledje-n ich".
                    group("Inverted", "v1 !imperative", "Mood Person Number"),
                    group("InvertedImperative", "v1 imperative", "Imperative"),
                    // malem: let us paint.
                    group("Inclusive", "inclusive", ""),
                ],
            )
            .inherent("Auxiliary")
            .citation("Infinitive PastThirdSingular ParticiplePast")
            .head_last(),
        ];
        s
    }

    fn prepare(&self, e: &mut Entry) {
        if !e.forms.iter().any(|f| has(&f.tags, "canonical")) {
            e.forms.push(Form { form: e.word.clone(), tags: vec!["canonical".into()], source: None });
        }
        if e.pos != "verb" {
            return;
        }
        let mut aux = None;
        for f in &mut e.forms {
            f.form = f.form.trim_end_matches('!').to_string();
            if has(&f.tags, "auxiliary") {
                f.form = match f.form.as_str() {
                    "han" | "hann" | "hebbe" | "höbbe" => "haane".into(),
                    "sie" | "ṣie̩" | "zeen" => "séëne".into(),
                    _ => f.form.clone(),
                };
                continue;
            }
            if f.source.is_some() {
                // "tö male" repeats the infinitive; "höbbe gemale" names
                // the auxiliary.
                if has(&f.tags, "infinitive") || (has(&f.tags, "participle") && has(&f.tags, "past")) {
                    if let Some((first, last)) = f.form.split_once(' ') {
                        if has(&f.tags, "participle") {
                            aux = Some(first.to_string());
                        }
                        f.form = last.to_string();
                    }
                }
            } else if !has(&f.tags, "alternative") && !has(&f.tags, "participle") {
                // Headword line: "léïp" (preterite), "klömt" (third person).
                if has(&f.tags, "preterite") || (has(&f.tags, "past") && !has(&f.tags, "subjunctive")) {
                    f.tags = vec!["past".into(), "singular".into(), "third-person".into()];
                } else if has(&f.tags, "third-person") {
                    f.tags = vec!["present".into(), "singular".into(), "third-person".into()];
                }
            }
        }
        if let Some(a) = aux.filter(|_| !e.forms.iter().any(|f| has(&f.tags, "auxiliary"))) {
            let a = if a == "höbbe" || a == "han" { "haane" } else { "séëne" };
            e.forms.push(Form { form: a.into(), tags: vec!["auxiliary".into()], source: None });
        }
    }

    fn samples(&self) -> Vec<&'static str> {
        vec!["book", "computer", "male", "loupe", "zeen", "good", "seek", "flómpe", "snoorig"]
    }

    fn readme(&self) -> String {
        r#"## Example

```rust
use limburgish::*;
println!("{}", noun_plural("computer")); // computers
println!("{}", noun_diminutive("computer")); // computerke
println!("{}", verb("male", Mood::Past, Person::Third, Number::Singular)); // maledje
println!("{}", verb_inverted("male", Mood::Past, Person::First, Number::Singular)); // maledje-n
println!("{}", adj_simple("good", Form::Comparative)); // baeter
```

Forms follow Wiktionary's tables, which mostly give the Venlo/Mestreech
type of spelling. Accents are optional on input.
"#
        .into()
    }
}

pub fn language() -> Box<dyn Language> {
    Box::new(Limburgish)
}
