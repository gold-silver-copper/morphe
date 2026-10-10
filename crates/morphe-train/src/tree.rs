//! Growing and pruning a suffix tree of label counts.
//!
//! Every node counts the labels of the training words whose reversed key
//! passes through it. A node's distribution is smoothed toward its nearest
//! kept ancestor's: p(c) = (n_c + β·p_anc(c)) / (n + β). A node is kept when
//! the log-likelihood its own distribution gains over the ancestor's exceeds
//! λ nats per label it stores (a description-length trade: what a node costs
//! to store against what it saves in coding its words). What survives is the
//! broad patterns near the root and the sharp exceptions deeper down.

use morphe::model::{KeyFeature, Node, Tree};

#[derive(Clone, Copy, Debug)]
pub struct Params {
    pub beta: f64,
    pub lambda: f64,
    pub min_count: f64,
    pub max_depth: usize,
    pub top_k: usize,
}

impl Default for Params {
    fn default() -> Self {
        Params { beta: 1.0, lambda: 3.0, min_count: 2.0, max_depth: 24, top_k: 10 }
    }
}

#[derive(Default)]
struct TNode {
    children: Vec<(u32, u32)>,
    /// Ordered, so sums and ties come out the same on every run.
    counts: std::collections::BTreeMap<u32, f64>,
    n: f64,
}

impl TNode {
    fn add(&mut self, label: u32, w: f64) {
        self.n += w;
        *self.counts.entry(label).or_default() += w;
    }
    fn count(&self, label: u32) -> f64 {
        self.counts.get(&label).copied().unwrap_or(0.0)
    }
}

pub fn train(key: &[KeyFeature], data: &[(String, u32, f64)], p: Params) -> Tree {
    let mut nodes: Vec<TNode> = vec![TNode::default()];
    let mut labels: Vec<u32> = Vec::new();
    for (word, label, w) in data {
        labels.push(*label);
        nodes[0].add(*label, *w);
        let mut cur = 0usize;
        for sym in Tree::symbols(key, word).into_iter().take(p.max_depth) {
            let next = match nodes[cur].children.iter().find(|c| c.0 == sym) {
                Some(c) => c.1 as usize,
                None => {
                    nodes.push(TNode::default());
                    let id = nodes.len() - 1;
                    nodes[cur].children.push((sym, id as u32));
                    id
                }
            };
            nodes[next].add(*label, *w);
            cur = next;
        }
    }
    labels.sort_unstable();
    labels.dedup();
    let nlabels = labels.len().max(1) as f64;

    // anc[i]: nearest kept proper ancestor of node i (for kept nodes).
    let mut anc: Vec<Option<u32>> = vec![None; nodes.len()];
    let mut kept = vec![false; nodes.len()];
    let mut dist: Vec<Vec<(u32, f32)>> = vec![Vec::new(); nodes.len()];
    kept[0] = true;

    let prob = |i: usize, label: u32, anc: &[Option<u32>]| -> f64 {
        // Walk up the kept chain, then smooth back down.
        let mut chain = vec![i];
        let mut cur = i;
        while let Some(a) = anc[cur] {
            chain.push(a as usize);
            cur = a as usize;
        }
        let root = &nodes[0];
        let mut pr = (root.count(label) + 0.5) / (root.n + 0.5 * nlabels);
        for &j in chain.iter().rev().skip(1) {
            let n = &nodes[j];
            pr = (n.count(label) + p.beta * pr) / (n.n + p.beta);
        }
        pr
    };

    let emit = |i: usize, parent: Option<usize>, anc: &[Option<u32>], dist: &[Vec<(u32, f32)>]| -> Vec<(u32, f32)> {
        let mut cands: Vec<u32> = nodes[i].counts.keys().copied().collect();
        if let Some(a) = parent {
            cands.extend(dist[a].iter().map(|d| d.0));
        }
        cands.sort_unstable();
        cands.dedup();
        let mut d: Vec<(u32, f32)> = cands.into_iter().map(|l| (l, prob(i, l, anc) as f32)).collect();
        d.sort_by(|a, b| b.1.total_cmp(&a.1).then(a.0.cmp(&b.0)));
        d.truncate(p.top_k);
        d
    };
    dist[0] = emit(0, None, &anc, &dist);

    // Depth-first, with the nearest kept ancestor carried along.
    let mut stack: Vec<(usize, usize)> = nodes[0].children.iter().map(|c| (c.1 as usize, 0)).collect();
    while let Some((i, a)) = stack.pop() {
        let mut next_anc = a;
        if nodes[i].n >= p.min_count {
            anc[i] = Some(a as u32);
            let gain: f64 = nodes[i]
                .counts
                .iter()
                .map(|(&l, &c)| c * (prob(i, l, &anc) / prob(a, l, &anc)).ln())
                .sum();
            if gain > p.lambda * nodes[i].counts.len() as f64 {
                kept[i] = true;
                dist[i] = emit(i, Some(a), &anc, &dist);
                next_anc = i;
            } else {
                anc[i] = None;
            }
        }
        if nodes[i].n >= p.min_count {
            stack.extend(nodes[i].children.iter().map(|c| (c.1 as usize, next_anc)));
        }
    }

    // Keep kept nodes and their ancestors, in breadth-first order.
    let mut needed = kept.clone();
    // Children always have larger indices than parents (created later).
    for i in (0..nodes.len()).rev() {
        if needed[i] {
            continue;
        }
        needed[i] = nodes[i].children.iter().any(|c| needed[c.1 as usize]);
    }
    let mut order: Vec<usize> = vec![0];
    let mut new_id = vec![u32::MAX; nodes.len()];
    new_id[0] = 0;
    let mut head = 0;
    while head < order.len() {
        let i = order[head];
        head += 1;
        let mut ch: Vec<(u32, u32)> = nodes[i].children.clone();
        ch.sort_unstable();
        for (_, c) in ch {
            if needed[c as usize] {
                new_id[c as usize] = order.len() as u32;
                order.push(c as usize);
            }
        }
    }
    let out = order
        .iter()
        .map(|&i| {
            let mut children: Vec<(u32, u32)> = nodes[i]
                .children
                .iter()
                .filter(|c| needed[c.1 as usize])
                .map(|c| (c.0, new_id[c.1 as usize]))
                .collect();
            children.sort_unstable();
            Node { children, dist: if kept[i] { std::mem::take(&mut dist[i]) } else { Vec::new() } }
        })
        .collect();
    Tree { key: key.to_vec(), nodes: out }
}
