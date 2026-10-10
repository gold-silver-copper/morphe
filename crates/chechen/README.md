# chechen

Chechen inflection for Rust, learned from Wiktionary by [morphe](https://github.com/gold-silver-copper/morphe).
Inflects any word, known or made up. The `_scored` functions give every candidate with a confidence.

Generated from the Kaikki Chechen dump of 2026-10-03. Do not edit by hand: change the config in morphe and regenerate.

## Example

```rust
use chechen::*;
println!("{}", noun("у", Case::Genitive, Number::Singular)); // уьнан
println!("{}", noun("у", Case::Nominative, Number::Plural)); // аннаш
println!("{:?}", noun_class("у")); // DD
```

Nouns have eight cases in both numbers, and numerals the eight cases;
nouns also have their
gender class in the singular and plural (JJ: й/й, BD: б/д, VB: в/б…).
Wiktionary marks long vowels (уьно̄) the standard spelling leaves out:
input matching ignores them, and `marks(false)` removes them from output.
Input may write the palochka as ӏ, I or 1.

## Coverage

| Part of speech | Entries | Cells | Edits | Classes | Tree nodes | Lexicon |
|---|---|---|---|---|---|---|
| noun | 358 | 16 | 1686 | 206 | 54 | 240 |
| num | 24 | 8 | 56 | 16 | 3 | 16 |

Model size: 54715 bytes.

**noun** cells: `NominativeSingular`, `NominativePlural`, `GenitiveSingular`, `GenitivePlural`, `DativeSingular`, `DativePlural`, `ErgativeSingular`, `ErgativePlural`, `AllativeSingular`, `AllativePlural`, `InstrumentalSingular`, `InstrumentalPlural`, `LativeSingular`, `LativePlural`, `ComparativeSingular`, `ComparativePlural`

Citation form: `NominativeSingular, GenitiveSingular` (comma-separated; parts you give are output exactly and narrow the prediction).

**num** cells: `Nominative`, `Genitive`, `Dative`, `Ergative`, `Allative`, `Instrumental`, `Lative`, `Comparative`

## How it works

Each attested form is stored as an edit tree from the lemma. A lemma's edits across all cells make its inflection class. A suffix tree over the word, read from its end and pruned by a description-length test, predicts the class of any word. Known words the tree gets wrong are kept in a small lexicon. Confidences are smoothed counts; variant weights follow Wiktionary's listing order (1, 1/2, 1/3…, nonstandard forms a tenth of that).

## License

Code: MIT OR Apache-2.0. Data: derived from [Wiktionary](https://www.wiktionary.org/) (via [kaikki.org](https://kaikki.org/)), CC BY-SA 4.0; the model bytes in `src/model.bin` are shared under the same license.
