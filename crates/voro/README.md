# voro

Võro inflection for Rust, learned from Wiktionary by [morphe](https://github.com/gold-silver-copper/morphe).
Inflects any word, known or made up. The `_scored` functions give every candidate with a confidence.

Generated from the Kaikki Võro dump of 2026-10-03. Do not edit by hand: change the config in morphe and regenerate.

## Example

```rust
use voro::*;
println!("{}", noun("jalg", Case::Genitive, Number::Plural)); // jalgo
println!("{}", noun("lammas", Case::Illative, Number::Singular)); // lambahe
println!("{}", noun("luu", Case::Comitative, Number::Plural)); // luiõgaq
println!("{}", verb_infinitive_da("elämä")); // elläq
```

Nouns and adjectives have fourteen cases in both numbers. Wiktionary has
no Võro conjugation tables; verbs have only the principal parts the
headword line gives (the ma- and da-infinitives and the first person
present).

## Coverage

| Part of speech | Entries | Cells | Edits | Classes | Tree nodes | Lexicon |
|---|---|---|---|---|---|---|
| noun | 160 | 28 | 2377 | 210 | 35 | 72 |
| adj | 35 | 29 | 728 | 62 | 4 | 26 |
| verb | 53 | 3 | 56 | 43 | 13 | 40 |

Model size: 101630 bytes.

**noun** cells: `NominativeSingular`, `NominativePlural`, `AccusativeSingular`, `AccusativePlural`, `GenitiveSingular`, `GenitivePlural`, `PartitiveSingular`, `PartitivePlural`, `IllativeSingular`, `IllativePlural`, `InessiveSingular`, `InessivePlural`, `ElativeSingular`, `ElativePlural`, `AllativeSingular`, `AllativePlural`, `AdessiveSingular`, `AdessivePlural`, `AblativeSingular`, `AblativePlural`, `TranslativeSingular`, `TranslativePlural`, `TerminativeSingular`, `TerminativePlural`, `AbessiveSingular`, `AbessivePlural`, `ComitativeSingular`, `ComitativePlural`

Citation form: `NominativeSingular, GenitiveSingular, PartitiveSingular` (comma-separated; parts you give are output exactly and narrow the prediction).

**adj** cells: `NominativeSingular`, `NominativePlural`, `AccusativeSingular`, `AccusativePlural`, `GenitiveSingular`, `GenitivePlural`, `PartitiveSingular`, `PartitivePlural`, `IllativeSingular`, `IllativePlural`, `InessiveSingular`, `InessivePlural`, `ElativeSingular`, `ElativePlural`, `AllativeSingular`, `AllativePlural`, `AdessiveSingular`, `AdessivePlural`, `AblativeSingular`, `AblativePlural`, `TranslativeSingular`, `TranslativePlural`, `TerminativeSingular`, `TerminativePlural`, `AbessiveSingular`, `AbessivePlural`, `ComitativeSingular`, `ComitativePlural`, `Comparative`

Citation form: `NominativeSingular, GenitiveSingular, PartitiveSingular` (comma-separated; parts you give are output exactly and narrow the prediction).

**verb** cells: `InfinitiveMa`, `InfinitiveDa`, `PresentFirstSingular`

Citation form: `InfinitiveMa, InfinitiveDa, PresentFirstSingular` (comma-separated; parts you give are output exactly and narrow the prediction).

## How it works

Each attested form is stored as an edit tree from the lemma. A lemma's edits across all cells make its inflection class. A suffix tree over the word, read from its end and pruned by a description-length test, predicts the class of any word. Known words the tree gets wrong are kept in a small lexicon. Confidences are smoothed counts; variant weights follow Wiktionary's listing order (1, 1/2, 1/3…, nonstandard forms a tenth of that).

## License

Code: MIT OR Apache-2.0. Data: derived from [Wiktionary](https://www.wiktionary.org/) (via [kaikki.org](https://kaikki.org/)), CC BY-SA 4.0; the model bytes in `src/model.bin` are shared under the same license.
