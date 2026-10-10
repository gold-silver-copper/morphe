//! The trained model of one language and how it answers queries.

use crate::codec::{Reader, Writer};
use crate::edit::Edit;

const MAGIC: &str = "morphe-1";

/// A whole language.
#[derive(Clone, Debug, Default)]
pub struct Model {
    pub name: String,
    /// Characters that users don't type (stress marks): removed from input,
    /// and from output when marks are turned off.
    pub marks: Vec<char>,
    /// Marked letters users type plain (ā → a): mapped on input, and on
    /// output when marks are turned off.
    pub unmark: Vec<(char, char)>,
    /// Letter pairs that users write interchangeably (ё → е), applied only
    /// when matching input against known words.
    pub fold: Vec<(char, char)>,
    /// Spellings users write interchangeably, longer than one letter
    /// (ß → ss), applied after `fold` when matching.
    pub fold_strs: Vec<(String, String)>,
    /// Letters that marks follow in the data (vowels, for stress).
    pub mark_bases: Vec<char>,
    pub pos: Vec<Pos>,
}

/// One part of speech.
#[derive(Clone, Debug, Default)]
pub struct Pos {
    pub name: String,
    pub cells: Vec<String>,
    /// Cells given, in order, by a citation like "книга, книги". The first is
    /// the lemma itself; each part may stand for one of several cells.
    pub citation: Vec<Vec<u32>>,
    pub edits: Vec<Edit>,
    /// class → cell → variants (edit id, weight).
    pub classes: Vec<Vec<Vec<(u32, f32)>>>,
    /// Restores a typed word to its dictionary spelling (stress marks).
    /// Labels index `restores`.
    pub restore: Tree,
    pub restores: Vec<Edit>,
    /// Predicts the inflection class of a word.
    pub paradigm: Tree,
    /// Known words whose answer the trees don't give: key → readings.
    pub lexicon: Vec<(String, Vec<Reading>)>,
    /// Known words whose top prediction is exactly right (sorted keys).
    pub known: Vec<String>,
    pub inherent: Vec<Inherent>,
    /// Which word of an unknown multi-word phrase inflects.
    pub head: Head,
    /// Predicts (cell, inverse edit) from a form, for analysis. Labels
    /// index `inverse`.
    pub analysis: Tree,
    pub inverse: Vec<(u32, Edit)>,
    /// Attested forms the analysis tree misses: form key → (lemma, cell).
    pub reverse: Vec<(String, Vec<(String, u32)>)>,
}

/// One analysis of a form: part of speech, lemma, and every cell the form
/// fills for that lemma (empty when it is only the dictionary form).
#[derive(Clone, Debug, PartialEq)]
pub struct Analysis {
    pub pos: usize,
    pub lemma: String,
    pub cells: Vec<u32>,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Head {
    /// The phrase as a whole.
    #[default]
    Whole,
    First,
    Last,
}

/// One reading of a known word.
#[derive(Clone, Debug, PartialEq)]
pub struct Reading {
    /// The lowercased lemma without marks, when it differs from the key.
    pub plain: Option<String>,
    pub restore: u32,
    pub class: u32,
    pub weight: f32,
}

/// A feature the word carries itself, like a noun's gender.
#[derive(Clone, Debug, Default)]
pub struct Inherent {
    pub name: String,
    pub values: Vec<String>,
    pub tree: Tree,
    pub lexicon: Vec<(String, Vec<(u32, f32)>)>,
}

/// Extra symbols read before the word's letters.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum KeyFeature {
    /// Number of vowel groups, capped.
    Syllables { vowels: String, cap: u32 },
    /// The word's first `n` letters (for prefixing languages).
    Initial { n: u32 },
}

pub const TOKEN_BASE: u32 = 0xF0000;
pub const WORD_START: u32 = '^' as u32;
/// Initial letters are read as `INITIAL_BASE + char`, past the end of Unicode.
pub const INITIAL_BASE: u32 = 0x110000;

/// A suffix tree over the word read from its end, with a distribution over
/// labels at every node that differed enough from its ancestors to keep.
#[derive(Clone, Debug, Default)]
pub struct Tree {
    pub key: Vec<KeyFeature>,
    pub nodes: Vec<Node>,
}

#[derive(Clone, Debug, Default)]
pub struct Node {
    pub children: Vec<(u32, u32)>,
    pub dist: Vec<(u32, f32)>,
}

