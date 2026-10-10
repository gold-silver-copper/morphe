//! From records to a trained model: edits, classes, trees and the lexicon of
//! words the trees don't get right.

use crate::extract::{Extractor, Record};
use crate::tree::{self, Params};
use morphe::model::{Inherent, Model, Pos, Reading};
use morphe::Edit;
use std::collections::{BTreeMap, HashMap};

type ClassKey = Vec<Vec<(u32, u16)>>;

pub struct Stats {
    pub pos: String,
    pub records: usize,
    pub edits: usize,
    pub classes: usize,
    pub lexicon: usize,
    pub nodes: usize,
}

/// Ids by descending frequency, ties broken by value, so output is deterministic.
fn index<T: Ord + Clone + std::hash::Hash>(items: impl Iterator<Item = T>) -> (Vec<T>, HashMap<T, u32>) {
    let mut counts: HashMap<T, usize> = HashMap::new();
    for i in items {
        *counts.entry(i).or_default() += 1;
    }
    let mut v: Vec<(T, usize)> = counts.into_iter().collect();
    v.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));
    let list: Vec<T> = v.into_iter().map(|x| x.0).collect();
    let map = list.iter().enumerate().map(|(i, t)| (t.clone(), i as u32)).collect();
    (list, map)
}

fn q(w: f32) -> u16 {
    (w.clamp(0.0, 1.0) * 65535.0).round() as u16
}

