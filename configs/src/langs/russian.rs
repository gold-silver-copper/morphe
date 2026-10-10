use morphe_train::kaikki::Entry;
use morphe_train::spec::*;

pub struct Russian;

const CASES: &str = "Case[Nominative,Genitive,Dative,Accusative,Instrumental,Prepositional]";

impl Language for Russian {
    fn spec(&self) -> Spec {
        let mut s = Spec::new("russian", "Russian", "ru");
        s.marks = vec!['\u{301}', '\u{300}'];
        s.fold = vec![('ё', 'е')];
        // Pre-reform spellings come as whole "dated" tables.
        s.drop_tags.extend(tags("dated"));
        // Russian Wiktionary adds words English Wiktionary lacks.
        s.extra_dumps = vec!["ru/Русский".into()];
        s.soft_tags.extend(tags("poetic"));
        // Prepositional pronoun forms come with their preposition (о вас).
        s.strip_words = tags("о об обо");
        s.features = vec![
            feature(
                "Case",
                &[
                    ("Nominative", "nominative"),
                    ("Genitive", "genitive !partitive"),
                    ("Dative", "dative"),
                    ("Accusative", "accusative"),
                    ("Instrumental", "instrumental"),
                    ("Prepositional", "prepositional !locative"),
                    ("Partitive", "partitive"),
                    ("Locative", "locative"),
                    ("Vocative", "vocative"),
                ],
            ),
            feature("Number", &[("Singular", "singular"), ("Plural", "plural")]),
            feature("Gender", &[("Masculine", "masculine"), ("Feminine", "feminine"), ("Neuter", "neuter")]),
            feature(
                "GenderNumber",
                &[
                    ("Masculine", "masculine !plural"),
                    ("Feminine", "feminine !plural"),
                    ("Neuter", "neuter !plural"),
                    ("Plural", "plural"),
                ],
            ),
            feature("Animacy", &[("Animate", "animate"), ("Inanimate", "inanimate")]),
            feature("Aspect", &[("Imperfective", "imperfective"), ("Perfective", "perfective")]),
            feature("Tense", &[("Present", "present"), ("Future", "future"), ("Past", "past")]),
            feature("Person", &[("First", "first-person"), ("Second", "second-person"), ("Third", "third-person")]),
            feature("Voice", &[("Active", "active"), ("Passive", "passive")]),
        ];
        s.pos = vec![
            pos("noun", "noun", vec![group("", "", "Case Number")])
                .inherent("Gender Animacy")
                .citation("NominativeSingular GenitiveSingular")
                .head_last(),
            pos(
                "adj",
                "adj",
                vec![
                    group("", "!short-form !comparative !superlative", &format!("{CASES} GenderNumber Animacy?")),
                    group("Short", "short-form", "GenderNumber"),
                    group("Comparative", "comparative", ""),
                    group("Superlative", "superlative", ""),
                ],
            ),
            pos(
                "verb",
                "verb",
                vec![
                    group("Infinitive", "infinitive", ""),
                    group("", "!participle !imperative !adverbial !infinitive", "Tense[Present,Future] Person Number"),
                    group("Past", "past !participle !adverbial", "GenderNumber"),
                    group("Imperative", "imperative", "Number"),
                    group("Participle", "participle !adverbial", "Voice Tense[Present,Past]").declines("adj", ""),
                    group("Gerund", "adverbial participle", "Tense[Present,Past]"),
                ],
            )
            .inherent("Aspect")
            .citation("Infinitive PresentFirstSingular|FutureFirstSingular PresentSecondSingular|FutureSecondSingular"),
            pos("pron", "pron", vec![group("", "", &format!("{CASES} GenderNumber? Animacy?"))]),
            pos("num", "num", vec![group("", "", &format!("{CASES} Gender? Animacy?"))]),
        ];
        s
    }

    fn prepare(&self, e: &mut Entry) {
        if e.pos == "pron" {
            keep_own_person(e);
        }
    }

    /// Russian Wiktionary: its proper nouns, possessive adjectives filed as
    /// pronouns and ordinals filed as numerals don't fit (English Wiktionary
    /// already covers the real pronouns and numerals). It calls perfective
    /// verbs "perfect" and tags the stressed headword "stressed".
    fn prepare_extra(&self, _dump: &str, e: &mut Entry) {
        if matches!(e.pos.as_str(), "pron" | "num") || e.word.starts_with(char::is_uppercase) {
            e.forms.clear();
            return;
        }
        for t in &mut e.tags {
            if t == "perfect" {
                *t = "perfective".into();
            }
        }
        for f in &mut e.forms {
            if f.tags.iter().any(|t| t == "stressed") {
                f.tags.push("canonical".into());
            }
        }
        if e.pos == "verb" {
            complete_ru_verb(e);
        }
    }

    /// Russian Wiktionary tags the past plural as all three genders, and the
    /// imperative without a number (the plural ends in -те); its first-person
    /// imperatives (запаку́ем) are the future forms.
    fn retag(&self, _pos: &PosSpec, form: &str, tags: &mut Vec<String>) {
        let has = |tags: &[String], t: &str| tags.iter().any(|x| x == t);
        let numbered = has(tags, "singular") || has(tags, "plural");
        if has(tags, "masculine") && has(tags, "feminine") && has(tags, "neuter") && !numbered {
            tags.retain(|t| !matches!(t.as_str(), "masculine" | "feminine" | "neuter"));
            tags.push("plural".into());
        }
        if has(tags, "imperative") && !numbered {
            if has(tags, "first-person") {
                tags.push("obsolete".into());
            } else if form.trim_end_matches(['\u{301}', '\u{300}']).ends_with("те") {
                tags.push("plural".into());
            } else {
                tags.push("singular".into());
            }
        }
    }