impl KeyFeature {
    /// The symbols this feature contributes for `word`.
    pub fn tokens(&self, word: &str) -> Vec<u32> {
        match self {
            KeyFeature::Initial { n } => {
                let mut v: Vec<u32> = word.chars().take(*n as usize).map(|c| INITIAL_BASE + u32::from(c)).collect();
                v.resize(*n as usize, INITIAL_BASE);
                v
            }
            k => vec![k.token(word)],
        }
    }

    pub fn token(&self, word: &str) -> u32 {
        match self {
            KeyFeature::Initial { .. } => INITIAL_BASE,
            KeyFeature::Syllables { vowels, cap } => {
                let mut n = 0u32;
                let mut prev = false;
                for c in word.chars() {
                    let v = vowels.contains(c);
                    if v && !prev {
                        n += 1;
                    }
                    prev = v;
                }
                TOKEN_BASE + n.min(*cap)
            }
        }
    }
}

impl Tree {
    /// The symbols the tree reads for `word`, root first.
    pub fn symbols(key: &[KeyFeature], word: &str) -> Vec<u32> {
        let mut s: Vec<u32> = key.iter().flat_map(|k| k.tokens(word)).collect();
        s.extend(word.chars().rev().map(u32::from));
        s.push(WORD_START);
        s
    }

    /// Distributions along the path for `word`, deepest first.
    pub fn path(&self, word: &str) -> Vec<&[(u32, f32)]> {
        let mut out = Vec::new();
        let mut node = 0usize;
        if let Some(n) = self.nodes.get(node) {
            out.push(n.dist.as_slice());
        }
        for sym in Tree::symbols(&self.key, word) {
            let Some(n) = self.nodes.get(node) else { break };
            let Ok(i) = n.children.binary_search_by_key(&sym, |c| c.0) else { break };
            let Some(&(_, child)) = n.children.get(i) else { break };
            node = child as usize;
            if let Some(c) = self.nodes.get(node) {
                out.push(c.dist.as_slice());
            }
        }
        out.retain(|d| !d.is_empty());
        out.reverse();
        out
    }

    /// The deepest distribution where most of the mass passes `keep`,
    /// renormalized. A node where only stragglers apply (its main classes
    /// can't apply to this word) defers to a shallower one; if none
    /// qualifies, the deepest with anything applicable is used.
    pub fn predict(&self, word: &str, mut keep: impl FnMut(u32) -> bool) -> Vec<(u32, f32)> {
        let mut fallback: Option<Vec<(u32, f32)>> = None;
        for dist in self.path(word) {
            let all: f32 = dist.iter().map(|x| x.1).sum();
            let kept: Vec<(u32, f32)> = dist.iter().copied().filter(|(l, _)| keep(*l)).collect();
            let total: f32 = kept.iter().map(|x| x.1).sum();
            if total <= 0.0 {
                continue;
            }
            let norm: Vec<(u32, f32)> = kept.into_iter().map(|(l, p)| (l, p / total)).collect();
            if total >= 0.5 * all {
                return norm;
            }
            fallback.get_or_insert(norm);
        }
        fallback.unwrap_or_default()
    }
}

/// How answers are written.
#[derive(Clone, Copy, Debug)]
pub struct Options {
    /// Keep stress and similar marks in the output.
    pub marks: bool,
}

impl Default for Options {
    fn default() -> Self {
        Options { marks: true }
    }
}

#[derive(Clone, Copy, PartialEq)]
enum Casing {
    Lower,
    Title,
    Upper,
}

/// A parsed query word: what the user typed, split into citation parts.
struct Query {
    casing: Casing,
    /// Lowercased, marks removed.
    plain: String,
    /// Lowercased as typed, marks kept (when the user typed any).
    typed: Option<String>,
    /// `plain` with letter folds: the lexicon key.
    key: String,
    /// Further citation parts, as typed.
    parts: Vec<String>,
}

impl Model {
    /// Without marks: what a user types.
    pub fn plain(&self, s: &str) -> String {
        self.strip(&s.to_lowercase())
    }

    fn strip(&self, s: &str) -> String {
        s.chars()
            .filter(|c| !self.marks.contains(c))
            .map(|c| self.unmark.iter().find(|u| u.0 == c).map_or(c, |u| u.1))
            .collect()
    }

