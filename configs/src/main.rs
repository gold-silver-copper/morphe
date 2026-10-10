//! `morphe train <lang|all>…` and `morphe show <lang> <pos> <word>`.

use std::path::PathBuf;

/// Print, ignoring a closed pipe (`morphe show … | head`).
macro_rules! out {
    ($($t:tt)*) => {{
        use std::io::Write as _;
        let _ = writeln!(std::io::stdout(), $($t)*);
    }};
}

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
            out!("{word} ({pos}) → {}", m.lemma(pi, word).unwrap_or_default());
            for (fi, f) in p.inherent.iter().enumerate() {
                let vals: Vec<String> = m
                    .inherent(pi, fi, word)
                    .iter()
                    .map(|(v, s)| format!("{} {:.2}", f.values.get(*v as usize).map_or("?", |x| x.as_str()), s))
                    .collect();
                out!("  {:<28} {}", f.name, vals.join(", "));
            }
            for (ci, c) in p.cells.iter().enumerate() {
                let forms = m.inflect(pi, ci as u32, word, morphe::Options::default());
                if forms.is_empty() {
                    continue;
                }
                let shown: Vec<String> = forms.iter().take(4).map(|(f, s)| format!("{f} {s:.2}")).collect();
                out!("  {c:<28} {}", shown.join(", "));
            }
        }
        Some("analyze") => {
            let (Some(lang), Some(word)) = (args.get(1), args.get(2)) else {
                eprintln!("usage: morphe analyze <lang> <form>");
                std::process::exit(2);
            };
            let path = root().join("crates").join(lang).join("src/model.bin");
            let Some(m) = std::fs::read(&path).ok().and_then(|b| morphe::Model::decode(&b)) else {
                eprintln!("can't read {}", path.display());
                std::process::exit(1);
            };
            for (a, p) in m.analyze(word).into_iter().take(10) {
                let pos = m.pos.get(a.pos).map_or("?", |p| p.name.as_str());
                let cells: Vec<&str> =
                    a.cells.iter().filter_map(|&c| m.pos.get(a.pos)?.cells.get(c as usize)).map(String::as_str).collect();
                let cells = if cells.is_empty() { "(dictionary form)".to_string() } else { cells.join(", ") };
                out!("  {p:.3}  {pos:<6} {:<20} {cells}", a.lemma);
            }
        }
        Some("playground") => {
            let Some(out) = args.get(1).map(PathBuf::from) else {
                eprintln!("usage: morphe playground <out dir>  (build morphe-wasm for wasm32 first)");
                std::process::exit(2);
            };
            let models = out.join("models");
            let _ = std::fs::create_dir_all(&models);
            let r = root();
            let copy = |from: PathBuf, to: PathBuf| {
                if let Err(e) = std::fs::copy(&from, &to) {
                    eprintln!("{}: {e}", from.display());
                    std::process::exit(1);
                }
            };
            copy(r.join("playground/index.html"), out.join("index.html"));
            copy(r.join("target/wasm32-unknown-unknown/release/morphe_wasm.wasm"), out.join("morphe.wasm"));
            let mut entries = Vec::new();
            let mut specs: Vec<_> = langs.iter().map(|l| (l.spec(), l.samples())).collect();
            specs.sort_by(|a, b| a.0.title.cmp(&b.0.title));
            for (spec, samples) in specs {
                let bin = r.join("crates").join(&spec.name).join("src/model.bin");
                let Ok(meta) = std::fs::metadata(&bin) else { continue };
                // Served as base64 text: artifact hosts don't serve raw binary.
                let Ok(bytes) = std::fs::read(&bin) else { continue };
                if let Err(e) = std::fs::write(models.join(format!("{}.txt", spec.name)), base64(&bytes)) {
                    eprintln!("{e}");
                    std::process::exit(1);
                }
                // No samples in the config: a few known irregular words,
                // spread evenly through the first part of speech's lexicon.
                let samples: Vec<String> = if samples.is_empty() {
                    morphe::Model::decode(&bytes)
                        .and_then(|m| {
                            let lex = &m.pos.first()?.lexicon;
                            let n = lex.len();
                            Some((1..=5).filter_map(|i| lex.get(i * n / 6)).map(|(k, _)| json(k)).collect())
                        })
                        .unwrap_or_default()
                } else {
                    samples.iter().map(|s| json(s)).collect()
                };
                entries.push(format!(
                    "{{\"name\":{},\"title\":{},\"file\":\"models/{}.txt\",\"bytes\":{},\"samples\":[{}]}}",
                    json(&spec.name),
                    json(&spec.title),
                    spec.name,
                    meta.len(),
                    samples.join(",")
                ));
            }
            let _ = std::fs::write(out.join("models.json"), format!("[{}]", entries.join(",")));
            out!("{} languages → {}", entries.len(), out.display());
        }
        Some("summary") => {
            // Rebuild the languages table in README.md from each crate's README.
            let r = root();
            let mut rows = Vec::new();
            let mut specs: Vec<_> = langs.iter().map(|l| l.spec()).collect();
            specs.sort_by(|a, b| a.title.cmp(&b.title));
            let (mut tot_entries, mut tot_bytes) = (0usize, 0u64);
            for spec in &specs {
                let dir = r.join("crates").join(&spec.name);
                let Ok(readme) = std::fs::read_to_string(dir.join("README.md")) else { continue };
                let bytes = std::fs::metadata(dir.join("src/model.bin")).map(|m| m.len()).unwrap_or(0);
                let mut poses = Vec::new();
                let (mut entries, mut cells) = (0usize, 0usize);
                for line in readme.lines() {
                    let cols: Vec<&str> = line.split('|').map(str::trim).collect();
                    if cols.len() >= 8 && cols[2].parse::<usize>().is_ok() {
                        poses.push(cols[1].to_string());
                        entries += cols[2].parse::<usize>().unwrap_or(0);
                        cells += cols[3].parse::<usize>().unwrap_or(0);
                    }
                }
                tot_entries += entries;
                tot_bytes += bytes;
                rows.push(format!(
                    "| {} | `{}` | {} | {} | {} | {:.1} MB |",
                    spec.title,
                    spec.name,
                    poses.join(", "),
                    entries,
                    cells,
                    bytes as f64 / 1e6
                ));
            }
            let table = format!(
                "<!-- languages -->\n{} languages, {} dictionary entries, {:.0} MB of models.\n\n| Language | Crate | Parts of speech | Entries | Cells | Model |\n|---|---|---|---|---|---|\n{}\n<!-- /languages -->",
                rows.len(),
                tot_entries,
                tot_bytes as f64 / 1e6,
                rows.join("\n")
            );
            let path = r.join("README.md");
            let readme = std::fs::read_to_string(&path).unwrap_or_default();
            let new = match (readme.find("<!-- languages -->"), readme.find("<!-- /languages -->")) {
                (Some(a), Some(b)) => format!("{}{}{}", &readme[..a], table, &readme[b + "<!-- /languages -->".len()..]),
                _ => readme.replacen("## Layout", &format!("## Languages\n\n{table}\n\n## Layout"), 1),
            };
            let _ = std::fs::write(&path, new);
            out!("{} languages", rows.len());
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
    out!("lang_code: {lang}");
    let mut poses: Vec<(&String, &usize)> = entries.iter().collect();
    poses.sort_by(|a, b| b.1.cmp(a.1));
    for (pos, n) in poses {
        out!("\n=== pos {pos}: {n} entries");
        let mut keys: Vec<&(String, String)> = stats.keys().filter(|k| &k.0 == pos).collect();
        keys.sort();
        for k in keys {
            let mut v: Vec<(&String, &(usize, String))> = stats[k].iter().collect();
            v.sort_by(|a, b| b.1.0.cmp(&a.1.0).then(a.0.cmp(b.0)));
            out!("  -- source {} ({} tag sets)", k.1, v.len());
            for (t, (c, ex)) in v.into_iter().take(60) {
                out!("    {c:>8}  [{t}]  e.g. {ex}");
            }
        }
    }
}

fn base64(bytes: &[u8]) -> String {
    const T: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut out = String::with_capacity(bytes.len().div_ceil(3) * 4);
    for chunk in bytes.chunks(3) {
        let b = [chunk[0], *chunk.get(1).unwrap_or(&0), *chunk.get(2).unwrap_or(&0)];
        let n = (u32::from(b[0]) << 16) | (u32::from(b[1]) << 8) | u32::from(b[2]);
        for i in 0..4 {
            if i <= chunk.len() {
                out.push(T[((n >> (18 - 6 * i)) & 63) as usize] as char);
            } else {
                out.push('=');
            }
        }
    }
    out
}

/// A JSON string literal.
fn json(s: &str) -> String {
    let mut o = String::from("\"");
    for c in s.chars() {
        match c {
            '"' => o.push_str("\\\""),
            '\\' => o.push_str("\\\\"),
            c if (c as u32) < 0x20 => o.push_str(&format!("\\u{:04x}", c as u32)),
            c => o.push(c),
        }
    }
    o.push('"');
    o
}
