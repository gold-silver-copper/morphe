# finnish

Finnish inflection for Rust, learned from Wiktionary by [morphe](https://github.com/gold-silver-copper/morphe).
Inflects any word, known or made up. The `_scored` functions give every candidate with a confidence.

Generated from the Kaikki Finnish dump of 2026-10-03. Do not edit by hand: change the config in morphe and regenerate.

## Example

```rust
use finnish::*;
assert_eq!(noun("talo", Case::Inessive, Number::Plural), "taloissa");
println!("{}", noun("käsi", Case::Partitive, Number::Singular)); // kättä
println!("{}", verb("puhua", Mood::IndicativePast, Person::Third, Number::Plural)); // puhuivat
println!("{}", verb_connegative("tietää")); // tiedä
println!("{}", adj_comparative("hyvä")); // parempi
// Citations pin principal parts: nominative, genitive, partitive, partitive plural.
println!("{}", noun("kuusi, kuuden", Case::Essive, Number::Singular)); // kuutena
```

Cells cover the fourteen cases in both numbers, the comitative, the finite
moods (indicative present and past, conditional, potential, imperative), the
passive, the connegative, the infinitives and the participles. Possessive
suffixes and compound tenses (perfect, pluperfect, negatives) are left out:
Wiktionary's possessive tables don't tag noun number, and compound tenses
are built from `olla`/`ei` plus a participle or the connegative.

## Coverage

| Part of speech | Entries | Cells | Edits | Classes | Tree nodes | Lexicon |
|---|---|---|---|---|---|---|
| noun | 128897 | 27 | 20561 | 2889 | 4117 | 7291 |
| adj | 17707 | 29 | 2488 | 691 | 940 | 6553 |
| verb | 12392 | 52 | 6752 | 454 | 362 | 667 |
| pron | 80 | 28 | 1071 | 116 | 9 | 70 |
| num | 77 | 26 | 1012 | 103 | 36 | 40 |

Model size: 11318990 bytes.

**noun** cells: `NominativeSingular`, `NominativePlural`, `GenitiveSingular`, `GenitivePlural`, `PartitiveSingular`, `PartitivePlural`, `InessiveSingular`, `InessivePlural`, `ElativeSingular`, `ElativePlural`, `IllativeSingular`, `IllativePlural`, `AdessiveSingular`, `AdessivePlural`, `AblativeSingular`, `AblativePlural`, `AllativeSingular`, `AllativePlural`, `EssiveSingular`, `EssivePlural`, `TranslativeSingular`, `TranslativePlural`, `AbessiveSingular`, `AbessivePlural`, `InstructiveSingular`, `InstructivePlural`, `Comitative`

Citation form: `NominativeSingular, GenitiveSingular, PartitiveSingular, PartitivePlural` (comma-separated; parts you give are output exactly and narrow the prediction).

**adj** cells: `NominativeSingular`, `NominativePlural`, `GenitiveSingular`, `GenitivePlural`, `PartitiveSingular`, `PartitivePlural`, `InessiveSingular`, `InessivePlural`, `ElativeSingular`, `ElativePlural`, `IllativeSingular`, `IllativePlural`, `AdessiveSingular`, `AdessivePlural`, `AblativeSingular`, `AblativePlural`, `AllativeSingular`, `AllativePlural`, `EssiveSingular`, `EssivePlural`, `TranslativeSingular`, `TranslativePlural`, `AbessiveSingular`, `AbessivePlural`, `InstructiveSingular`, `InstructivePlural`, `Comitative`, `Comparative`, `Superlative`

Citation form: `NominativeSingular, GenitiveSingular, PartitiveSingular, PartitivePlural` (comma-separated; parts you give are output exactly and narrow the prediction).

**verb** cells: `Infinitive`, `IndicativePresentFirstSingular`, `IndicativePresentFirstPlural`, `IndicativePresentSecondSingular`, `IndicativePresentSecondPlural`, `IndicativePresentThirdSingular`, `IndicativePresentThirdPlural`, `IndicativePastFirstSingular`, `IndicativePastFirstPlural`, `IndicativePastSecondSingular`, `IndicativePastSecondPlural`, `IndicativePastThirdSingular`, `IndicativePastThirdPlural`, `ConditionalFirstSingular`, `ConditionalFirstPlural`, `ConditionalSecondSingular`, `ConditionalSecondPlural`, `ConditionalThirdSingular`, `ConditionalThirdPlural`, `PotentialFirstSingular`, `PotentialFirstPlural`, `PotentialSecondSingular`, `PotentialSecondPlural`, `PotentialThirdSingular`, `PotentialThirdPlural`, `ImperativeSecondSingular`, `ImperativeSecondPlural`, `ImperativeThirdSingular`, `ImperativeThirdPlural`, `ImperativeFirstPlural`, `Connegative`, `PassiveIndicativePresent`, `PassiveIndicativePast`, `PassiveConditional`, `PassivePotential`, `PassiveImperative`, `SecondInfinitiveInessiveActive`, `SecondInfinitiveInessivePassive`, `SecondInfinitiveInstructive`, `ThirdInfinitiveInessive`, `ThirdInfinitiveElative`, `ThirdInfinitiveIllative`, `ThirdInfinitiveAdessive`, `ThirdInfinitiveAbessive`, `ThirdInfinitiveInstructive`, `FourthInfinitive`, `ParticipleActivePresent`, `ParticipleActivePast`, `ParticiplePassivePresent`, `ParticiplePassivePast`, `AgentParticiple`, `NegativeParticiple`

Citation form: `Infinitive, IndicativePresentFirstSingular, IndicativePastThirdSingular` (comma-separated; parts you give are output exactly and narrow the prediction).

**pron** cells: `NominativeSingular`, `NominativePlural`, `GenitiveSingular`, `GenitivePlural`, `PartitiveSingular`, `PartitivePlural`, `InessiveSingular`, `InessivePlural`, `ElativeSingular`, `ElativePlural`, `IllativeSingular`, `IllativePlural`, `AdessiveSingular`, `AdessivePlural`, `AblativeSingular`, `AblativePlural`, `AllativeSingular`, `AllativePlural`, `EssiveSingular`, `EssivePlural`, `TranslativeSingular`, `TranslativePlural`, `AbessiveSingular`, `AbessivePlural`, `InstructiveSingular`, `InstructivePlural`, `AccusativeSingular`, `AccusativePlural`

**num** cells: `NominativeSingular`, `NominativePlural`, `GenitiveSingular`, `GenitivePlural`, `PartitiveSingular`, `PartitivePlural`, `InessiveSingular`, `InessivePlural`, `ElativeSingular`, `ElativePlural`, `IllativeSingular`, `IllativePlural`, `AdessiveSingular`, `AdessivePlural`, `AblativeSingular`, `AblativePlural`, `AllativeSingular`, `AllativePlural`, `EssiveSingular`, `EssivePlural`, `TranslativeSingular`, `TranslativePlural`, `AbessiveSingular`, `AbessivePlural`, `InstructiveSingular`, `InstructivePlural`

## How it works

Each attested form is stored as an edit tree from the lemma. A lemma's edits across all cells make its inflection class. A suffix tree over the word, read from its end and pruned by a description-length test, predicts the class of any word. Known words the tree gets wrong are kept in a small lexicon. Confidences are smoothed counts; variant weights follow Wiktionary's listing order (1, 1/2, 1/3…, nonstandard forms a tenth of that).

## License

Code: MIT OR Apache-2.0. Data: derived from [Wiktionary](https://www.wiktionary.org/) (via [kaikki.org](https://kaikki.org/)), CC BY-SA 4.0; the model bytes in `src/model.bin` are shared under the same license.