    /// Lowercase, without marks, with letter folds: how two spellings are compared.
    pub fn key(&self, s: &str) -> String {
        let mut k: String = self
            .strip(&s.to_lowercase())
            .chars()
            .map(|c| self.fold.iter().find(|f| f.0 == c).map_or(c, |f| f.1))
            .collect();
        for (a, b) in &self.fold_strs {
            if k.contains(a.as_str()) {
                k = k.replace(a.as_str(), b);
            }
        }
        k
    }

    /// Every mark follows a letter that carries marks in the data.
    fn marks_placed(&self, s: &str) -> bool {
        let mut prev = None;
        for c in s.chars() {
            if self.marks.contains(&c) && !prev.is_some_and(|p| self.mark_bases.contains(&p)) {
                return false;
            }
            prev = Some(c);
        }
        true
    }

    fn query(&self, input: &str) -> Query {
        let mut parts = input.split(',').map(str::trim).filter(|p| !p.is_empty());
        let lemma = parts.next().unwrap_or("");
        let letters: Vec<char> = lemma.chars().filter(|c| c.is_alphabetic()).collect();
        let casing = if letters.len() > 1 && letters.iter().all(|c| c.is_uppercase()) {
            Casing::Upper
        } else if letters.first().is_some_and(|c| c.is_uppercase()) {
            Casing::Title
        } else {
            Casing::Lower
        };
        let lower = lemma.to_lowercase();
        let plain = self.strip(&lower);
        let typed = (lower != plain && self.marks_placed(&lower)).then_some(lower);
        Query { casing, key: self.key(lemma), plain, typed, parts: parts.map(String::from).collect() }
    }

    fn finish(&self, s: &str, casing: Casing, opts: Options) -> String {
        let s = if opts.marks { s.to_string() } else { self.strip(s) };
        match casing {
            Casing::Lower => s,
            Casing::Upper => s.to_uppercase(),
            Casing::Title => {
                let mut c = s.chars();
                match c.next() {
                    Some(f) => f.to_uppercase().chain(c).collect(),
                    None => s,
                }
            }
        }
    }

    /// Readings (dictionary spelling, class, weight) for a query word.
    fn readings(&self, pos: &Pos, q: &Query, cell: u32) -> Vec<(String, u32, f32)> {
        let needs = |class: u32| -> bool {
            let Some(c) = pos.classes.get(class as usize) else { return false };
            cell == u32::MAX || c.get(cell as usize).is_some_and(|v| !v.is_empty())
        };
        let cites = |canon: &str, class: u32| -> bool {
            q.parts.iter().zip(pos.citation.iter().skip(1)).all(|(part, alts)| {
                let want = self.key(part);
                alts.iter().any(|&cc| {
                    pos.classes.get(class as usize).and_then(|c| c.get(cc as usize)).is_some_and(|vs| {
                        vs.iter().any(|&(e, _)| {
                            pos.edits
                                .get(e as usize)
                                .and_then(|e| e.apply(canon))
                                .is_some_and(|f| self.key(&f) == want)
                        })
                    })
                })
            })
        };

        if let Ok(i) = pos.lexicon.binary_search_by(|e| e.0.as_str().cmp(&q.key)) {
            if let Some((_, rs)) = pos.lexicon.get(i) {
                let known: Vec<(String, u32, f32, bool)> = rs
                    .iter()
                    .filter_map(|r| {
                        let plain = r.plain.as_deref().unwrap_or(&q.key);
                        let canon = pos.restores.get(r.restore as usize)?.apply(plain)?;
                        Some((canon, r.class, r.weight, plain == q.plain))
                    })
                    .filter(|(canon, class, _, _)| cites(canon, *class))
                    .collect();
                // The spelling the user typed picks among readings: marks
                // first, then letters that only matching folds together.
                if let Some(t) = &q.typed {
                    let m: Vec<(String, u32, f32)> =
                        known.iter().filter(|r| &r.0 == t).map(|r| (r.0.clone(), r.1, r.2)).collect();
                    if !m.is_empty() {
                        return m;
                    }
                }
                let exact: Vec<(String, u32, f32)> = known.iter().filter(|r| r.3).map(|r| (r.0.clone(), r.1, r.2)).collect();
                if !exact.is_empty() {
                    return exact;
                }
                let known: Vec<(String, u32, f32)> = known.into_iter().map(|r| (r.0, r.1, r.2)).collect();
                if !known.is_empty() {
                    return known;
                }
            }
        }

        if pos.known.binary_search(&q.key).is_ok() {
            if let Some((canon, class)) = self.top_reading(pos, &q.plain) {
                // A known word lacking this cell has no form for it.
                if cites(&canon, class) && q.typed.as_ref().is_none_or(|t| *t == canon) {
                    return vec![(canon, class, 1.0)];
                }
            }
        }

        // Unknown word: restore its spelling, then predict its class.
        let restored = pos.restore.predict(&q.plain, |l| {
            pos.restores.get(l as usize).and_then(|e| e.apply(&q.plain)).is_some_and(|c| self.marks_placed(&c))
        });
        // Marks the user typed are kept as the spelling.
        let restored: Vec<(Option<u32>, f32)> = match &q.typed {
            Some(_) => vec![(None, 1.0)],
            None => restored.into_iter().take(3).map(|(r, p)| (Some(r), p)).collect(),
        };
        let mut out = Vec::new();
        for (r, rp) in restored {
            let canon = match (r, &q.typed) {
                (Some(r), _) => pos.restores.get(r as usize).and_then(|e| e.apply(&q.plain)),
                (None, t) => t.clone(),
            };
            let Some(canon) = canon else { continue };
            let applies = |class: u32| -> bool {
                needs(class)
                    && pos.classes.get(class as usize).is_some_and(|c| {
                        c.get(cell as usize).is_none_or(|vs| {
                            vs.is_empty()
                                || vs.iter().any(|&(e, _)| {
                                    pos.edits.get(e as usize).is_some_and(|e| e.is_none() || e.apply(&canon).is_some())
                                })
                        })
                    })
                    && cites(&canon, class)
            };
            let mut classes = pos.paradigm.predict(&q.plain, applies);
            if classes.is_empty() && !q.parts.is_empty() {
                // The citation rules out every class the tree knows here:
                // search all classes.
                let all: Vec<u32> = (0..pos.classes.len() as u32).filter(|&c| applies(c)).collect();
                let p = 1.0 / all.len().max(1) as f32;
                classes = all.into_iter().map(|c| (c, p)).collect();
            }
            out.extend(classes.into_iter().map(|(c, p)| (canon.clone(), c, p * rp)));
        }
        out
    }

