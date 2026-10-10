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
        Some("tags") => {
            let Some(title) = args.get(1) else {
                eprintln!("usage: morphe tags <Kaikki title, e.g. Russian> [pos…]");
                std::process::exit(2);
            };
            tags(title, &args[2..]);
        }
        _ => eprintln!("usage: morphe train <lang|all>… | morphe show <lang> <pos> <word> | morphe tags <Title> [pos…]"),
    }
}

/// Tag sets of forms per part of speech, most common first, with examples:
/// what a config is written from.
fn tags(title: &str, only: &[String]) {
    use std::collections::HashMap;
    use std::io::BufRead;
    let path = dumps().join(format!("{title}.jsonl"));
    let Ok(file) = std::fs::File::open(&path) else {
        eprintln!("can't open {}", path.display());
        std::process::exit(1);
    };
    // (pos, source) → tag set → (count, example)
    let mut stats: HashMap<(String, String), HashMap<String, (usize, String)>> = HashMap::new();
    let mut entries: HashMap<String, usize> = HashMap::new();
    let mut lang = String::new();
    for line in std::io::BufReader::with_capacity(1 << 22, file).lines().map_while(Result::ok) {
        let Ok(e) = serde_json::from_str::<morphe_train::kaikki::Entry>(&line) else { continue };
        if lang.is_empty() {
            lang.clone_from(&e.lang_code);
        }
        if e.lang_code != lang || (!only.is_empty() && !only.contains(&e.pos)) {
            continue;
        }
        *entries.entry(e.pos.clone()).or_default() += 1;
        for f in &e.forms {
            let src = f.source.clone().unwrap_or_else(|| "head".into());
            let mut t = f.tags.clone();
            t.sort();
            let slot = stats.entry((e.pos.clone(), src)).or_default().entry(t.join(" ")).or_insert((0, String::new()));
            slot.0 += 1;
            if slot.1.is_empty() {
                slot.1 = format!("{} → {}", e.word, f.form);
            }
        }
    }
    println!("lang_code: {lang}");
    let mut poses: Vec<(&String, &usize)> = entries.iter().collect();
    poses.sort_by(|a, b| b.1.cmp(a.1));
    for (pos, n) in poses {
        println!("\n=== pos {pos}: {n} entries");
        let mut keys: Vec<&(String, String)> = stats.keys().filter(|k| &k.0 == pos).collect();
        keys.sort();
        for k in keys {
            let mut v: Vec<(&String, &(usize, String))> = stats[k].iter().collect();
            v.sort_by(|a, b| b.1.0.cmp(&a.1.0).then(a.0.cmp(b.0)));
            println!("  -- source {} ({} tag sets)", k.1, v.len());
            for (t, (c, ex)) in v.into_iter().take(60) {
                println!("    {c:>8}  [{t}]  e.g. {ex}");
            }
        }
    }
}
