//! Reading Kaikki's Wiktionary JSONL, one line at a time.

use serde::Deserialize;
use std::io::BufRead;

#[derive(Debug, Deserialize)]
pub struct Entry {
    pub word: String,
    pub pos: String,
    #[serde(default)]
    pub lang_code: String,
    /// Language name in the dump's own edition ("Russian", "Русский"), which
    /// tells entries from extra dumps apart.
    #[serde(default)]
    pub lang: String,
    #[serde(default)]
    pub forms: Vec<Form>,
    #[serde(default)]
    pub senses: Vec<Sense>,
    /// Entry-level tags (other Wiktionary editions put gender here).
    #[serde(default)]
    pub tags: Vec<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Form {
    pub form: String,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(default)]
    pub source: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Sense {
    #[serde(default)]
    pub tags: Vec<String>,
}

/// Calls `f` for every entry of the given language whose `pos` is in `pos`.
/// Lines that can't be read are counted and skipped.
pub fn read(
    path: &std::path::Path,
    lang_code: &str,
    pos: &[String],
    mut f: impl FnMut(Entry),
) -> std::io::Result<(usize, usize)> {
    let file = std::fs::File::open(path)?;
    let reader = std::io::BufReader::with_capacity(1 << 22, file);
    let needles: Vec<String> = pos.iter().map(|p| format!("\"pos\": \"{p}\"")).collect();
    let (mut used, mut bad) = (0, 0);
    for line in reader.lines() {
        let line = line?;
        if !needles.iter().any(|n| line.contains(n.as_str())) {
            continue;
        }
        match serde_json::from_str::<Entry>(&line) {
            Ok(e) if e.lang_code == lang_code && pos.contains(&e.pos) => {
                used += 1;
                f(e);
            }
            Ok(_) => {}
            Err(_) => bad += 1,
        }
    }
    Ok((used, bad))
}