    /// The variant position a citation picked, if it picked a later one.
    fn aligned_variant(&self, p: &Pos, q: &Query, canon: &str, class: u32) -> Option<usize> {
        let c = p.classes.get(class as usize)?;
        q.parts.iter().zip(p.citation.iter().skip(1)).find_map(|(part, alts)| {
            let want = self.key(part);
            alts.iter().find_map(|&cc| {
                c.get(cc as usize)?
                    .iter()
                    .position(|&(e, _)| p.edits.get(e as usize).and_then(|e| e.apply(canon)).is_some_and(|f| self.key(&f) == want))
                    .filter(|&j| j > 0)
            })
        })
    }

    fn is_known(&self, p: &Pos, key: &str) -> bool {
        p.known.binary_search_by(|k| k.as_str().cmp(key)).is_ok()
            || p.lexicon.binary_search_by(|e| e.0.as_str().cmp(key)).is_ok()
    }

    /// The single most likely (dictionary spelling, class) of a word: the
    /// best restoration, and the best class whose edits all apply to it.
    pub fn top_reading(&self, pos: &Pos, plain: &str) -> Option<(String, u32)> {
        let (r, _) = *pos
            .restore
            .predict(plain, |l| {
                pos.restores.get(l as usize).and_then(|e| e.apply(plain)).is_some_and(|c| self.marks_placed(&c))
            })
            .first()?;
        let canon = pos.restores.get(r as usize)?.apply(plain)?;
        let (class, _) = *pos
            .paradigm
            .predict(plain, |c| {
                pos.classes.get(c as usize).is_some_and(|cells| {
                    cells.iter().flatten().all(|&(e, _)| pos.edits.get(e as usize).is_some_and(|e| e.is_none() || e.apply(&canon).is_some()))
                })
            })
            .first()?;
        Some((canon, class))
    }

