//! Learns a language's inflection from a Kaikki Wiktionary dump plus a
//! language description, and writes a finished Rust crate.
//!
//! Nothing in here knows any particular language: everything specific lives
//! in the config (`spec::Language`).

pub mod emit;
pub mod extract;
pub mod kaikki;
pub mod learn;
pub mod spec;
pub mod tree;

use std::path::Path;

/// Train `lang` on `dump` and write its crate into `out`.
pub fn run(lang: &dyn spec::Language, dump: &Path, out: &Path, dump_date: &str, threads: usize) -> std::io::Result<()> {
    let t = std::time::Instant::now();
    let mut ex = extract::Extractor::new(lang);
    let all_pos: Vec<String> = ex.spec.pos.iter().flat_map(|p| p.kaikki.clone()).collect();
    let code = ex.spec.lang_code.clone();
    let (used, bad) = kaikki::read(dump, &code, &all_pos, |mut e| {
        lang.prepare(&mut e);
        ex.add(&e)
    })?;
    eprintln!("[{}] read {used} entries ({bad} unreadable) in {:.1?}", ex.spec.name, t.elapsed());
    let (model, stats) = learn::learn(&ex, threads);
    emit::emit(lang, &ex.spec, &ex.cells, &model, &stats, out, dump_date)?;
    eprintln!("[{}] wrote {} in {:.1?}", ex.spec.name, out.display(), t.elapsed());
    Ok(())
}
