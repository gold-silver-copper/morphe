//! `morphe train <lang|all>…` and `morphe show <lang> <pos> <word>`.

use std::path::PathBuf;

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("..")
}

fn dumps() -> PathBuf {
    std::env::var_os("MORPHE_DUMPS")
        .map(PathBuf::from)
        .or_else(|| std::env::var_os("HOME").map(|h| PathBuf::from(h).join(".cache/kaikki")))
        .unwrap_or_else(|| PathBuf::from("dumps"))
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let langs = morphe_configs::all();
    match args.first().map(String::as_str) {
        Some("train") => {
            let threads = std::env::var("MORPHE_THREADS").ok().and_then(|t| t.parse().ok()).unwrap_or(8);
            for l in &langs {
                let spec = l.spec();
                if !args[1..].iter().any(|a| a == "all" || *a == spec.name) {
                    continue;
                }
                let dump = dumps().join(format!("{}.jsonl", spec.title));
                let date = std::fs::read_to_string(dumps().join(format!("{}.date", spec.title)))
                    .map(|s| s.trim().to_string())
                    .unwrap_or_else(|_| "unknown date".into());
                let out = root().join("crates").join(&spec.name);
                if let Err(e) = morphe_train::run(l.as_ref(), &dump, &out, &date, threads) {
                    eprintln!("{}: {e}", spec.name);
                    std::process::exit(1);
                }
            }
        }
        Some("show") => {
            let (Some(lang), Some(pos), Some(word)) = (args.get(1), args.get(2), args.get(3)) else {
                eprintln!("usage: morphe show <lang> <pos> <word>");
                std::process::exit(2);
            };
            let path = root().join("crates").join(lang).join("src/model.bin");
            let Some(m) = std::fs::read(&path).ok().and_then(|b| morphe::Model::decode(&b)) else {
                eprintln!("can't read {}", path.display());
                std::process::exit(1);
            };
            let Some(pi) = m.pos.iter().position(|p| &p.name == pos) else {
                eprintln!("no part of speech {pos}");
                std::process::exit(1);
            };
            let p = &m.pos[pi];
            println!("{word} ({pos}) → {}", m.lemma(pi, word).unwrap_or_default());
            for (fi, f) in p.inherent.iter().enumerate() {
                let vals: Vec<String> = m
                    .inherent(pi, fi, word)
                    .iter()
                    .map(|(v, s)| format!("{} {:.2}", f.values.get(*v as usize).map_or("?", |x| x.as_str()), s))
                    .collect();
                println!("  {:<28} {}", f.name, vals.join(", "));
            }
            for (ci, c) in p.cells.iter().enumerate() {
                let forms = m.inflect(pi, ci as u32, word, morphe::Options::default());
                if forms.is_empty() {
                    continue;
                }
                let shown: Vec<String> = forms.iter().take(4).map(|(f, s)| format!("{f} {s:.2}")).collect();
                println!("  {c:<28} {}", shown.join(", "));
            }
        }
        _ => eprintln!("usage: morphe train <lang|all>… | morphe show <lang> <pos> <word>"),
    }
}