    /// Every candidate form of `input` in `cell` of part of speech `pos`,
    /// best first, with confidences summing to 1. Empty only when nothing
    /// is known or predictable.
    pub fn inflect(&self, pos: usize, cell: u32, input: &str, opts: Options) -> Vec<(String, f32)> {
        let Some(p) = self.pos.get(pos) else { return Vec::new() };
        let q = self.query(input);
        if q.plain.is_empty() {
            return Vec::new();
        }
        if p.head != Head::Whole && q.parts.is_empty() && q.plain.contains(' ') && !self.is_known(p, &q.key) {
            let words: Vec<&str> = input.trim().split(' ').filter(|w| !w.is_empty()).collect();
            let hi = if p.head == Head::First { 0 } else { words.len().saturating_sub(1) };
            if let Some(head) = words.get(hi) {
                return self
                    .inflect(pos, cell, head, opts)
                    .into_iter()
                    .map(|(f, s)| {
                        let mut w = words.clone();
                        if let Some(slot) = w.get_mut(hi) {
                            *slot = &f;
                        }
                        (w.join(" "), s)
                    })
                    .collect();
            }
        }
        let readings = self.readings(p, &q, cell);
        // Principal parts the caller gave are output exactly.
        if let Some(i) = p.citation.iter().position(|alts| alts.contains(&cell)).filter(|&i| i > 0) {
            if let Some(g) = q.parts.get(i - 1) {
                if !readings.is_empty() {
                    return vec![(g.clone(), 1.0)];
                }
            }
        }
        let mut forms: Vec<(String, f32)> = Vec::new();
        let mut no_form = 0.0f32;
        for (canon, class, w) in readings {
            let Some(vs) = p.classes.get(class as usize).and_then(|c| c.get(cell as usize)) else { continue };
            // A cited part that matched a later-listed variant picks the
            // variant at the same position in other cells (Wiktionary lists
            // parallel variants in parallel order: backte/buk in every person).
            let aligned = self.aligned_variant(p, &q, &canon, class).filter(|&j| j < vs.len());
            let vs: Vec<(u32, f32)> = match aligned {
                Some(j) => vs.get(j).map(|&(e, _)| vec![(e, 1.0)]).unwrap_or_default(),
                None => vs.clone(),
            };
            for &(e, vw) in &vs {
                if p.edits.get(e as usize).is_some_and(Edit::is_none) {
                    no_form += w * vw;
                    continue;
                }
                if let Some(f) = p.edits.get(e as usize).and_then(|e| e.apply(&canon)).filter(|f| self.marks_placed(f)) {
                    let f = self.finish(&f, q.casing, opts);
                    match forms.iter_mut().find(|x| x.0 == f) {
                        Some(x) => x.1 += w * vw,
                        None => forms.push((f, w * vw)),
                    }
                }
            }
        }
        // When the data says the form mostly doesn't exist, there is none;
        // otherwise its share stays out of the confidences.
        let total: f32 = forms.iter().map(|f| f.1).sum();
        if no_form > total {
            return Vec::new();
        }
        if total > 0.0 {
            for f in &mut forms {
                f.1 /= total + no_form;
            }
        }
        forms.sort_by(|a, b| b.1.total_cmp(&a.1).then_with(|| a.0.cmp(&b.0)));
        forms
    }

    /// The dictionary spelling of a word (with stress marks where known).
    pub fn lemma(&self, pos: usize, input: &str) -> Option<String> {
        let p = self.pos.get(pos)?;
        let q = self.query(input);
        self.readings(p, &q, u32::MAX).into_iter().next().map(|r| self.finish(&r.0, q.casing, Options::default()))
    }

    /// Every analysis of a form, best first: which lemma and cell it could
    /// be. Candidates come from the analysis tree and are kept only if
    /// inflecting the lemma back gives the form.
    pub fn analyze(&self, input: &str) -> Vec<(Analysis, f32)> {
        let mut out: Vec<(Analysis, f32)> = Vec::new();
        for pi in 0..self.pos.len() {
            out.extend(self.analyze_pos(pi, input));
        }
        let total: f32 = out.iter().map(|x| x.1).sum();
        if total > 0.0 {
            out.iter_mut().for_each(|x| x.1 /= total);
        }
        out.sort_by(|a, b| b.1.total_cmp(&a.1));
        out
    }

    /// Whether the analysis tree alone proposes (`lemma`, `cell`) for
    /// `form`: the inverse edit for that cell turns the form into the lemma.
    /// The trainer checks this for every attested form; the misses go in the
    /// reverse lexicon. (Inflecting back is exact for known lemmas, so it
    /// isn't rechecked.)
    pub fn analysis_finds(&self, pi: usize, form: &str, lemma: &str, cell: u32) -> bool {
        let Some(p) = self.pos.get(pi) else { return false };
        let q = self.query(form);
        let lemma_key = self.key(lemma);
        p.analysis
            .predict(&q.plain, |l| p.inverse.get(l as usize).is_some_and(|(_, e)| e.apply(&q.plain).is_some()))
            .iter()
            .any(|&(l, pr)| {
                pr > 0.0
                    && p.inverse.get(l as usize).is_some_and(|(c, e)| {
                        *c == cell && e.apply(&q.plain).is_some_and(|lm| self.key(&lm) == lemma_key)
                    })
            })
    }

