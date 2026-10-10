use morphe_train::kaikki::{Entry, Form};
use morphe_train::spec::*;
use std::sync::OnceLock;

pub struct French;

impl Language for French {
    fn spec(&self) -> Spec {
        let mut s = Spec::new("french", "French", "fr");
        // Reflexive tables (se fier, me fie…) and compound tenses (avoir +
        // past participle) are phrases; alternative spellings are other words.
        s.drop_tags.extend(tags("reflexive multiword-construction alternative abbreviation dated"));
        s.soft_tags.extend(tags("poetic Louisiana"));
        // French Wiktionary adds words English Wiktionary lacks.
        s.extra_dumps = vec!["fr/Français".into()];
        s.features = vec![
            feature("Number", &[("Singular", "!plural"), ("Plural", "plural")]),
            feature("Gender", &[("Masculine", "masculine"), ("Feminine", "feminine")]),
            feature("Person", &[("First", "first-person"), ("Second", "second-person"), ("Third", "third-person")]),
            feature(
                "Tense",
                &[("Present", "present"), ("Imperfect", "imperfect"), ("PastHistoric", "historic past"), ("Future", "future")],
            ),
        ];
        let nominal = "!emphatic !possessive !negative";
        s.pos = vec![
            pos("noun", "noun", vec![group("", "", "Number[Plural]"), group("Counterpart", "", "Gender")])
                .inherent("Gender")
                .citation("Lemma Plural")
                .head_first(),
            pos("adj", "adj", vec![group("", "!before-vowel", "Gender? Number"), group("BeforeVowel", "before-vowel", "")]).citation("MasculineSingular FeminineSingular"),
            pos(
                "verb",
                "verb",
                vec![
                    group("Infinitive", "infinitive", ""),
                    group("PresentParticiple", "participle present", ""),
                    group("PastParticiple", "participle past", "").declines("adj", ""),
                    group("Indicative", "indicative", "Tense Person Number"),
                    group("Conditional", "conditional", "Person Number"),
                    group("Subjunctive", "subjunctive", "Tense[Present,Imperfect] Person Number"),
                    group("Imperative", "imperative", "Person[First,Second] Number"),
                ],
            )
            .citation("Infinitive PresentParticiple PastParticiple IndicativePresentFirstSingular IndicativePastHistoricFirstSingular")
            .head_first(),
            pos("pron", "pron", vec![group("", nominal, "Gender? Number")]),
            pos(
                "det",
                "det",
                vec![group("", &format!("{nominal} !before-vowel"), "Gender? Number"), group("BeforeVowel", "before-vowel", "")],
            ),
            pos("article", "article", vec![group("", nominal, "Gender? Number")]),
            pos("num", "num", vec![group("", "", "Gender? Number")]),
        ];
        s
    }

    fn keep_entry(&self, e: &Entry) -> bool {
        e.word.chars().any(char::is_alphabetic) && !e.word.chars().any(|c| c.is_ascii_digit())
    }

