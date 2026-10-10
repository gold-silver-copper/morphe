# classical_syriac

ClassicalSyriac inflection for Rust, learned from Wiktionary by [morphe](https://github.com/gold-silver-copper/morphe).
Inflects any word, known or made up. The `_scored` functions give every candidate with a confidence.

Generated from the Kaikki ClassicalSyriac dump of 2026-10-03. Do not edit by hand: change the config in morphe and regenerate.

## Example

```rust
use classical_syriac::*;
println!("{}", noun("ܟܠܒܐ", State::Absolute, Number::Singular)); // ܟܠܒ
println!("{}", noun("ܟܠܒܐ", State::Emphatic, Number::Plural));
println!("{}", noun_possessed("ܟܠܒܐ", Possessor::FirstSingular)); // ܟܠܒܝ
println!("{:?}", noun_gender("ܟܠܒܐ"));
```

Nouns have the absolute, construct and emphatic states in both numbers,
and the possessed forms for ten possessors; adjectives decline for state,
gender and number. Input matching ignores vowel points; output is
unpointed.

## Coverage

| Part of speech | Entries | Cells | Edits | Classes | Tree nodes | Lexicon |
|---|---|---|---|---|---|---|
| noun | 2856 | 16 | 806 | 389 | 143 | 697 |
| adj | 96 | 12 | 17 | 10 | 2 | 11 |

Model size: 70219 bytes.

**noun** cells: `AbsoluteSingular`, `AbsolutePlural`, `ConstructSingular`, `ConstructPlural`, `EmphaticSingular`, `EmphaticPlural`, `PossessedFirstSingular`, `PossessedSecondMasculineSingular`, `PossessedSecondFeminineSingular`, `PossessedThirdMasculineSingular`, `PossessedThirdFeminineSingular`, `PossessedFirstPlural`, `PossessedSecondMasculinePlural`, `PossessedSecondFemininePlural`, `PossessedThirdMasculinePlural`, `PossessedThirdFemininePlural`

Citation form: `EmphaticSingular, EmphaticPlural` (comma-separated; parts you give are output exactly and narrow the prediction).

**adj** cells: `AbsoluteMasculineSingular`, `AbsoluteMasculinePlural`, `AbsoluteFeminineSingular`, `AbsoluteFemininePlural`, `ConstructMasculineSingular`, `ConstructMasculinePlural`, `ConstructFeminineSingular`, `ConstructFemininePlural`, `EmphaticMasculineSingular`, `EmphaticMasculinePlural`, `EmphaticFeminineSingular`, `EmphaticFemininePlural`

## How it works

Each attested form is stored as an edit tree from the lemma. A lemma's edits across all cells make its inflection class. A suffix tree over the word, read from its end and pruned by a description-length test, predicts the class of any word. Known words the tree gets wrong are kept in a small lexicon. Confidences are smoothed counts; variant weights follow Wiktionary's listing order (1, 1/2, 1/3…, nonstandard forms a tenth of that).

## License

Code: MIT OR Apache-2.0. Data: derived from [Wiktionary](https://www.wiktionary.org/) (via [kaikki.org](https://kaikki.org/)), CC BY-SA 4.0; the model bytes in `src/model.bin` are shared under the same license.
