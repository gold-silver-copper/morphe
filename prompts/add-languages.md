# Add languages to morphe (YOLO mode)

You're one of several agents adding languages to **morphe**
(`gold-silver-copper/morphe`), a generic trainer that reads a Kaikki
Wiktionary dump plus a Rust language config and emits a finished Rust crate
that inflects any word of that language, with confidences. YOLO mode: you
write no tests and no evaluation. The only gate is that `cargo build` passes.
Build, eyeball a few words, push, move on to the next language.

## How morphe works (read these first, they're short)
- `configs/src/langs/english.rs`, `configs/src/langs/russian.rs`: two
  complete configs. A config is `impl Language` with `spec()` plus optional
  hooks, and `pub fn language() -> Box<dyn Language>`. Any file in
  `configs/src/langs/` is registered automatically.
- `crates/morphe-train/src/spec.rs`: the config vocabulary.
  - `feature(name, &[(Value, "tag tag !forbidden")])` declares a feature.
  - `group(name, fixed_tags, "Feature Feature[Subset,Values] Loose?")`
    declares a product of features. A cell's name is the group name plus its
    value names. A loose feature (`?`) also matches forms that name none of
    its values.
  - `pos(rust_name, kaikki_pos, groups)` declares a part of speech, with
    options:
    - `.inherent("Gender")`: features predicted for the word itself, read
      from the canonical form's tags (or else the sense tags);
    - `.citation("Lemma CellA|CellB CellC")`: principal parts, where `Lemma`
      is a placeholder for the first part;
    - `.syllables(vowels, cap)`: an extra symbol for the tree;
    - `.head_first()` / `.head_last()`: which word of an unknown phrase
      inflects.
  - `Spec` fields:
    - `drop_tags`: forms with these tags are ignored;
    - `soft_tags`: low-weight alternatives;
    - `marks`: combining marks users don't type;
    - `unmark`: precomposed marked letter → plain letter, e.g. Latin macrons,
      Greek accents;
    - `fold`: letters written interchangeably, for matching only;
    - `source`: Head, Table or Merge.
- More generic options, added since this prompt was first written:
  - PosSpec: `.initial(n)` (the tree also reads the first n letters, for
    prefixing languages) and `.lemma_cell("Cell")` (fills that cell with the
    lemma when a table omits it).
  - Spec: `first_table_only` (ignore later tables), `fold_strs`
    (multi-letter folds such as ß → ss) and `canonical_spelling`.
  - Hooks: `prepare(&mut Entry)`, `reshape`, `retag(pos, form, tags)`,
    `split_form` and `samples()` (playground words).
  - A "-" in a table is now stored as "this form doesn't exist" rather than
    being treated as unknown.
- `crates/morphe-train/src/extract.rs`: how forms become cells. A form goes to
  the cell(s) whose tags it matches with the most matched tags.
- `./target/release/morphe tags <Title> [pos…]` prints the tag sets of every
  part of speech in a dump, with examples. Write the config from it.
- `./target/release/morphe train <name>` writes `crates/<name>/` in seconds.
- `./target/release/morphe show <name> <pos> <word>` prints a word's whole
  paradigm, inherent features and confidences. Try real words and made-up
  ones.
- Dumps are at `~/.cache/kaikki/<Title>.jsonl`, with the date in
  `<Title>.date`. Titles have no spaces: `OldChurchSlavonic`,
  `AncientGreek`, `SerboCroatian`. Some are still downloading (a `.part`
  file means in progress). Work on the ones present first. If one is missing
  after the others are done, download it yourself:
  `curl -sfL -o ~/.cache/kaikki/<Title>.jsonl "https://kaikki.org/dictionary/<Name%20With%20Spaces>/kaikki.org-dictionary-<Title>.jsonl"`,
  plus `echo 2026-10-03 > ~/.cache/kaikki/<Title>.date`.

## Your job
For each language assigned to you, as many as you can:
1. Run `morphe tags <Title>` on its main parts of speech (noun, verb, adj, plus
   pron/num/det/article where they have tables).
2. Write `configs/src/langs/<name>.rs`.
   - `<name>` is the crate name: the lowercase language name, with
     underscores (`ancient_greek`, `old_church_slavonic`).
   - Cover every inflected part of speech and every cell the data has.
     Include inherent features where the canonical tags carry them (noun
     gender, verb aspect…), and a citation format where the language has a
     traditional one.
   - Use `unmark` or `marks` so that input without diacritics works when the
     dictionary spelling has them (Latin macrons, Greek accents, Arabic
     harakat, Hebrew niqqud, Sanskrit/Lithuanian accents…), and `fold` for
     interchangeable letters.
   - Drop `dated`, `archaic` or obsolete tables and forms the way the Russian
     config drops pre-reform tables.
   - Write a `readme()` with a short example.
3. Train it, look at `show` output for 5–10 real words and 2–3 made-up words,
   and fix the config until the paradigms look right to you. If a cell comes
   out empty, a tag mapping is wrong.
4. `cargo build --release -j 4` must pass for the whole workspace.
5. Commit only your language's files (`configs/src/langs/<name>*.rs`,
   `crates/<name>/`). Then `git pull --rebase origin main && git push origin HEAD:main`.
   Do that after every language. Don't wait to batch them.

## Rules
- The trainer and runtime must stay free of language names. Language quirks
  go in the config: hooks such as `keep_entry` and `adjust`, or hand-written
  code via `extra_code()`.
  - You may add a new **default-method hook** to the `Language` trait, or a
    new generic `Spec` option, when a language truly needs it. Keep it
    additive, so that existing configs and crates don't change.
  - **Never change the runtime's binary model format** (`crates/morphe/src/model.rs`
    encode/decode) or anything that changes existing languages' output. If
    you think the core needs a change, say so in your final report instead.
- Never write to `/tmp` (RAM-backed). Use `~/.cache/claude-tmp/`.
- Use `cargo build -j 4`, never more. Don't start other heavy jobs. Stream
  dumps. Never load one whole.
- Don't publish crates, create tags, or touch other repos.
- No tests, no fixtures, no benchmarks.
- When done, kill any processes you started. Your final report (a few lines
  per language) should cover:
  - parts of speech and number of cells;
  - records, classes and lexicon size, from the train output;
  - 2–3 sample outputs;
  - anything the core should do better.