    /// Some tables put the romanization next to the form (идя́, idjá), or a
    /// footnote mark after it (него́ ²). Pre-reform spellings (мнѣ, васъ)
    /// that slipped past the dated tables are dropped.
    fn split_form(&self, raw: &str) -> Vec<String> {
        let latin = raw.chars().any(|c| c.is_alphabetic() && !matches!(c, '\u{400}'..='\u{52f}'));
        if latin || raw.contains(['ѣ', 'і', 'ѳ', 'ѵ']) || raw.trim_end_matches(['¹', '²', '³', ' ']).ends_with('ъ') {
            return Vec::new();
        }
        vec![raw.trim_end_matches(['¹', '²', '³', '⁴', ' ']).to_string()]
    }

    fn extra_code(&self) -> String {
        include_str!("russian_extra.rs").into()
    }

    fn samples(&self) -> Vec<&'static str> {
        vec!["кракозябра", "путь, пути", "писать", "бузять", "кофе", "книг", "большой", "тетрадь"]
    }

    fn readme(&self) -> String {
        r#"## Example

```rust
use russian::*;
assert_eq!(noun("книга", Case::Genitive, Number::Plural), "книг");
let plain = Russian::default().marks(false);
println!("{}", plain.noun("стол", Case::Instrumental, Number::Singular));
println!("{:?}", noun_gender("тетрадь"));
println!("{}", verb("писать", Tense::Present, Person::First, Number::Singular));
// Citations pin principal parts: nominative and genitive singular.
println!("{}", noun("путь, пути", Case::Instrumental, Number::Singular));
```

Input matching ignores stress marks and treats ё as е. Output carries stress
marks where Wiktionary has them (`marks(false)` removes them). Unknown words
get their stress predicted from similar words, which is often wrong.
"#
        .into()
    }
}

/// Every personal pronoun's entry carries the whole personal-pronoun table;
/// keep only the entry's own row. Its tags don't tell я from мы or ты from
/// вы, so the plural rows are told apart by their н-/в- forms.
fn keep_own_person(e: &mut Entry) {
    let row = |f: &morphe_train::kaikki::Form| -> Option<&'static str> {
        let has = |t: &str| f.tags.iter().any(|x| x == t);
        if !has("personal") {
            return None;
        }
        let text = f.form.trim();
        Some(if has("reflexive") {
            "refl"
        } else if has("first-person") {
            if text == "мы" || text.starts_with('н') { "1p" } else { "1s" }
        } else if has("second-person") {
            if text.starts_with('в') { "2p" } else { "2s" }
        } else if has("feminine") {
            "3f"
        } else if has("masculine") && has("neuter") {
            "3mn"
        } else if has("masculine") {
            "3m"
        } else if has("neuter") {
            "3n"
        } else {
            "3p"
        })
    };
    let plain = |s: &str| s.chars().filter(|c| !matches!(c, '\u{301}' | '\u{300}')).collect::<String>();
    let Some(own) = e.forms.iter().filter(|f| plain(f.form.trim()) == e.word).find_map(row) else { return };
    let fits = |r: &str| r == own || (r == "3mn" && matches!(own, "3m" | "3n"));
    e.forms.retain(|f| row(f).is_none_or(fits));
}

/// Russian Wiktionary's conjugations leave out the infinitive, and a
/// perfective verb's missing present forms, which English Wiktionary's write
/// as "-", and an imperfective verb's analytic future. The infinitive takes its stress from the past masculine
/// (запакова́л → запакова́ть).
fn complete_ru_verb(e: &mut Entry) {
    use morphe_train::kaikki::Form;
    let plain = |s: &str| s.chars().filter(|c| !matches!(c, '\u{301}' | '\u{300}')).collect::<String>();
    let past = e
        .forms
        .iter()
        .find(|f| f.tags.iter().any(|t| t == "past") && f.tags.iter().any(|t| t == "masculine") && !f.tags.iter().any(|t| t == "participle"))
        .map(|f| f.form.trim().to_string());
    let infinitive = past
        .as_deref()
        .and_then(|p| p.strip_suffix('л'))
        .map(|stem| format!("{stem}ть"))
        .filter(|inf| plain(inf) == e.word)
        .unwrap_or_else(|| e.word.clone());
    let row = |form: &str, t: &str| Form { form: form.into(), tags: tags(t), source: Some("conjugation".into()) };
    for f in &mut e.forms {
        f.source = Some("conjugation".into());
    }
    if e.forms.iter().all(|f| !f.tags.iter().any(|t| t == "infinitive")) {
        e.forms.push(row(&infinitive, "infinitive"));
    }
    if e.tags.iter().any(|t| t == "perfective") {
        for p in ["first-person", "second-person", "third-person"] {
            for n in ["singular", "plural"] {
                e.forms.push(row("-", &format!("present {p} {n}")));
            }
        }
        for t in ["active participle present", "passive participle present", "adverbial participle present"] {
            e.forms.push(row("-", t));
        }
    } else if e.tags.iter().any(|t| t == "imperfective") {
        // The analytic future, which the table gives only as "бу́ду/бу́дешь… X".
        let aux = [("бу́ду", "first-person singular"), ("бу́дешь", "second-person singular"), ("бу́дет", "third-person singular"),
            ("бу́дем", "first-person plural"), ("бу́дете", "second-person plural"), ("бу́дут", "third-person plural")];
        for (a, t) in aux {
            e.forms.push(row(&format!("{a} {infinitive}"), &format!("future {t}")));
        }
    }
}

pub fn language() -> Box<dyn Language> {
    Box::new(Russian)
}
