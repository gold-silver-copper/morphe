//! Edit trees: the change from one string to another, stored so that it can be
//! applied to other strings of the same shape. The two strings are aligned on
//! their longest common substring, which is copied; the parts before and after
//! it are handled recursively, and parts with nothing in common become a
//! literal substitution.

/// One edit tree.
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Edit {
    /// Keep a middle part of any length. `pre` and `suf` are the lengths (in
    /// chars) of the parts before and after it, rewritten by `left`/`right`
    /// (`None` means the part is empty on both sides).
    Match {
        pre: u32,
        suf: u32,
        left: Option<Box<Edit>>,
        right: Option<Box<Edit>>,
    },
    /// Replace exactly `from` with `to`.
    Subst { from: String, to: String },
}

impl Edit {
    /// The edit that turns `a` into `b`.
    pub fn between(a: &str, b: &str) -> Edit {
        let a: Vec<char> = a.chars().collect();
        let b: Vec<char> = b.chars().collect();
        build(&a, &b)
    }

    /// Apply to `w`, or `None` if `w` doesn't have the shape this edit needs.
    pub fn apply(&self, w: &str) -> Option<String> {
        let mut out = String::with_capacity(w.len() + 8);
        apply(self, w, &mut out).then_some(out)
    }

    /// "This cell has no form": never applies.
    pub fn none() -> Edit {
        Edit::Subst { from: "\u{0}".into(), to: "\u{0}".into() }
    }

    pub fn is_none(&self) -> bool {
        matches!(self, Edit::Subst { from, .. } if from == "\u{0}")
    }

    /// True if this edit leaves every string unchanged.
    pub fn is_identity(&self) -> bool {
        matches!(self, Edit::Match { left: None, right: None, .. })
    }
}

fn build(a: &[char], b: &[char]) -> Edit {
    let (ia, ib, len) = longest_common(a, b);
    if len == 0 {
        return Edit::Subst {
            from: a.iter().collect(),
            to: b.iter().collect(),
        };
    }
    let (a1, a2) = (&a[..ia], &a[ia + len..]);
    let (b1, b2) = (&b[..ib], &b[ib + len..]);
    let side = |x: &[char], y: &[char]| (!x.is_empty() || !y.is_empty()).then(|| Box::new(build(x, y)));
    Edit::Match {
        pre: a1.len() as u32,
        suf: a2.len() as u32,
        left: side(a1, b1),
        right: side(a2, b2),
    }
}

/// Longest common substring: (start in a, start in b, length). Ties go to the
/// match that starts earliest in `a`, then in `b`.
fn longest_common(a: &[char], b: &[char]) -> (usize, usize, usize) {
    let mut best = (0, 0, 0);
    let mut prev = vec![0usize; b.len() + 1];
    let mut cur = vec![0usize; b.len() + 1];
    for (i, ca) in a.iter().enumerate() {
        for (j, cb) in b.iter().enumerate() {
            cur[j + 1] = if ca == cb { prev[j] + 1 } else { 0 };
            let l = cur[j + 1];
            let (si, sj) = (i + 1 - l, j + 1 - l);
            if l > best.2 || (l == best.2 && l > 0 && (si, sj) < (best.0, best.1)) {
                best = (si, sj, l);
            }
        }
        std::mem::swap(&mut prev, &mut cur);
    }
    best
}

fn apply(e: &Edit, w: &str, out: &mut String) -> bool {
    match e {
        Edit::Subst { from, to } => {
            if from == w {
                out.push_str(to);
                true
            } else {
                false
            }
        }
        Edit::Match { pre, suf, left, right } => {
            let (pre, suf) = (*pre as usize, *suf as usize);
            // Byte offsets after the first `pre` chars and before the last `suf`.
            let mut it = w.char_indices();
            let start = if pre == 0 {
                0
            } else {
                match it.nth(pre - 1) {
                    Some((i, c)) => i + c.len_utf8(),
                    None => return false,
                }
            };
            let mut back = w[start..].char_indices().rev();
            let end = if suf == 0 {
                w.len()
            } else {
                match back.nth(suf - 1) {
                    Some((i, _)) => start + i,
                    None => return false,
                }
            };
            if end <= start {
                return false;
            }
            let (w1, mid, w2) = (&w[..start], &w[start..end], &w[end..]);
            let ok_left = match left {
                Some(l) => apply(l, w1, out),
                None => w1.is_empty(),
            };
            if !ok_left {
                return false;
            }
            out.push_str(mid);
            match right {
                Some(r) => apply(r, w2, out),
                None => w2.is_empty(),
            }
        }
    }
}
