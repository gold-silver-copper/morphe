//! A small binary format: varints, strings and quantized probabilities.

use crate::edit::Edit;

#[derive(Default)]
pub struct Writer {
    pub buf: Vec<u8>,
}

impl Writer {
    pub fn u(&mut self, mut v: u64) {
        loop {
            let b = (v & 0x7f) as u8;
            v >>= 7;
            if v == 0 {
                self.buf.push(b);
                return;
            }
            self.buf.push(b | 0x80);
        }
    }
    pub fn s(&mut self, s: &str) {
        self.u(s.len() as u64);
        self.buf.extend_from_slice(s.as_bytes());
    }
    /// A probability in [0, 1], stored in two bytes.
    pub fn p(&mut self, p: f32) {
        let q = (p.clamp(0.0, 1.0) * 65535.0).round() as u16;
        self.buf.extend_from_slice(&q.to_le_bytes());
    }
    pub fn edit(&mut self, e: &Edit) {
        match e {
            Edit::Match { pre, suf, left, right } => {
                let flags = 2 * u64::from(left.is_some()) + u64::from(right.is_some());
                self.u(flags);
                self.u(u64::from(*pre));
                self.u(u64::from(*suf));
                if let Some(l) = left {
                    self.edit(l);
                }
                if let Some(r) = right {
                    self.edit(r);
                }
            }
            Edit::Subst { from, to } => {
                self.u(4);
                self.s(from);
                self.s(to);
            }
        }
    }
}

pub struct Reader<'a> {
    buf: &'a [u8],
    pos: usize,
}

impl<'a> Reader<'a> {
    pub fn new(buf: &'a [u8]) -> Self {
        Reader { buf, pos: 0 }
    }
    fn byte(&mut self) -> Option<u8> {
        let b = *self.buf.get(self.pos)?;
        self.pos += 1;
        Some(b)
    }
    pub fn u(&mut self) -> Option<u64> {
        let mut v = 0u64;
        let mut shift = 0;
        loop {
            let b = self.byte()?;
            v |= u64::from(b & 0x7f).checked_shl(shift)?;
            if b & 0x80 == 0 {
                return Some(v);
            }
            shift += 7;
        }
    }
    pub fn n(&mut self) -> Option<usize> {
        usize::try_from(self.u()?).ok()
    }
    pub fn u32(&mut self) -> Option<u32> {
        u32::try_from(self.u()?).ok()
    }
    pub fn s(&mut self) -> Option<String> {
        let len = self.n()?;
        let bytes = self.buf.get(self.pos..self.pos.checked_add(len)?)?;
        self.pos += len;
        String::from_utf8(bytes.to_vec()).ok()
    }
    pub fn p(&mut self) -> Option<f32> {
        let lo = self.byte()?;
        let hi = self.byte()?;
        Some(f32::from(u16::from_le_bytes([lo, hi])) / 65535.0)
    }
    pub fn edit(&mut self) -> Option<Edit> {
        let flags = self.u()?;
        if flags == 4 {
            return Some(Edit::Subst { from: self.s()?, to: self.s()? });
        }
        let pre = self.u32()?;
        let suf = self.u32()?;
        let left = if flags & 2 != 0 { Some(Box::new(self.edit()?)) } else { None };
        let right = if flags & 1 != 0 { Some(Box::new(self.edit()?)) } else { None };
        Some(Edit::Match { pre, suf, left, right })
    }
    pub fn vec<T>(&mut self, mut f: impl FnMut(&mut Self) -> Option<T>) -> Option<Vec<T>> {
        let n = self.n()?;
        let mut v = Vec::with_capacity(n.min(1 << 20));
        for _ in 0..n {
            v.push(f(self)?);
        }
        Some(v)
    }
}
