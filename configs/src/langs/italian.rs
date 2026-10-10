use morphe_train::kaikki::{Entry, Form};
use morphe_train::spec::*;
use std::sync::OnceLock;

pub struct Italian;

impl Language for Italian {
    fn spec(&self) -> Spec {
        let mut s = Spec::new("italian", "Italian", "it");
        // Alternative spellings, diminutives and the like are other words.
        s.drop_tags.extend(tags(
            "auxiliary alternative abbreviation diminutive augmentative pejorative derogatory relational enclitic apocopic dated",
        ));
        s.soft_tags.extend(tags("poetic literary Traditional traditional regional Tuscany"));
        s.features = vec![
            feature("Number", &[("Singular", "!plural"), ("Plural", "plural")]),
            feature("Gender", &[("Masculine", "masculine"), ("Feminine", "feminine")]),
            feature("Person", &[("First", "first-person"), ("Second", "second-person"), ("Third", "third-person")]),
            feature(
                "Tense",
                &[("Present", "present"), ("Imperfect", "imperfect"), ("PastHistoric", "historic past"), ("Future", "future")],
            ),
        ];
        // Headword lines give principal parts without the mood tag.
        let indicative = "!subjunctive !conditional !imperative !participle !infinitive !gerund";
        let nominal = "!possessive";
        s.pos = vec![
            pos("noun", "noun", vec![group("", "", "Number[Plural]"), group("Counterpart", "!plural", "Gender")])
                .inherent("Gender")
                .citation("Lemma Plural")
                .head_first(),
            pos(
                "adj",
                "adj",
                vec![
                    group("", "!superlative !adverb", "Gender? Number"),
                    group("Superlative", "superlative", ""),
                    group("Adverb", "adverb", ""),
                ],
            ),
            pos(
                "verb",
                "verb",
                vec![
                    group("Infinitive", "infinitive", ""),
                    group("Gerund", "gerund", ""),
                    group("PresentParticiple", "participle present", "").declines("adj", ""),
                    group("PastParticiple", "participle past", "").declines("adj", ""),
                    group("Indicative", indicative, "Tense Person Number"),
                    group("Conditional", "conditional", "Person Number"),
                    group("Subjunctive", "subjunctive", "Tense[Present,Imperfect] Person Number"),
                    group("Imperative", "imperative !negative", "Person Number"),
                    group("ImperativeNegative", "imperative negative", "Person Number"),
                ],
            )
            .citation("Infinitive IndicativePresentFirstSingular IndicativePastHistoricFirstSingular PastParticiple")
            .head_first(),
            pos("pron", "pron", vec![group("", nominal, "Gender? Number")]),
            pos("det", "det", vec![group("", &format!("{nominal} !superlative"), "Gender? Number")]),
            pos("article", "article", vec![group("", "", "Gender? Number")]),
            pos("num", "num", vec![group("", "", "Number[Plural]")]),
        ];
        s
    }

    fn prepare(&self, e: &mut Entry) {
        // A conjugation table may be followed by fuller ones with
        // traditional, poetic and dialectal variants: keep the first.
        if let Some(second) = e.forms.iter().enumerate().filter(|(_, f)| f.tags.iter().any(|t| t == "table-tags")).nth(1).map(|x| x.0) {
            e.forms.truncate(second);
        }
        // The adjective table of bello/quello is tagged as unrecognized.
        if e.pos == "adj" {
            for f in &mut e.forms {
                f.tags.retain(|t| t != "error-unrecognized-form");
            }
        }
        // Invariable words (città, blu) have no plural on the headword line.
        let invariable = e.senses.iter().any(|s| s.tags.iter().any(|t| t == "invariable"));
        if invariable && (e.pos == "noun" || e.pos == "adj") && !e.forms.iter().any(|f| f.tags.iter().any(|t| t == "plural")) {
            e.forms.push(Form { form: e.word.clone(), tags: vec!["plural".into()], source: None });
        }
    }

