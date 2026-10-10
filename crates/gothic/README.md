# gothic

Gothic inflection for Rust, learned from Wiktionary by [morphe](https://github.com/gold-silver-copper/morphe).
Inflects any word, known or made up. The `_scored` functions give every candidate with a confidence.

Generated from the Kaikki Gothic dump of 2026-10-03. Do not edit by hand: change the config in morphe and regenerate.

## Example

```rust
use gothic::*;
assert_eq!(noun("𐌷𐌿𐌽𐌳𐍃", Case::Dative, Number::Plural), "𐌷𐌿𐌽𐌳𐌰𐌼");
println!("{:?}", noun_gender("𐌲𐌹𐌱𐌰"));
println!("{}", verb("𐌽𐌹𐌼𐌰𐌽", Mood::Indicative, Tense::Past, Person::Third, Number::Singular, Voice::Active));
println!("{}", adj_weak("𐌱𐌻𐌹𐌽𐌳𐍃", Case::Nominative, Number::Singular, Gender::Masculine));
```

Words are written in the Gothic alphabet. The passive exists only in the
present.

## Coverage

| Part of speech | Entries | Cells | Edits | Classes | Tree nodes | Lexicon |
|---|---|---|---|---|---|---|
| noun | 1303 | 10 | 222 | 112 | 96 | 397 |
| adj | 453 | 50 | 383 | 81 | 38 | 73 |
| verb | 1214 | 80 | 1773 | 254 | 171 | 240 |

Model size: 247073 bytes.

**noun** cells: `NominativeSingular`, `NominativePlural`, `VocativeSingular`, `VocativePlural`, `AccusativeSingular`, `AccusativePlural`, `GenitiveSingular`, `GenitivePlural`, `DativeSingular`, `DativePlural`

Citation form: `Lemma, GenitiveSingular` (comma-separated; parts you give are output exactly and narrow the prediction).

**adj** cells: `NominativeSingularMasculine`, `NominativeSingularFeminine`, `NominativeSingularNeuter`, `NominativePluralMasculine`, `NominativePluralFeminine`, `NominativePluralNeuter`, `AccusativeSingularMasculine`, `AccusativeSingularFeminine`, `AccusativeSingularNeuter`, `AccusativePluralMasculine`, `AccusativePluralFeminine`, `AccusativePluralNeuter`, `GenitiveSingularMasculine`, `GenitiveSingularFeminine`, `GenitiveSingularNeuter`, `GenitivePluralMasculine`, `GenitivePluralFeminine`, `GenitivePluralNeuter`, `DativeSingularMasculine`, `DativeSingularFeminine`, `DativeSingularNeuter`, `DativePluralMasculine`, `DativePluralFeminine`, `DativePluralNeuter`, `WeakNominativeSingularMasculine`, `WeakNominativeSingularFeminine`, `WeakNominativeSingularNeuter`, `WeakNominativePluralMasculine`, `WeakNominativePluralFeminine`, `WeakNominativePluralNeuter`, `WeakAccusativeSingularMasculine`, `WeakAccusativeSingularFeminine`, `WeakAccusativeSingularNeuter`, `WeakAccusativePluralMasculine`, `WeakAccusativePluralFeminine`, `WeakAccusativePluralNeuter`, `WeakGenitiveSingularMasculine`, `WeakGenitiveSingularFeminine`, `WeakGenitiveSingularNeuter`, `WeakGenitivePluralMasculine`, `WeakGenitivePluralFeminine`, `WeakGenitivePluralNeuter`, `WeakDativeSingularMasculine`, `WeakDativeSingularFeminine`, `WeakDativeSingularNeuter`, `WeakDativePluralMasculine`, `WeakDativePluralFeminine`, `WeakDativePluralNeuter`, `Comparative`, `Superlative`

**verb** cells: `IndicativePresentFirstSingularActive`, `IndicativePresentFirstSingularPassive`, `IndicativePresentFirstDualActive`, `IndicativePresentFirstDualPassive`, `IndicativePresentFirstPluralActive`, `IndicativePresentFirstPluralPassive`, `IndicativePresentSecondSingularActive`, `IndicativePresentSecondSingularPassive`, `IndicativePresentSecondDualActive`, `IndicativePresentSecondDualPassive`, `IndicativePresentSecondPluralActive`, `IndicativePresentSecondPluralPassive`, `IndicativePresentThirdSingularActive`, `IndicativePresentThirdSingularPassive`, `IndicativePresentThirdDualActive`, `IndicativePresentThirdDualPassive`, `IndicativePresentThirdPluralActive`, `IndicativePresentThirdPluralPassive`, `IndicativePastFirstSingularActive`, `IndicativePastFirstSingularPassive`, `IndicativePastFirstDualActive`, `IndicativePastFirstDualPassive`, `IndicativePastFirstPluralActive`, `IndicativePastFirstPluralPassive`, `IndicativePastSecondSingularActive`, `IndicativePastSecondSingularPassive`, `IndicativePastSecondDualActive`, `IndicativePastSecondDualPassive`, `IndicativePastSecondPluralActive`, `IndicativePastSecondPluralPassive`, `IndicativePastThirdSingularActive`, `IndicativePastThirdSingularPassive`, `IndicativePastThirdDualActive`, `IndicativePastThirdDualPassive`, `IndicativePastThirdPluralActive`, `IndicativePastThirdPluralPassive`, `SubjunctivePresentFirstSingularActive`, `SubjunctivePresentFirstSingularPassive`, `SubjunctivePresentFirstDualActive`, `SubjunctivePresentFirstDualPassive`, `SubjunctivePresentFirstPluralActive`, `SubjunctivePresentFirstPluralPassive`, `SubjunctivePresentSecondSingularActive`, `SubjunctivePresentSecondSingularPassive`, `SubjunctivePresentSecondDualActive`, `SubjunctivePresentSecondDualPassive`, `SubjunctivePresentSecondPluralActive`, `SubjunctivePresentSecondPluralPassive`, `SubjunctivePresentThirdSingularActive`, `SubjunctivePresentThirdSingularPassive`, `SubjunctivePresentThirdDualActive`, `SubjunctivePresentThirdDualPassive`, `SubjunctivePresentThirdPluralActive`, `SubjunctivePresentThirdPluralPassive`, `SubjunctivePastFirstSingularActive`, `SubjunctivePastFirstSingularPassive`, `SubjunctivePastFirstDualActive`, `SubjunctivePastFirstDualPassive`, `SubjunctivePastFirstPluralActive`, `SubjunctivePastFirstPluralPassive`, `SubjunctivePastSecondSingularActive`, `SubjunctivePastSecondSingularPassive`, `SubjunctivePastSecondDualActive`, `SubjunctivePastSecondDualPassive`, `SubjunctivePastSecondPluralActive`, `SubjunctivePastSecondPluralPassive`, `SubjunctivePastThirdSingularActive`, `SubjunctivePastThirdSingularPassive`, `SubjunctivePastThirdDualActive`, `SubjunctivePastThirdDualPassive`, `SubjunctivePastThirdPluralActive`, `SubjunctivePastThirdPluralPassive`, `ImperativeSecondSingular`, `ImperativeSecondDual`, `ImperativeSecondPlural`, `ImperativeThirdSingular`, `ImperativeThirdDual`, `ImperativeThirdPlural`, `ParticiplePresent`, `ParticiplePast`

Citation form: `Lemma, IndicativePastThirdSingularActive, ParticiplePast` (comma-separated; parts you give are output exactly and narrow the prediction).

## How it works

Each attested form is stored as an edit tree from the lemma. A lemma's edits across all cells make its inflection class. A suffix tree over the word, read from its end and pruned by a description-length test, predicts the class of any word. Known words the tree gets wrong are kept in a small lexicon. Confidences are smoothed counts; variant weights follow Wiktionary's listing order (1, 1/2, 1/3…, nonstandard forms a tenth of that).

## License

Code: MIT OR Apache-2.0. Data: derived from [Wiktionary](https://www.wiktionary.org/) (via [kaikki.org](https://kaikki.org/)), CC BY-SA 4.0; the model bytes in `src/model.bin` are shared under the same license.
