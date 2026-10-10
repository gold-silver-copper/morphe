use morphe_train::kaikki::Entry;
use morphe_train::spec::*;

pub struct OldEnglish;

impl Language for OldEnglish {
    fn spec(&self) -> Spec {
        let mut s = Spec::new("old_english", "OldEnglish", "ang");
        // Editorial vowel length and palatal dots (ā, ǣ, ċ, ġ) are optional
        // in input.
        s.unmark = unmark("āa ēe īi ōo ūu ȳy ǣæ ǽæ ċc ġg áa ée íi óo úu ýy");
        s.marks = vec!['\u{304}', '\u{306}', '\u{307}', '\u{301}'];
        // Wynn is w; eth and thorn are written interchangeably.
        s.fold = vec![('ƿ', 'w'), ('ð', 'þ')];
        s.features = vec![
            feature(
                "Case",
                &[
                    ("Nominative", "nominative"),
                    ("Accusative", "accusative"),
                    ("Genitive", "genitive"),
                    ("Dative", "dative"),
                    ("Instrumental", "instrumental"),
                ],
            ),
            feature("Number", &[("Singular", "singular"), ("Plural", "plural")]),
            feature("Gender", &[("Masculine", "masculine"), ("Feminine", "feminine"), ("Neuter", "neuter")]),
            feature("Tense", &[("Present", "present"), ("Past", "past")]),
            feature(
                "PersonNumber",
                &[
                    ("FirstSingular", "first-person singular"),
                    ("SecondSingular", "second-person singular"),
                    ("ThirdSingular", "third-person singular"),
                    ("Plural", "plural"),
                ],
            ),
        ];
        let cases = "Case[Nominative,Accusative,Genitive,Dative]";
        s.pos = vec![
            pos("noun", "noun", vec![group("", "", &format!("{cases} Number"))])
                .inherent("Gender")
                .citation("NominativeSingular GenitiveSingular NominativePlural")
                .head_last(),
            pos(
                "adj",
                "adj",
                vec![
                    group("", "strong", "Case Gender Number"),
                    group("Weak", "weak", "Case Gender Number"),
                    group("Comparative", "comparative", ""),
                    group("Superlative", "superlative", ""),
                ],
            ),
            pos(
                "verb",
                "verb",
                vec![
                    group("Infinitive", "infinitive !inflected", ""),
                    group("InflectedInfinitive", "infinitive inflected", ""),
                    group("", "indicative", "Tense PersonNumber"),
                    group("Subjunctive", "subjunctive", "Tense Number"),
                    group("Imperative", "imperative", "Number"),
                    group("Participle", "participle", "Tense").declines("adj", ""),
                ],
            )
            .citation("Infinitive PastFirstSingular PastPlural ParticiplePast")
            .head_first(),
            pos("pron", "pron", vec![group("", "", "Case Gender? Number?")]),
            pos("det", "det", vec![group("", "", "Case Gender? Number")]),
            pos("num", "num", vec![group("", "", "Case Gender? Number?")]),
        ];
        s
    }

    fn keep_entry(&self, e: &Entry) -> bool {
        // Runic headwords.
        !e.word.chars().any(|c| ('\u{16a0}'..='\u{16ff}').contains(&c))
    }

    /// Kaikki's Old English tables need help:
    /// - conjugation tables come with scrambled tags: the subjunctive repeats
    ///   the indicative's person-less tags, the imperative is "imperative
    ///   past present singular", and the participles are "imperative
    ///   present"/"imperative past". The table layout is fixed, so the forms
    ///   are retagged from it;
    /// - adjective tables say strong or weak only in their heading;
    /// - every personal-pronoun page repeats the whole personal-pronoun table;
    /// - a noun's gender is in its declension template (ang-decl-noun-a-m).
    fn prepare(&self, e: &mut Entry) {
        let has = |tags: &[String], t: &str| tags.iter().any(|x| x == t);
        if e.pos == "pron" {
            e.forms.retain(|f| !has(&f.tags, "personal"));
        }
        let gender = e
            .forms
            .iter()
            .filter(|f| has(&f.tags, "inflection-template"))
            .find_map(|f| match f.form.rsplit('-').next() {
                Some("m") => Some("masculine"),
                Some("f") => Some("feminine"),
                Some("n") => Some("neuter"),
                _ => None,
            });
        let mut heading: Option<String> = None;
        let mut subjunctive = false;
        for f in &mut e.forms {
            if f.source.is_none() {
                if e.pos == "noun" && has(&f.tags, "canonical") {
                    if let Some(g) = gender {
                        if !f.tags.iter().any(|t| matches!(t.as_str(), "masculine" | "feminine" | "neuter")) {
                            f.tags.push(g.into());
                        }
                    }
                }
                continue;
            }
            if has(&f.tags, "table-tags") {
                heading = Some(f.form.clone());
                subjunctive = false;
                continue;
            }
            if has(&f.tags, "inflection-template") || has(&f.tags, "class") {
                continue;
            }
            if e.pos != "verb" {
                if let Some(h) = heading.as_deref().filter(|h| matches!(*h, "strong" | "weak")) {
                    f.tags.push(h.into());
                }
                continue;
            }
            let t = &f.tags;
            let tense = if has(t, "past") { "past" } else { "present" };
            let number = if has(t, "plural") { "plural" } else { "singular" };
            let person = t.iter().find(|x| x.ends_with("-person")).cloned();
            let new: Vec<String> = if has(t, "infinitive") {
                if f.form.ends_with("nne") { vec!["infinitive".into(), "inflected".into()] } else { vec!["infinitive".into()] }
            } else if has(t, "imperative") {
                if has(t, "singular") || has(t, "plural") {
                    vec!["imperative".into(), number.into()]
                } else {
                    vec!["participle".into(), tense.into()]
                }
            } else if let Some(p) = person {
                vec!["indicative".into(), tense.into(), p, number.into()]
            } else {
                // Person-less rows: the indicative plural, then the
                // subjunctive (singular first).
                if number == "singular" {
                    subjunctive = true;
                }
                let mood = if subjunctive { "subjunctive" } else { "indicative" };
                vec![mood.into(), tense.into(), number.into()]
            };
            f.tags = new;
        }
    }

    fn readme(&self) -> String {
        r#"## Example

```rust
use old_english::*;
println!("{}", noun("stān", Case::Genitive, Number::Plural)); // stāna
println!("{}", noun("word", Case::Dative, Number::Singular)); // worde
println!("{:?}", noun_gender("giefu"));
println!("{}", verb("beran", Tense::Past, PersonNumber::FirstSingular)); // bær
println!("{}", verb_subjunctive("lufian", Tense::Present, Number::Plural)); // lufiġen
println!("{}", verb_participle("singan", Tense::Past)); // sungen
println!("{}", adj("gōd", Case::Dative, Gender::Masculine, Number::Singular)); // gōdum
println!("{}", adj_weak("gōd", Case::Nominative, Gender::Feminine, Number::Singular)); // gōde
// Citations pin principal parts: infinitive, past singular, past plural, past participle.
println!("{}", verb("drincan, dranc, druncon, druncen", Tense::Past, PersonNumber::Plural));
```

West Saxon spellings as Wiktionary normalizes them. Input may leave out the
editorial macrons and the dots of ċ and ġ; ƿ matches w and ð matches þ.
Output carries the marks (`marks(false)` removes them). The indicative has
one plural form for all persons, hence `PersonNumber`.
"#
        .into()
    }
}

pub fn language() -> Box<dyn Language> {
    Box::new(OldEnglish)
}
