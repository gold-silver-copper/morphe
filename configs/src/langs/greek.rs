use morphe_train::kaikki::{Entry, Form};
use morphe_train::spec::*;

pub struct Greek;

const CASES: &str = "Case[Nominative,Genitive,Accusative,Vocative]";

impl Language for Greek {
    fn spec(&self) -> Spec {
        let mut s = Spec::new("greek", "Greek", "el");
        s.unmark = unmark("άα έε ήη ίι όο ύυ ώω ΐϊ ΰϋ");
        // Katharevousa and other learned forms come tagged archaic; weak
        // pronoun forms are clitics, kept behind the strong ones.
        s.soft_tags.extend(tags("weak formal literary Katharevousa"));
        // Greek Wiktionary adds words English Wiktionary lacks.
        s.extra_dumps = vec!["el/Greek".into()];
        s.features = vec![
            feature(
                "Case",
                &[("Nominative", "nominative"), ("Genitive", "genitive"), ("Accusative", "accusative"), ("Vocative", "vocative")],
            ),
            feature("Number", &[("Singular", "singular"), ("Plural", "plural")]),
            feature("Gender", &[("Masculine", "masculine"), ("Feminine", "feminine"), ("Neuter", "neuter")]),
            feature("Person", &[("First", "first-person"), ("Second", "second-person"), ("Third", "third-person")]),
            // Verbs without a passive leave the voice unnamed.
            feature("Voice", &[("Active", "!passive"), ("Passive", "passive")]),
            feature(
                "Tense",
                &[
                    ("Present", "present !imperfect"),
                    ("Imperfect", "imperfect"),
                    ("Aorist", "past perfective"),
                    ("Dependent", "dependent"),
                ],
            ),
            feature("Aspect", &[("Imperfective", "imperfective"), ("Perfective", "perfective")]),
        ];
        s.pos = vec![
            pos("noun", "noun", vec![group("", "", &format!("{CASES} Number"))])
                .inherent("Gender")
                .citation("NominativeSingular GenitiveSingular")
                .head_last(),
            pos(
                "adj",
                "adj",
                vec![
                    group("", "!comparative !superlative", &format!("{CASES} Gender Number")),
                    group("Comparative", "comparative", &format!("{CASES} Gender Number")),
                ],
            )
            .citation("NominativeMasculineSingular NominativeFeminineSingular NominativeNeuterSingular")
            .head_last(),
            pos(
                "verb",
                "verb",
                vec![
                    // The future and subjunctive are θα/να + these forms.
                    group("", "indicative !future !participle", "Voice Tense Person Number"),
                    group("Imperative", "imperative", "Voice Aspect Number"),
                    group("Infinitive", "infinitive-aorist", "Voice"),
                    group("Gerund", "participle present !passive", ""),
                    group("ParticiplePresent", "participle passive present", "").declines("adj", ""),
                    group("ParticiplePerfect", "participle passive !present", "").declines("adj", ""),
                ],
            )
            .citation("ActivePresentFirstSingular|PassivePresentFirstSingular ActiveAoristFirstSingular|PassiveAoristFirstSingular")
            .head_first(),
            pos("pron", "pron det", vec![group("", "", &format!("{CASES} Gender? Number?"))]),
            pos("num", "num", vec![group("", "", &format!("{CASES} Gender? Number?"))]),
            pos("article", "article", vec![group("", "", &format!("{CASES} Gender Number"))]),
        ];
        s
    }

