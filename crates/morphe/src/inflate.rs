//! A small raw-DEFLATE (RFC 1951) decoder, so models can ship compressed
//! without a dependency. Returns `None` on malformed input; never panics.

struct Bits<'a> {
    data: &'a [u8],
    pos: usize,
    bit: u32,
    nbits: u32,
}

impl<'a> Bits<'a> {
    fn need(&mut self, n: u32) -> Option<()> {
        while self.nbits < n {
            let b = *self.data.get(self.pos)?;
            self.pos += 1;
            self.bit |= u32::from(b) << self.nbits;
            self.nbits += 8;
        }
        Some(())
    }
    fn bits(&mut self, n: u32) -> Option<u32> {
        if n == 0 {
            return Some(0);
        }
        self.need(n)?;
        let v = self.bit & ((1u32 << n) - 1);
        self.bit >>= n;
        self.nbits -= n;
        Some(v)
    }
    fn align(&mut self) {
        let r = self.nbits % 8;
        self.bit >>= r;
        self.nbits -= r;
    }
}

/// Canonical Huffman table: counts per length and symbols in code order.
struct Huffman {
    counts: [u16; 16],
    symbols: Vec<u16>,
}

impl Huffman {
    fn new(lengths: &[u8]) -> Option<Huffman> {
        let mut counts = [0u16; 16];
        for &l in lengths {
            *counts.get_mut(usize::from(l))? += 1;
        }
        counts[0] = 0;
        let mut offs = [0u16; 16];
        for i in 1..16 {
            offs[i] = offs[i - 1] + counts[i - 1];
        }
        let mut symbols = vec![0u16; lengths.len()];
        for (sym, &l) in lengths.iter().enumerate() {
            if l != 0 {
                let o = offs.get_mut(usize::from(l))?;
                *symbols.get_mut(usize::from(*o))? = sym as u16;
                *o += 1;
            }
        }
        Some(Huffman { counts, symbols })
    }

    fn decode(&self, b: &mut Bits) -> Option<u16> {
        let (mut code, mut first, mut index) = (0i32, 0i32, 0i32);
        for len in 1..16 {
            code |= b.bits(1)? as i32;
            let count = i32::from(self.counts[len]);
            if code - count < first {
                return self.symbols.get((index + code - first) as usize).copied();
            }
            index += count;
            first += count;
            first <<= 1;
            code <<= 1;
        }
        None
    }
}

const LEN_BASE: [u16; 29] =
    [3, 4, 5, 6, 7, 8, 9, 10, 11, 13, 15, 17, 19, 23, 27, 31, 35, 43, 51, 59, 67, 83, 99, 115, 131, 163, 195, 227, 258];
const LEN_EXTRA: [u8; 29] = [0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 2, 2, 2, 2, 3, 3, 3, 3, 4, 4, 4, 4, 5, 5, 5, 5, 0];
const DIST_BASE: [u16; 30] = [
    1, 2, 3, 4, 5, 7, 9, 13, 17, 25, 33, 49, 65, 97, 129, 193, 257, 385, 513, 769, 1025, 1537, 2049, 3073, 4097, 6145,
    8193, 12289, 16385, 24577,
];
const DIST_EXTRA: [u8; 30] = [0, 0, 0, 0, 1, 1, 2, 2, 3, 3, 4, 4, 5, 5, 6, 6, 7, 7, 8, 8, 9, 9, 10, 10, 11, 11, 12, 12, 13, 13];

fn block(b: &mut Bits, out: &mut Vec<u8>, lit: &Huffman, dist: &Huffman) -> Option<()> {
    loop {
        let sym = lit.decode(b)?;
        match sym {
            0..=255 => out.push(sym as u8),
            256 => return Some(()),
            _ => {
                let i = usize::from(sym - 257);
                let len = usize::from(*LEN_BASE.get(i)?) + b.bits(u32::from(*LEN_EXTRA.get(i)?))? as usize;
                let d = usize::from(dist.decode(b)?);
                let back = usize::from(*DIST_BASE.get(d)?) + b.bits(u32::from(*DIST_EXTRA.get(d)?))? as usize;
                let start = out.len().checked_sub(back)?;
                for k in 0..len {
                    let c = *out.get(start + k)?;
                    out.push(c);
                }
            }
        }
    }
}

/// Decompress a raw DEFLATE stream.
pub fn inflate(data: &[u8]) -> Option<Vec<u8>> {
    let mut b = Bits { data, pos: 0, bit: 0, nbits: 0 };
    let mut out = Vec::with_capacity(data.len() * 3);
    loop {
        let last = b.bits(1)?;
        match b.bits(2)? {
            0 => {
                b.align();
                let len = b.bits(16)? as usize;
                let _nlen = b.bits(16)?;
                // Remaining whole bytes are now aligned in the bit buffer.
                for _ in 0..len {
                    out.push(b.bits(8)? as u8);
                }
            }
            1 => {
                let mut l = [0u8; 288];
                for (i, x) in l.iter_mut().enumerate() {
                    *x = match i {
                        0..=143 => 8,
                        144..=255 => 9,
                        256..=279 => 7,
                        _ => 8,
                    };
                }
                let lit = Huffman::new(&l)?;
                let dist = Huffman::new(&[5u8; 30])?;
                block(&mut b, &mut out, &lit, &dist)?;
            }
            2 => {
                let hlit = b.bits(5)? as usize + 257;
                let hdist = b.bits(5)? as usize + 1;
                let hclen = b.bits(4)? as usize + 4;
                const ORDER: [usize; 19] = [16, 17, 18, 0, 8, 7, 9, 6, 10, 5, 11, 4, 12, 3, 13, 2, 14, 1, 15];
                let mut cl = [0u8; 19];
                for &o in ORDER.iter().take(hclen) {
                    *cl.get_mut(o)? = b.bits(3)? as u8;
                }
                let clh = Huffman::new(&cl)?;
                let mut lengths = Vec::with_capacity(hlit + hdist);
                while lengths.len() < hlit + hdist {
                    match clh.decode(&mut b)? {
                        s @ 0..=15 => lengths.push(s as u8),
                        16 => {
                            let prev = *lengths.last()?;
                            for _ in 0..3 + b.bits(2)? {
                                lengths.push(prev);
                            }
                        }
                        17 => lengths.extend(std::iter::repeat_n(0u8, 3 + b.bits(3)? as usize)),
                        _ => lengths.extend(std::iter::repeat_n(0u8, 11 + b.bits(7)? as usize)),
                    }
                }
                let lit = Huffman::new(lengths.get(..hlit)?)?;
                let dist = Huffman::new(lengths.get(hlit..hlit + hdist)?)?;
                block(&mut b, &mut out, &lit, &dist)?;
            }
            _ => return None,
        }
        if last == 1 {
            return Some(out);
        }
    }
}
