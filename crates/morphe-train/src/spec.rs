//! The language description a config provides.

use crate::kaikki::Entry;
use morphe::model::KeyFeature;

/// Which of an entry's `forms` carry inflections.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Source {
    /// Forms from the headword line (no `source` field).
    Head,
    /// Forms from inflection tables (`source` is set).
    Table,
    /// Per cell: table forms when the table has that cell, otherwise
    /// headword forms.
    Merge,
}

#[derive(Clone, Debug)]
pub struct Spec {
    /// Crate name.
    pub name: String,
    /// Human name, also the Kaikki file name ("Russian").
    pub title: String,
    pub lang_code: String,
    pub version: String,
    pub source: Source,
    /// Forms with any of these tags are ignored.
    pub drop_tags: Vec<String>,
    /// Forms with any of these tags are kept only as low-weight alternatives.
    pub soft_tags: Vec<String>,
    /// Form strings that mean "no form".
    pub sentinels: Vec<String>,
    /// Characters users don't type (stress marks).
    pub marks: Vec<char>,
    /// Marked letters users type plain (ā → a).
    pub unmark: Vec<(char, char)>,
    /// Letters users write interchangeably, for matching (ё → е).
    pub fold: Vec<(char, char)>,
    /// Tag of the form that gives the lemma's dictionary spelling.
    pub canonical_tag: Option<String>,
    pub features: Vec<Feature>,
    pub pos: Vec<PosSpec>,
}

#[derive(Clone, Debug)]
pub struct Feature {
    pub name: String,
    pub values: Vec<Value>,
}

#[derive(Clone, Debug)]
pub struct Value {
    pub name: String,
    /// Tags that must be present; a leading `!` means the tag must be absent.
    pub tags: Vec<String>,
}

#[derive(Clone, Debug)]
pub struct PosSpec {
    /// Rust-facing name: `noun`, `verb`.
    pub name: String,
    /// Kaikki `pos` values.
    pub kaikki: Vec<String>,
    pub groups: Vec<Group>,
    /// Feature names predicted for the word itself (gender, aspect).
    pub inherent: Vec<String>,
    /// Cells given by a citation, in order; the first is the lemma. Each
    /// part may name alternatives (`PresentFirstSingular|FutureFirstSingular`).
    pub citation: Vec<Vec<String>>,
    pub key: Vec<KeyFeature>,
    /// Which word of an unknown phrase inflects.
    pub head: morphe::model::Head,
}

/// A product of features, all sharing some fixed tags.
#[derive(Clone, Debug)]
pub struct Group {
    pub name: String,
    /// (feature, allowed values, loose). A loose feature (`Animacy?`) also
    /// fits forms that name none of its values.
    pub features: Vec<(String, Option<Vec<String>>, bool)>,
    pub tags: Vec<String>,
    /// The forms of this group inflect further like another part of speech's
    /// group (participles declining as adjectives): (pos, group name).
    pub declines: Option<(String, String)>,
}

impl Group {
    pub fn declines(mut self, pos: &str, group: &str) -> Self {
        self.declines = Some((pos.into(), group.into()));
        self
    }
}

/// One cell, resolved.
#[derive(Clone, Debug)]
pub struct Cell {
    pub name: String,
    pub group: usize,
    /// (feature index, value index) per group feature.
    pub values: Vec<(usize, usize)>,
    pub need: Vec<String>,
    pub forbid: Vec<String>,
    /// For loose features: (feature tags, this value's tags).
    pub loose: Vec<(Vec<String>, Vec<String>)>,
}

impl Spec {
    pub fn feature(&self, name: &str) -> Option<(usize, &Feature)> {
        self.features.iter().enumerate().find(|(_, f)| f.name == name)
    }

    /// Every cell of a part of speech, in group order.
    pub fn cells(&self, pos: &PosSpec) -> Vec<Cell> {
        let mut out = Vec::new();
        for (gi, g) in pos.groups.iter().enumerate() {
            let mut combos: Vec<Vec<(usize, usize)>> = vec![Vec::new()];
            for (fname, subset, _) in &g.features {
                let Some((fi, f)) = self.feature(fname) else { continue };
                let vals: Vec<usize> = f
                    .values
                    .iter()
                    .enumerate()
                    .filter(|(_, v)| subset.as_ref().is_none_or(|s| s.contains(&v.name)))
                    .map(|(i, _)| i)
                    .collect();
                combos = combos
                    .into_iter()
                    .flat_map(|c| {
                        vals.iter().map(move |&v| {
                            let mut c = c.clone();
                            c.push((fi, v));
                            c
                        })
                    })
                    .collect();
            }
            for combo in combos {
                let mut name = g.name.clone();
                let mut need: Vec<String> = Vec::new();
                let mut forbid = Vec::new();
                let mut loose = Vec::new();
                for t in &g.tags {
                    push_tag(t, &mut need, &mut forbid);
                }
                for (k, &(fi, vi)) in combo.iter().enumerate() {
                    let f = &self.features[fi];
                    let v = &f.values[vi];
                    name.push_str(&v.name);
                    if g.features.get(k).is_some_and(|x| x.2) {
                        let all: Vec<String> = f.values.iter().flat_map(|v| v.tags.iter().filter(|t| !t.starts_with('!')).cloned()).collect();
                        let mine: Vec<String> = v.tags.iter().filter(|t| !t.starts_with('!')).cloned().collect();
                        loose.push((all, mine));
                    } else {
                        for t in &v.tags {
                            push_tag(t, &mut need, &mut forbid);
                        }
                    }
                }
                out.push(Cell { name, group: gi, values: combo, need, forbid, loose });
            }
        }
        out
    }
}

