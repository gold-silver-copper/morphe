use morphe_train::kaikki::{Entry, Form};
use morphe_train::spec::*;
use std::collections::HashMap;
use std::sync::OnceLock;

pub struct German;

/// Words an inflection table puts in front of a form ("die freie", "keine freien").
const ARTICLES: &[&str] = &[
    "der", "die", "das", "den", "dem", "des", "ein", "eine", "einen", "einem", "einer", "eines", "kein", "keine",
    "keinen", "keinem", "keiner", "keines",
];

/// Finite verb groups: (name, fixed tags, features). Each also gets a
/// subordinate-clause twin named `Subordinate…` (separable verbs: "sieht an" / "ansieht").
const FINITE: &[(&str, &str, &str)] = &[
    ("Present", "indicative present", "Person Number"),
    ("Preterite", "indicative preterite", "Person Number"),
    ("Subjunctive", "subjunctive-i", "Person Number"),
    ("PastSubjunctive", "subjunctive-ii", "Person Number"),
];

impl German {
    /// Cell names per part of speech, for `adjust`.
    fn names(&self, pos: &PosSpec) -> Vec<String> {
        static NAMES: OnceLock<HashMap<String, Vec<String>>> = OnceLock::new();
        NAMES
            .get_or_init(|| {
                let s = self.spec();
                s.pos.iter().map(|p| (p.name.clone(), s.cells(p).into_iter().map(|c| c.name).collect())).collect()
            })
            .get(&pos.name)
            .cloned()
            .unwrap_or_default()
    }
}

