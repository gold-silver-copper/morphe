# gagauz

Gagauz inflection for Rust, learned from Wiktionary by [morphe](https://github.com/gold-silver-copper/morphe).
Inflects any word, known or made up. The `_scored` functions give every candidate with a confidence.

Generated from the Kaikki Gagauz dump of 2026-10-03. Do not edit by hand: change the config in morphe and regenerate.

## Example

```rust
use gagauz::*;
println!("{}", noun("en", Case::Ablative, Number::Plural)); // ennerdän
println!("{}", noun("ev", Case::Dative, Number::Singular));
println!("{}", verb_present_third_singular("üürenmää")); // üürenär
```

Nouns have six cases in both numbers. Wiktionary has no Gagauz
conjugation tables; verbs have only the third person present the headword
line gives.

## Coverage

| Part of speech | Entries | Cells | Edits | Classes | Tree nodes | Lexicon |
|---|---|---|---|---|---|---|
| noun | 249 | 12 | 319 | 100 | 26 | 123 |
| verb | 190 | 2 | 20 | 26 | 17 | 67 |

Model size: 19161 bytes.

**noun** cells: `NominativeSingular`, `NominativePlural`, `GenitiveSingular`, `GenitivePlural`, `DativeSingular`, `DativePlural`, `AccusativeSingular`, `AccusativePlural`, `LocativeSingular`, `LocativePlural`, `AblativeSingular`, `AblativePlural`

Citation form: `NominativeSingular, AccusativeSingular, NominativePlural` (comma-separated; parts you give are output exactly and narrow the prediction).

**verb** cells: `Infinitive`, `PresentThirdSingular`

## How it works

Each attested form is stored as an edit tree from the lemma. A lemma's edits across all cells make its inflection class. A suffix tree over the word, read from its end and pruned by a description-length test, predicts the class of any word. Known words the tree gets wrong are kept in a small lexicon. Confidences are smoothed counts; variant weights follow Wiktionary's listing order (1, 1/2, 1/3…, nonstandard forms a tenth of that).

## License

Code: MIT OR Apache-2.0. Data: derived from [Wiktionary](https://www.wiktionary.org/) (via [kaikki.org](https://kaikki.org/)), CC BY-SA 4.0; the model bytes in `src/model.bin` are shared under the same license.
