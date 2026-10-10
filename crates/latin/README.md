# latin

Latin inflection for Rust, learned from Wiktionary by [morphe](https://github.com/gold-silver-copper/morphe).
Inflects any word, known or made up. The `_scored` functions give every candidate with a confidence.

Generated from the Kaikki Latin dump of 2026-10-03. Do not edit by hand: change the config in morphe and regenerate.

## Example

```rust
use latin::*;
assert_eq!(noun("rosa", Case::Genitive, Number::Plural), "rosārum");
println!("{}", noun("rex", Case::Ablative, Number::Singular)); // rēge
println!("{:?}", noun_gender("nox"));
println!("{}", verb_indicative("amo", Tense::Perfect, Person::Third, Number::Plural)); // amāvērunt
println!("{}", verb_subjunctive_passive("capio", Tense::Imperfect, Person::First, Number::Singular));
println!("{}", adj("bonus", Case::Accusative, Gender::Feminine, Number::Singular)); // bonam
// Participles decline like adjectives:
println!("{}", verb_participle_passive_declined("amo", Tense::Perfect, Case::Genitive, Gender::Feminine, Number::Plural));
// Citations pin principal parts: amō, amāre, amāvī, amātum.
println!("{}", verb_indicative("fero, ferre, tuli, latum", Tense::Perfect, Person::First, Number::Plural));
// Without macrons in the output:
println!("{}", Latin::default().marks(false).noun("puella", Case::Dative, Number::Plural));
```

Input may omit macrons and breves, and may write j for i. Output carries
macrons as Wiktionary gives them (`marks(false)` removes them). Perfect
passive and other periphrastic tenses (amātus sum) are left to the caller:
combine `verb_participle_passive` with a form of *sum*.

## Coverage

| Part of speech | Entries | Cells | Edits | Classes | Tree nodes | Lexicon |
|---|---|---|---|---|---|---|
| noun | 16735 | 14 | 2724 | 980 | 2285 | 6984 |
| adj | 10881 | 39 | 1317 | 589 | 1660 | 4414 |
| verb | 6393 | 118 | 12367 | 1463 | 1281 | 3265 |
| pron | 70 | 36 | 489 | 103 | 46 | 43 |
| det | 35 | 42 | 220 | 45 | 16 | 22 |
| num | 126 | 36 | 111 | 21 | 58 | 61 |

Model size: 4721228 bytes.

**noun** cells: `NominativeSingular`, `NominativePlural`, `GenitiveSingular`, `GenitivePlural`, `DativeSingular`, `DativePlural`, `AccusativeSingular`, `AccusativePlural`, `AblativeSingular`, `AblativePlural`, `VocativeSingular`, `VocativePlural`, `LocativeSingular`, `LocativePlural`

Citation form: `NominativeSingular, GenitiveSingular` (comma-separated; parts you give are output exactly and narrow the prediction).

**adj** cells: `NominativeMasculineSingular`, `NominativeMasculinePlural`, `NominativeFeminineSingular`, `NominativeFemininePlural`, `NominativeNeuterSingular`, `NominativeNeuterPlural`, `GenitiveMasculineSingular`, `GenitiveMasculinePlural`, `GenitiveFeminineSingular`, `GenitiveFemininePlural`, `GenitiveNeuterSingular`, `GenitiveNeuterPlural`, `DativeMasculineSingular`, `DativeMasculinePlural`, `DativeFeminineSingular`, `DativeFemininePlural`, `DativeNeuterSingular`, `DativeNeuterPlural`, `AccusativeMasculineSingular`, `AccusativeMasculinePlural`, `AccusativeFeminineSingular`, `AccusativeFemininePlural`, `AccusativeNeuterSingular`, `AccusativeNeuterPlural`, `AblativeMasculineSingular`, `AblativeMasculinePlural`, `AblativeFeminineSingular`, `AblativeFemininePlural`, `AblativeNeuterSingular`, `AblativeNeuterPlural`, `VocativeMasculineSingular`, `VocativeMasculinePlural`, `VocativeFeminineSingular`, `VocativeFemininePlural`, `VocativeNeuterSingular`, `VocativeNeuterPlural`, `Comparative`, `Superlative`, `Adverb`

Citation form: `NominativeMasculineSingular, NominativeFeminineSingular|GenitiveMasculineSingular, NominativeNeuterSingular` (comma-separated; parts you give are output exactly and narrow the prediction).

**verb** cells: `IndicativePresentFirstSingular`, `IndicativePresentFirstPlural`, `IndicativePresentSecondSingular`, `IndicativePresentSecondPlural`, `IndicativePresentThirdSingular`, `IndicativePresentThirdPlural`, `IndicativeImperfectFirstSingular`, `IndicativeImperfectFirstPlural`, `IndicativeImperfectSecondSingular`, `IndicativeImperfectSecondPlural`, `IndicativeImperfectThirdSingular`, `IndicativeImperfectThirdPlural`, `IndicativeFutureFirstSingular`, `IndicativeFutureFirstPlural`, `IndicativeFutureSecondSingular`, `IndicativeFutureSecondPlural`, `IndicativeFutureThirdSingular`, `IndicativeFutureThirdPlural`, `IndicativePerfectFirstSingular`, `IndicativePerfectFirstPlural`, `IndicativePerfectSecondSingular`, `IndicativePerfectSecondPlural`, `IndicativePerfectThirdSingular`, `IndicativePerfectThirdPlural`, `IndicativePluperfectFirstSingular`, `IndicativePluperfectFirstPlural`, `IndicativePluperfectSecondSingular`, `IndicativePluperfectSecondPlural`, `IndicativePluperfectThirdSingular`, `IndicativePluperfectThirdPlural`, `IndicativeFuturePerfectFirstSingular`, `IndicativeFuturePerfectFirstPlural`, `IndicativeFuturePerfectSecondSingular`, `IndicativeFuturePerfectSecondPlural`, `IndicativeFuturePerfectThirdSingular`, `IndicativeFuturePerfectThirdPlural`, `SubjunctivePresentFirstSingular`, `SubjunctivePresentFirstPlural`, `SubjunctivePresentSecondSingular`, `SubjunctivePresentSecondPlural`, `SubjunctivePresentThirdSingular`, `SubjunctivePresentThirdPlural`, `SubjunctiveImperfectFirstSingular`, `SubjunctiveImperfectFirstPlural`, `SubjunctiveImperfectSecondSingular`, `SubjunctiveImperfectSecondPlural`, `SubjunctiveImperfectThirdSingular`, `SubjunctiveImperfectThirdPlural`, `SubjunctivePerfectFirstSingular`, `SubjunctivePerfectFirstPlural`, `SubjunctivePerfectSecondSingular`, `SubjunctivePerfectSecondPlural`, `SubjunctivePerfectThirdSingular`, `SubjunctivePerfectThirdPlural`, `SubjunctivePluperfectFirstSingular`, `SubjunctivePluperfectFirstPlural`, `SubjunctivePluperfectSecondSingular`, `SubjunctivePluperfectSecondPlural`, `SubjunctivePluperfectThirdSingular`, `SubjunctivePluperfectThirdPlural`, `IndicativePassivePresentFirstSingular`, `IndicativePassivePresentFirstPlural`, `IndicativePassivePresentSecondSingular`, `IndicativePassivePresentSecondPlural`, `IndicativePassivePresentThirdSingular`, `IndicativePassivePresentThirdPlural`, `IndicativePassiveImperfectFirstSingular`, `IndicativePassiveImperfectFirstPlural`, `IndicativePassiveImperfectSecondSingular`, `IndicativePassiveImperfectSecondPlural`, `IndicativePassiveImperfectThirdSingular`, `IndicativePassiveImperfectThirdPlural`, `IndicativePassiveFutureFirstSingular`, `IndicativePassiveFutureFirstPlural`, `IndicativePassiveFutureSecondSingular`, `IndicativePassiveFutureSecondPlural`, `IndicativePassiveFutureThirdSingular`, `IndicativePassiveFutureThirdPlural`, `SubjunctivePassivePresentFirstSingular`, `SubjunctivePassivePresentFirstPlural`, `SubjunctivePassivePresentSecondSingular`, `SubjunctivePassivePresentSecondPlural`, `SubjunctivePassivePresentThirdSingular`, `SubjunctivePassivePresentThirdPlural`, `SubjunctivePassiveImperfectFirstSingular`, `SubjunctivePassiveImperfectFirstPlural`, `SubjunctivePassiveImperfectSecondSingular`, `SubjunctivePassiveImperfectSecondPlural`, `SubjunctivePassiveImperfectThirdSingular`, `SubjunctivePassiveImperfectThirdPlural`, `ImperativeSingular`, `ImperativePlural`, `FutureImperativeSecondSingular`, `FutureImperativeSecondPlural`, `FutureImperativeThirdSingular`, `FutureImperativeThirdPlural`, `ImperativePassiveSingular`, `ImperativePassivePlural`, `FutureImperativePassiveSecondSingular`, `FutureImperativePassiveSecondPlural`, `FutureImperativePassiveThirdSingular`, `FutureImperativePassiveThirdPlural`, `InfinitiveActivePresent`, `InfinitiveActiveFuture`, `InfinitiveActivePerfect`, `InfinitivePassivePresent`, `InfinitivePassiveFuture`, `InfinitivePassivePerfect`, `ParticiplePresent`, `ParticipleFuture`, `ParticiplePassiveFuture`, `ParticiplePassivePerfect`, `GerundGenitive`, `GerundDative`, `GerundAccusative`, `GerundAblative`, `SupineAccusative`, `SupineAblative`

Citation form: `IndicativePresentFirstSingular, InfinitiveActivePresent, IndicativePerfectFirstSingular, SupineAccusative|ParticiplePassivePerfect` (comma-separated; parts you give are output exactly and narrow the prediction).

**pron** cells: `NominativeMasculineSingular`, `NominativeMasculinePlural`, `NominativeFeminineSingular`, `NominativeFemininePlural`, `NominativeNeuterSingular`, `NominativeNeuterPlural`, `GenitiveMasculineSingular`, `GenitiveMasculinePlural`, `GenitiveFeminineSingular`, `GenitiveFemininePlural`, `GenitiveNeuterSingular`, `GenitiveNeuterPlural`, `DativeMasculineSingular`, `DativeMasculinePlural`, `DativeFeminineSingular`, `DativeFemininePlural`, `DativeNeuterSingular`, `DativeNeuterPlural`, `AccusativeMasculineSingular`, `AccusativeMasculinePlural`, `AccusativeFeminineSingular`, `AccusativeFemininePlural`, `AccusativeNeuterSingular`, `AccusativeNeuterPlural`, `AblativeMasculineSingular`, `AblativeMasculinePlural`, `AblativeFeminineSingular`, `AblativeFemininePlural`, `AblativeNeuterSingular`, `AblativeNeuterPlural`, `VocativeMasculineSingular`, `VocativeMasculinePlural`, `VocativeFeminineSingular`, `VocativeFemininePlural`, `VocativeNeuterSingular`, `VocativeNeuterPlural`

**det** cells: `NominativeMasculineSingular`, `NominativeMasculinePlural`, `NominativeFeminineSingular`, `NominativeFemininePlural`, `NominativeNeuterSingular`, `NominativeNeuterPlural`, `GenitiveMasculineSingular`, `GenitiveMasculinePlural`, `GenitiveFeminineSingular`, `GenitiveFemininePlural`, `GenitiveNeuterSingular`, `GenitiveNeuterPlural`, `DativeMasculineSingular`, `DativeMasculinePlural`, `DativeFeminineSingular`, `DativeFemininePlural`, `DativeNeuterSingular`, `DativeNeuterPlural`, `AccusativeMasculineSingular`, `AccusativeMasculinePlural`, `AccusativeFeminineSingular`, `AccusativeFemininePlural`, `AccusativeNeuterSingular`, `AccusativeNeuterPlural`, `AblativeMasculineSingular`, `AblativeMasculinePlural`, `AblativeFeminineSingular`, `AblativeFemininePlural`, `AblativeNeuterSingular`, `AblativeNeuterPlural`, `VocativeMasculineSingular`, `VocativeMasculinePlural`, `VocativeFeminineSingular`, `VocativeFemininePlural`, `VocativeNeuterSingular`, `VocativeNeuterPlural`, `LocativeMasculineSingular`, `LocativeMasculinePlural`, `LocativeFeminineSingular`, `LocativeFemininePlural`, `LocativeNeuterSingular`, `LocativeNeuterPlural`

**num** cells: `NominativeMasculineSingular`, `NominativeMasculinePlural`, `NominativeFeminineSingular`, `NominativeFemininePlural`, `NominativeNeuterSingular`, `NominativeNeuterPlural`, `GenitiveMasculineSingular`, `GenitiveMasculinePlural`, `GenitiveFeminineSingular`, `GenitiveFemininePlural`, `GenitiveNeuterSingular`, `GenitiveNeuterPlural`, `DativeMasculineSingular`, `DativeMasculinePlural`, `DativeFeminineSingular`, `DativeFemininePlural`, `DativeNeuterSingular`, `DativeNeuterPlural`, `AccusativeMasculineSingular`, `AccusativeMasculinePlural`, `AccusativeFeminineSingular`, `AccusativeFemininePlural`, `AccusativeNeuterSingular`, `AccusativeNeuterPlural`, `AblativeMasculineSingular`, `AblativeMasculinePlural`, `AblativeFeminineSingular`, `AblativeFemininePlural`, `AblativeNeuterSingular`, `AblativeNeuterPlural`, `VocativeMasculineSingular`, `VocativeMasculinePlural`, `VocativeFeminineSingular`, `VocativeFemininePlural`, `VocativeNeuterSingular`, `VocativeNeuterPlural`

## How it works

Each attested form is stored as an edit tree from the lemma. A lemma's edits across all cells make its inflection class. A suffix tree over the word, read from its end and pruned by a description-length test, predicts the class of any word. Known words the tree gets wrong are kept in a small lexicon. Confidences are smoothed counts; variant weights follow Wiktionary's listing order (1, 1/2, 1/3…, nonstandard forms a tenth of that).

## License

Code: MIT OR Apache-2.0. Data: derived from [Wiktionary](https://www.wiktionary.org/) (via [kaikki.org](https://kaikki.org/)), CC BY-SA 4.0; the model bytes in `src/model.bin` are shared under the same license.
