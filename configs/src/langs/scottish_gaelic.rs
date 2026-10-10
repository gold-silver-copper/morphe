use morphe_train::kaikki::{Entry, Form};
use morphe_train::spec::*;

pub struct ScottishGaelic;

const PRONOUNS: &[&str] = &["mi", "thu", "tu", "e", "i", "sinn", "sibh", "iad"];
/// Articles a declension table writes in front of definite forms.
const ARTICLES: &[&str] = &["(a')", "an", "am", "a'", "a", "na", "nan", "nam"];

fn has(tags: &[String], t: &str) -> bool {
    tags.iter().any(|x| x == t)
}

impl Language for ScottishGaelic {
    fn spec(&self) -> Spec {
        let mut s = Spec::new("scottish_gaelic", "ScottishGaelic", "gd");
        s.unmark = unmark("àa èe ìi òo ùu áa ée íi óo úu");
        s.drop_tags.extend(tags("dated"));
        s.features = vec![
            feature("Gender", &[("Masculine", "masculine"), ("Feminine", "feminine")]),
            feature("Definiteness", &[("Indefinite", "indefinite"), ("Definite", "definite")]),
            feature(
                "Case",
                &[("Nominative", "nominative"), ("Genitive", "genitive"), ("Dative", "dative"), ("Vocative", "vocative")],
            ),
            feature("Number", &[("Singular", "singular"), ("Plural", "plural")]),
            feature(
                "Mutation",
                &[
                    ("Lenition", "lenition"),
                    ("Eclipsis", "eclipsis"),
                    ("HProthesis", "prothesis-h"),
                    ("TProthesis", "prothesis-t"),
                ],
            ),
            feature("Clause", &[("Independent", "independent !dependent"), ("Dependent", "dependent")]),
            feature("Voice", &[("Personal", "personal"), ("Impersonal", "impersonal")]),
            feature(
                "Subject",
                &[
                    ("FirstSingular", "first-person singular"),
                    ("FirstPlural", "first-person plural"),
                    ("Other", "personal"),
                    ("Impersonal", "impersonal"),
                ],
            ),
            feature(
                "Person",
                &[
                    ("FirstSingular", "first-person singular"),
                    ("SecondSingular", "second-person singular"),
                    ("ThirdSingular", "third-person singular !feminine"),
                    ("ThirdSingularFeminine", "third-person singular feminine"),
                    ("FirstPlural", "first-person plural"),
                    ("SecondPlural", "second-person plural"),
                    ("ThirdPlural", "third-person plural"),
                    ("Impersonal", "impersonal"),
                ],
            ),
            feature("Emphasis", &[("Plain", "!emphatic"), ("Emphatic", "emphatic")]),
        ];
        let mutated = || group("Mutated", "mutation", "Mutation");
        let persons = "Person[FirstSingular,SecondSingular,ThirdSingular,ThirdSingularFeminine,FirstPlural,SecondPlural,ThirdPlural]";
        s.pos = vec![
            pos(
                "noun",
                "noun",
                vec![
                    group("", "!vocative !mutation", "Definiteness Case[Nominative,Genitive,Dative] Number"),
                    group("Vocative", "vocative", "Number"),
                    mutated(),
                ],
            )
            .inherent("Gender")
            .citation("Lemma IndefiniteGenitiveSingular IndefiniteNominativePlural")
            .initial(2)
            .head_first(),
            pos(
                "adj",
                "adj",
                vec![
                    group("", "!plural !mutation !comparative", "Case Gender"),
                    group("Plural", "plural", ""),
                    group("Comparative", "comparative", ""),
                    mutated(),
                ],
            )
            .initial(2),
            pos(
                "verb",
                "verb",
                vec![
                    group("Past", "past", "Clause Voice"),
                    group("Future", "future", "Clause Voice"),
                    // chanas, chanar: the relative future.
                    group("Relative", "relative", "Voice"),
                    group("Conditional", "conditional", "Clause Subject"),
                    group("Imperative", "imperative", "Person"),
                    group("VerbalNoun", "noun-from-verb", ""),
                    group("Participle", "participle", ""),
                    mutated(),
                ],
            )
            .citation("Lemma VerbalNoun")
            .initial(2)
            .head_first(),
            // Prepositions inflect for person (aig: agam, agad, aige…), and
            // Wiktionary also lists each such form as a pronoun.
            pos("prep", "prep", vec![group("", "!mutation", &format!("{persons} Emphasis"))]),
            pos("pron", "pron", vec![group("", "!mutation", &format!("{persons} Emphasis"))]),
        ];
        s
    }

