use morphe_train::kaikki::{Entry, Form};
use morphe_train::spec::*;

pub struct Czech;

const CASES: &str = "Case[Nominative,Genitive,Dative,Accusative,Instrumental,Locative]";

const CASE_TAGS: [&str; 7] = ["nominative", "genitive", "dative", "accusative", "instrumental", "locative", "vocative"];

fn has(tags: &[String], t: &str) -> bool {
    tags.iter().any(|x| x == t)
}

impl Language for Czech {
    fn spec(&self) -> Spec {
        let mut s = Spec::new("czech", "Czech", "cs");
        s.drop_tags.extend(tags("dated"));
        // Czech Wiktionary adds words English Wiktionary lacks.
        s.extra_dumps = vec!["cs/čeština".into()];
        s.soft_tags.extend(tags("poetic literary"));
        s.features = vec![
            feature(
                "Case",
                &[
                    ("Nominative", "nominative"),
                    ("Genitive", "genitive"),
                    ("Dative", "dative"),
                    ("Accusative", "accusative"),
                    ("Instrumental", "instrumental"),
                    ("Locative", "locative"),
                    ("Vocative", "vocative"),
                ],
            ),
            feature("Number", &[("Singular", "singular"), ("Plural", "plural")]),
            // Czech splits the masculine by animacy (tags made by `retag`).
            feature(
                "Gender",
                &[
                    ("MasculineAnimate", "masculine-animate"),
                    ("MasculineInanimate", "masculine-inanimate"),
                    ("Feminine", "feminine"),
                    ("Neuter", "neuter"),
                ],
            ),
            feature("Aspect", &[("Imperfective", "imperfective"), ("Perfective", "perfective")]),
            feature("Person", &[("First", "first-person"), ("Second", "second-person"), ("Third", "third-person")]),
            feature("Voice", &[("Active", "active"), ("Passive", "passive")]),
            feature("Tense", &[("Present", "present"), ("Past", "past")]),
            feature(
                "Agreement",
                &[
                    ("MasculineSingular", "masculine singular"),
                    ("FeminineNeuterSingular", "feminine neuter singular"),
                    ("Plural", "plural"),
                ],
            ),
        ];
        let declined = format!("{CASES} Number Gender?");
        s.pos = vec![
            pos("noun", "noun", vec![group("", "", "Case Number")])
                .inherent("Gender")
                .citation("NominativeSingular GenitiveSingular")
                .head_last(),
            pos(
                "adj",
                "adj",
                vec![
                    group("", "!comparative !superlative", &declined),
                    group("Comparative", "comparative", ""),
                    group("Superlative", "superlative", ""),
                ],
            ),
            pos(
                "verb",
                "verb",
                vec![
                    group("Infinitive", "infinitive", ""),
                    group("Present", "indicative present", "Person Number"),
                    group("Imperative", "imperative second-person", "Number"),
                    group("ImperativeFirstPlural", "imperative first-person plural", ""),
                    group("Past", "participle past !transgressive", "Number Gender"),
                    group("Passive", "participle passive !adjectival", "Number Gender"),
                    group("Transgressive", "transgressive", "Tense Agreement"),
                    group("Participle", "adjectival participle", "Voice").declines("adj", ""),
                    group("VerbalNoun", "noun-from-verb", ""),
                ],
            )
            .inherent("Aspect")
            .citation("Infinitive PresentFirstSingular PastSingularMasculineAnimate"),
            pos("pron", "pron", vec![group("", "", "Case Number Gender?")]),
            pos("det", "det", vec![group("", "", &declined)]),
            pos("num", "num", vec![group("", "", &declined)]),
        ];
        s
    }

    /// Wiktionary's Czech tables merge columns, and the merged headers lose
    /// tags: a masculine that names no animacy is both; the plural column
    /// "masculine inanimate / feminine" of the nominative comes without
    /// "plural" (and the inanimate one without "masculine"); the accusative
    /// plural "masculine / feminine" comes as feminine only. Each masculine
    /// gets one tag per animacy, so that it weighs as much as "feminine".
    fn retag(&self, _pos: &PosSpec, _form: &str, t: &mut Vec<String>) {
        let push = |t: &mut Vec<String>, s: &str| {
            if !has(t, s) {
                t.push(s.into());
            }
        };
        if has(t, "nominative") && !has(t, "singular") && !has(t, "plural") {
            push(t, "plural");
            if has(t, "inanimate") {
                push(t, "masculine");
            }
        }
        if has(t, "accusative") && has(t, "feminine") && has(t, "plural") && !has(t, "masculine") {
            push(t, "masculine");
        }
        if has(t, "masculine") {
            let both = !has(t, "animate") && !has(t, "inanimate");
            if both || has(t, "animate") {
                push(t, "masculine-animate");
            }
            if both || has(t, "inanimate") {
                push(t, "masculine-inanimate");
            }
        }
    }

