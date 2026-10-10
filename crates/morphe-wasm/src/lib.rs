//! A C-style interface over the morphe runtime for the browser playground:
//! load models from bytes, then ask for a word's full paradigm and analyses
//! as JSON.

use morphe::{Model, Options};
use std::fmt::Write as _;
use std::sync::Mutex;

static MODELS: Mutex<Vec<Model>> = Mutex::new(Vec::new());
static OUT: Mutex<Vec<u8>> = Mutex::new(Vec::new());

/// Memory for the host to write input into.
#[unsafe(no_mangle)]
pub extern "C" fn alloc(len: usize) -> *mut u8 {
    let mut v = Vec::<u8>::with_capacity(len);
    let p = v.as_mut_ptr();
    std::mem::forget(v);
    p
}

/// # Safety
/// `ptr` must come from `alloc(len)`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn dealloc(ptr: *mut u8, len: usize) {
    // SAFETY: allocated by `alloc` with this capacity.
    drop(unsafe { Vec::from_raw_parts(ptr, 0, len) });
}

fn input<'a>(ptr: *const u8, len: usize) -> &'a [u8] {
    // SAFETY: the host passes a buffer it wrote through `alloc`.
    unsafe { std::slice::from_raw_parts(ptr, len) }
}

/// Load a model; returns its handle, or -1.
#[unsafe(no_mangle)]
pub extern "C" fn load(ptr: *const u8, len: usize) -> i32 {
    let Some(m) = Model::decode(input(ptr, len)) else { return -1 };
    let Ok(mut models) = MODELS.lock() else { return -1 };
    models.push(m);
    models.len() as i32 - 1
}

fn esc(s: &str) -> String {
    let mut o = String::with_capacity(s.len() + 2);
    o.push('"');
    for c in s.chars() {
        match c {
            '"' => o.push_str("\\\""),
            '\\' => o.push_str("\\\\"),
            c if (c as u32) < 0x20 => {
                let _ = write!(o, "\\u{:04x}", c as u32);
            }
            c => o.push(c),
        }
    }
    o.push('"');
    o
}

/// The paradigm of a word in every part of speech, plus its analyses, as
/// JSON. Returns the length of the result; read it from `out_ptr()`.
pub fn query_json(m: &Model, word: &str, marks: bool) -> String {
    let opts = Options { marks };
    let mut s = String::from("{\"pos\":[");
    for (pi, p) in m.pos.iter().enumerate() {
        if pi > 0 {
            s.push(',');
        }
        let _ = write!(s, "{{\"name\":{},\"lemma\":{},\"inherent\":[", esc(&p.name), esc(&m.lemma(pi, word).unwrap_or_default()));
        for (fi, f) in p.inherent.iter().enumerate() {
            if fi > 0 {
                s.push(',');
            }
            let vals: Vec<String> = m
                .inherent(pi, fi, word)
                .iter()
                .take(3)
                .map(|(v, pr)| format!("[{},{pr:.3}]", esc(f.values.get(*v as usize).map_or("?", |x| x.as_str()))))
                .collect();
            let _ = write!(s, "{{\"name\":{},\"values\":[{}]}}", esc(&f.name), vals.join(","));
        }
        s.push_str("],\"cells\":[");
        let mut first = true;
        for (ci, c) in p.cells.iter().enumerate() {
            let forms = m.inflect(pi, ci as u32, word, opts);
            if forms.is_empty() {
                continue;
            }
            if !first {
                s.push(',');
            }
            first = false;
            let fs: Vec<String> = forms.iter().take(4).map(|(f, pr)| format!("[{},{pr:.3}]", esc(f))).collect();
            let _ = write!(s, "{{\"cell\":{},\"forms\":[{}]}}", esc(c), fs.join(","));
        }
        s.push_str("]}");
    }
    s.push_str("],\"analyses\":[");
    let an: Vec<String> = m
        .analyze(word)
        .into_iter()
        .take(8)
        .map(|(a, pr)| {
            let pos = m.pos.get(a.pos).map_or("?", |p| p.name.as_str());
            let cells: Vec<String> = a
                .cells
                .iter()
                .filter_map(|&c| m.pos.get(a.pos)?.cells.get(c as usize))
                .map(|c| esc(c))
                .collect();
            format!("{{\"pos\":{},\"lemma\":{},\"cells\":[{}],\"p\":{pr:.3}}}", esc(pos), esc(&a.lemma), cells.join(","))
        })
        .collect();
    s.push_str(&an.join(","));
    s.push_str("]}");
    s
}

#[unsafe(no_mangle)]
pub extern "C" fn query(model: i32, ptr: *const u8, len: usize, marks: i32) -> usize {
    let word = String::from_utf8_lossy(input(ptr, len)).into_owned();
    let json = match MODELS.lock() {
        Ok(models) => match models.get(model as usize) {
            Some(m) => query_json(m, &word, marks != 0),
            None => "{\"error\":\"no model\"}".into(),
        },
        Err(_) => "{\"error\":\"lock\"}".into(),
    };
    let Ok(mut out) = OUT.lock() else { return 0 };
    *out = json.into_bytes();
    out.len()
}

#[unsafe(no_mangle)]
pub extern "C" fn out_ptr() -> *const u8 {
    OUT.lock().map(|o| o.as_ptr()).unwrap_or(std::ptr::null())
}