    fn prepare(&self, e: &mut Entry) {
        // A word with a mutation table but no form for some mutation keeps
        // its radical there (l, n, r and sg do not lenite).
        if e.forms.iter().any(|f| f.source.as_deref() == Some("mutation")) {
            for m in ["lenition", "eclipsis", "prothesis-h", "prothesis-t"] {
                if !e.forms.iter().any(|f| has(&f.tags, m)) {
                    let tags = vec!["mutation".into(), m.into()];
                    e.forms.push(Form { form: e.word.clone(), tags, source: Some("mutation".into()) });
                }
            }
        }
        if e.pos != "verb" {
            return;
        }
        // The conjugation table's headers come out garbled: rebuild each
        // form's tags from what is left and from its place in the table.
        e.forms.retain(|f| f.source.as_deref() != Some("inflection"));
        #[derive(PartialEq)]
        enum Row {
            Other,
            Conditional,
            Imperative,
        }
        let mut row = Row::Other;
        let mut passive_seen = false;
        for f in e.forms.iter_mut() {
            if f.source.as_deref() != Some("conjugation") {
                continue;
            }
            let mut t = std::mem::take(&mut f.tags);
            if has(&t, "autonomous") {
                t.push("impersonal".into());
            }
            let keep = |names: &str| -> Vec<String> {
                names.split(' ').filter(|n| has(&t, n)).map(String::from).collect()
            };
            f.tags = if has(&t, "table-tags") || has(&t, "inflection-template") {
                t.clone()
            } else if has(&t, "noun-from-verb") {
                tags("noun-from-verb")
            } else if has(&t, "participle") {
                tags("participle")
            } else if has(&t, "imperative") {
                row = Row::Imperative;
                let mut out = tags("imperative");
                if has(&t, "dependent") {
                    passive_seen = true;
                    out.push("impersonal".into());
                } else if has(&t, "first-person") || has(&t, "second-person") || has(&t, "third-person") {
                    out.extend(keep("first-person second-person third-person singular plural"));
                } else if f.form.ends_with("ibh") {
                    out.extend(tags("second-person plural"));
                } else if !passive_seen {
                    out.extend(tags("first-person plural"));
                } else {
                    out.extend(tags("third-person singular"));
                }
                out
            } else if has(&t, "conditional") || (row == Row::Conditional && has(&t, "dependent")) {
                row = Row::Conditional;
                let mut out = tags("conditional");
                out.extend(keep("independent dependent"));
                if has(&t, "impersonal") {
                    out.push("impersonal".into());
                } else if has(&t, "first-person") {
                    out.extend(keep("first-person singular plural"));
                } else {
                    out.push("personal".into());
                }
                out
            } else if has(&t, "past") || has(&t, "future") {
                row = Row::Other;
                keep("past future independent dependent personal impersonal")
            } else if has(&t, "error-unrecognized-form") && has(&t, "independent") {
                let mut out = tags("relative");
                out.extend(keep("personal impersonal"));
                out
            } else {
                Vec::new()
            };
        }
    }

    fn retag(&self, pos: &PosSpec, _form: &str, tags: &mut Vec<String>) {
        if pos.name != "noun" || has(tags, "mutation") || has(tags, "vocative") {
            return;
        }
        // Headword forms (cait, genitive and plural) are indefinite.
        let case = ["nominative", "genitive", "dative"].iter().any(|c| has(tags, c));
        if (case || has(tags, "plural")) && !has(tags, "definite") && !has(tags, "indefinite") {
            tags.push("indefinite".into());
        }
        if has(tags, "plural") && !case {
            tags.push("nominative".into());
        }
        if has(tags, "genitive") && !has(tags, "plural") && !has(tags, "singular") {
            tags.push("singular".into());
        }
    }

    fn split_form(&self, raw: &str) -> Vec<String> {
        // "an cat", "(a') chait": the table's definite forms carry the article.
        let raw = match raw.split_once(' ') {
            Some((first, rest)) if ARTICLES.contains(&first) && !rest.is_empty() => rest,
            _ => raw,
        };
        // Analytic verb forms come with their pronoun (chanadh sinn).
        if PRONOUNS.contains(&raw) {
            return Vec::new();
        }
        match raw.rsplit_once(' ') {
            Some((head, last)) if PRONOUNS.contains(&last) && !head.contains(' ') => vec![head.to_string()],
            _ => vec![raw.to_string()],
        }
    }

    fn adjust(&self, _pos: &PosSpec, _lemma: &str, forms: &mut Vec<(usize, String, bool)>) {
        // Comparatives come as "nas duaile" and superlatives as "as duaile".
        for f in forms.iter_mut() {
            if let Some(rest) = f.1.strip_prefix("nas ").or_else(|| f.1.strip_prefix("as ")) {
                f.1 = rest.to_string();
            }
        }
    }

    fn samples(&self) -> Vec<&'static str> {
        vec!["cat", "bean", "cù", "can", "dèan", "mòr", "bog", "aig", "glumag", "snàrcaich"]
    }

    fn readme(&self) -> String {
        r#"## Example

```rust
use scottish_gaelic::*;
assert_eq!(noun("cat", Definiteness::Indefinite, Case::Genitive, Number::Singular), "cait");
assert_eq!(noun("cat", Definiteness::Definite, Case::Genitive, Number::Singular), "chait");
assert_eq!(noun_mutated("cat", Mutation::Lenition), "chat");
println!("{:?}", noun_gender("bean"));
assert_eq!(verb_past("can", Clause::Independent, Voice::Personal), "chan");
assert_eq!(verb_future("can", Clause::Independent, Voice::Personal), "canaidh");
assert_eq!(verb_conditional("can", Clause::Independent, Subject::FirstSingular), "chanainn");
assert_eq!(adj("bog", Case::Genitive, Gender::Feminine), "buige");
assert_eq!(prep("aig", Person::FirstSingular, Emphasis::Plain), "agam");
// Citations pin principal parts: lemma, genitive singular, plural.
println!("{}", noun("taigh, taighe, taighean", Definiteness::Indefinite, Case::Nominative, Number::Plural));
```

Definite forms are given without the article (an cat, a' chait → cat,
chait). Analytic verb forms are given without their pronoun. Comparatives
drop "nas"/"as". Accents are optional on input.
"#
        .into()
    }
}

pub fn language() -> Box<dyn Language> {
    Box::new(ScottishGaelic)
}