    /// The bookish infinitive in -ti/-ci (dělati, péci) is listed beside the
    /// plain one without a tag; keep it as a low-weight alternative.
    /// Czech Wiktionary's tables leave out every form spelled like the
    /// headword (nominative singular, genitive singular of růže…); the past
    /// participle is "active participle"; the non-past of perfective verbs is
    /// "future"; the plural "masculine inanimate / feminine" comes as
    /// "masculine animate feminine"; imperfective verbs are "imperfect".
    fn prepare_extra(&self, _dump: &str, e: &mut Entry) {
        let word = e.word.clone();
        if e.tags.iter().any(|t| t == "form-of") || word.contains(' ') || word.starts_with(char::is_uppercase) {
            e.forms.clear();
            return;
        }
        for t in &mut e.tags {
            if t == "imperfect" {
                *t = "imperfective".into();
            }
        }
        let tagged = |f: &Form, t: &str| f.tags.iter().any(|x| x == t);
        let tags = |t: &str| t.split(' ').map(String::from).collect::<Vec<_>>();
        let noise = ["alternative", "dialectal", "obsolete", "archaic", "literary"];
        e.forms.retain(|f| !noise.iter().any(|t| tagged(f, t)) && !f.form.contains([' ', '(', '/']) && !f.form.is_empty());
        let has_form = |e: &Entry, t: &[&str]| e.forms.iter().any(|f| t.iter().all(|x| tagged(f, x)));
        match e.pos.as_str() {
            "noun" => {
                for number in ["singular", "plural"] {
                    if e.forms.iter().filter(|f| tagged(f, number)).count() < 3 {
                        continue;
                    }
                    for case in CASE_TAGS {
                        if !has_form(e, &[case, number]) {
                            e.forms.push(Form { form: word.clone(), tags: vec![case.into(), number.into()], source: None });
                        }
                    }
                }
                let g = |t: &str| e.tags.iter().any(|x| x == t);
                if g("masculine") {
                    match (g("animate"), g("inanimate")) {
                        (true, false) => e.tags.push("masculine-animate".into()),
                        (false, true) => e.tags.push("masculine-inanimate".into()),
                        _ => {}
                    }
                }
            }
            "adj" => {
                // Short forms (mlád, mláda) name no case.
                e.forms.retain(|f| CASE_TAGS.iter().any(|c| tagged(f, c)) || tagged(f, "comparative") || tagged(f, "superlative"));
                if e.forms.len() < 40 {
                    return;
                }
                for number in ["singular", "plural"] {
                    for case in CASE_TAGS {
                        for gender in ["masculine animate", "masculine inanimate", "feminine", "neuter"] {
                            let t = format!("{case} {number} {gender}");
                            let need: Vec<&str> = t.split(' ').collect();
                            if !has_form(e, &need) {
                                e.forms.push(Form { form: word.clone(), tags: tags(&t), source: None });
                            }
                        }
                    }
                }
            }
            "verb" => {
                let perfective = e.tags.iter().any(|t| t == "perfective");
                for f in &mut e.forms {
                    let has = |f: &Form, t: &str| f.tags.iter().any(|x| x == t);
                    if has(f, "future") && perfective {
                        f.tags.retain(|t| t != "future");
                        f.tags.push("present".into());
                    }
                    if has(f, "participle") && has(f, "active") {
                        f.tags.push("past".into());
                    }
                    if has(f, "transgressive") && !has(f, "present") {
                        f.tags.push("past".into());
                    }
                    if has(f, "plural") && has(f, "masculine") && has(f, "animate") && has(f, "feminine") {
                        f.tags.retain(|t| t != "animate");
                        f.tags.push("inanimate".into());
                    }
                }
                // Imperfective futures are analytic (budu dělat).
                e.forms.retain(|f| !tagged(f, "future"));
                e.forms.push(Form { form: word, tags: tags("infinitive"), source: None });
            }
            _ => e.forms.clear(),
        }
    }

    fn adjust(&self, pos: &PosSpec, _lemma: &str, forms: &mut Vec<(usize, String, bool)>) {
        if pos.name != "verb" {
            return;
        }
        let bookish = |f: &str| f.ends_with("ti") || f.ends_with("ci");
        if forms.iter().any(|(c, f, _)| *c == 0 && !bookish(f)) {
            for (c, f, soft) in forms.iter_mut() {
                if *c == 0 && bookish(f) {
                    *soft = true;
                }
            }
        }
    }

    fn readme(&self) -> String {
        r#"## Example

```rust
use czech::*;
println!("{}", noun("žena", Case::Instrumental, Number::Plural));
println!("{}", noun("hrad", Case::Locative, Number::Singular));
println!("{:?}", noun_gender("pes"));
println!("{}", verb_present("dělat", Person::Third, Number::Plural));
println!("{}", verb_past("být", Number::Plural, Gender::MasculineAnimate));
println!("{}", adj("mladý", Case::Genitive, Number::Singular, Gender::Feminine));
// Citations pin principal parts: nominative and genitive singular.
println!("{}", noun("pes, psa", Case::Dative, Number::Singular));
```

Input is matched as written: Czech diacritics (á, č, ě, ů…) are letters, not
optional marks, so they are not ignored.
"#
        .into()
    }
}

pub fn language() -> Box<dyn Language> {
    Box::new(Czech)
}
