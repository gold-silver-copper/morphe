use morphe_train::kaikki::{Entry, Form};
use morphe_train::spec::*;

pub struct Polish;

fn has(tags: &[String], t: &str) -> bool {
    tags.iter().any(|x| x == t)
}

const PERSONS: [&str; 3] = ["first-person", "second-person", "third-person"];
const CASES: [&str; 7] = ["nominative", "genitive", "dative", "accusative", "instrumental", "locative", "vocative"];

impl Language for Polish {
    fn spec(&self) -> Spec {
        let mut s = Spec::new("polish", "Polish", "pl");
        s.drop_tags.extend(tags("dated Middle"));
        s.soft_tags.extend(tags("poetic literary derogatory regional Łowicz"));
        // Polish Wiktionary adds words English Wiktionary lacks.
        s.extra_dumps = vec!["pl/językpolski".into()];
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
            // A noun's gender: the masculine splits into personal, animal and
            // inanimate.
            feature(
                "Gender",
                &[
                    ("MasculinePersonal", "masculine person"),
                    ("MasculineAnimal", "masculine animal-not-person"),
                    ("MasculineInanimate", "masculine inanimate"),
                    ("Feminine", "feminine"),
                    ("Neuter", "neuter"),
                ],
            ),
            // What adjectives, past forms and participles agree with.
            feature(
                "GenderNumber",
                &[
                    ("Masculine", "masculine singular"),
                    ("Feminine", "feminine singular"),
                    ("Neuter", "neuter singular"),
                    ("Virile", "virile plural"),
                    ("Nonvirile", "nonvirile plural"),
                ],
            ),
            // The same, for pronoun and numeral tables that may name none.
            feature(
                "Agreement",
                &[
                    ("Masculine", "masculine"),
                    ("Feminine", "feminine"),
                    ("Neuter", "neuter"),
                    ("Virile", "virile"),
                    ("Nonvirile", "nonvirile"),
                ],
            ),
            feature("Animacy", &[("Animate", "animate"), ("Inanimate", "inanimate")]),
            feature("Aspect", &[("Imperfective", "imperfective"), ("Perfective", "perfective")]),
            feature("Person", &[("First", "first-person"), ("Second", "second-person"), ("Third", "third-person")]),
            feature("Voice", &[("Active", "active"), ("Passive", "passive")]),
            feature("Tense", &[("Present", "present"), ("Future", "future"), ("Past", "past")]),
        ];
        let adjlike = "Case GenderNumber Animacy?";
        let finite = "!impersonal !masculine !feminine !neuter !virile !nonvirile";
        s.pos = vec![
            pos("noun", "noun", vec![group("", "", "Case Number")])
                .inherent("Gender")
                .citation("NominativeSingular GenitiveSingular")
                .head_last(),
            pos(
                "adj",
                "adj",
                vec![
                    group("", "!comparative !superlative", adjlike),
                    group("Comparative", "comparative", ""),
                    group("Superlative", "superlative", ""),
                ],
            ),
            pos(
                "verb",
                "verb",
                vec![
                    group("Infinitive", "infinitive", ""),
                    group("Present", &format!("present {finite}"), "Person Number"),
                    group("Future", &format!("future {finite}"), "Person Number"),
                    group("Past", "past !impersonal !conditional", "Person GenderNumber"),
                    group("Conditional", "conditional !impersonal", "Person GenderNumber"),
                    group("Imperative", "imperative", "Person Number"),
                    group("Impersonal", "impersonal !conditional", "Tense"),
                    group("ImpersonalConditional", "impersonal conditional", ""),
                    group("Participle", "adjectival participle masculine singular", "Voice").declines("adj", ""),
                    group("ContemporaryParticiple", "contemporary participle", ""),
                    group("AnteriorParticiple", "anterior participle", ""),
                    group("VerbalNoun", "noun-from-verb", ""),
                ],
            )
            .inherent("Aspect")
            .citation("Infinitive PresentFirstSingular|FutureFirstSingular PresentThirdSingular|FutureThirdSingular"),
            pos("pron", "pron", vec![group("", "", "Case Number Agreement? Animacy?")]),
            pos("det", "det", vec![group("", "", adjlike)]),
            pos("num", "num", vec![group("", "", "Case Number Agreement? Animacy?")]),
        ];
        s
    }

    /// Wiktionary's Polish tables lose tags in merged headers: the
    /// "nominative/vocative" row and the "nonvirile" column of adjective
    /// tables come back as `error-unrecognized-form`, and verb tables name
    /// only the third person. The first and second persons are read off the
    /// endings, which are regular (-m/-ę, -sz/-ś, -my, -cie; "niech …" for the
    /// first-person imperative).
    fn retag(&self, pos: &PosSpec, form: &str, t: &mut Vec<String>) {
        if let Some(i) = t.iter().position(|x| x == "error-unrecognized-form") {
            t.remove(i);
            if !CASES.iter().any(|c| has(t, c)) {
                t.push("nominative".into());
            }
            if has(t, "plural") && !has(t, "virile") {
                t.push("nonvirile".into());
            }
        }
        if pos.name != "verb" || form.is_empty() || PERSONS.iter().any(|p| has(t, p)) || has(t, "impersonal") {
            return;
        }
        let finite = ["present", "future", "past", "conditional", "imperative"].iter().any(|x| has(t, x));
        let (sg, pl) = (has(t, "singular"), has(t, "plural"));
        if !finite || sg == pl {
            return;
        }
        let first = form.split_whitespace().next().unwrap_or("");
        let person = if has(t, "imperative") {
            match (first, sg) {
                ("niech", _) => "first-person",
                (w, false) if w.ends_with("my") => "first-person",
                _ => "second-person",
            }
        } else if pl {
            if first.ends_with("my") {
                "first-person"
            } else if first.ends_with("cie") {
                "second-person"
            } else {
                return;
            }
        } else if first.ends_with("sz") || first.ends_with('ś') {
            "second-person"
        } else if first.ends_with('m') || first.ends_with('ę') {
            "first-person"
        } else {
            return;
        };
        t.push(person.into());
    }

    /// Polish Wiktionary's tables: past and conditional forms name no gender
    /// (read off the endings: -ł, -ła, -ło, -li, -ły), adjective rows run the
    /// three degrees together, participles and the verbal noun name no
    /// gender or number (the first form listed is the masculine singular),
    /// the masculine personal gender is plain "masculine", and the
    /// infinitive and the adjective's masculine nominative are missing.
    fn prepare_extra(&self, _dump: &str, e: &mut Entry) {
        let tagged = |f: &Form, t: &str| f.tags.iter().any(|x| x == t);
        let word = e.word.clone();
        if e.tags.iter().any(|t| t == "form-of") || word.contains(' ') {
            e.forms.clear();
            return;
        }
        let noise = ["dated", "obsolete", "rare", "potential", "colloquial", "literary", "depreciative", "elevatedly", "alternative", "neutral"];
        let forms = std::mem::take(&mut e.forms);
        let keep = |e: &mut Entry, f: Form| {
            for alt in f.form.split(" / ").flat_map(|x| x.split(" lub ")) {
                let alt = alt.trim();
                if !alt.is_empty() && !alt.contains(['\n', ',']) {
                    e.forms.push(Form { form: alt.to_string(), ..f.clone() });
                }
            }
        };
        match e.pos.as_str() {
            "noun" => {
                let g = |t: &str| e.tags.iter().any(|x| x == t);
                if g("masculine") && !g("feminine") && !g("neuter") {
                    match (g("animate"), g("inanimate")) {
                        (false, false) => e.tags.push("person".into()),
                        (true, false) => e.tags.push("animal-not-person".into()),
                        _ => {}
                    }
                }
                for f in forms {
                    if !f.tags.is_empty() && !noise.iter().any(|t| tagged(&f, t)) {
                        keep(e, f);
                    }
                }
                // Tables leave out the forms spelled like the headword.
                let sg = e.forms.iter().any(|f| tagged(f, "singular"));
                let missing = |e: &Entry, t: &[&str]| sg && !e.forms.iter().any(|f| t.iter().all(|x| tagged(f, x)));
                for t in [["nominative", "singular"], ["accusative", "singular"]] {
                    if missing(e, &t) {
                        e.forms.push(Form { form: word.clone(), tags: t.map(String::from).to_vec(), source: None });
                    }
                }
            }
            "adj" => {
                // The positive degree only: comparative and superlative rows
                // start with their own stems.
                let stem = |t: &str| {
                    forms.iter().find(|f| f.tags.iter().map(String::as_str).eq([t])).map(|f| f.form.trim_end_matches(['y', 'i']).to_string())
                };
                let (cmp, sup) = (stem("comparative"), stem("superlative"));
                let other = |x: &str| {
                    [&cmp, &sup].iter().any(|s| s.as_ref().is_some_and(|s| !s.is_empty() && x.starts_with(s.as_str()) && !word.starts_with(s.as_str())))
                };
                for t in ["masculine nominative singular", "masculine singular vocative", "accusative inanimate masculine singular"] {
                    e.forms.push(Form { form: word.clone(), tags: t.split(' ').map(String::from).collect(), source: None });
                }
                for mut f in forms {
                    if f.form.contains(' ') || noise.iter().any(|t| tagged(&f, t)) {
                        continue;
                    }
                    let degree = tagged(&f, "comparative") || tagged(&f, "superlative");
                    if !degree && (other(&f.form) || tagged(&f, "masculine") && tagged(&f, "singular") && tagged(&f, "nominative")) {
                        continue;
                    }
                    if tagged(&f, "plural") && tagged(&f, "masculine") {
                        f.tags.retain(|t| t != "masculine");
                        f.tags.push("virile".into());
                    }
                    keep(e, f);
                }
                // Tables leave out the forms spelled like the headword
                // (krogulczy: nominative virile krogulczy).
                if e.forms.len() > 3 {
                    for case in ["nominative", "vocative"] {
                        if !e.forms.iter().any(|f| tagged(f, case) && tagged(f, "virile")) {
                            e.forms.push(Form { form: word.clone(), tags: vec![case.into(), "virile".into(), "plural".into()], source: None });
                        }
                    }
                }
            }
            "verb" => {
                e.forms.push(Form { form: word.clone(), tags: vec!["infinitive".into()], source: None });
                let (mut active, mut passive, mut noun) = (false, false, false);
                for mut f in forms {
                    let text = f.form.trim().to_string();
                    let multi = text.contains(' ') && !(tagged(&f, "imperative") && text.starts_with("niech "));
                    if multi || text.starts_with("nie") && !word.starts_with("nie") || tagged(&f, "pluperfect") {
                        continue;
                    }
                    f.tags.retain(|t| t != "potential" && t != "rare");
                    let nonfinite = ["participle", "gerund", "impersonal", "infinitive"].iter().any(|t| tagged(&f, t));
                    if nonfinite {
                        f.tags.retain(|t| !PERSONS.contains(&t.as_str()) && t != "singular" && t != "plural");
                    }
                    if tagged(&f, "gerund") {
                        if noun {
                            continue;
                        }
                        noun = true;
                        f.tags = vec!["noun-from-verb".into()];
                    } else if tagged(&f, "participle") && !tagged(&f, "adverbial") {
                        let first = if tagged(&f, "active") { &mut active } else { &mut passive };
                        if *first {
                            continue;
                        }
                        *first = true;
                        let voice = if tagged(&f, "active") { "active" } else { "passive" };
                        f.tags = ["adjectival", "participle", "masculine", "singular", voice].map(String::from).to_vec();
                    } else if (tagged(&f, "past") || tagged(&f, "conditional")) && !nonfinite {
                        // czytałbym → czytał, czytałam → czytała, czytaliśmy → czytali.
                        let mut x = text.as_str();
                        if tagged(&f, "conditional") {
                            x = ["byśmy", "byście", "bym", "byś", "by"].iter().find_map(|s| x.strip_suffix(s)).unwrap_or(x);
                        }
                        let x = ["śmy", "ście", "em", "eś", "m", "ś"].iter().find_map(|s| x.strip_suffix(s)).unwrap_or(x);
                        let g = if x.ends_with('ł') {
                            "masculine"
                        } else if x.ends_with("ła") {
                            "feminine"
                        } else if x.ends_with("ło") {
                            "neuter"
                        } else if x.ends_with("li") {
                            "virile"
                        } else if x.ends_with("ły") {
                            "nonvirile"
                        } else {
                            continue;
                        };
                        f.tags.push(g.into());
                    }
                    f.form = text;
                    keep(e, f);
                }
            }
            _ => {}
        }
    }

    fn readme(&self) -> String {
        r#"## Example

```rust
use polish::*;
println!("{}", noun("kobieta", Case::Instrumental, Number::Plural));
println!("{}", noun("pies", Case::Genitive, Number::Singular));
println!("{:?}", noun_gender("nauczyciel"));
println!("{}", verb_present("pisać", Person::First, Number::Singular));
println!("{}", verb_past("zrobić", Person::Third, GenderNumber::Virile));
println!("{}", adj("dobry", Case::Locative, GenderNumber::Feminine, Animacy::Inanimate));
// Citations pin principal parts: nominative and genitive singular.
println!("{}", noun("kot, kota", Case::Dative, Number::Singular));
```

Input is matched as written: Polish diacritics (ą, ć, ę, ł, ń, ó, ś, ź, ż)
are letters, not optional marks.
"#
        .into()
    }
}

pub fn language() -> Box<dyn Language> {
    Box::new(Polish)
}