    /// French Wiktionary's tables: conjugated forms carry their pronoun
    /// ("que nous lisions") instead of person and number tags, imperatives
    /// come in order (lis, lisons, lisez), the passé simple is "indicative
    /// past"; noun forms list the counterpart's plural with gender tags.
    fn prepare_extra(&self, _dump: &str, e: &mut Entry) {
        let tagged = |f: &Form, t: &str| f.tags.iter().any(|x| x == t);
        let junk = |f: &Form| f.form.contains(['\\', '^', '·', '.']) || f.form.trim().is_empty();
        let word = e.word.clone();
        match e.pos.as_str() {
            "noun" | "adj" if e.tags.iter().any(|t| t == "form-of") => e.forms.clear(),
            "noun" => {
                let own = |g: &str| e.tags.iter().any(|t| t == g);
                let genders = ["masculine", "feminine"];
                let forms = std::mem::take(&mut e.forms);
                for mut f in forms {
                    if junk(&f) || ["obsolete", "rare", "archaic", "dated", "neuter"].iter().any(|t| tagged(&f, t)) {
                        continue;
                    }
                    let g: Vec<&str> = genders.iter().copied().filter(|g| tagged(&f, g)).collect();
                    let other = g.len() == 1 && !own(g[0]) && genders.iter().any(|x| own(x));
                    if tagged(&f, "plural") {
                        // The counterpart's plural (Roumaines for Roumain) has no cell.
                        if other {
                            continue;
                        }
                        f.tags = vec!["plural".into()];
                    } else if other {
                        f.tags = vec![g[0].into()];
                    } else {
                        continue;
                    }
                    e.forms.push(f);
                }
            }
            "adj" => e.forms.retain(|f| !junk(f) && !f.tags.is_empty()),
            "verb" => {
                // Pronominal verbs (s'absenter) and phrases (faire suer) are
                // left to English Wiktionary.
                if word.contains(' ') || word.starts_with("s’") || word.starts_with("s'") {
                    e.forms.clear();
                    return;
                }
                let imperatives = e.forms.iter().filter(|f| tagged(f, "imperative") && tagged(f, "present")).count();
                let mut imperative = 0;
                let forms = std::mem::take(&mut e.forms);
                for mut f in forms {
                    let compound = tagged(&f, "past") && !(tagged(&f, "indicative") || tagged(&f, "participle"));
                    if f.source.is_none() || compound || tagged(&f, "gerund") || tagged(&f, "multiword-construction") {
                        continue;
                    }
                    if tagged(&f, "indicative") && tagged(&f, "past") {
                        if tagged(&f, "anterior") {
                            continue;
                        }
                        f.tags.push("historic".into());
                    }
                    let mut text = f.form.trim().to_string();
                    if tagged(&f, "imperative") {
                        imperative += 1;
                        let pn: &[&str] = match (imperatives, imperative) {
                            (3, 1) => &["second-person"],
                            (3, 2) => &["first-person", "plural"],
                            (3, 3) => &["second-person", "plural"],
                            _ => continue,
                        };
                        f.tags.extend(pn.iter().map(|t| t.to_string()));
                    } else if tagged(&f, "indicative") || tagged(&f, "subjunctive") || tagged(&f, "conditional") {
                        for p in ["que ", "qu’", "qu'"] {
                            if let Some(rest) = text.strip_prefix(p) {
                                text = rest.to_string();
                            }
                        }
                        let pronouns: &[(&str, &[&str])] = &[
                            ("je ", &["first-person"]),
                            ("j’", &["first-person"]),
                            ("j'", &["first-person"]),
                            ("tu ", &["second-person"]),
                            ("il/elle/on ", &["third-person"]),
                            ("il ", &["third-person"]),
                            ("nous ", &["first-person", "plural"]),
                            ("vous ", &["second-person", "plural"]),
                            ("ils/elles ", &["third-person", "plural"]),
                            ("ils ", &["third-person", "plural"]),
                        ];
                        let Some((rest, pn)) = pronouns.iter().find_map(|(p, pn)| Some((text.strip_prefix(p)?, *pn))) else {
                            continue;
                        };
                        text = rest.to_string();
                        f.tags.extend(pn.iter().map(|t| t.to_string()));
                    }
                    // "pleus ou pleux": two forms.
                    for alt in text.split(" ou ") {
                        e.forms.push(Form { form: alt.trim().to_string(), ..f.clone() });
                    }
                }
            }
            _ => e.forms.clear(),
        }
    }