    /// Unnormalized analyses within one part of speech: (lemma, cell, weight)
    /// merged by lemma.
    pub fn analyze_pos(&self, pi: usize, input: &str) -> Vec<(Analysis, f32)> {
        let Some(p) = self.pos.get(pi) else { return Vec::new() };
        let q = self.query(input);
        if q.plain.is_empty() {
            return Vec::new();
        }
        let mut out: Vec<(Analysis, f32)> = Vec::new();
        let mut push = |lemma: String, cell: Option<u32>, w: f32| {
            let i = match out.iter().position(|x| x.0.lemma == lemma) {
                Some(i) => i,
                None => {
                    out.push((Analysis { pos: pi, lemma, cells: Vec::new() }, 0.0));
                    out.len() - 1
                }
            };
            if let Some(x) = out.get_mut(i) {
                x.1 += w;
                if let Some(c) = cell {
                    if !x.0.cells.contains(&c) {
                        x.0.cells.push(c);
                    }
                }
            }
        };
        let display = |plain: &str| self.finish(&self.lemma(pi, plain).unwrap_or_else(|| plain.to_string()), q.casing, Options::default());
        if self.is_known(p, &q.key) {
            push(display(input), None, 0.5);
        }
        if let Ok(i) = p.reverse.binary_search_by(|e| e.0.as_str().cmp(&q.key)) {
            for (lemma, cell) in p.reverse.get(i).map(|e| e.1.as_slice()).unwrap_or_default() {
                push(display(lemma), Some(*cell), 1.0);
            }
        }
        let cands = p.analysis.predict(&q.plain, |l| p.inverse.get(l as usize).is_some_and(|(_, e)| e.apply(&q.plain).is_some()));
        for (l, pr) in cands {
            let Some((cell, e)) = p.inverse.get(l as usize) else { continue };
            let Some(lemma) = e.apply(&q.plain) else { continue };
            let forms = self.inflect(pi, *cell, &lemma, Options::default());
            let Some((_, fp)) = forms.iter().find(|(f, _)| self.key(f) == q.key) else { continue };
            let known = if self.is_known(p, &self.key(&lemma)) { 1.0 } else { 0.1 };
            push(display(&lemma), Some(*cell), pr * fp * known);
        }
        out
    }

    /// Scored values of an inherent feature (e.g. gender) of a word.
    pub fn inherent(&self, pos: usize, feature: usize, input: &str) -> Vec<(u32, f32)> {
        let Some(f) = self.pos.get(pos).and_then(|p| p.inherent.get(feature)) else { return Vec::new() };
        let q = self.query(input);
        if let Ok(i) = f.lexicon.binary_search_by(|e| e.0.as_str().cmp(&q.key)) {
            if let Some((_, d)) = f.lexicon.get(i) {
                return d.clone();
            }
        }
        f.tree.predict(&q.plain, |_| true)
    }