    fn keep_entry(&self, e: &Entry) -> bool {
        e.word.chars().any(char::is_alphabetic) && !e.word.chars().any(|c| c.is_ascii_digit())
    }

    fn adjust(&self, pos: &PosSpec, lemma: &str, forms: &mut Vec<(usize, String, bool)>) {
        static CELLS: OnceLock<Vec<(String, Vec<String>)>> = OnceLock::new();
        let cells = CELLS.get_or_init(|| {
            let s = self.spec();
            s.pos.iter().map(|p| (p.name.clone(), s.cells(p).into_iter().map(|c| c.name).collect())).collect()
        });
        let Some((_, names)) = cells.iter().find(|c| c.0 == pos.name) else { return };
        if pos.name == "verb" {
            for f in forms.iter_mut() {
                f.1 = written(&f.1);
            }
        }
        fill_lemma(names, lemma, forms);
    }

    fn samples(&self) -> Vec<&'static str> {
        vec!["uomo", "città", "andare", "dare", "bello", "rosso", "sbrumare", "gattore"]
    }

    fn readme(&self) -> String {
        r#"## Example

```rust
use italian::*;
assert_eq!(noun("uomo", Number::Plural), "uomini");
assert_eq!(noun("uovo", Number::Plural), "uova");
assert_eq!(adj("bello", Gender::Feminine, Number::Plural), "belle");
assert_eq!(verb_indicative("andare", Tense::Present, Person::First, Number::Singular), "vado");
assert_eq!(verb_conditional("essere", Person::Third, Number::Singular), "sarebbe");
assert_eq!(verb_past_participle_declined("prendere", Gender::Feminine, Number::Plural), "prese");
println!("{:?}", noun_gender("mano"));
// Made-up words inflect by analogy:
println!("{}", verb_indicative("sbrumare", Tense::PastHistoric, Person::Third, Number::Singular));
```

Output is standard spelling: Wiktionary's conjugation tables mark stress and
vowel quality on every form (`dàre`, `darèbbe`); only the written final
accent is kept (`darò`, `città`).
"#
        .into()
    }
}

/// Standard spelling of a table form: Wiktionary marks the stressed vowel of
/// every form, but only a final stressed vowel is written with its accent,
/// and of monosyllables only those that need it (dà, è, può).
fn written(form: &str) -> String {
    const ACCENTED: &[&str] = &["dà", "è", "può", "più", "già", "giù", "ciò", "sì", "né", "sé", "lì", "là", "diè", "dì"];
    let chars: Vec<char> = form.chars().collect();
    let n = chars.len();
    let vowel = |c: char| "aeiouàèéìíòóùú".contains(c);
    let groups = chars.iter().enumerate().filter(|&(i, &c)| vowel(c) && (i == 0 || !chars.get(i - 1).is_some_and(|&p| vowel(p)))).count();
    let keep_final = groups > 1 || ACCENTED.contains(&form);
    chars
        .iter()
        .enumerate()
        .map(|(i, &c)| {
            if i + 1 == n && keep_final {
                return c;
            }
            match c {
                'à' => 'a',
                'è' | 'é' => 'e',
                'ì' | 'í' => 'i',
                'ò' | 'ó' => 'o',
                'ù' | 'ú' => 'u',
                _ => c,
            }
        })
        .collect()
}

/// The dictionary form is a cell Wiktionary doesn't list: the masculine
/// singular of adjectives and pronouns (or the feminine singular when the
/// lemma is feminine, `la` → `lo`). Gender-invariable words (`acclive`) get
/// the lemma in both singular cells. Forms with spaces on one-word lemmas
/// are notes, not forms.
fn fill_lemma(names: &[String], lemma: &str, forms: &mut Vec<(usize, String, bool)>) {
    forms.retain(|f| f.1.chars().any(char::is_alphabetic) && (lemma.contains(' ') || !f.1.contains(' ')));
    // Entries without forms are form-of entries (`rosse`), not lemmas.
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
            let invariable = set(mp, forms) == set(fp, forms);
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
    Box::new(Italian)
}
