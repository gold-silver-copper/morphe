# malay

Malay inflection for Rust, learned from Wiktionary by [morphe](https://github.com/gold-silver-copper/morphe).
Inflects any word, known or made up. The `_scored` functions give every candidate with a confidence.

Generated from the Kaikki Malay dump of 2026-10-03. Do not edit by hand: change the config in morphe and regenerate.

## Example

```rust
use malay::*;
println!("{}", verb_active("lihat")); // melihat
println!("{}", verb_passive("tulis")); // ditulis
println!("{}", adj("besar", Degree::Comparative)); // lebih besar
println!("{}", noun_plural("buku")); // buku-buku
```

Verbs have the active (meN-, with its nasal assimilation) and the passive
(di-), from the headword lines of Wiktionary, which has no Malay
inflection tables; the tree reads the start of the word, where the
prefixes attach. Adjectives compare with lebih and paling. Noun plurals
are reduplication and are written by hand (`noun_plural`). Words are in
Rumi (Latin) spelling; input matching ignores the ĕ dictionaries use for
the schwa.

## Coverage

| Part of speech | Entries | Cells | Edits | Classes | Tree nodes | Lexicon |
|---|---|---|---|---|---|---|
| verb | 351 | 2 | 34 | 36 | 34 | 119 |
| adj | 1240 | 2 | 3 | 2 | 2 | 1 |

Model size: 7545 bytes.

**verb** cells: `Active`, `Passive`

**adj** cells: `Comparative`, `Superlative`

## How it works

Each attested form is stored as an edit tree from the lemma. A lemma's edits across all cells make its inflection class. A suffix tree over the word, read from its end and pruned by a description-length test, predicts the class of any word. Known words the tree gets wrong are kept in a small lexicon. Confidences are smoothed counts; variant weights follow Wiktionary's listing order (1, 1/2, 1/3…, nonstandard forms a tenth of that).

## License

Code: MIT OR Apache-2.0. Data: derived from [Wiktionary](https://www.wiktionary.org/) (via [kaikki.org](https://kaikki.org/)), CC BY-SA 4.0; the model bytes in `src/model.bin` are shared under the same license.
