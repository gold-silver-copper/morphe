# uyghur

Uyghur inflection for Rust, learned from Wiktionary by [morphe](https://github.com/gold-silver-copper/morphe).
Inflects any word, known or made up. The `_scored` functions give every candidate with a confidence.

Generated from the Kaikki Uyghur dump of 2026-10-03. Do not edit by hand: change the config in morphe and regenerate.

## Example

```rust
use uyghur::*;
println!("{}", noun("نېفىت", Case::Dative, Number::Plural)); // نېفىتلارغا
println!("{}", noun("نېفىت", Case::Equative, Number::Singular)); // نېفىتچىلىك
println!("{}", noun_possessive("نېفىت", Possessor::FirstSingular, Possessed::One, Case::Ablative)); // نېفىتىمدىن
```

Nouns have eleven cases (with the locative-qualitative -دىكى, the
limitative, the similative, the equative and the representative -نىڭكى)
in both numbers, and the possessed forms in six cases for eight
possessors, with one thing or many owned. Wiktionary has no Uyghur
conjugation tables.

## Coverage

| Part of speech | Entries | Cells | Edits | Classes | Tree nodes | Lexicon |
|---|---|---|---|---|---|---|
| noun | 303 | 118 | 920 | 59 | 45 | 93 |

Model size: 85778 bytes.

**noun** cells: `NominativeSingular`, `NominativePlural`, `GenitiveSingular`, `GenitivePlural`, `AccusativeSingular`, `AccusativePlural`, `DativeSingular`, `DativePlural`, `LocativeSingular`, `LocativePlural`, `AblativeSingular`, `AblativePlural`, `LocativeQualitativeSingular`, `LocativeQualitativePlural`, `LimitativeSingular`, `LimitativePlural`, `SimilativeSingular`, `SimilativePlural`, `EquativeSingular`, `EquativePlural`, `RepresentativeSingular`, `RepresentativePlural`, `PossessiveFirstSingularOneNominative`, `PossessiveFirstSingularOneGenitive`, `PossessiveFirstSingularOneAccusative`, `PossessiveFirstSingularOneDative`, `PossessiveFirstSingularOneLocative`, `PossessiveFirstSingularOneAblative`, `PossessiveFirstSingularManyNominative`, `PossessiveFirstSingularManyGenitive`, `PossessiveFirstSingularManyAccusative`, `PossessiveFirstSingularManyDative`, `PossessiveFirstSingularManyLocative`, `PossessiveFirstSingularManyAblative`, `PossessiveSecondSingularOneNominative`, `PossessiveSecondSingularOneGenitive`, `PossessiveSecondSingularOneAccusative`, `PossessiveSecondSingularOneDative`, `PossessiveSecondSingularOneLocative`, `PossessiveSecondSingularOneAblative`, `PossessiveSecondSingularManyNominative`, `PossessiveSecondSingularManyGenitive`, `PossessiveSecondSingularManyAccusative`, `PossessiveSecondSingularManyDative`, `PossessiveSecondSingularManyLocative`, `PossessiveSecondSingularManyAblative`, `PossessiveSecondSingularFormalOneNominative`, `PossessiveSecondSingularFormalOneGenitive`, `PossessiveSecondSingularFormalOneAccusative`, `PossessiveSecondSingularFormalOneDative`, `PossessiveSecondSingularFormalOneLocative`, `PossessiveSecondSingularFormalOneAblative`, `PossessiveSecondSingularFormalManyNominative`, `PossessiveSecondSingularFormalManyGenitive`, `PossessiveSecondSingularFormalManyAccusative`, `PossessiveSecondSingularFormalManyDative`, `PossessiveSecondSingularFormalManyLocative`, `PossessiveSecondSingularFormalManyAblative`, `PossessiveSecondPoliteOneNominative`, `PossessiveSecondPoliteOneGenitive`, `PossessiveSecondPoliteOneAccusative`, `PossessiveSecondPoliteOneDative`, `PossessiveSecondPoliteOneLocative`, `PossessiveSecondPoliteOneAblative`, `PossessiveSecondPoliteManyNominative`, `PossessiveSecondPoliteManyGenitive`, `PossessiveSecondPoliteManyAccusative`, `PossessiveSecondPoliteManyDative`, `PossessiveSecondPoliteManyLocative`, `PossessiveSecondPoliteManyAblative`, `PossessiveThirdOneNominative`, `PossessiveThirdOneGenitive`, `PossessiveThirdOneAccusative`, `PossessiveThirdOneDative`, `PossessiveThirdOneLocative`, `PossessiveThirdOneAblative`, `PossessiveThirdManyNominative`, `PossessiveThirdManyGenitive`, `PossessiveThirdManyAccusative`, `PossessiveThirdManyDative`, `PossessiveThirdManyLocative`, `PossessiveThirdManyAblative`, `PossessiveFirstPluralOneNominative`, `PossessiveFirstPluralOneGenitive`, `PossessiveFirstPluralOneAccusative`, `PossessiveFirstPluralOneDative`, `PossessiveFirstPluralOneLocative`, `PossessiveFirstPluralOneAblative`, `PossessiveFirstPluralManyNominative`, `PossessiveFirstPluralManyGenitive`, `PossessiveFirstPluralManyAccusative`, `PossessiveFirstPluralManyDative`, `PossessiveFirstPluralManyLocative`, `PossessiveFirstPluralManyAblative`, `PossessiveSecondPluralOneNominative`, `PossessiveSecondPluralOneGenitive`, `PossessiveSecondPluralOneAccusative`, `PossessiveSecondPluralOneDative`, `PossessiveSecondPluralOneLocative`, `PossessiveSecondPluralOneAblative`, `PossessiveSecondPluralManyNominative`, `PossessiveSecondPluralManyGenitive`, `PossessiveSecondPluralManyAccusative`, `PossessiveSecondPluralManyDative`, `PossessiveSecondPluralManyLocative`, `PossessiveSecondPluralManyAblative`, `PossessiveSecondPluralFormalOneNominative`, `PossessiveSecondPluralFormalOneGenitive`, `PossessiveSecondPluralFormalOneAccusative`, `PossessiveSecondPluralFormalOneDative`, `PossessiveSecondPluralFormalOneLocative`, `PossessiveSecondPluralFormalOneAblative`, `PossessiveSecondPluralFormalManyNominative`, `PossessiveSecondPluralFormalManyGenitive`, `PossessiveSecondPluralFormalManyAccusative`, `PossessiveSecondPluralFormalManyDative`, `PossessiveSecondPluralFormalManyLocative`, `PossessiveSecondPluralFormalManyAblative`

Citation form: `NominativeSingular, NominativePlural` (comma-separated; parts you give are output exactly and narrow the prediction).

## How it works

Each attested form is stored as an edit tree from the lemma. A lemma's edits across all cells make its inflection class. A suffix tree over the word, read from its end and pruned by a description-length test, predicts the class of any word. Known words the tree gets wrong are kept in a small lexicon. Confidences are smoothed counts; variant weights follow Wiktionary's listing order (1, 1/2, 1/3…, nonstandard forms a tenth of that).

## License

Code: MIT OR Apache-2.0. Data: derived from [Wiktionary](https://www.wiktionary.org/) (via [kaikki.org](https://kaikki.org/)), CC BY-SA 4.0; the model bytes in `src/model.bin` are shared under the same license.
