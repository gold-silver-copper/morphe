# tatar

Tatar inflection for Rust, learned from Wiktionary by [morphe](https://github.com/gold-silver-copper/morphe).
Inflects any word, known or made up. The `_scored` functions give every candidate with a confidence.

Generated from the Kaikki Tatar dump of 2026-10-03. Do not edit by hand: change the config in morphe and regenerate.

## Example

```rust
use tatar::*;
println!("{}", noun("республика", Case::Dative, Number::Plural)); // республикаларга
println!("{}", noun_possessive("республика", Possessor::FirstSingular, Possessed::Many, Case::Nominative)); // республикаларым
```

Nouns have six cases in both numbers and the possessed forms for six
possessors, with one thing or many owned. Wiktionary has Tatar tables for
a few hundred nouns; verbs are too few to learn from and are left out.

## Coverage

| Part of speech | Entries | Cells | Edits | Classes | Tree nodes | Lexicon |
|---|---|---|---|---|---|---|
| noun | 174 | 84 | 473 | 21 | 25 | 45 |

Model size: 53520 bytes.

**noun** cells: `NominativeSingular`, `NominativePlural`, `GenitiveSingular`, `GenitivePlural`, `DativeSingular`, `DativePlural`, `AccusativeSingular`, `AccusativePlural`, `LocativeSingular`, `LocativePlural`, `AblativeSingular`, `AblativePlural`, `PossessiveFirstSingularOneNominative`, `PossessiveFirstSingularOneGenitive`, `PossessiveFirstSingularOneDative`, `PossessiveFirstSingularOneAccusative`, `PossessiveFirstSingularOneLocative`, `PossessiveFirstSingularOneAblative`, `PossessiveFirstSingularManyNominative`, `PossessiveFirstSingularManyGenitive`, `PossessiveFirstSingularManyDative`, `PossessiveFirstSingularManyAccusative`, `PossessiveFirstSingularManyLocative`, `PossessiveFirstSingularManyAblative`, `PossessiveSecondSingularOneNominative`, `PossessiveSecondSingularOneGenitive`, `PossessiveSecondSingularOneDative`, `PossessiveSecondSingularOneAccusative`, `PossessiveSecondSingularOneLocative`, `PossessiveSecondSingularOneAblative`, `PossessiveSecondSingularManyNominative`, `PossessiveSecondSingularManyGenitive`, `PossessiveSecondSingularManyDative`, `PossessiveSecondSingularManyAccusative`, `PossessiveSecondSingularManyLocative`, `PossessiveSecondSingularManyAblative`, `PossessiveThirdSingularOneNominative`, `PossessiveThirdSingularOneGenitive`, `PossessiveThirdSingularOneDative`, `PossessiveThirdSingularOneAccusative`, `PossessiveThirdSingularOneLocative`, `PossessiveThirdSingularOneAblative`, `PossessiveThirdSingularManyNominative`, `PossessiveThirdSingularManyGenitive`, `PossessiveThirdSingularManyDative`, `PossessiveThirdSingularManyAccusative`, `PossessiveThirdSingularManyLocative`, `PossessiveThirdSingularManyAblative`, `PossessiveFirstPluralOneNominative`, `PossessiveFirstPluralOneGenitive`, `PossessiveFirstPluralOneDative`, `PossessiveFirstPluralOneAccusative`, `PossessiveFirstPluralOneLocative`, `PossessiveFirstPluralOneAblative`, `PossessiveFirstPluralManyNominative`, `PossessiveFirstPluralManyGenitive`, `PossessiveFirstPluralManyDative`, `PossessiveFirstPluralManyAccusative`, `PossessiveFirstPluralManyLocative`, `PossessiveFirstPluralManyAblative`, `PossessiveSecondPluralOneNominative`, `PossessiveSecondPluralOneGenitive`, `PossessiveSecondPluralOneDative`, `PossessiveSecondPluralOneAccusative`, `PossessiveSecondPluralOneLocative`, `PossessiveSecondPluralOneAblative`, `PossessiveSecondPluralManyNominative`, `PossessiveSecondPluralManyGenitive`, `PossessiveSecondPluralManyDative`, `PossessiveSecondPluralManyAccusative`, `PossessiveSecondPluralManyLocative`, `PossessiveSecondPluralManyAblative`, `PossessiveThirdPluralOneNominative`, `PossessiveThirdPluralOneGenitive`, `PossessiveThirdPluralOneDative`, `PossessiveThirdPluralOneAccusative`, `PossessiveThirdPluralOneLocative`, `PossessiveThirdPluralOneAblative`, `PossessiveThirdPluralManyNominative`, `PossessiveThirdPluralManyGenitive`, `PossessiveThirdPluralManyDative`, `PossessiveThirdPluralManyAccusative`, `PossessiveThirdPluralManyLocative`, `PossessiveThirdPluralManyAblative`

Citation form: `NominativeSingular, NominativePlural` (comma-separated; parts you give are output exactly and narrow the prediction).

## How it works

Each attested form is stored as an edit tree from the lemma. A lemma's edits across all cells make its inflection class. A suffix tree over the word, read from its end and pruned by a description-length test, predicts the class of any word. Known words the tree gets wrong are kept in a small lexicon. Confidences are smoothed counts; variant weights follow Wiktionary's listing order (1, 1/2, 1/3…, nonstandard forms a tenth of that).

## License

Code: MIT OR Apache-2.0. Data: derived from [Wiktionary](https://www.wiktionary.org/) (via [kaikki.org](https://kaikki.org/)), CC BY-SA 4.0; the model bytes in `src/model.bin` are shared under the same license.
