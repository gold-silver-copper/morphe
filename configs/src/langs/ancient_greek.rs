use morphe_train::kaikki::{Entry, Form};
use morphe_train::spec::*;
use std::collections::HashMap;
use std::sync::OnceLock;

pub struct AncientGreek;

/// Precomposed polytonic letters and their plain base letters.
const UNMARK: &str = "ΐι άα έε ήη ίι ΰυ ϊι ϋυ όο ύυ ώω ἀα ἁα ἂα ἃα ἄα ἅα ἆα ἇα ἐε ἑε ἒε ἓε ἔε ἕε ἠη ἡη ἢη ἣη ἤη ἥη \
ἦη ἧη ἰι ἱι ἲι ἳι ἴι ἵι ἶι ἷι ὀο ὁο ὂο ὃο ὄο ὅο ὐυ ὑυ ὒυ ὓυ ὔυ ὕυ ὖυ ὗυ ὠω ὡω ὢω ὣω ὤω ὥω ὦω ὧω ὰα άα ὲε έε ὴη ήη \
ὶι ίι ὸο όο ὺυ ύυ ὼω ώω ᾀα ᾁα ᾂα ᾃα ᾄα ᾅα ᾆα ᾇα ᾐη ᾑη ᾒη ᾓη ᾔη ᾕη ᾖη ᾗη ᾠω ᾡω ᾢω ᾣω ᾤω ᾥω ᾦω ᾧω ᾰα ᾱα ᾲα ᾳα ᾴα \
ᾶα ᾷα ῂη ῃη ῄη ῆη ῇη ῐι ῑι ῒι ΐι ῖι ῗι ῠυ ῡυ ῢυ ΰυ ῤρ ῥρ ῦυ ῧυ ῲω ῳω ῴω ῶω ῷω";

/// Precomposed polytonic letters followed by their canonical decomposition.
/// Training on decomposed spellings lets accents, breathings and iota
/// subscripts move independently of their letters.
const DECOMPOSE: &str = "ΆΆ ΈΈ ΉΉ ΊΊ ΌΌ ΎΎ ΏΏ ΐΐ ΪΪ ΫΫ άά έέ ήή ίί ΰΰ ϊϊ ϋϋ όό ύύ ώώ ϓϓ ϔϔ ἀἀ ἁἁ \
ἂἂ ἃἃ ἄἄ ἅἅ ἆἆ ἇἇ ἈἈ ἉἉ ἊἊ ἋἋ ἌἌ ἍἍ ἎἎ ἏἏ ἐἐ ἑἑ ἒἒ ἓἓ ἔἔ ἕἕ ἘἘ \
ἙἙ ἚἚ ἛἛ ἜἜ ἝἝ ἠἠ ἡἡ ἢἢ ἣἣ ἤἤ ἥἥ ἦἦ ἧἧ ἨἨ ἩἩ ἪἪ ἫἫ ἬἬ ἭἭ ἮἮ ἯἯ \
ἰἰ ἱἱ ἲἲ ἳἳ ἴἴ ἵἵ ἶἶ ἷἷ ἸἸ ἹἹ ἺἺ ἻἻ ἼἼ ἽἽ ἾἾ ἿἿ ὀὀ ὁὁ ὂὂ ὃὃ ὄὄ \
ὅὅ ὈὈ ὉὉ ὊὊ ὋὋ ὌὌ ὍὍ ὐὐ ὑὑ ὒὒ ὓὓ ὔὔ ὕὕ ὖὖ ὗὗ ὙὙ ὛὛ ὝὝ ὟὟ ὠὠ ὡὡ \
ὢὢ ὣὣ ὤὤ ὥὥ ὦὦ ὧὧ ὨὨ ὩὩ ὪὪ ὫὫ ὬὬ ὭὭ ὮὮ ὯὯ ὰὰ άά ὲὲ έέ ὴὴ ήή ὶὶ ίί \
ὸὸ όό ὺὺ ύύ ὼὼ ώώ ᾀᾀ ᾁᾁ ᾂᾂ ᾃᾃ ᾄᾄ ᾅᾅ ᾆᾆ ᾇᾇ ᾈᾈ ᾉᾉ ᾊᾊ ᾋᾋ ᾌᾌ \
ᾍᾍ ᾎᾎ ᾏᾏ ᾐᾐ ᾑᾑ ᾒᾒ ᾓᾓ ᾔᾔ ᾕᾕ ᾖᾖ ᾗᾗ ᾘᾘ ᾙᾙ ᾚᾚ ᾛᾛ ᾜᾜ ᾝᾝ \
ᾞᾞ ᾟᾟ ᾠᾠ ᾡᾡ ᾢᾢ ᾣᾣ ᾤᾤ ᾥᾥ ᾦᾦ ᾧᾧ ᾨᾨ ᾩᾩ ᾪᾪ ᾫᾫ ᾬᾬ ᾭᾭ ᾮᾮ \
ᾯᾯ ᾰᾰ ᾱᾱ ᾲᾲ ᾳᾳ ᾴᾴ ᾶᾶ ᾷᾷ ᾸᾸ ᾹᾹ ᾺᾺ ΆΆ ᾼᾼ ιι ῂῂ ῃῃ ῄῄ ῆῆ ῇῇ ῈῈ ΈΈ ῊῊ ΉΉ \
ῌῌ ῐῐ ῑῑ ῒῒ ΐΐ ῖῖ ῗῗ ῘῘ ῙῙ ῚῚ ΊΊ ῠῠ ῡῡ ῢῢ ΰΰ ῤῤ ῥῥ ῦῦ ῧῧ ῨῨ ῩῩ ῪῪ ΎΎ \
ῬῬ ῲῲ ῳῳ ῴῴ ῶῶ ῷῷ ῸῸ ΌΌ ῺῺ ΏΏ ῼῼ";

