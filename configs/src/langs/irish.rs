use morphe_train::spec::*;

pub struct Irish;

const PRONOUNS: [&str; 8] = ["mé", "tú", "sé", "sí", "sinn", "muid", "sibh", "siad"];

impl Language for Irish {
    fn spec(&self) -> Spec {
        let mut s = Spec::new("irish", "Irish", "ga");
        // Adjective plurals, relative verb forms and the radical (unmutated)
        // form all come tagged as unrecognized.
        s.drop_tags.retain(|t| t != "error-unrecognized-form");
        s.soft_tags.extend(tags("Ulster Munster Connacht Galway Cois-Fharraige"));
        s.features = vec![
            feature(
                "Case",
                &[("Nominative", "nominative"), ("Vocative", "vocative"), ("Genitive", "genitive"), ("Dative", "dative")],
            ),
            feature("Number", &[("Singular", "singular"), ("Plural", "plural")]),
            feature("Gender", &[("Masculine", "masculine"), ("Feminine", "feminine")]),
            feature("Mutation", &[("Lenition", "lenition"), ("Eclipsis", "eclipsis")]),
            feature(
                "Mood",
                &[
                    ("Present", "present !subjunctive"),
                    ("Past", "past !habitual !subjunctive"),
                    ("Habitual", "habitual past"),
                    ("Future", "future"),
                    ("Conditional", "conditional"),
                    ("Subjunctive", "present subjunctive"),
                    ("PastSubjunctive", "past subjunctive"),
                    ("Imperative", "imperative"),
                ],
            ),
            feature(
                "Subject",
                &[
                    ("FirstSingular", "first-person singular"),
                    ("SecondSingular", "second-person singular"),
                    ("ThirdSingular", "third-person singular"),
                    ("FirstPlural", "first-person plural"),
                    ("SecondPlural", "second-person plural"),
                    ("ThirdPlural", "third-person plural"),
                    ("Autonomous", "autonomous"),
                ],
            ),
        ];
        s.pos = vec![
            pos(
                "noun",
                "noun",
                vec![
                    group("GenitiveSingular", "genitive singular !definite", ""),
                    group("Plural", "nominative plural !definite", ""),
                    // With the article: an cat, an chait, leis an gcat, na gcat.
                    group("Definite", "definite", "Case[Nominative,Genitive,Dative] Number"),
                    group("", "", "Mutation"),
                ],
            )
            .inherent("Gender")
            .citation("Lemma GenitiveSingular Plural")
            .initial(2)
            .head_first(),
            pos(
                "adj",
                "adj",
                vec![
                    group("", "singular", "Case Gender"),
                    group("Plural", "plural !singular", "Case"),
                    group("Comparative", "comparative", ""),
                    group("Mutated", "", "Mutation"),
                ],
            )
            .initial(2),
            pos(
                "verb",
                "verb",
                vec![
                    group("", "!error-unrecognized-form !noun-from-verb !participle", "Mood Subject"),
                    // a chanann, a chan, a chanfaidh…
                    group("Relative", "error-unrecognized-form", "Mood[Present,Past,Habitual,Future,Conditional]"),
                    group("VerbalNoun", "noun-from-verb", ""),
                    group("Participle", "participle past", ""),
                ],
            )
            .citation("Lemma VerbalNoun")
            .initial(2)
            .head_first(),
        ];
        s
    }

    fn split_form(&self, raw: &str) -> Vec<String> {
        // Analytic verb forms come with their pronoun (canann sé) and the
        // comparative with níos: keep the inflected word.
        let raw = raw.strip_prefix("níos ").unwrap_or(raw);
        // "canann sé" is followed by a bare "sí" for the feminine.
        if PRONOUNS.contains(&raw) {
            return Vec::new();
        }
        let words: Vec<&str> = raw.split(' ').collect();
        match words.split_last() {
            Some((last, rest)) if !rest.is_empty() && PRONOUNS.contains(last) => vec![rest.join(" ")],
            _ => vec![raw.to_string()],
        }
    }

    fn readme(&self) -> String {
        r#"## Example

```rust
use irish::*;
assert_eq!(noun_genitive_singular("cat"), "cait");
println!("{}", noun_plural("bean")); // mná
println!("{}", noun_definite("bean", Case::Genitive, Number::Plural)); // na mban
println!("{}", noun("bord", Mutation::Eclipsis)); // mbord
println!("{:?}", noun_gender("fuinneog"));
println!("{}", adj("mór", Case::Nominative, Gender::Feminine)); // mhór
println!("{}", adj_mutated("mór", Mutation::Lenition)); // mhór
println!("{}", verb("can", Mood::Past, Subject::ThirdSingular)); // chan
println!("{}", verb("can", Mood::Conditional, Subject::FirstPlural)); // chanfaimis
println!("{}", verb_verbal_noun("ól"));
// Citations pin principal parts: lemma, genitive singular, plural.
println!("{}", noun_plural("leabhar, leabhair, leabhair"));
```

Initial mutations (lenition, eclipsis) are part of the forms: the past
lenites (chan), the definite genitive plural eclipses (na gcat). Analytic
verb forms are given without their pronoun (canann for canann sé); subject
pronouns are separate words. Definite forms include the article.
"#
        .into()
    }
}

pub fn language() -> Box<dyn Language> {
    Box::new(Irish)
}