    /// Greek Wiktionary: a noun's gender is a form tagged only with it; a
    /// verb has one table per voice (the passive one's template ends in
    /// -μαι), its tenses carry no mood or voice, the aorist is "aorist", the
    /// imperfective imperative comes before the aorist rows and the
    /// perfective one after, and the dependent is the future without θα.
    fn prepare_extra(&self, _dump: &str, e: &mut Entry) {
        let word = e.word.clone();
        let skip = ["form-of", "participle"];
        if e.tags.iter().any(|t| skip.contains(&t.as_str())) || word.contains(' ') || word.starts_with(char::is_uppercase) {
            e.forms.clear();
            return;
        }
        let tagged = |f: &Form, t: &str| f.tags.iter().any(|x| x == t);
        let tags = |t: &str| t.split(' ').map(String::from).collect::<Vec<_>>();
        let cases = ["nominative", "genitive", "accusative", "vocative"];
        let genders = ["masculine", "feminine", "neuter"];
        let forms = std::mem::take(&mut e.forms);
        match e.pos.as_str() {
            "noun" => {
                // Adjectives filed as nouns carry gendered case forms.
                if forms.iter().any(|f| cases.iter().any(|c| tagged(f, c)) && genders.iter().any(|g| tagged(f, g))) {
                    return;
                }
                for f in forms {
                    if f.form == word && f.tags.iter().all(|t| genders.contains(&t.as_str())) {
                        e.tags.extend(f.tags.iter().cloned());
                    } else if cases.iter().any(|c| tagged(&f, c)) {
                        e.forms.push(f);
                    }
                }
            }
            "adj" => e.forms = forms.into_iter().filter(|f| cases.iter().any(|c| tagged(f, c))).collect(),
            "verb" => {
                // Deponents (έρχομαι) are left to English Wiktionary.
                if word.ends_with("μαι") {
                    return;
                }
                let mut table: Option<bool> = None;
                let mut perfective = false;
                for f in forms {
                    if tagged(&f, "inflection-template") {
                        table = Some(f.form.ends_with("μαι'"));
                        perfective = false;
                        continue;
                    }
                    let Some(passive) = table else { continue };
                    let voice = if passive { " passive" } else { "" };
                    let mut form = f.form.trim().to_string();
                    if tagged(&f, "aorist") || tagged(&f, "infinitive") {
                        perfective = true;
                    }
                    let person = ["first-person", "second-person", "third-person"].into_iter().find(|p| tagged(&f, p));
                    let number = ["singular", "plural"].into_iter().find(|n| tagged(&f, n));
                    let pn = match (person, number) {
                        (Some(p), Some(n)) => format!(" {p} {n}"),
                        _ => String::new(),
                    };
                    let t = if tagged(&f, "participle") {
                        if form.ends_with("ντας") && !passive {
                            "participle present".to_string()
                        } else if form.ends_with("μένος") {
                            "participle passive".to_string()
                        } else if form.ends_with("μενος") {
                            "participle passive present".to_string()
                        } else {
                            continue;
                        }
                    } else if tagged(&f, "infinitive") {
                        format!("infinitive-aorist{voice}")
                    } else if tagged(&f, "imperative") {
                        let aspect = if perfective { "perfective" } else { "imperfective" };
                        format!("imperative {aspect}{voice}{pn}")
                    } else if pn.is_empty() || tagged(&f, "subjunctive") || ["perfect", "pluperfect"].iter().any(|t| tagged(&f, t)) {
                        continue;
                    } else if tagged(&f, "future") {
                        // θα + dependent; the imperfective future is θα + present.
                        if tagged(&f, "imperfect") {
                            continue;
                        }
                        let Some(rest) = form.strip_prefix("θα ") else { continue };
                        form = rest.to_string();
                        format!("indicative dependent{voice}{pn}")
                    } else if tagged(&f, "aorist") {
                        format!("indicative past perfective{voice}{pn}")
                    } else if tagged(&f, "imperfect") {
                        format!("indicative imperfect{voice}{pn}")
                    } else if tagged(&f, "present") {
                        format!("indicative present{voice}{pn}")
                    } else {
                        continue;
                    };
                    if form.contains(' ') || form.is_empty() {
                        continue;
                    }
                    e.forms.push(Form { form, tags: tags(&t), source: Some("table".into()) });
                }
            }
            _ => {}
        }
    }

    fn keep_entry(&self, e: &Entry) -> bool {
        // A few Ancient Greek tables slipped into the Modern Greek dump.
        !e.forms.iter().any(|f| f.tags.iter().any(|t| t == "inflection-template") && f.form.starts_with("grc"))
    }

    fn split_form(&self, raw: &str) -> Vec<String> {
        // Verb tables carry editorial notes: "μιλάς (μιλείς → ομιλώ)",
        // "{γραμμένος", "θέλουν²", "a - [b]", "‑η" suffix stubs. Keep the
        // plain words; bracketed forms are learned variants, left out.
        let head = raw.split('(').next().unwrap_or("");
        head.split([',', '-'])
            .map(|p| p.chars().filter(|c| !"{}|^➤¹²³⁴".contains(*c)).collect::<String>().trim().to_string())
            .filter(|w| {
                !w.is_empty()
                    && w.chars().all(|c| (c.is_alphabetic() && !c.is_ascii()) || c == ' ' || ('\u{300}'..='\u{36f}').contains(&c))
            })
            .collect()
    }

    fn readme(&self) -> String {
        r#"## Example

```rust
use greek::*;
assert_eq!(noun("δρόμος", Case::Genitive, Number::Plural), "δρόμων");
println!("{:?}", noun_gender("θάλασσα"));
println!("{}", adj("καλός", Case::Accusative, Gender::Feminine, Number::Plural));
println!("{}", verb("γράφω", Voice::Active, Tense::Aorist, Person::Third, Number::Plural));
println!("{}", verb_imperative("γράφω", Voice::Active, Aspect::Perfective, Number::Plural));
// Accents are optional on input:
println!("{}", noun("θαλασσα", Case::Genitive, Number::Singular));
// Citations pin principal parts: nominative and genitive singular.
println!("{}", noun("γάλα, γάλακτος", Case::Genitive, Number::Plural));
```

Verbs give the synthetic forms; the future (θα + present or dependent), the
subjunctive (να + the same) and the perfect tenses (έχω + infinitive) are
built from them. Input matching ignores the tonos accent.
"#
        .into()
    }
}

pub fn language() -> Box<dyn Language> {
    Box::new(Greek)
}
