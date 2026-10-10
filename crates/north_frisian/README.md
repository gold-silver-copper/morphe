# north_frisian

NorthFrisian inflection for Rust, learned from Wiktionary by [morphe](https://github.com/gold-silver-copper/morphe).
Inflects any word, known or made up. The `_scored` functions give every candidate with a confidence.

Generated from the Kaikki NorthFrisian dump of 2026-10-03. Do not edit by hand: change the config in morphe and regenerate.

## Example

```rust
use north_frisian::*;
println!("{}", noun_plural("sun")); // sunen
println!("{}", verb_present("spring", PersonNumber::ThirdSingular)); // sprangt
println!("{}", verb_past("spring", PersonNumber::ThirdSingular)); // sproong
println!("{}", adj("green", Degree::Comparative, Form::Predicative)); // greener
```

North Frisian is a group of dialects. Each word follows the first table
Wiktionary gives, which is the dialect of its headword (mostly Föhr-Amrum
or Mooring). Compound tenses are left out. Umlauts are optional on input.

## Coverage

| Part of speech | Entries | Cells | Edits | Classes | Tree nodes | Lexicon |
|---|---|---|---|---|---|---|
| noun | 423 | 1 | 68 | 86 | 112 | 226 |
| adj | 56 | 30 | 156 | 38 | 5 | 42 |
| verb | 309 | 13 | 864 | 251 | 71 | 217 |

Model size: 48726 bytes.

**noun** cells: `Plural`

Citation form: `Lemma, Plural` (comma-separated; parts you give are output exactly and narrow the prediction).

**adj** cells: `PositivePredicative`, `PositiveAttributiveMasculine`, `PositiveAttributiveNeuter`, `PositiveAttributiveDefinite`, `PositiveAttributivePlural`, `PositiveIndependentIndefinite`, `PositiveIndependentNeuter`, `PositiveIndependentDefinite`, `PositiveIndependentPlural`, `PositivePartitive`, `ComparativePredicative`, `ComparativeAttributiveMasculine`, `ComparativeAttributiveNeuter`, `ComparativeAttributiveDefinite`, `ComparativeAttributivePlural`, `ComparativeIndependentIndefinite`, `ComparativeIndependentNeuter`, `ComparativeIndependentDefinite`, `ComparativeIndependentPlural`, `ComparativePartitive`, `SuperlativePredicative`, `SuperlativeAttributiveMasculine`, `SuperlativeAttributiveNeuter`, `SuperlativeAttributiveDefinite`, `SuperlativeAttributivePlural`, `SuperlativeIndependentIndefinite`, `SuperlativeIndependentNeuter`, `SuperlativeIndependentDefinite`, `SuperlativeIndependentPlural`, `SuperlativePartitive`

Citation form: `Lemma, ComparativePredicative, SuperlativePredicative` (comma-separated; parts you give are output exactly and narrow the prediction).

**verb** cells: `Infinitive`, `LongInfinitive`, `ParticiplePast`, `ImperativeSingular`, `ImperativePlural`, `PresentFirstSingular`, `PresentSecondSingular`, `PresentThirdSingular`, `PresentPlural`, `PastFirstSingular`, `PastSecondSingular`, `PastThirdSingular`, `PastPlural`

Citation form: `Infinitive, PastThirdSingular, ParticiplePast` (comma-separated; parts you give are output exactly and narrow the prediction).

## How it works

Each attested form is stored as an edit tree from the lemma. A lemma's edits across all cells make its inflection class. A suffix tree over the word, read from its end and pruned by a description-length test, predicts the class of any word. Known words the tree gets wrong are kept in a small lexicon. Confidences are smoothed counts; variant weights follow Wiktionary's listing order (1, 1/2, 1/3…, nonstandard forms a tenth of that).

## License

Code: MIT OR Apache-2.0. Data: derived from [Wiktionary](https://www.wiktionary.org/) (via [kaikki.org](https://kaikki.org/)), CC BY-SA 4.0; the model bytes in `src/model.bin` are shared under the same license.