fn decompose(s: &str) -> String {
    static TABLE: OnceLock<HashMap<char, Vec<char>>> = OnceLock::new();
    let table = TABLE.get_or_init(|| {
        DECOMPOSE
            .split_whitespace()
            .filter_map(|p| {
                let mut c = p.chars();
                Some((c.next()?, c.collect()))
            })
            .collect()
    });
    // Marks belong on vowels: a stray one on a consonant is dropped, and rho
    // keeps its breathing precomposed (ῥ), so that predicted spellings never
    // put an accent on a consonant.
    let mut out = String::with_capacity(s.len() + 8);
    let mut vowel = false;
    for c in s.chars() {
        let d = match table.get(&c) {
            Some(d) if !matches!(c, 'ῤ' | 'ῥ' | 'Ῥ') => d.clone(),
            _ => vec![c],
        };
        for c in d {
            if is_mark(c) {
                // Vowel length (ᾱ, ῠ) is marked in Wiktionary's tables but
                // not in ordinary polytonic spelling.
                if vowel && !matches!(c, '\u{304}' | '\u{306}') {
                    out.push(c);
                }
            } else {
                vowel = "αεηιουωΑΕΗΙΟΥΩ".contains(c);
                out.push(c);
            }
        }
    }
    out
}

fn is_greek(c: char) -> bool {
    matches!(c, '\u{370}'..='\u{3ff}' | '\u{1f00}'..='\u{1fff}')
}

fn is_mark(c: char) -> bool {
    matches!(c, '\u{300}'..='\u{36f}')
}

/// Tense words of a conjugation table's heading ("contracted aorist").
const HEADING_WORDS: &[&str] = &["present", "imperfect", "future", "aorist", "perfect", "pluperfect", "contracted"];

/// Article forms that some declension tables put before the noun.
const ARTICLES: &[&str] = &[
    "ὁ", "ἡ", "τό", "τὸ", "οἱ", "αἱ", "τά", "τὰ", "τᾰ́", "τᾰ̀", "τοῦ", "τῆς", "τῷ", "τῇ", "τόν", "τὸν", "τήν", "τὴν", "τοῖς",
    "ταῖς", "τούς", "τοὺς", "τάς", "τὰς", "τᾱ́ς", "τᾱ̀ς", "τώ", "τὼ", "τοῖν", "τῶν", "τᾱ̀ν", "τᾱ́ν", "τῇσῐν", "τῇσῐ", "τοῖσῐ",
    "τοῖσῐν", "τοὺ", "τοῦν", "τὸνς", "ᾱ̓", "ᾱ̔",
];

const CASES: &str = "Case[Nominative,Genitive,Dative,Accusative]";
const IMPERATIVE: &str = "PersonNumber[SecondSingular,ThirdSingular,SecondDual,ThirdDual,SecondPlural,ThirdPlural]";