    pub fn encode(&self) -> Vec<u8> {
        let mut w = Writer::default();
        w.s(MAGIC);
        w.s(&self.name);
        w.s(&self.marks.iter().collect::<String>());
        w.u(self.unmark.len() as u64);
        for (a, b) in &self.unmark {
            w.u(u64::from(*a));
            w.u(u64::from(*b));
        }
        w.u(self.fold.len() as u64);
        for (a, b) in &self.fold {
            w.u(u64::from(*a));
            w.u(u64::from(*b));
        }
        w.s(&self.mark_bases.iter().collect::<String>());
        w.u(self.pos.len() as u64);
        for p in &self.pos {
            w.s(&p.name);
            w.u(p.cells.len() as u64);
            for c in &p.cells {
                w.s(c);
            }
            w.u(p.citation.len() as u64);
            for alts in &p.citation {
                w.u(alts.len() as u64);
                for c in alts {
                    w.u(u64::from(*c));
                }
            }
            w.u(p.edits.len() as u64);
            for e in &p.edits {
                w.edit(e);
            }
            w.u(p.classes.len() as u64);
            for class in &p.classes {
                for vs in class {
                    w.u(vs.len() as u64);
                    for &(e, wt) in vs {
                        w.u(u64::from(e));
                        if vs.len() > 1 {
                            w.p(wt);
                        }
                    }
                }
            }
            write_tree(&mut w, &p.restore);
            w.u(p.restores.len() as u64);
            for e in &p.restores {
                w.edit(e);
            }
            write_tree(&mut w, &p.paradigm);
            w.u(p.lexicon.len() as u64);
            let mut prev = "";
            for (k, rs) in &p.lexicon {
                write_key(&mut w, prev, k);
                prev = k;
                w.u(rs.len() as u64);
                for r in rs {
                    match &r.plain {
                        Some(s) => {
                            w.u(1);
                            w.s(s);
                        }
                        None => w.u(0),
                    }
                    w.u(u64::from(r.restore));
                    w.u(u64::from(r.class));
                    if rs.len() > 1 {
                        w.p(r.weight);
                    }
                }
            }
            w.u(p.known.len() as u64);
            let mut prev = "";
            for k in &p.known {
                write_key(&mut w, prev, k);
                prev = k;
            }
            w.u(match p.head {
                Head::Whole => 0,
                Head::First => 1,
                Head::Last => 2,
            });
            w.u(p.inherent.len() as u64);
            for f in &p.inherent {
                w.s(&f.name);
                w.u(f.values.len() as u64);
                for v in &f.values {
                    w.s(v);
                }
                write_tree(&mut w, &f.tree);
                w.u(f.lexicon.len() as u64);
                let mut prev = "";
                for (k, d) in &f.lexicon {
                    write_key(&mut w, prev, k);
                    prev = k;
                    write_dist(&mut w, d);
                }
            }
        }
        // Optional trailing sections: decoders that don't know them stop
        // before them.
        w.s("analysis-1");
        for p in &self.pos {
            write_tree(&mut w, &p.analysis);
            w.u(p.inverse.len() as u64);
            for (c, e) in &p.inverse {
                w.u(u64::from(*c));
                w.edit(e);
            }
        }
        w.s("reverse-1");
        for p in &self.pos {
            w.u(p.reverse.len() as u64);
            let mut prev = "";
            for (k, rs) in &p.reverse {
                write_key(&mut w, prev, k);
                prev = k;
                w.u(rs.len() as u64);
                for (l, c) in rs {
                    w.s(l);
                    w.u(u64::from(*c));
                }
            }
        }
        w.s("fold-strs-1");
        w.u(self.fold_strs.len() as u64);
        for (a, b) in &self.fold_strs {
            w.s(a);
            w.s(b);
        }
        w.buf
    }