pub fn learn(ex: &Extractor, threads: usize) -> (Model, Vec<Stats>) {
    let spec = &ex.spec;
    let mut mark_bases: Vec<char> = Vec::new();
    for recs in &ex.records {
        for r in recs {
            let chars: Vec<char> = r.canon.chars().collect();
            for w in chars.windows(2) {
                if spec.marks.contains(&w[1]) && !spec.marks.contains(&w[0]) {
                    mark_bases.push(w[0]);
                }
            }
        }
    }
    mark_bases.sort_unstable();
    mark_bases.dedup();
    let mut model =
        Model {
            name: spec.name.clone(),
            marks: spec.marks.clone(),
            unmark: spec.unmark.clone(),
            fold: spec.fold.clone(),
            mark_bases,
            pos: Vec::new(),
        };
    let mut stats = Vec::new();
    for (pi, ps) in spec.pos.iter().enumerate() {
        let recs = &ex.records[pi];
        let cells = &ex.cells[pi];
        eprintln!("[{}] {}: {} records, {} cells", spec.name, ps.name, recs.len(), cells.len());

        let rec_edits: Vec<Vec<Vec<(Edit, f32)>>> = par_map(recs, threads, |r: &Record| {
            r.cells.iter().map(|vs| vs.iter().map(|(f, w)| (Edit::between(&r.canon, f), *w)).collect()).collect()
        });
        let (edits, edit_id) = index(rec_edits.iter().flatten().flatten().map(|(e, _)| e.clone()));
        let full: Vec<ClassKey> = rec_edits
            .iter()
            .map(|cs| cs.iter().map(|vs| vs.iter().map(|(e, w)| (edit_id[e], q(*w))).collect()).collect())
            .collect();
        let primary: Vec<ClassKey> = full
            .iter()
            .map(|cs| cs.iter().map(|vs| vs.first().map(|&(e, _)| (e, u16::MAX)).into_iter().collect()).collect())
            .collect();
        let (class_list, class_id) = index(primary.iter().cloned().chain(full.iter().cloned()));
        let classes: Vec<Vec<Vec<(u32, f32)>>> = class_list
            .iter()
            .map(|c| c.iter().map(|vs| vs.iter().map(|&(e, w)| (e, f32::from(w) / 65535.0)).collect()).collect())
            .collect();

        let rest: Vec<Edit> = recs.iter().map(|r| Edit::between(&r.plain, &r.canon)).collect();
        let (restores, restore_id) = index(rest.iter().cloned());

        // Trees learn from single words; phrases are idiosyncratic, and an
        // unknown phrase is inflected through its head word.
        let single = |r: &&Record| !r.plain.contains(' ');
        let para_data: Vec<(String, u32, f64)> =
            recs.iter().zip(&full).filter(|(r, _)| single(r)).map(|(r, c)| (r.plain.clone(), class_id[c], 1.0)).collect();
        let rest_data: Vec<(String, u32, f64)> =
            recs.iter().zip(&rest).filter(|(r, _)| single(r)).map(|(r, e)| (r.plain.clone(), restore_id[e], 1.0)).collect();
        let inh_data: Vec<Vec<(String, u32, f64)>> = (0..ps.inherent.len())
            .map(|fi| {
                recs.iter()
                    .filter(single)
                    .flat_map(|r| {
                        let vs = &r.inherent[fi];
                        let w = 1.0 / vs.len().max(1) as f64;
                        vs.iter().map(move |&v| (r.plain.clone(), v as u32, w))
                    })
                    .collect()
            })
            .collect();

        // Analysis: (cell, inverse edit) from single-word forms.
        let strip = |f: &str| -> String {
            f.to_lowercase()
                .chars()
                .filter(|c| !spec.marks.contains(c))
                .map(|c| spec.unmark.iter().find(|u| u.0 == c).map_or(c, |u| u.1))
                .collect()
        };
        let inv_pairs: Vec<(String, (u32, Edit))> = recs
            .iter()
            .filter(single)
            .flat_map(|r| {
                r.cells.iter().enumerate().flat_map(move |(ci, vs)| {
                    vs.iter().filter(|(f, _)| !f.contains(' ')).map(move |(f, _)| {
                        let fp = strip(f);
                        let e = Edit::between(&fp, &r.plain);
                        (fp, (ci as u32, e))
                    })
                })
            })
            .collect();
        let (inverse, inverse_id) = index(inv_pairs.iter().map(|x| x.1.clone()));
        let ana_data: Vec<(String, u32, f64)> = inv_pairs.iter().map(|(f, l)| (f.clone(), inverse_id[l], 1.0)).collect();

        let (paradigm, restore, inh_trees, analysis) = std::thread::scope(|s| {
            let d = s.spawn(|| tree::train(&[], &ana_data, Params { top_k: 24, ..Params::default() }));
            let a = s.spawn(|| tree::train(&ps.key, &para_data, Params::default()));
            let b = s.spawn(|| tree::train(&[], &rest_data, Params { top_k: 4, ..Params::default() }));
            let c: Vec<_> = inh_data
                .iter()
                .map(|d| s.spawn(move || tree::train(&[], d, Params { top_k: 6, ..Params::default() })))
                .collect();
            (
                a.join().unwrap_or_default(),
                b.join().unwrap_or_default(),
                c.into_iter().map(|h| h.join().unwrap_or_default()).collect::<Vec<_>>(),
                d.join().unwrap_or_default(),
            )
        });
        let nodes = paradigm.nodes.len() + restore.nodes.len();

        let mut pos = Pos {
            name: ps.name.clone(),
            cells: cells.iter().map(|c| c.name.clone()).collect(),
            citation: ps
                .citation
                .iter()
                .map(|alts| {
                    alts.iter().filter_map(|n| cells.iter().position(|c| &c.name == n).map(|i| i as u32)).collect()
                })
                .collect(),
            edits,
            classes,
            restore,
            restores,
            paradigm,
            lexicon: Vec::new(),
            known: Vec::new(),
            inherent: Vec::new(),
            head: ps.head,
            analysis,
            inverse,
        };
        let probe = Model { pos: Vec::new(), ..model.clone() };

        // Group records by lookup key, in dump order.
        let mut groups: BTreeMap<String, Vec<usize>> = BTreeMap::new();
        for (i, r) in recs.iter().enumerate() {
            groups.entry(probe.key(&r.plain)).or_default().push(i);
        }
        let groups: Vec<(String, Vec<usize>)> = groups.into_iter().collect();

        // Readings per key: distinct (plain, restore, class), weighted by
        // listing order of the entries.
        let readings: Vec<Vec<Reading>> = groups
            .iter()
            .map(|(key, idx)| {
                let mut rs: Vec<Reading> = Vec::new();
                for (order, &i) in idx.iter().enumerate() {
                    let plain = (recs[i].plain != *key).then(|| recs[i].plain.clone());
                    let restore = restore_id[&rest[i]];
                    let class = class_id[&full[i]];
                    let w = 1.0 / (order as f32 + 1.0);
                    match rs.iter_mut().find(|r| r.plain == plain && r.restore == restore && r.class == class) {
                        Some(r) => r.weight += w,
                        None => rs.push(Reading { plain, restore, class, weight: w }),
                    }
                }
                let total: f32 = rs.iter().map(|r| r.weight).sum();
                for r in &mut rs {
                    r.weight /= total;
                }
                rs
            })
            .collect();

        // A known word whose top prediction gives exactly its forms (every
        // cell, every variant, in order) only needs its key stored; any
        // other known word goes in the lexicon with its readings.
        let check = Model { pos: vec![pos.clone()], ..probe.clone() };
        let jobs: Vec<usize> = (0..groups.len()).collect();
        let exact: Vec<bool> = par_map(&jobs, threads, |&g| {
            let rs = &readings[g];
            if rs.len() != 1 {
                return false;
            }
            let r = &rs[0];
            let cp = &check.pos[0];
            let plain = r.plain.as_deref().unwrap_or(&groups[g].0);
            if r.plain.is_some() {
                return false;
            }
            let forms = |canon: &str, class: u32| -> Option<Vec<Vec<String>>> {
                cp.classes
                    .get(class as usize)?
                    .iter()
                    .map(|vs| vs.iter().map(|&(e, _)| cp.edits.get(e as usize)?.apply(canon)).collect())
                    .collect()
            };
            let Some(canon) = cp.restores.get(r.restore as usize).and_then(|e| e.apply(plain)) else { return false };
            let Some((pc, pclass)) = check.top_reading(cp, plain) else { return false };
            pc == canon && forms(&pc, pclass).is_some() && forms(&pc, pclass) == forms(&canon, r.class)
        });
        let mut lexicon = Vec::new();
        for ((g, rs), ok) in groups.iter().zip(readings).zip(&exact) {
            if *ok {
                pos.known.push(g.0.clone());
            } else {
                lexicon.push((g.0.clone(), rs));
            }
        }
        pos.lexicon = lexicon;

        for (fi, fname) in ps.inherent.iter().enumerate() {
            let values = spec.feature(fname).map(|(_, f)| f.values.iter().map(|v| v.name.clone()).collect()).unwrap_or_default();
            let tree = inh_trees.get(fi).cloned().unwrap_or_default();
            let mut lexicon = Vec::new();
            for (key, idx) in &groups {
                let mut d: Vec<(u32, f32)> = Vec::new();
                for &i in idx {
                    let vs = &recs[i].inherent[fi];
                    for &v in vs {
                        let w = 1.0 / vs.len() as f32;
                        match d.iter_mut().find(|x| x.0 == v as u32) {
                            Some(x) => x.1 += w,
                            None => d.push((v as u32, w)),
                        }
                    }
                }
                if d.is_empty() {
                    continue;
                }
                let total: f32 = d.iter().map(|x| x.1).sum();
                d.iter_mut().for_each(|x| x.1 /= total);
                d.sort_by(|a, b| b.1.total_cmp(&a.1).then(a.0.cmp(&b.0)));
                let pred = tree.predict(&recs[idx[0]].plain, |_| true);
                // Known words get their attested values, unless the tree
                // already says the same with confidence.
                if pred.first().map(|x| x.0) != d.first().map(|x| x.0) || d.len() > 1 || pred.first().is_some_and(|x| x.1 < 0.95) {
                    lexicon.push((key.clone(), d));
                }
            }
            pos.inherent.push(Inherent { name: fname.clone(), values, tree, lexicon });
        }

        stats.push(Stats {
            pos: ps.name.clone(),
            records: recs.len(),
            edits: pos.edits.len(),
            classes: pos.classes.len(),
            lexicon: pos.lexicon.len(),
            nodes,
        });
        eprintln!(
            "[{}] {}: {} edits, {} classes, {} tree nodes, lexicon {}, known {}",
            spec.name,
            ps.name,
            pos.edits.len(),
            pos.classes.len(),
            nodes,
            pos.lexicon.len(),
            pos.known.len()
        );
        model.pos.push(pos);
    }
    (model, stats)
}

/// Map over a slice on `threads` threads, keeping order.
pub fn par_map<T: Sync, R: Send>(items: &[T], threads: usize, f: impl Fn(&T) -> R + Sync) -> Vec<R> {
    let chunk = items.len().div_ceil(threads.max(1)).max(1);
    std::thread::scope(|s| {
        let handles: Vec<_> = items.chunks(chunk).map(|c| s.spawn(|| c.iter().map(&f).collect::<Vec<R>>())).collect();
        handles.into_iter().flat_map(|h| h.join().unwrap_or_default()).collect()
    })
}