impl Language for German {
    fn spec(&self) -> Spec {
        let mut s = Spec::new("german", "German", "de");
        // Swiss spelling and ASCII input write ß as ss.
        s.fold_strs = vec![("ß".into(), "ss".into())];
        // Compound tenses are auxiliary + participle; old spellings come tagged.
        s.drop_tags.extend(tags("multiword-construction dated"));
        s.soft_tags.extend(tags("poetic regional"));
        s.inherent_form_tags = tags("auxiliary");
        // German Wiktionary adds words English Wiktionary lacks.
        s.extra_dumps = vec!["de/Deutsch".into()];
        s.features = vec![
            feature("Auxiliary", &[("Haben", "haben"), ("Sein", "sein")]),
            feature(
                "Case",
                &[("Nominative", "nominative"), ("Genitive", "genitive"), ("Dative", "dative"), ("Accusative", "accusative")],
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
            feature(
                "Degree",
                &[("Positive", "!comparative !superlative"), ("Comparative", "comparative"), ("Superlative", "superlative")],
            ),
            feature("Person", &[("First", "first-person"), ("Second", "second-person"), ("Third", "third-person")]),
        ];
        let adj = vec![
            group("Strong", "strong !predicative", "Degree Case GenderNumber"),
            group("Weak", "weak !predicative", "Degree Case GenderNumber"),
            group("Mixed", "mixed !predicative", "Degree Case GenderNumber"),
            group("Predicative", "!nominative !genitive !dative !accusative", "Degree[Comparative,Superlative]"),
        ];
        let mut verb = vec![
            group("Infinitive", "infinitive !infinitive-zu", ""),
            group("ZuInfinitive", "infinitive-zu", ""),
            group("ParticiplePresent", "participle present", ""),
            group("ParticiplePast", "participle past", ""),
            group("Imperative", "imperative !subordinate-clause", "Number"),
        ];
        for (name, fixed, feats) in FINITE {
            verb.push(group(name, &format!("{fixed} !subordinate-clause"), feats));
        }
        for (name, fixed, feats) in FINITE {
            verb.push(group(&format!("Subordinate{name}"), &format!("{fixed} subordinate-clause"), feats));
        }
        s.pos = vec![
            // Adjectival nouns (der Delegierte) keep their strong forms, which
            // match the headword (Delegierter).
            pos(
                "noun",
                "noun",
                vec![group("", "!weak !mixed", "Case Number"), group("Diminutive", "diminutive", "")],
            )
            .inherent("Gender")
            .citation("NominativeSingular GenitiveSingular NominativePlural")
            .head_last(),
            pos("adj", "adj", adj).citation("Lemma PredicativeComparative PredicativeSuperlative"),
            pos("verb", "verb", verb)
                .inherent("Auxiliary")
                .citation("Infinitive PresentThirdSingular PreteriteThirdSingular ParticiplePast")
                .head_last(),
            pos("pron", "pron", vec![group("", "!personal", "Case GenderNumber?")]),
            pos("det", "det", vec![group("", "", "Case GenderNumber")]),
            pos("article", "article", vec![group("", "", "Case GenderNumber")]),
            pos("num", "num", vec![group("", "!strong !weak !mixed", "Case Number")]),
        ];
        s
    }

    /// German Wiktionary's tables: finite forms come with their pronoun
    /// ("er/sie/es sieht an"), imperatives with "!", the preterite is "past",
    /// the zu-infinitive "extended infinitive"; compound tenses, passives,
    /// reflexive tables (whose tags are unreliable) and variants are dropped.
    fn prepare_extra(&self, _dump: &str, e: &mut Entry) {
        let has = |f: &Form, t: &str| f.tags.iter().any(|x| x == t);
        let drop = |t: &str| {
            t.starts_with(char::is_uppercase)
                || matches!(
                    t,
                    "variant" | "alternative" | "obsolete" | "archaic" | "rare" | "abbreviation" | "symbol" | "jargon"
                        | "colloquial" | "outdated" | "affective" | "augmentative" | "regional" | "usually" | "casual"
                        | "poetic" | "uncommon" | "predicative" | "perfect" | "pluperfect" | "future-i" | "future-ii"
                        | "honorific" | "gerundive" | "processual-passive" | "statal-passive" | "reflexive"
                        | "statal-reflexive"
                )
        };
        // Pronoun, determiner and numeral tables are left to English Wiktionary.
        if !matches!(e.pos.as_str(), "noun" | "adj" | "verb") {
            e.forms.clear();
            return;
        }
        // Acronyms and letters (CD-ROM, API, O).
        if e.word.chars().filter(|c| c.is_uppercase()).count() > 1 || e.word.chars().count() < 2 {
            e.forms.clear();
            return;
        }
        let verb = e.pos == "verb";
        let word = e.word.clone();
        let mut out = Vec::new();
        for mut f in std::mem::take(&mut e.forms) {
            if verb && has(&f, "auxiliary") && has(&f, "perfect") && f.source.is_none() {
                out.push(f);
                continue;
            }
            // Participles are tagged "perfect" too.
            if f.tags.iter().any(|t| drop(t) && !(t == "perfect" && has(&f, "participle"))) {
                continue;
            }
            // Wiktionary lists augmentatives and related words as "diminutives" too.
            if has(&f, "diminutive") && !(f.form.ends_with("chen") || f.form.ends_with("lein")) {
                continue;
            }
            if !verb {
                out.push(f);
                continue;
            }
            // Only the plain active table and a few headword forms are kept.
            if f.source.is_some() && !has(&f, "active") {
                continue;
            }
            f.tags.retain(|t| t != "auxiliary" && t != "verb");
            let mut text = f.form.trim().trim_end_matches('!').trim().to_string();
            let finite = has(&f, "first-person") || has(&f, "second-person") || has(&f, "third-person");
            if finite {
                let Some((_, rest)) = text.split_once(' ') else { continue };
                text = rest.to_string();
            }
            for t in &mut f.tags {
                match t.as_str() {
                    "past" => *t = "preterite".into(),
                    "extended" => *t = "infinitive-zu".into(),
                    "participle-2" => *t = "participle".into(),
                    _ => {}
                }
            }
            if has(&f, "participle") && !has(&f, "present") {
                f.tags.retain(|t| t != "perfect" && t != "passive");
                f.tags.push("past".into());
            }
            if has(&f, "subjunctive-ii") {
                f.tags.retain(|t| t != "preterite");
            }
            let words = text.split(' ').count();
            if has(&f, "infinitive-zu") && words > 2 {
                continue;
            }
            // Separable verbs: "sieht an" in main clauses, "ansieht" in subordinate ones.
            if let Some((stem, particle)) = text.rsplit_once(' ') {
                if finite && !stem.contains(' ') && word.starts_with(particle) && !has(&f, "imperative") {
                    let mut sub = f.clone();
                    sub.form = format!("{particle}{stem}");
                    sub.tags.push("subordinate-clause".into());
                    out.push(sub);
                }
            }
            f.form = text;
            out.push(f);
        }
        e.forms = out;
    }

    fn adjust(&self, pos: &PosSpec, lemma: &str, forms: &mut Vec<(usize, String, bool)>) {
        // Notes that leaked into the article tables ("older also: daß", "also der").
        forms.retain(|f| !f.1.contains(':'));
        for f in forms.iter_mut() {
            if let Some(rest) = f.1.strip_prefix("also ") {
                f.1 = rest.to_string();
            }
        }
        // Tables spell weak and mixed adjective forms with their article.
        for f in forms.iter_mut() {
            if let Some((first, rest)) = f.1.split_once(' ') {
                if ARTICLES.contains(&first) && !rest.is_empty() {
                    f.1 = rest.to_string();
                }
            }
        }
        if pos.name != "verb" || forms.is_empty() {
            return;
        }
        // Only separable verbs have their own subordinate-clause forms; the
        // others use the main-clause forms. Without a table form, the
        // zu-infinitive is "zu" plus the infinitive.
        let names = self.names(pos);
        let index = |n: &str| names.iter().position(|x| x == n);
        let mut extra = Vec::new();
        for (ci, name) in names.iter().enumerate() {
            let source = match name.strip_prefix("Subordinate") {
                Some(main) => index(main),
                None => continue,
            };
            if let Some(src) = source {
                if !forms.iter().any(|f| f.0 == ci) {
                    extra.extend(forms.iter().filter(|f| f.0 == src).map(|f| (ci, f.1.clone(), f.2)));
                }
            }
        }
        if let (Some(zu), Some(inf)) = (index("ZuInfinitive"), index("Infinitive")) {
            if !forms.iter().any(|f| f.0 == zu) {
                let base = forms.iter().find(|f| f.0 == inf).map_or(lemma.to_string(), |f| f.1.clone());
                extra.push((zu, format!("zu {base}"), false));
            }
        }
        forms.extend(extra);
    }

    fn samples(&self) -> Vec<&'static str> {
        vec!["Haus", "sehen", "ansehen", "gut", "Delegierter", "schnurpeln", "Klabuster", "schnelzig"]
    }

    fn readme(&self) -> String {
        r#"## Example

```rust
use german::*;
assert_eq!(noun("Haus", Case::Dative, Number::Plural), "Häusern");
println!("{:?}", noun_gender("Haus"));
assert_eq!(verb_present("sehen", Person::Third, Number::Singular), "sieht");
// Separable verbs split in main clauses and join in subordinate ones:
assert_eq!(verb_present("ansehen", Person::Third, Number::Singular), "sieht an");
assert_eq!(verb_subordinate_present("ansehen", Person::Third, Number::Singular), "ansieht");
assert_eq!(adj_weak("alt", Degree::Comparative, Case::Dative, GenderNumber::Plural), "älteren");
assert_eq!(adj_predicative("gut", Degree::Superlative), "am besten");
// Citations pin principal parts: infinitive, present, preterite, participle.
assert_eq!(verb_preterite("backen, bäckt, buk, gebacken", Person::Third, Number::Singular), "buk");
```

Weak and mixed adjective forms are given without their article ("freien",
not "den freien"). Compound tenses (perfect, future) are left to the caller:
auxiliary plus `verb_participle_past`. Personal pronouns are not covered.
"#
        .into()
    }
}

pub fn language() -> Box<dyn Language> {
    Box::new(German)
}