    pub fn decode(buf: &[u8]) -> Option<Model> {
        let mut r = Reader::new(buf);
        if r.s()? != MAGIC {
            return None;
        }
        let name = r.s()?;
        let marks = r.s()?.chars().collect();
        let unmark = r.vec(|r| Some((char::from_u32(r.u32()?)?, char::from_u32(r.u32()?)?)))?;
        let fold = r.vec(|r| Some((char::from_u32(r.u32()?)?, char::from_u32(r.u32()?)?)))?;
        let mark_bases = r.s()?.chars().collect();
        let pos = r.vec(|r| {
            let name = r.s()?;
            let cells = r.vec(|r| r.s())?;
            let citation = r.vec(|r| r.vec(|r| r.u32()))?;
            let edits = r.vec(|r| r.edit())?;
            let ncells = cells.len();
            let classes = r.vec(|r| {
                (0..ncells)
                    .map(|_| {
                        let n = r.n()?;
                        (0..n)
                            .map(|_| {
                                let e = r.u32()?;
                                let w = if n > 1 { r.p()? } else { 1.0 };
                                Some((e, w))
                            })
                            .collect::<Option<Vec<_>>>()
                    })
                    .collect::<Option<Vec<_>>>()
            })?;
            let restore = read_tree(r)?;
            let restores = r.vec(|r| r.edit())?;
            let paradigm = read_tree(r)?;
            let mut prev = String::new();
            let lexicon = r.vec(|r| {
                let k = read_key(r, &prev)?;
                prev.clone_from(&k);
                let n = r.n()?;
                let rs = (0..n)
                    .map(|_| {
                        let plain = if r.u()? == 1 { Some(r.s()?) } else { None };
                        let restore = r.u32()?;
                        let class = r.u32()?;
                        let weight = if n > 1 { r.p()? } else { 1.0 };
                        Some(Reading { plain, restore, class, weight })
                    })
                    .collect::<Option<Vec<_>>>()?;
                Some((k, rs))
            })?;
            let mut prev = String::new();
            let known = r.vec(|r| {
                let k = read_key(r, &prev)?;
                prev.clone_from(&k);
                Some(k)
            })?;
            let head = match r.u()? {
                1 => Head::First,
                2 => Head::Last,
                _ => Head::Whole,
            };
            let inherent = r.vec(|r| {
                let name = r.s()?;
                let values = r.vec(|r| r.s())?;
                let tree = read_tree(r)?;
                let mut prev = String::new();
                let lexicon = r.vec(|r| {
                    let k = read_key(r, &prev)?;
                    prev.clone_from(&k);
                    Some((k, read_dist(r)?))
                })?;
                Some(Inherent { name, values, tree, lexicon })
            })?;
            Some(Pos {
                name,
                cells,
                citation,
                edits,
                classes,
                restore,
                restores,
                paradigm,
                lexicon,
                known,
                inherent,
                head,
                analysis: Tree::default(),
                inverse: Vec::new(),
                reverse: Vec::new(),
            })
        })?;
        let mut model = Model { name, marks, unmark, fold, fold_strs: Vec::new(), mark_bases, pos };
        if r.s().as_deref() == Some("analysis-1") {
            for p in &mut model.pos {
                let (Some(t), Some(inv)) = (read_tree(&mut r), r.vec(|r| Some((r.u32()?, r.edit()?)))) else { break };
                p.analysis = t;
                p.inverse = inv;
            }
        }
        let mut tag = r.s();
        if tag.as_deref() == Some("reverse-1") {
            for p in &mut model.pos {
                let mut prev = String::new();
                let rev = r.vec(|r| {
                    let k = read_key(r, &prev)?;
                    prev.clone_from(&k);
                    Some((k, r.vec(|r| Some((r.s()?, r.u32()?)))?))
                });
                let Some(rev) = rev else { break };
                p.reverse = rev;
            }
            tag = r.s();
        }
        if tag.as_deref() == Some("fold-strs-1") {
            model.fold_strs = r.vec(|r| Some((r.s()?, r.s()?))).unwrap_or_default();
        }
        Some(model)
    }
}

fn write_key(w: &mut Writer, prev: &str, k: &str) {
    let shared = prev.chars().zip(k.chars()).take_while(|(a, b)| a == b).map(|(a, _)| a.len_utf8()).sum::<usize>();
    w.u(shared as u64);
    w.s(k.get(shared..).unwrap_or(""));
}

fn read_key(r: &mut Reader, prev: &str) -> Option<String> {
    let shared = r.n()?;
    let rest = r.s()?;
    Some(format!("{}{}", prev.get(..shared)?, rest))
}

fn write_dist(w: &mut Writer, d: &[(u32, f32)]) {
    w.u(d.len() as u64);
    for &(l, p) in d {
        w.u(u64::from(l));
        w.p(p);
    }
}

fn read_dist(r: &mut Reader) -> Option<Vec<(u32, f32)>> {
    r.vec(|r| Some((r.u32()?, r.p()?)))
}

fn write_tree(w: &mut Writer, t: &Tree) {
    w.u(t.key.len() as u64);
    for k in &t.key {
        match k {
            KeyFeature::Syllables { vowels, cap } => {
                w.u(0);
                w.s(vowels);
                w.u(u64::from(*cap));
            }
            KeyFeature::Initial { n } => {
                w.u(1);
                w.u(u64::from(*n));
            }
        }
    }
    w.u(t.nodes.len() as u64);
    for (i, n) in t.nodes.iter().enumerate() {
        w.u(n.children.len() as u64);
        for &(sym, child) in &n.children {
            w.u(u64::from(sym));
            w.u(u64::from(child) - i as u64);
        }
        write_dist(w, &n.dist);
    }
}

fn read_tree(r: &mut Reader) -> Option<Tree> {
    let key = r.vec(|r| match r.u()? {
        0 => Some(KeyFeature::Syllables { vowels: r.s()?, cap: r.u32()? }),
        1 => Some(KeyFeature::Initial { n: r.u32()? }),
        _ => None,
    })?;
    let n = r.n()?;
    let mut nodes = Vec::with_capacity(n.min(1 << 24));
    for i in 0..n {
        let children = r.vec(|r| Some((r.u32()?, u32::try_from(r.u()? + i as u64).ok()?)))?;
        let dist = read_dist(r)?;
        nodes.push(Node { children, dist });
    }
    Some(Tree { key, nodes })
}
