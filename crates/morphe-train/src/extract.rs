//! From Kaikki entries to (lemma, cell, form) records, following the config.

use crate::kaikki::{Entry, Form};
use crate::spec::{Cell, Language, PosSpec, Source, Spec};

/// One dictionary entry's paradigm.
#[derive(Clone, Debug)]
pub struct Record {
    /// Lowercased, without marks: what a user types.
    pub plain: String,
    /// Lowercased dictionary spelling, with marks.
    pub canon: String,
    /// Per cell: forms (lowercased, with marks) and weights; the first is primary.
    pub cells: Vec<Vec<(String, f32)>>,
    /// Per inherent feature: the values the entry names.
    pub inherent: Vec<Vec<usize>>,
}

pub struct Extractor<'a> {
    pub lang: &'a dyn Language,
    pub spec: Spec,
    pub cells: Vec<Vec<Cell>>,
    pub records: Vec<Vec<Record>>,
}

/// Weight of the i-th listed form: plain forms in listing order get 1, 1/2,
/// 1/3…; soft forms get a tenth of that.
pub fn variant_weight(i: usize, soft: bool) -> f32 {
    let w = 1.0 / (i as f32 + 1.0);
    if soft { w * 0.1 } else { w }
}

impl<'a> Extractor<'a> {
    pub fn new(lang: &'a dyn Language) -> Self {
        let spec = lang.spec();
        let cells = spec.pos.iter().map(|p| spec.cells(p)).collect();
        let records = spec.pos.iter().map(|_| Vec::new()).collect();
        Extractor { lang, spec, cells, records }
    }

    pub fn strip(&self, s: &str) -> String {
        s.chars()
            .filter(|c| !self.spec.marks.contains(c))
            .map(|c| self.spec.unmark.iter().find(|u| u.0 == c).map_or(c, |u| u.1))
            .collect()
    }

    pub fn add(&mut self, e: &Entry) {
        if !self.lang.keep_entry(e) {
            return;
        }
        for pi in 0..self.spec.pos.len() {
            if self.spec.pos[pi].kaikki.contains(&e.pos) {
                if let Some(r) = self.record(pi, e) {
                    self.records[pi].push(r);
                }
            }
        }
    }

    fn record(&self, pi: usize, e: &Entry) -> Option<Record> {
        let spec = &self.spec;
        let pos: &PosSpec = &spec.pos[pi];
        let cells = &self.cells[pi];
        let word = e.word.trim();
        if word.is_empty() || word.contains(',') {
            return None;
        }
        let plain = self.strip(&word.to_lowercase());
        let reshaped = self.lang.reshape(pos, e);
        let forms: &[Form] = reshaped.as_deref().unwrap_or(&e.forms);
        let canon = spec
            .canonical_tag
            .as_ref()
            .filter(|_| spec.canonical_spelling)
            .and_then(|t| forms.iter().find(|f| f.tags.contains(t)))
            .map(|f| f.form.trim().to_lowercase())
            .filter(|c| self.strip(c) == plain)
            .unwrap_or_else(|| plain.clone());

        let retagged = |form: &str, t: &[String]| {
            let mut t = t.to_vec();
            self.lang.retag(pos, form, &mut t);
            t
        };
        // (cell, form, soft, from a table)
        let mut raw: Vec<(usize, String, bool, bool)> = Vec::new();
        for f in forms {
            let table = f.source.is_some();
            if (spec.source == Source::Head && table) || (spec.source == Source::Table && !table) {
                continue;
            }
            let tags = retagged(f.form.trim(), &f.tags);
            if tags.iter().any(|t| spec.drop_tags.contains(t)) {
                continue;
            }
            let soft = tags.iter().any(|t| spec.soft_tags.contains(t));
            let matched: Vec<(usize, usize)> =
                cells.iter().enumerate().filter_map(|(ci, c)| c.matches(&tags).map(|n| (ci, n))).collect();
            let Some(best) = matched.iter().map(|m| m.1).max() else { continue };
            for form in self.lang.split_form(f.form.trim()) {
                let form = form.trim();
                if spec.sentinels.iter().any(|s| s == form) || form.contains(['(', ')', '/', '[', ']']) {
                    continue;
                }
                for &(ci, n) in &matched {
                    if n == best {
                        raw.push((ci, form.to_lowercase(), soft, table));
                    }
                }
            }
        }
        // A table speaks for the cells it has; the headword line fills the rest.
        let mut found: Vec<(usize, String, bool)> = raw
            .iter()
            .filter(|(ci, _, _, table)| *table || !raw.iter().any(|r| r.0 == *ci && r.3))
            .map(|(ci, f, soft, _)| (*ci, f.clone(), *soft))
            .collect();
        self.lang.adjust(pos, &plain, &mut found);

        let mut out: Vec<Vec<(String, f32)>> = vec![Vec::new(); cells.len()];
        for (ci, slot) in out.iter_mut().enumerate() {
            let mut list: Vec<(String, bool)> = Vec::new();
            for soft in [false, true] {
                for (c, f, s) in &found {
                    if *c == ci && *s == soft && !list.iter().any(|x| &x.0 == f) {
                        list.push((f.clone(), *s));
                    }
                }
            }
            let (mut np, mut ns) = (0, 0);
            for (f, soft) in list {
                let i = if soft { &mut ns } else { &mut np };
                slot.push((f, variant_weight(*i, soft)));
                *i += 1;
            }
            let total: f32 = slot.iter().map(|x| x.1).sum();
            for x in slot.iter_mut() {
                x.1 /= total;
            }
        }
        if out.iter().all(Vec::is_empty) {
            return None;
        }

        let canon_tags: Vec<String> = spec
            .canonical_tag
            .as_ref()
            .map(|t| forms.iter().filter(|f| f.tags.contains(t)).flat_map(|f| retagged(f.form.trim(), &f.tags)).collect())
            .unwrap_or_default();
        let sense_tags: Vec<String> = e.senses.iter().flat_map(|s| retagged("", &s.tags)).collect();
        let inherent = pos
            .inherent
            .iter()
            .map(|fname| {
                let Some((_, f)) = spec.feature(fname) else { return Vec::new() };
                let pick = |tags: &[String]| -> Vec<usize> {
                    f.values
                        .iter()
                        .enumerate()
                        .filter(|(_, v)| {
                            v.tags.iter().all(|t| match t.strip_prefix('!') {
                                Some(n) => !tags.iter().any(|x| *x == n),
                                None => tags.iter().any(|x| x == t),
                            })
                        })
                        .map(|(i, _)| i)
                        .collect()
                };
                let v = pick(&canon_tags);
                if v.is_empty() { pick(&sense_tags) } else { v }
            })
            .collect();
        Some(Record { plain, canon, cells: out, inherent })
    }
}