impl Language for AncientGreek {
    fn spec(&self) -> Spec {
        let mut s = Spec::new("ancient_greek", "AncientGreek", "grc");
        // Accents, breathings, iota subscripts, vowel length and diaeresis are
        // optional in input.
        s.unmark = unmark(UNMARK);
        s.marks = vec!['\u{300}', '\u{301}', '\u{304}', '\u{306}', '\u{308}', '\u{313}', '\u{314}', '\u{342}', '\u{343}', '\u{345}'];
        // Forms are trained decomposed (see `prepare`).
        s.fold = vec![('ς', 'σ')];
        s.soft_tags.extend(tags("uncontracted poetic"));
        s.features = vec![
            feature(
                "Case",
                &[
                    ("Nominative", "nominative"),
                    ("Genitive", "genitive"),
                    ("Dative", "dative"),
                    ("Accusative", "accusative"),
                    ("Vocative", "vocative"),
                ],
            ),
            feature("Number", &[("Singular", "singular"), ("Dual", "dual"), ("Plural", "plural")]),
            feature("Gender", &[("Masculine", "masculine"), ("Feminine", "feminine"), ("Neuter", "neuter")]),
            feature(
                "Tense",
                &[
                    ("Present", "present"),
                    ("Imperfect", "imperfect"),
                    ("Future", "future !perfect"),
                    ("Aorist", "aorist"),
                    ("Perfect", "perfect !future"),
                    ("Pluperfect", "pluperfect"),
                    ("FuturePerfect", "future perfect"),
                ],
            ),
            // Present, imperfect, perfect and pluperfect have one middle-passive
            // form: it fills both voices.
            feature("Voice", &[("Active", "active"), ("Middle", "middle"), ("Passive", "passive")]),
            feature(
                "PersonNumber",
                &[
                    ("FirstSingular", "first-person singular"),
                    ("SecondSingular", "second-person singular"),
                    ("ThirdSingular", "third-person singular"),
                    ("SecondDual", "second-person dual"),
                    ("ThirdDual", "third-person dual"),
                    ("FirstPlural", "first-person plural"),
                    ("SecondPlural", "second-person plural"),
                    ("ThirdPlural", "third-person plural"),
                ],
            ),
        ];
        s.pos = vec![
            pos("noun", "noun", vec![group("", "", "Case Number")])
                .inherent("Gender")
                .citation("NominativeSingular GenitiveSingular")
                .head_last(),
            pos(
                "adj",
                "adj",
                vec![
                    group("", "!comparative !superlative !adverbial", "Case Gender Number"),
                    group("Comparative", "comparative", ""),
                    group("Superlative", "superlative", ""),
                    group("Adverb", "adverbial", ""),
                ],
            )
            .citation("NominativeMasculineSingular NominativeFeminineSingular NominativeNeuterSingular"),
            pos(
                "verb",
                "verb",
                vec![
                    group("Indicative", "indicative", "Tense Voice PersonNumber"),
                    group("Subjunctive", "subjunctive", "Tense[Present,Aorist,Perfect] Voice PersonNumber"),
                    group("Optative", "optative", "Tense[Present,Future,Aorist,Perfect] Voice PersonNumber"),
                    group("Imperative", "imperative", &format!("Tense[Present,Aorist,Perfect] Voice {IMPERATIVE}")),
                    group("Infinitive", "infinitive", "Tense[Present,Future,Aorist,Perfect] Voice"),
                    group("Participle", "participle masculine", "Tense[Present,Future,Aorist,Perfect] Voice").declines("adj", ""),
                ],
            )
            .citation(
                "IndicativePresentActiveFirstSingular \
                 IndicativeFutureActiveFirstSingular|IndicativeFutureMiddleFirstSingular \
                 IndicativeAoristActiveFirstSingular|IndicativeAoristMiddleFirstSingular \
                 IndicativePerfectActiveFirstSingular IndicativePerfectPassiveFirstSingular \
                 IndicativeAoristPassiveFirstSingular",
            )
            .head_first(),
            pos("pron", "pron", vec![group("", "", &format!("{CASES} Gender? Number"))]),
            pos("det", "det", vec![group("", "", &format!("{CASES} Gender? Number"))]),
            pos("article", "article", vec![group("", "", &format!("{CASES} Gender Number"))]),
            pos("num", "num", vec![group("", "", &format!("{CASES} Gender? Number"))]),
        ];
        s
    }