fn push_tag(t: &str, need: &mut Vec<String>, forbid: &mut Vec<String>) {
    match t.strip_prefix('!') {
        Some(f) => forbid.push(f.to_string()),
        None => need.push(t.to_string()),
    }
}

impl Cell {
    /// How many of the form's tags this cell matched, or `None` if it doesn't fit.
    pub fn matches(&self, tags: &[String]) -> Option<usize> {
        let has = |t: &String| tags.contains(t);
        if !self.need.iter().all(has) || self.forbid.iter().any(has) {
            return None;
        }
        let mut n = self.need.len();
        for (all, mine) in &self.loose {
            if mine.iter().all(has) {
                n += mine.len();
            } else if all.iter().any(has) {
                return None;
            }
        }
        Some(n)
    }
}

/// A language: its description plus code for the quirks a description can't say.
pub trait Language: Sync {
    fn spec(&self) -> Spec;

    /// Whether an entry is used at all.
    fn keep_entry(&self, _entry: &Entry) -> bool {
        true
    }

    /// Rewrite a raw form string before it is read: strip editorial notes,
    /// split listed alternatives. The default keeps it as is.
    fn split_form(&self, raw: &str) -> Vec<String> {
        vec![raw.to_string()]
    }

    /// Adjust a lemma's extracted forms: (cell index, form, soft).
    fn adjust(&self, _pos: &PosSpec, _lemma: &str, _forms: &mut Vec<(usize, String, bool)>) {}

    /// Hand-written Rust appended to the generated crate's lib.rs.
    fn extra_code(&self) -> String {
        String::new()
    }

    /// A few words to try in the playground: real ones and made-up ones.
    fn samples(&self) -> Vec<&'static str> {
        Vec::new()
    }

    /// Extra README text (examples, notes).
    fn readme(&self) -> String {
        String::new()
    }
}

/// Builder helpers for writing configs tersely.
/// `unmark("āa ēe īi")`: pairs of (marked, plain) letters.
pub fn unmark(pairs: &str) -> Vec<(char, char)> {
    pairs
        .split_whitespace()
        .filter_map(|p| {
            let mut c = p.chars();
            Some((c.next()?, c.next()?))
        })
        .collect()
}

pub fn tags(s: &str) -> Vec<String> {
    s.split_whitespace().map(String::from).collect()
}

/// `feature("Case", &[("Nominative", "nominative"), ...])`
pub fn feature(name: &str, values: &[(&str, &str)]) -> Feature {
    Feature {
        name: name.into(),
        values: values.iter().map(|(n, t)| Value { name: (*n).into(), tags: tags(t) }).collect(),
    }
}

/// `group("Past", "past", "Gender Number[Singular] Animacy?")`: a name, fixed
/// tags, and features (optionally restricted to some values, or loose).
pub fn group(name: &str, fixed: &str, features: &str) -> Group {
    let features = features
        .split_whitespace()
        .map(|f| {
            let (f, loose) = match f.strip_suffix('?') {
                Some(f) => (f, true),
                None => (f, false),
            };
            match f.split_once('[') {
                Some((n, rest)) => (n.to_string(), Some(rest.trim_end_matches(']').split(',').map(String::from).collect()), loose),
                None => (f.to_string(), None, loose),
            }
        })
        .collect();
    Group { name: name.into(), features, tags: tags(fixed), declines: None }
}

pub fn pos(name: &str, kaikki: &str, groups: Vec<Group>) -> PosSpec {
    PosSpec {
        name: name.into(),
        kaikki: tags(kaikki),
        groups,
        inherent: Vec::new(),
        citation: Vec::new(),
        key: Vec::new(),
        head: morphe::model::Head::Whole,
    }
}

impl PosSpec {
    pub fn inherent(mut self, f: &str) -> Self {
        self.inherent = tags(f);
        self
    }
    pub fn citation(mut self, cells: &str) -> Self {
        self.citation = cells.split_whitespace().map(|p| p.split('|').map(String::from).collect()).collect();
        self
    }
    pub fn head_first(mut self) -> Self {
        self.head = morphe::model::Head::First;
        self
    }
    pub fn head_last(mut self) -> Self {
        self.head = morphe::model::Head::Last;
        self
    }
    pub fn syllables(mut self, vowels: &str, cap: u32) -> Self {
        self.key.push(KeyFeature::Syllables { vowels: vowels.into(), cap });
        self
    }
}

impl Spec {
    pub fn new(name: &str, title: &str, lang_code: &str) -> Spec {
        Spec {
            name: name.into(),
            title: title.into(),
            lang_code: lang_code.into(),
            version: "0.1.0".into(),
            source: Source::Merge,
            drop_tags: tags("table-tags inflection-template class romanization canonical error-unknown-tag error-unrecognized-form obsolete archaic"),
            soft_tags: tags("nonstandard dialectal colloquial informal rare humorous proscribed uncommon"),
            sentinels: vec!["-".into(), "—".into(), "–".into(), "no-table-tags".into(), "".into()],
            marks: Vec::new(),
            unmark: Vec::new(),
            fold: Vec::new(),
            canonical_tag: Some("canonical".into()),
            features: Vec::new(),
            pos: Vec::new(),
        }
    }
}
