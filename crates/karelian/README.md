# karelian

Karelian inflection for Rust, learned from Wiktionary by [morphe](https://github.com/gold-silver-copper/morphe).
Inflects any word, known or made up. The `_scored` functions give every candidate with a confidence.

Generated from the Kaikki Karelian dump of 2026-10-03. Do not edit by hand: change the config in morphe and regenerate.

## Example

```rust
use karelian::*;
println!("{}", noun("kala", Case::Inessive, Number::Singular)); // kalašša
println!("{}", noun("kala", Case::Partitive, Number::Plural)); // kaloja
```

North Karelian (Viena) nouns, adjectives, pronouns and numerals with
fourteen cases in both numbers. Wiktionary has no Karelian verb tables.

## Coverage

| Part of speech | Entries | Cells | Edits | Classes | Tree nodes | Lexicon |
|---|---|---|---|---|---|---|
| noun | 646 | 28 | 1832 | 275 | 97 | 387 |
| adj | 73 | 28 | 563 | 46 | 20 | 44 |
| pron | 23 | 28 | 361 | 29 | 2 | 19 |
| num | 15 | 28 | 257 | 14 | 2 | 6 |

Model size: 128173 bytes.

**noun** cells: `NominativeSingular`, `NominativePlural`, `GenitiveSingular`, `GenitivePlural`, `PartitiveSingular`, `PartitivePlural`, `IllativeSingular`, `IllativePlural`, `InessiveSingular`, `InessivePlural`, `ElativeSingular`, `ElativePlural`, `AdessiveSingular`, `AdessivePlural`, `AblativeSingular`, `AblativePlural`, `TranslativeSingular`, `TranslativePlural`, `EssiveSingular`, `EssivePlural`, `ComitativeSingular`, `ComitativePlural`, `AbessiveSingular`, `AbessivePlural`, `ProlativeSingular`, `ProlativePlural`, `InstructiveSingular`, `InstructivePlural`

Citation form: `NominativeSingular, GenitiveSingular, PartitiveSingular` (comma-separated; parts you give are output exactly and narrow the prediction).

**adj** cells: `NominativeSingular`, `NominativePlural`, `GenitiveSingular`, `GenitivePlural`, `PartitiveSingular`, `PartitivePlural`, `IllativeSingular`, `IllativePlural`, `InessiveSingular`, `InessivePlural`, `ElativeSingular`, `ElativePlural`, `AdessiveSingular`, `AdessivePlural`, `AblativeSingular`, `AblativePlural`, `TranslativeSingular`, `TranslativePlural`, `EssiveSingular`, `EssivePlural`, `ComitativeSingular`, `ComitativePlural`, `AbessiveSingular`, `AbessivePlural`, `ProlativeSingular`, `ProlativePlural`, `InstructiveSingular`, `InstructivePlural`

**pron** cells: `NominativeSingular`, `NominativePlural`, `GenitiveSingular`, `GenitivePlural`, `PartitiveSingular`, `PartitivePlural`, `IllativeSingular`, `IllativePlural`, `InessiveSingular`, `InessivePlural`, `ElativeSingular`, `ElativePlural`, `AdessiveSingular`, `AdessivePlural`, `AblativeSingular`, `AblativePlural`, `TranslativeSingular`, `TranslativePlural`, `EssiveSingular`, `EssivePlural`, `ComitativeSingular`, `ComitativePlural`, `AbessiveSingular`, `AbessivePlural`, `ProlativeSingular`, `ProlativePlural`, `InstructiveSingular`, `InstructivePlural`

**num** cells: `NominativeSingular`, `NominativePlural`, `GenitiveSingular`, `GenitivePlural`, `PartitiveSingular`, `PartitivePlural`, `IllativeSingular`, `IllativePlural`, `InessiveSingular`, `InessivePlural`, `ElativeSingular`, `ElativePlural`, `AdessiveSingular`, `AdessivePlural`, `AblativeSingular`, `AblativePlural`, `TranslativeSingular`, `TranslativePlural`, `EssiveSingular`, `EssivePlural`, `ComitativeSingular`, `ComitativePlural`, `AbessiveSingular`, `AbessivePlural`, `ProlativeSingular`, `ProlativePlural`, `InstructiveSingular`, `InstructivePlural`

## How it works

Each attested form is stored as an edit tree from the lemma. A lemma's edits across all cells make its inflection class. A suffix tree over the word, read from its end and pruned by a description-length test, predicts the class of any word. Known words the tree gets wrong are kept in a small lexicon. Confidences are smoothed counts; variant weights follow Wiktionary's listing order (1, 1/2, 1/3…, nonstandard forms a tenth of that).

## License

Code: MIT OR Apache-2.0. Data: derived from [Wiktionary](https://www.wiktionary.org/) (via [kaikki.org](https://kaikki.org/)), CC BY-SA 4.0; the model bytes in `src/model.bin` are shared under the same license.
