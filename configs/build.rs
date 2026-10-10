//! Registers every language in `src/langs/*.rs`. Each file defines
//! `pub fn language() -> Box<dyn Language>`.

use std::fmt::Write as _;

fn main() {
    let dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src/langs");
    println!("cargo:rerun-if-changed={}", dir.display());
    let mut names: Vec<String> = std::fs::read_dir(&dir)
        .map(|rd| {
            rd.filter_map(|e| e.ok())
                .map(|e| e.path())
                .filter(|p| p.extension().is_some_and(|x| x == "rs"))
                .filter_map(|p| p.file_stem().and_then(|s| s.to_str()).map(String::from))
                .filter(|n| !n.ends_with("_extra"))
                .collect()
        })
        .unwrap_or_default();
    names.sort();
    let mut s = String::new();
    for n in &names {
        let _ = writeln!(s, "#[path = {:?}]\npub mod {n};", dir.join(format!("{n}.rs")).display().to_string());
    }
    let _ = writeln!(s, "pub fn all() -> Vec<Box<dyn morphe_train::spec::Language>> {{\n    vec![{}]\n}}", names.iter().map(|n| format!("{n}::language()")).collect::<Vec<_>>().join(", "));
    let out = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap_or_default()).join("langs.rs");
    let _ = std::fs::write(out, s);
}
