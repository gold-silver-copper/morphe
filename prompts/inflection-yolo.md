ultracode

# Prompt: YOLO inflection compiler. Every language, as fast as possible, no tests, no brakes

Build a machine that reads **Wiktionary (Kaikki dumps)** and writes **a finished
Rust crate for every language it can reach**. Each crate inflects any word of
its language, real or made up, and gives a confidence for each answer. Start
with **English and Russian**, then keep going: Latin, German, Spanish, French,
Italian, Portuguese, Polish, Czech, Ukrainian, Greek, Finnish, Hungarian,
Turkish, Arabic, Hebrew, Sanskrit, Old Church Slavonic, and anything else with
enough Wiktionary data. More languages, more parts of speech, more cells, more
features, all the time.

**This is YOLO mode.** You write no tests and no fixtures. You build no
evaluation harness, held-out split, calibration report, benchmark or accuracy
table. You don't wait for CI and don't open a PR. The only gate is
**`cargo build` succeeds**. If it compiles, ship it and move on to the next
thing. If you want to see output, run the binary on a few words by hand and
glance at it, then keep building. Never stop to measure, document or polish.

Spend tokens without holding back. Fan out subagents and workflows as wide as
you like: one per language, one per part of speech, one per feature idea, one
per refactor. While one agent trains English, others write the configs for
Latin, German and Turkish. When something works, copy it to every language.
When something doesn't, rewrite it. There is no budget and no "good enough".
Keep going until the user stops you.

You own everything: architecture, names, crates, formats, API. Nothing below is
a spec. It's the direction. Change any of it the moment you have a better idea.

---

## 1. What already exists (steal from it)

All under `~/code/linguistics/`:

- **`english/`** (`gold-silver-copper/english`, published `english` 0.4.0).
  Hand-written rules in `english-core`, Wiktionary-derived perfect-hash
  exception tables in `english`, and an `extractor` that streams the Kaikki
  dump. The extractor's editorial policy took a lot of work to get right: what
  counts as a plain form vs. nonstandard or dialectal, how listing order marks
  the canonical form, periphrastic adjectives, flat and suppletive adverbs,
  casing, acronyms, multi-word entries. Take all of it.
- **`church-slavonic/`**: `crates/church-slavonic-tools/src/sources/kaikki.rs`
  is a second Kaikki reader (sentinels, table forms, "form of", tag → case,
  number and person). `crates/church-slavonic/src/cell.rs` has typed cells with
  round-trip names. Copy whatever helps. Don't modify that repo.
- **`ruthenian/crates/sound-law`**: a generic rewrite engine with declared
  segment classes (vowels, consonants, soft/hard). Use it as a dependency or a
  model. Don't modify it in place.

Download whatever Kaikki per-language JSONL dumps you need (kaikki.org) into
`~/.cache/` or a gitignored repo directory. Grab as many languages as you like.

## 2. The machine

```
trainer    Kaikki reader + language config → (lemma, cell, form) pairs
           → edit trees → inflection classes → class predictor → emit crate
runtime    tiny shared crate: table walk, apply edits, parse citations, scored results
configs/   one per language
crates/    one generated crate per language, committed
```

- **No language names in the trainer.** Everything language-specific goes in a
  config: a Rust declarative macro (features with their Wiktionary tags, cells
  per part of speech, inherent features like gender, animacy and aspect,
  citation format, sound classes, spelling normalization such as stress marks
  and ё/е) plus a hooks trait for editorial quirks. Writing a new language
  should mean writing one config file and running one command.
- **Edits are edit trees** (spaCy/UDPipe style): suffixes, prefixes, internal
  changes, fleeting vowels, consonant alternations, stress moves.
- **Learn whole inflection classes**, not independent cells. Predict the class
  from the word: a tree over the word read from its end, then sound classes,
  syllable count, prefixes, compound heads, capitalization, inherent features.
  Prune it, smooth it, and let citations ("книга, книги", "lie, lied") narrow
  the class.
- **Generated crates are just types plus tables**: feature enums, cell types,
  typed functions (`english::noun("cat", Number::Plural)`,
  `russian::noun_scored("кофе", Case::Genitive, Number::Singular)`,
  `russian::gender("тетрадь")`), scored alternatives, and citation input.
- Want a neural model, analysis (form → lemma), derivation, sentence-level
  agreement, a CLI, a web demo, or a universal cross-language model? Build it.
  Nothing is out of scope if it makes the machine smarter.

## 3. Rules (the only ones)

These exist so the machine doesn't crash, not to slow you down.

- Never write to `/tmp` (it's RAM-backed). Use `~/.cache/claude-tmp/` or a
  gitignored repo directory.
- Don't OOM the machine (32 cores, 125 GB RAM; a runaway build has killed the
  terminal before). Stream dumps line by line, never load a whole one into
  memory. Use `cargo build -j 4` per agent, and run at most about 6 cargo
  builds at once across all agents. Spread the other work as wide as you like.
- Don't publish to crates.io, create tags or releases, or touch the existing
  `english`, `church-slavonic` and `ruthenian` repos.
- Kill every process, tmux session and background shell you started when you
  stop.

## 4. Repo and Git

- Create a new repo under `gold-silver-copper` with a short name you choose.
  Build it in `~/code/<repo-name>/`. Copy this prompt to
  `prompts/inflection-yolo.md`.
- Commit and push straight to `main` whenever it compiles. Commit often: every
  new language, every new idea. No branches, no PRs, no CI gate.
- The generated English crate takes over the `english` crate name with a
  breaking version bump. Every generated crate states Wiktionary attribution
  and the CC BY-SA license.

## 5. When the user stops you

Reply in a few lines:

- the repo URL;
- the languages that have crates, and what each covers;
- a full command starting with `cd ~/code/<repo-name>` to inflect a few words
  in each language;
- a full command to generate a crate for a new language from its dump.
