# breton

Breton inflection for Rust, learned from Wiktionary by [morphe](https://github.com/gold-silver-copper/morphe).
Inflects any word, known or made up. The `_scored` functions give every candidate with a confidence.

Generated from the Kaikki Breton dump of 2026-10-03. Do not edit by hand: change the config in morphe and regenerate.

## Example

```rust
use breton::*;
assert_eq!(noun("bro", Number::Plural), "broioù");
assert_eq!(noun_mutated("bro", Mutation::Soft), "vro");
assert_eq!(noun_mutated_plural("bro", Mutation::Hard), "proioù");
assert_eq!(verb("barn", Tense::Future, PersonNumber::FirstSingular), "barnin");
assert_eq!(adj("brav", Degree::Comparative), "bravoc'h");
```

The data is small: about 60 verb tables and 300 noun mutation tables.
Typographic and straight apostrophes are interchangeable on input (c’h = c'h).

## Coverage

| Part of speech | Entries | Cells | Edits | Classes | Tree nodes | Lexicon |
|---|---|---|---|---|---|---|
| noun | 756 | 11 | 370 | 300 | 83 | 537 |
| adj | 44 | 6 | 35 | 30 | 12 | 21 |
| verb | 62 | 55 | 858 | 47 | 8 | 45 |

Model size: 50372 bytes.

**noun** cells: `Plural`, `Singulative`, `Dual`, `MutatedSoft`, `MutatedHard`, `MutatedAspirate`, `MutatedMixed`, `MutatedPluralSoft`, `MutatedPluralHard`, `MutatedPluralAspirate`, `MutatedPluralMixed`

Citation form: `Lemma, Plural` (comma-separated; parts you give are output exactly and narrow the prediction).

**adj** cells: `Comparative`, `Superlative`, `MutatedSoft`, `MutatedHard`, `MutatedAspirate`, `MutatedMixed`

Citation form: `Lemma, Comparative, Superlative` (comma-separated; parts you give are output exactly and narrow the prediction).

**verb** cells: `PresentFirstSingular`, `PresentSecondSingular`, `PresentThirdSingular`, `PresentFirstPlural`, `PresentSecondPlural`, `PresentThirdPlural`, `PresentImpersonal`, `FutureFirstSingular`, `FutureSecondSingular`, `FutureThirdSingular`, `FutureFirstPlural`, `FutureSecondPlural`, `FutureThirdPlural`, `FutureImpersonal`, `ImperfectFirstSingular`, `ImperfectSecondSingular`, `ImperfectThirdSingular`, `ImperfectFirstPlural`, `ImperfectSecondPlural`, `ImperfectThirdPlural`, `ImperfectImpersonal`, `PreteriteFirstSingular`, `PreteriteSecondSingular`, `PreteriteThirdSingular`, `PreteriteFirstPlural`, `PreteriteSecondPlural`, `PreteriteThirdPlural`, `PreteriteImpersonal`, `ConditionalFirstSingular`, `ConditionalSecondSingular`, `ConditionalThirdSingular`, `ConditionalFirstPlural`, `ConditionalSecondPlural`, `ConditionalThirdPlural`, `ConditionalImpersonal`, `PastConditionalFirstSingular`, `PastConditionalSecondSingular`, `PastConditionalThirdSingular`, `PastConditionalFirstPlural`, `PastConditionalSecondPlural`, `PastConditionalThirdPlural`, `PastConditionalImpersonal`, `ImperativeFirstSingular`, `ImperativeSecondSingular`, `ImperativeThirdSingular`, `ImperativeFirstPlural`, `ImperativeSecondPlural`, `ImperativeThirdPlural`, `ImperativeImpersonal`, `ParticiplePresent`, `ParticiplePast`, `MutatedSoft`, `MutatedHard`, `MutatedAspirate`, `MutatedMixed`

## How it works

Each attested form is stored as an edit tree from the lemma. A lemma's edits across all cells make its inflection class. A suffix tree over the word, read from its end and pruned by a description-length test, predicts the class of any word. Known words the tree gets wrong are kept in a small lexicon. Confidences are smoothed counts; variant weights follow Wiktionary's listing order (1, 1/2, 1/3…, nonstandard forms a tenth of that).

## License

Code: MIT OR Apache-2.0. Data: derived from [Wiktionary](https://www.wiktionary.org/) (via [kaikki.org](https://kaikki.org/)), CC BY-SA 4.0; the model bytes in `src/model.bin` are shared under the same license.
