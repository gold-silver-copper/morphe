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
    check(&ex.spec, &ex.cells)?;
    let all_pos: Vec<String> = ex.spec.pos.iter().flat_map(|p| p.kaikki.clone()).collect();
    let code = ex.spec.lang_code.clone();
    let (used, bad) = kaikki::read(dump, &code, &all_pos, |mut e| {
        lang.prepare(&mut e);
        ex.add(&e)
    })?;
    eprintln!("[{}] read {used} entries ({bad} unreadable) in {:.1?}", ex.spec.name, t.elapsed());
    // Other Wiktionary editions fill in words the main dump lacks.
    let dir = dump.parent().unwrap_or(Path::new("."));
    ex.extra = true;
    for stem in ex.spec.extra_dumps.clone() {
        let path = dir.join(format!("{stem}.jsonl"));
        match kaikki::read(&path, &code, &all_pos, |mut e| {
            lang.prepare_extra(&stem, &mut e);
            ex.add(&e)
        }) {
            Ok((used, _)) => eprintln!("[{}] read {used} entries from {stem} in {:.1?}", ex.spec.name, t.elapsed()),
            Err(e) => eprintln!("[{}] skipping {stem}: {e}", ex.spec.name),
        }
    }
    let (model, stats) = learn::learn(&ex, threads);
    emit::emit(lang, &ex.spec, &ex.cells, &model, &stats, out, dump_date)?;
    eprintln!("[{}] wrote {} in {:.1?}", ex.spec.name, out.display(), t.elapsed());
    Ok(())
}

/// Config mistakes that would generate a broken crate: duplicate cell names,
/// duplicate function names, unknown features, citations naming no cell.
fn check(spec: &spec::Spec, cells: &[Vec<spec::Cell>]) -> std::io::Result<()> {
    let bad = |m: String| Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, format!("{}: {m}", spec.name)));
    let mut funcs: Vec<String> = Vec::new();
    for (ps, cs) in spec.pos.iter().zip(cells) {
        let mut names: Vec<&str> = cs.iter().map(|c| c.name.as_str()).collect();
        names.sort_unstable();
        if let Some(w) = names.windows(2).find(|w| w[0] == w[1]) {
            return bad(format!("{}: cell {} appears twice", ps.name, w[0]));
        }
        for g in &ps.groups {
            for (f, _, _) in &g.features {
                if spec.feature(f).is_none() {
                    return bad(format!("{}: group {:?} uses unknown feature {f}", ps.name, g.name));
                }
            }
            let fname = if g.name.is_empty() { ps.name.clone() } else { format!("{}_{}", ps.name, emit::snake(&g.name)) };
            if funcs.contains(&fname) {
                return bad(format!("two groups generate the function {fname}"));
            }
            funcs.push(fname);
        }
        for (i, alts) in ps.citation.iter().enumerate().skip(1) {
            for a in alts {
                if !cs.iter().any(|c| &c.name == a) {
                    return bad(format!("{}: citation part {i} names unknown cell {a}", ps.name));
                }
            }
        }
    }
    Ok(())
}
