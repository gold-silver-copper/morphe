# indonesian

Indonesian inflection for Rust, learned from Wiktionary by [morphe](https://github.com/gold-silver-copper/morphe).
Inflects any word, known or made up. The `_scored` functions give every candidate with a confidence.

Generated from the Kaikki Indonesian dump of 2026-10-03. Do not edit by hand: change the config in morphe and regenerate.

## Example

```rust
use indonesian::*;
println!("{}", verb_active("tulis")); // menulis
println!("{}", verb_active("pukul")); // memukul
println!("{}", verb_active("cat")); // mengecat
println!("{}", verb_passive("lihat")); // dilihat
println!("{}", verb_stative("balik")); // terbalik
println!("{}", noun_plural("buku")); // buku-buku
```

Verbs have the active (meN-, with its nasal assimilation), the passive
(di-), the stative/accidental ter- form, the imperative and the jussive
(-lah), from the headword lines of Wiktionary; the tree reads the start of
the word, where the prefixes attach. Noun plurals are reduplication and are
written by hand (`noun_plural`). Input matching ignores the ê that
dictionaries use for the schwa.

## Coverage

| Part of speech | Entries | Cells | Edits | Classes | Tree nodes | Lexicon |
|---|---|---|---|---|---|---|
| verb | 1497 | 5 | 139 | 232 | 189 | 693 |

Model size: 17844 bytes.

**verb** cells: `Active`, `Passive`, `Stative`, `Imperative`, `Jussive`

## How it works

Each attested form is stored as an edit tree from the lemma. A lemma's edits across all cells make its inflection class. A suffix tree over the word, read from its end and pruned by a description-length test, predicts the class of any word. Known words the tree gets wrong are kept in a small lexicon. Confidences are smoothed counts; variant weights follow Wiktionary's listing order (1, 1/2, 1/3…, nonstandard forms a tenth of that).

## License

Code: MIT OR Apache-2.0. Data: derived from [Wiktionary](https://www.wiktionary.org/) (via [kaikki.org](https://kaikki.org/)), CC BY-SA 4.0; the model bytes in `src/model.bin` are shared under the same license.