    fn adjust(&self, pos: &PosSpec, lemma: &str, forms: &mut Vec<(usize, String, bool)>) {
        static CELLS: OnceLock<Vec<(String, Vec<String>)>> = OnceLock::new();
        let cells = CELLS.get_or_init(|| {
            let s = self.spec();
            s.pos.iter().map(|p| (p.name.clone(), s.cells(p).into_iter().map(|c| c.name).collect())).collect()
        });
        let Some((_, names)) = cells.iter().find(|c| c.0 == pos.name) else { return };
        if pos.name == "verb" {
            // Pronominal-only verbs (s'absenter) list forms with their
            // reflexive pronoun: the verb's own forms are what's left.
            for f in forms.iter_mut() {
                for p in ["me ", "te ", "se ", "nous ", "vous ", "m'", "t'", "s'", "m’", "t’", "s’"] {
                    if let Some(rest) = f.1.strip_prefix(p) {
                        f.1 = rest.to_string();
                    }
                }
                for p in ["-toi", "-nous", "-vous"] {
                    if let Some(rest) = f.1.strip_suffix(p) {
                        f.1 = rest.to_string();
                    }
                }
            }
        }
        fill_lemma(names, lemma, forms);
    }

    fn samples(&self) -> Vec<&'static str> {
        vec!["cheval", "chien", "aller", "prendre", "beau", "rapide", "flurger", "glorbeau"]
    }

    fn readme(&self) -> String {
        r#"## Example

```rust
use french::*;
assert_eq!(noun("cheval", Number::Plural), "chevaux");
assert_eq!(noun_counterpart("chien", Gender::Feminine), "chienne");
assert_eq!(adj("beau", Gender::Feminine, Number::Singular), "belle");
assert_eq!(verb_indicative("aller", Tense::Present, Person::First, Number::Singular), "vais");
assert_eq!(verb_subjunctive("faire", Tense::Present, Person::First, Number::Plural), "fassions");
assert_eq!(verb_past_participle_declined("prendre", Gender::Feminine, Number::Plural), "prises");
println!("{:?}", noun_gender("maison"));
// Made-up words inflect by analogy:
println!("{}", verb_indicative("flurger", Tense::PastHistoric, Person::Third, Number::Plural));
```

Spelling is matched as written, accents included. Reflexive conjugations
(`se fier`) and compound tenses (`avoir` + past participle) are not
modelled: build them from the simple forms.
"#
        .into()
    }
}

/// The dictionary form is a cell Wiktionary doesn't list: the masculine
/// singular of adjectives and pronouns (or the feminine singular when the
/// lemma is feminine, `une` → `un`). Gender-invariable words (`rapide`) get
/// the lemma in both singular cells. Forms with spaces on one-word lemmas
/// are notes, not forms.
fn fill_lemma(names: &[String], lemma: &str, forms: &mut Vec<(usize, String, bool)>) {
    forms.retain(|f| f.1.chars().any(char::is_alphabetic) && (lemma.contains(' ') || !f.1.contains(' ')));
    // Entries without forms are form-of entries (`lentes`), not lemmas.
    if forms.is_empty() {
        return;
    }
    let idx = |n: &str| names.iter().position(|c| c == n);
    let Some((ms, fs, mp, fp)) = [("MasculineSingular", "FeminineSingular", "MasculinePlural", "FemininePlural"), ("Masculine", "Feminine", "", "")]
        .into_iter()
        .find_map(|(m, f, mp, fp)| Some((idx(m)?, idx(f)?, idx(mp), idx(fp))))
    else {
        return;
    };
    let set = |ci: Option<usize>, forms: &[(usize, String, bool)]| -> Vec<String> {
        let mut v: Vec<String> = forms.iter().filter(|f| Some(f.0) == ci).map(|f| f.1.clone()).collect();
        v.sort();
        v
    };
    let has = |ci: usize, forms: &[(usize, String, bool)]| forms.iter().any(|f| f.0 == ci);
    match (has(ms, forms), has(fs, forms)) {
        (false, false) => {
            // A feminine singular that differs would be listed (feliç,
            // feliços, felices).
            let invariable = set(mp, forms) == set(fp, forms) || !set(fp, forms).is_empty();
            forms.push((ms, lemma.to_string(), false));
            if invariable {
                forms.push((fs, lemma.to_string(), false));
            }
        }
        (true, false) => forms.push((fs, lemma.to_string(), false)),
        (false, true) => forms.push((ms, lemma.to_string(), false)),
        (true, true) => {}
    }
}

pub fn language() -> Box<dyn Language> {
    Box::new(French)
}
