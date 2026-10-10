# ottoman_turkish

OttomanTurkish inflection for Rust, learned from Wiktionary by [morphe](https://github.com/gold-silver-copper/morphe).
Inflects any word, known or made up. The `_scored` functions give every candidate with a confidence.

Generated from the Kaikki OttomanTurkish dump of 2026-10-03. Do not edit by hand: change the config in morphe and regenerate.

## Example

```rust
use ottoman_turkish::*;
println!("{}", noun("امام", Case::Dative, Number::Singular)); // امامه
println!("{}", noun("امام", Case::Nominative, Number::Plural)); // ائمه
println!("{}", noun_possessive("امام", Possessor::FirstSingular, Possessed::One, Case::Ablative)); // اماممدن
```

Nouns have six cases in both numbers (with the Arabic broken plurals
Wiktionary gives) and the possessed forms for six possessors, with one
thing or many owned, in every case. Wiktionary has no Ottoman Turkish
conjugation tables.

## Coverage

| Part of speech | Entries | Cells | Edits | Classes | Tree nodes | Lexicon |
|---|---|---|---|---|---|---|
| noun | 310 | 84 | 735 | 25 | 20 | 60 |

Model size: 28014 bytes.

**noun** cells: `NominativeSingular`, `NominativePlural`, `AccusativeSingular`, `AccusativePlural`, `DativeSingular`, `DativePlural`, `LocativeSingular`, `LocativePlural`, `AblativeSingular`, `AblativePlural`, `GenitiveSingular`, `GenitivePlural`, `PossessiveFirstSingularOneNominative`, `PossessiveFirstSingularOneAccusative`, `PossessiveFirstSingularOneDative`, `PossessiveFirstSingularOneLocative`, `PossessiveFirstSingularOneAblative`, `PossessiveFirstSingularOneGenitive`, `PossessiveFirstSingularManyNominative`, `PossessiveFirstSingularManyAccusative`, `PossessiveFirstSingularManyDative`, `PossessiveFirstSingularManyLocative`, `PossessiveFirstSingularManyAblative`, `PossessiveFirstSingularManyGenitive`, `PossessiveSecondSingularOneNominative`, `PossessiveSecondSingularOneAccusative`, `PossessiveSecondSingularOneDative`, `PossessiveSecondSingularOneLocative`, `PossessiveSecondSingularOneAblative`, `PossessiveSecondSingularOneGenitive`, `PossessiveSecondSingularManyNominative`, `PossessiveSecondSingularManyAccusative`, `PossessiveSecondSingularManyDative`, `PossessiveSecondSingularManyLocative`, `PossessiveSecondSingularManyAblative`, `PossessiveSecondSingularManyGenitive`, `PossessiveThirdSingularOneNominative`, `PossessiveThirdSingularOneAccusative`, `PossessiveThirdSingularOneDative`, `PossessiveThirdSingularOneLocative`, `PossessiveThirdSingularOneAblative`, `PossessiveThirdSingularOneGenitive`, `PossessiveThirdSingularManyNominative`, `PossessiveThirdSingularManyAccusative`, `PossessiveThirdSingularManyDative`, `PossessiveThirdSingularManyLocative`, `PossessiveThirdSingularManyAblative`, `PossessiveThirdSingularManyGenitive`, `PossessiveFirstPluralOneNominative`, `PossessiveFirstPluralOneAccusative`, `PossessiveFirstPluralOneDative`, `PossessiveFirstPluralOneLocative`, `PossessiveFirstPluralOneAblative`, `PossessiveFirstPluralOneGenitive`, `PossessiveFirstPluralManyNominative`, `PossessiveFirstPluralManyAccusative`, `PossessiveFirstPluralManyDative`, `PossessiveFirstPluralManyLocative`, `PossessiveFirstPluralManyAblative`, `PossessiveFirstPluralManyGenitive`, `PossessiveSecondPluralOneNominative`, `PossessiveSecondPluralOneAccusative`, `PossessiveSecondPluralOneDative`, `PossessiveSecondPluralOneLocative`, `PossessiveSecondPluralOneAblative`, `PossessiveSecondPluralOneGenitive`, `PossessiveSecondPluralManyNominative`, `PossessiveSecondPluralManyAccusative`, `PossessiveSecondPluralManyDative`, `PossessiveSecondPluralManyLocative`, `PossessiveSecondPluralManyAblative`, `PossessiveSecondPluralManyGenitive`, `PossessiveThirdPluralOneNominative`, `PossessiveThirdPluralOneAccusative`, `PossessiveThirdPluralOneDative`, `PossessiveThirdPluralOneLocative`, `PossessiveThirdPluralOneAblative`, `PossessiveThirdPluralOneGenitive`, `PossessiveThirdPluralManyNominative`, `PossessiveThirdPluralManyAccusative`, `PossessiveThirdPluralManyDative`, `PossessiveThirdPluralManyLocative`, `PossessiveThirdPluralManyAblative`, `PossessiveThirdPluralManyGenitive`

Citation form: `NominativeSingular, NominativePlural` (comma-separated; parts you give are output exactly and narrow the prediction).

## How it works

Each attested form is stored as an edit tree from the lemma. A lemma's edits across all cells make its inflection class. A suffix tree over the word, read from its end and pruned by a description-length test, predicts the class of any word. Known words the tree gets wrong are kept in a small lexicon. Confidences are smoothed counts; variant weights follow Wiktionary's listing order (1, 1/2, 1/3…, nonstandard forms a tenth of that).

## License

Code: MIT OR Apache-2.0. Data: derived from [Wiktionary](https://www.wiktionary.org/) (via [kaikki.org](https://kaikki.org/)), CC BY-SA 4.0; the model bytes in `src/model.bin` are shared under the same license.