    /// Conjugation tables name their tense only in the heading ("aorist",
    /// "contracted present", "Epic aorist"): carry it into the forms' tags.
    /// Dialect tables (Epic, Ionic, Doric, Koine…) are dropped; Attic and
    /// unlabelled ones are kept. Where a contracted table exists, the
    /// uncontracted one becomes a low-weight alternative.
    fn prepare(&self, e: &mut Entry) {
        let mut heads: Vec<(Vec<String>, bool)> = Vec::new();
        let mut current: Option<usize> = None;
        let mut which: Vec<Option<usize>> = Vec::with_capacity(e.forms.len());
        for f in &e.forms {
            if f.tags.iter().any(|t| t == "table-tags") {
                let dialect = f.form.split_whitespace().any(|w| w.starts_with(char::is_uppercase) && w != "Attic");
                let words = f.form.split_whitespace().filter(|w| HEADING_WORDS.contains(w)).map(String::from).collect();
                heads.push((words, dialect));
                current = Some(heads.len() - 1);
            }
            which.push(if f.source.is_some() { current } else { None });
        }
        let contracted: Vec<Vec<String>> = heads
            .iter()
            .filter(|(w, d)| !d && w.iter().any(|x| x == "contracted"))
            .map(|(w, _)| w.iter().filter(|x| *x != "contracted").cloned().collect())
            .collect();
        let forms = std::mem::take(&mut e.forms);
        for (mut f, h) in forms.into_iter().zip(which) {
            if let Some((words, dialect)) = h.and_then(|i| heads.get(i)) {
                if *dialect {
                    continue;
                }
                if !words.iter().any(|x| x == "contracted") && contracted.contains(words) {
                    f.tags.push("uncontracted".into());
                }
                f.tags.extend(words.iter().cloned());
                if let Some((first, rest)) = f.form.trim().split_once(' ') {
                    if ARTICLES.contains(&first) && !rest.contains(' ') {
                        f.form = rest.to_string();
                    }
                }
                // Some tables add a romanized row.
                if f.form.chars().any(|c| c.is_alphabetic() && !is_greek(c)) {
                    continue;
                }
            }
            // ‡ marks rare emphatic forms (ἔγωγε).
            if f.form.contains('‡') {
                f.form = f.form.replace('‡', "");
                f.tags.push("rare".into());
            }
            if f.tags.iter().any(|t| t == "canonical") {
                // "αὐτός •"
                f.form = f.form.trim_end_matches(|c: char| !c.is_alphabetic() && !is_mark(c)).to_string();
            }
            f.form = decompose(&f.form);
            e.forms.push(f);
        }
        // Some entries have no canonical form: the headword keeps its accents.
        if !e.forms.iter().any(|f| f.tags.iter().any(|t| t == "canonical")) {
            e.forms.push(Form { form: decompose(&e.word), tags: vec!["canonical".into()], source: None });
        }
    }

    fn readme(&self) -> String {
        r#"## Example

```rust
use ancient_greek::*;
println!("{}", noun("λόγος", Case::Genitive, Number::Plural)); // λόγων
println!("{}", noun("logos", Case::Dative, Number::Singular)); // accents are optional
println!("{:?}", noun_gender("χώρα"));
println!("{}", verb_indicative("λύω", Tense::Aorist, Voice::Active, PersonNumber::ThirdSingular)); // ἔλυσε
println!("{}", verb_optative("λύω", Tense::Present, Voice::Middle, PersonNumber::FirstPlural));
println!("{}", verb_participle_declined("λύω", Tense::Present, Voice::Active, Case::Genitive, Gender::Feminine, Number::Plural));
println!("{}", adj("καλός", Case::Accusative, Gender::Feminine, Number::Singular));
// Citations pin principal parts: λύω, λύσω, ἔλυσα, λέλυκα, λέλυμαι, ἐλύθην.
println!("{}", verb_indicative("φέρω, οἴσω, ἤνεγκα", Tense::Aorist, Voice::Active, PersonNumber::FirstPlural));
println!("{}", AncientGreek::default().marks(false).noun("θάλασσα", Case::Genitive, Number::Singular));
```

Attic forms. Input may leave out accents, breathings, iota subscripts and
vowel-length marks. Output is in ordinary polytonic spelling, without the
ᾰ/ᾱ length marks of Wiktionary's tables, and decomposed (Unicode NFD: letter,
then its marks); `marks(false)` removes all marks. In the
present, imperfect, perfect and pluperfect the middle and passive share one
form, given for both voices. For contract verbs the contracted forms come
first.
"#
        .into()
    }
}

pub fn language() -> Box<dyn Language> {
    Box::new(AncientGreek)
}
