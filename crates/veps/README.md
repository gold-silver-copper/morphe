# veps

Veps inflection for Rust, learned from Wiktionary by [morphe](https://github.com/gold-silver-copper/morphe).
Inflects any word, known or made up. The `_scored` functions give every candidate with a confidence.

Generated from the Kaikki Veps dump of 2026-10-03. Do not edit by hand: change the config in morphe and regenerate.

## Example

```rust
use veps::*;
println!("{}", noun("kala", Case::Inessive, Number::Plural)); // kaloiš
println!("{}", noun("kala", Case::Illative, Number::Singular)); // kalaha
println!("{}", verb("torta", Mood::Past, Person::First, Number::Singular)); // torzin
println!("{}", verb("torta", Mood::Conditional, Person::Third, Number::Plural)); // tordaižiba
```

Nouns have twenty-three cases in both numbers, including the
approximatives, terminatives and additives. Verbs have the indicative
present and past, the imperative, the conditional present and past and the
potential in all persons, the connegatives, the three infinitives and the
participles; compound tenses and negatives are left out.

## Coverage

| Part of speech | Entries | Cells | Edits | Classes | Tree nodes | Lexicon |
|---|---|---|---|---|---|---|
| noun | 872 | 46 | 1629 | 139 | 90 | 318 |
| adj | 219 | 46 | 954 | 42 | 26 | 51 |
| verb | 656 | 60 | 1099 | 153 | 54 | 230 |
| pron | 14 | 46 | 229 | 14 | 4 | 12 |
| num | 37 | 46 | 270 | 25 | 19 | 24 |

Model size: 304728 bytes.

**noun** cells: `NominativeSingular`, `NominativePlural`, `AccusativeSingular`, `AccusativePlural`, `GenitiveSingular`, `GenitivePlural`, `PartitiveSingular`, `PartitivePlural`, `EssiveInstructiveSingular`, `EssiveInstructivePlural`, `TranslativeSingular`, `TranslativePlural`, `InessiveSingular`, `InessivePlural`, `ElativeSingular`, `ElativePlural`, `IllativeSingular`, `IllativePlural`, `AdessiveSingular`, `AdessivePlural`, `AblativeSingular`, `AblativePlural`, `AllativeSingular`, `AllativePlural`, `AbessiveSingular`, `AbessivePlural`, `ComitativeSingular`, `ComitativePlural`, `ProlativeSingular`, `ProlativePlural`, `ApproximativeISingular`, `ApproximativeIPlural`, `ApproximativeIISingular`, `ApproximativeIIPlural`, `EgressiveSingular`, `EgressivePlural`, `TerminativeISingular`, `TerminativeIPlural`, `TerminativeIISingular`, `TerminativeIIPlural`, `TerminativeIIISingular`, `TerminativeIIIPlural`, `AdditiveISingular`, `AdditiveIPlural`, `AdditiveIISingular`, `AdditiveIIPlural`

Citation form: `NominativeSingular, GenitiveSingular, PartitiveSingular` (comma-separated; parts you give are output exactly and narrow the prediction).

**adj** cells: `NominativeSingular`, `NominativePlural`, `AccusativeSingular`, `AccusativePlural`, `GenitiveSingular`, `GenitivePlural`, `PartitiveSingular`, `PartitivePlural`, `EssiveInstructiveSingular`, `EssiveInstructivePlural`, `TranslativeSingular`, `TranslativePlural`, `InessiveSingular`, `InessivePlural`, `ElativeSingular`, `ElativePlural`, `IllativeSingular`, `IllativePlural`, `AdessiveSingular`, `AdessivePlural`, `AblativeSingular`, `AblativePlural`, `AllativeSingular`, `AllativePlural`, `AbessiveSingular`, `AbessivePlural`, `ComitativeSingular`, `ComitativePlural`, `ProlativeSingular`, `ProlativePlural`, `ApproximativeISingular`, `ApproximativeIPlural`, `ApproximativeIISingular`, `ApproximativeIIPlural`, `EgressiveSingular`, `EgressivePlural`, `TerminativeISingular`, `TerminativeIPlural`, `TerminativeIISingular`, `TerminativeIIPlural`, `TerminativeIIISingular`, `TerminativeIIIPlural`, `AdditiveISingular`, `AdditiveIPlural`, `AdditiveIISingular`, `AdditiveIIPlural`

**verb** cells: `Infinitive`, `InfinitiveIIInessive`, `InfinitiveIIInstructive`, `InfinitiveIIIInessive`, `InfinitiveIIIIllative`, `InfinitiveIIIElative`, `InfinitiveIIIAdessive`, `InfinitiveIIIAbessive`, `PresentFirstSingular`, `PresentFirstPlural`, `PresentSecondSingular`, `PresentSecondPlural`, `PresentThirdSingular`, `PresentThirdPlural`, `PastFirstSingular`, `PastFirstPlural`, `PastSecondSingular`, `PastSecondPlural`, `PastThirdSingular`, `PastThirdPlural`, `ImperativeFirstSingular`, `ImperativeFirstPlural`, `ImperativeSecondSingular`, `ImperativeSecondPlural`, `ImperativeThirdSingular`, `ImperativeThirdPlural`, `ConditionalFirstSingular`, `ConditionalFirstPlural`, `ConditionalSecondSingular`, `ConditionalSecondPlural`, `ConditionalThirdSingular`, `ConditionalThirdPlural`, `ConditionalPastFirstSingular`, `ConditionalPastFirstPlural`, `ConditionalPastSecondSingular`, `ConditionalPastSecondPlural`, `ConditionalPastThirdSingular`, `ConditionalPastThirdPlural`, `PotentialFirstSingular`, `PotentialFirstPlural`, `PotentialSecondSingular`, `PotentialSecondPlural`, `PotentialThirdSingular`, `PotentialThirdPlural`, `ConnegativePresentSingular`, `ConnegativePresentPlural`, `ConnegativePastSingular`, `ConnegativePastPlural`, `ConnegativeImperativeSingular`, `ConnegativeImperativePlural`, `ConnegativeConditionalSingular`, `ConnegativeConditionalPlural`, `ConnegativeConditionalPastSingular`, `ConnegativeConditionalPastPlural`, `ConnegativePotentialSingular`, `ConnegativePotentialPlural`, `ParticipleActivePresent`, `ParticipleActivePast`, `ParticiplePassivePresent`, `ParticiplePassivePast`

Citation form: `Infinitive, PresentFirstSingular, PastFirstSingular` (comma-separated; parts you give are output exactly and narrow the prediction).

**pron** cells: `NominativeSingular`, `NominativePlural`, `AccusativeSingular`, `AccusativePlural`, `GenitiveSingular`, `GenitivePlural`, `PartitiveSingular`, `PartitivePlural`, `EssiveInstructiveSingular`, `EssiveInstructivePlural`, `TranslativeSingular`, `TranslativePlural`, `InessiveSingular`, `InessivePlural`, `ElativeSingular`, `ElativePlural`, `IllativeSingular`, `IllativePlural`, `AdessiveSingular`, `AdessivePlural`, `AblativeSingular`, `AblativePlural`, `AllativeSingular`, `AllativePlural`, `AbessiveSingular`, `AbessivePlural`, `ComitativeSingular`, `ComitativePlural`, `ProlativeSingular`, `ProlativePlural`, `ApproximativeISingular`, `ApproximativeIPlural`, `ApproximativeIISingular`, `ApproximativeIIPlural`, `EgressiveSingular`, `EgressivePlural`, `TerminativeISingular`, `TerminativeIPlural`, `TerminativeIISingular`, `TerminativeIIPlural`, `TerminativeIIISingular`, `TerminativeIIIPlural`, `AdditiveISingular`, `AdditiveIPlural`, `AdditiveIISingular`, `AdditiveIIPlural`

**num** cells: `NominativeSingular`, `NominativePlural`, `AccusativeSingular`, `AccusativePlural`, `GenitiveSingular`, `GenitivePlural`, `PartitiveSingular`, `PartitivePlural`, `EssiveInstructiveSingular`, `EssiveInstructivePlural`, `TranslativeSingular`, `TranslativePlural`, `InessiveSingular`, `InessivePlural`, `ElativeSingular`, `ElativePlural`, `IllativeSingular`, `IllativePlural`, `AdessiveSingular`, `AdessivePlural`, `AblativeSingular`, `AblativePlural`, `AllativeSingular`, `AllativePlural`, `AbessiveSingular`, `AbessivePlural`, `ComitativeSingular`, `ComitativePlural`, `ProlativeSingular`, `ProlativePlural`, `ApproximativeISingular`, `ApproximativeIPlural`, `ApproximativeIISingular`, `ApproximativeIIPlural`, `EgressiveSingular`, `EgressivePlural`, `TerminativeISingular`, `TerminativeIPlural`, `TerminativeIISingular`, `TerminativeIIPlural`, `TerminativeIIISingular`, `TerminativeIIIPlural`, `AdditiveISingular`, `AdditiveIPlural`, `AdditiveIISingular`, `AdditiveIIPlural`

## How it works

Each attested form is stored as an edit tree from the lemma. A lemma's edits across all cells make its inflection class. A suffix tree over the word, read from its end and pruned by a description-length test, predicts the class of any word. Known words the tree gets wrong are kept in a small lexicon. Confidences are smoothed counts; variant weights follow Wiktionary's listing order (1, 1/2, 1/3…, nonstandard forms a tenth of that).

## License

Code: MIT OR Apache-2.0. Data: derived from [Wiktionary](https://www.wiktionary.org/) (via [kaikki.org](https://kaikki.org/)), CC BY-SA 4.0; the model bytes in `src/model.bin` are shared under the same license.
