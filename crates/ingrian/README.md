# ingrian

Ingrian inflection for Rust, learned from Wiktionary by [morphe](https://github.com/gold-silver-copper/morphe).
Inflects any word, known or made up. The `_scored` functions give every candidate with a confidence.

Generated from the Kaikki Ingrian dump of 2026-10-03. Do not edit by hand: change the config in morphe and regenerate.

## Example

```rust
use ingrian::*;
println!("{}", noun("kana", Case::Inessive, Number::Singular)); // kanas
println!("{}", noun("kana", Case::Partitive, Number::Plural)); // kanoja
println!("{}", verb("tulla", Mood::Present, Person::Third, Number::Singular)); // tulloo
println!("{}", verb("tulla", Mood::Conditional, Person::First, Number::Singular)); // tulisin
println!("{}", verb_participle("tulla", Voice::Passive, Tense::Past)); // tultu
```

Nouns have twelve cases in both numbers. Verbs have the present, past,
conditional, imperative and potential in all persons and the impersonal,
the infinitives and the participles; compound tenses and negatives (en
tule, oon tult) are built with auxiliaries and left out.

## Coverage

| Part of speech | Entries | Cells | Edits | Classes | Tree nodes | Lexicon |
|---|---|---|---|---|---|---|
| noun | 4731 | 24 | 3374 | 960 | 620 | 1362 |
| adj | 853 | 24 | 1010 | 214 | 74 | 221 |
| verb | 1409 | 48 | 5268 | 656 | 175 | 430 |
| pron | 26 | 24 | 317 | 36 | 7 | 21 |
| num | 13 | 24 | 235 | 26 | 2 | 11 |

Model size: 2018489 bytes.

**noun** cells: `NominativeSingular`, `NominativePlural`, `GenitiveSingular`, `GenitivePlural`, `PartitiveSingular`, `PartitivePlural`, `IllativeSingular`, `IllativePlural`, `InessiveSingular`, `InessivePlural`, `ElativeSingular`, `ElativePlural`, `AllativeSingular`, `AllativePlural`, `AdessiveSingular`, `AdessivePlural`, `AblativeSingular`, `AblativePlural`, `TranslativeSingular`, `TranslativePlural`, `EssiveSingular`, `EssivePlural`, `ExessiveSingular`, `ExessivePlural`

Citation form: `NominativeSingular, GenitiveSingular, PartitiveSingular` (comma-separated; parts you give are output exactly and narrow the prediction).

**adj** cells: `NominativeSingular`, `NominativePlural`, `GenitiveSingular`, `GenitivePlural`, `PartitiveSingular`, `PartitivePlural`, `IllativeSingular`, `IllativePlural`, `InessiveSingular`, `InessivePlural`, `ElativeSingular`, `ElativePlural`, `AllativeSingular`, `AllativePlural`, `AdessiveSingular`, `AdessivePlural`, `AblativeSingular`, `AblativePlural`, `TranslativeSingular`, `TranslativePlural`, `EssiveSingular`, `EssivePlural`, `ExessiveSingular`, `ExessivePlural`

**verb** cells: `Infinitive`, `InfinitiveCaseIiInessive`, `InfinitiveCaseIiInstructive`, `InfinitiveCaseIiiIllative`, `InfinitiveCaseIiiInessive`, `InfinitiveCaseIiiElative`, `InfinitiveCaseIiiAbessive`, `InfinitiveCaseIvNominative`, `InfinitiveCaseIvPartitive`, `PresentFirstSingular`, `PresentFirstPlural`, `PresentSecondSingular`, `PresentSecondPlural`, `PresentThirdSingular`, `PresentThirdPlural`, `PastFirstSingular`, `PastFirstPlural`, `PastSecondSingular`, `PastSecondPlural`, `PastThirdSingular`, `PastThirdPlural`, `ConditionalFirstSingular`, `ConditionalFirstPlural`, `ConditionalSecondSingular`, `ConditionalSecondPlural`, `ConditionalThirdSingular`, `ConditionalThirdPlural`, `ImperativeFirstSingular`, `ImperativeFirstPlural`, `ImperativeSecondSingular`, `ImperativeSecondPlural`, `ImperativeThirdSingular`, `ImperativeThirdPlural`, `PotentialFirstSingular`, `PotentialFirstPlural`, `PotentialSecondSingular`, `PotentialSecondPlural`, `PotentialThirdSingular`, `PotentialThirdPlural`, `ImpersonalPresent`, `ImpersonalPast`, `ImpersonalConditional`, `ImpersonalImperative`, `ImpersonalPotential`, `ParticipleActivePresent`, `ParticipleActivePast`, `ParticiplePassivePresent`, `ParticiplePassivePast`

Citation form: `Infinitive, PresentFirstSingular, PastFirstSingular` (comma-separated; parts you give are output exactly and narrow the prediction).

**pron** cells: `NominativeSingular`, `NominativePlural`, `GenitiveSingular`, `GenitivePlural`, `PartitiveSingular`, `PartitivePlural`, `IllativeSingular`, `IllativePlural`, `InessiveSingular`, `InessivePlural`, `ElativeSingular`, `ElativePlural`, `AllativeSingular`, `AllativePlural`, `AdessiveSingular`, `AdessivePlural`, `AblativeSingular`, `AblativePlural`, `TranslativeSingular`, `TranslativePlural`, `EssiveSingular`, `EssivePlural`, `ExessiveSingular`, `ExessivePlural`

**num** cells: `NominativeSingular`, `NominativePlural`, `GenitiveSingular`, `GenitivePlural`, `PartitiveSingular`, `PartitivePlural`, `IllativeSingular`, `IllativePlural`, `InessiveSingular`, `InessivePlural`, `ElativeSingular`, `ElativePlural`, `AllativeSingular`, `AllativePlural`, `AdessiveSingular`, `AdessivePlural`, `AblativeSingular`, `AblativePlural`, `TranslativeSingular`, `TranslativePlural`, `EssiveSingular`, `EssivePlural`, `ExessiveSingular`, `ExessivePlural`

## How it works

Each attested form is stored as an edit tree from the lemma. A lemma's edits across all cells make its inflection class. A suffix tree over the word, read from its end and pruned by a description-length test, predicts the class of any word. Known words the tree gets wrong are kept in a small lexicon. Confidences are smoothed counts; variant weights follow Wiktionary's listing order (1, 1/2, 1/3…, nonstandard forms a tenth of that).

## License

Code: MIT OR Apache-2.0. Data: derived from [Wiktionary](https://www.wiktionary.org/) (via [kaikki.org](https://kaikki.org/)), CC BY-SA 4.0; the model bytes in `src/model.bin` are shared under the same license.
