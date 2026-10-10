# votic

Votic inflection for Rust, learned from Wiktionary by [morphe](https://github.com/gold-silver-copper/morphe).
Inflects any word, known or made up. The `_scored` functions give every candidate with a confidence.

Generated from the Kaikki Votic dump of 2026-10-03. Do not edit by hand: change the config in morphe and regenerate.

## Example

```rust
use votic::*;
println!("{}", noun("kana", Case::Inessive, Number::Singular)); // kanaz
println!("{}", noun("kana", Case::Partitive, Number::Plural)); // kanoit
println!("{}", verb("liikkua", Mood::Present, Person::Third, Number::Singular)); // liikub
println!("{}", verb("liikkua", Mood::Past, Person::First, Number::Singular)); // liikkuzin
println!("{}", verb_supine("liikkua", Supine::Illative)); // liikkuma
```

Nouns have ten cases in both numbers (the standardized Vaipooli forms).
Verbs have the present, past, conditional and imperative in all persons
and the impersonal, the infinitive, the supine and the participles;
compound tenses and negatives are built with auxiliaries and left out.

## Coverage

| Part of speech | Entries | Cells | Edits | Classes | Tree nodes | Lexicon |
|---|---|---|---|---|---|---|
| noun | 1632 | 20 | 2079 | 747 | 227 | 660 |
| adj | 265 | 20 | 557 | 147 | 44 | 89 |
| verb | 745 | 37 | 3209 | 278 | 132 | 277 |
| pron | 20 | 20 | 219 | 33 | 2 | 11 |
| num | 35 | 20 | 266 | 34 | 3 | 15 |

Model size: 818707 bytes.

**noun** cells: `NominativeSingular`, `NominativePlural`, `GenitiveSingular`, `GenitivePlural`, `PartitiveSingular`, `PartitivePlural`, `IllativeSingular`, `IllativePlural`, `InessiveSingular`, `InessivePlural`, `ElativeSingular`, `ElativePlural`, `AllativeSingular`, `AllativePlural`, `AdessiveSingular`, `AdessivePlural`, `AblativeSingular`, `AblativePlural`, `TranslativeSingular`, `TranslativePlural`

Citation form: `NominativeSingular, GenitiveSingular, PartitiveSingular` (comma-separated; parts you give are output exactly and narrow the prediction).

**adj** cells: `NominativeSingular`, `NominativePlural`, `GenitiveSingular`, `GenitivePlural`, `PartitiveSingular`, `PartitivePlural`, `IllativeSingular`, `IllativePlural`, `InessiveSingular`, `InessivePlural`, `ElativeSingular`, `ElativePlural`, `AllativeSingular`, `AllativePlural`, `AdessiveSingular`, `AdessivePlural`, `AblativeSingular`, `AblativePlural`, `TranslativeSingular`, `TranslativePlural`

**verb** cells: `Infinitive`, `SupineIllative`, `SupineInessive`, `SupineElative`, `SupineAbessive`, `PresentFirstSingular`, `PresentFirstPlural`, `PresentSecondSingular`, `PresentSecondPlural`, `PresentThirdSingular`, `PresentThirdPlural`, `PastFirstSingular`, `PastFirstPlural`, `PastSecondSingular`, `PastSecondPlural`, `PastThirdSingular`, `PastThirdPlural`, `ConditionalFirstSingular`, `ConditionalFirstPlural`, `ConditionalSecondSingular`, `ConditionalSecondPlural`, `ConditionalThirdSingular`, `ConditionalThirdPlural`, `ImperativeFirstSingular`, `ImperativeFirstPlural`, `ImperativeSecondSingular`, `ImperativeSecondPlural`, `ImperativeThirdSingular`, `ImperativeThirdPlural`, `ImpersonalPresent`, `ImpersonalPast`, `ImpersonalConditional`, `ImpersonalImperative`, `ParticipleActivePresent`, `ParticipleActivePast`, `ParticiplePassivePresent`, `ParticiplePassivePast`

Citation form: `Infinitive, PresentFirstSingular, PastFirstSingular` (comma-separated; parts you give are output exactly and narrow the prediction).

**pron** cells: `NominativeSingular`, `NominativePlural`, `GenitiveSingular`, `GenitivePlural`, `PartitiveSingular`, `PartitivePlural`, `IllativeSingular`, `IllativePlural`, `InessiveSingular`, `InessivePlural`, `ElativeSingular`, `ElativePlural`, `AllativeSingular`, `AllativePlural`, `AdessiveSingular`, `AdessivePlural`, `AblativeSingular`, `AblativePlural`, `TranslativeSingular`, `TranslativePlural`

**num** cells: `NominativeSingular`, `NominativePlural`, `GenitiveSingular`, `GenitivePlural`, `PartitiveSingular`, `PartitivePlural`, `IllativeSingular`, `IllativePlural`, `InessiveSingular`, `InessivePlural`, `ElativeSingular`, `ElativePlural`, `AllativeSingular`, `AllativePlural`, `AdessiveSingular`, `AdessivePlural`, `AblativeSingular`, `AblativePlural`, `TranslativeSingular`, `TranslativePlural`

## How it works

Each attested form is stored as an edit tree from the lemma. A lemma's edits across all cells make its inflection class. A suffix tree over the word, read from its end and pruned by a description-length test, predicts the class of any word. Known words the tree gets wrong are kept in a small lexicon. Confidences are smoothed counts; variant weights follow Wiktionary's listing order (1, 1/2, 1/3…, nonstandard forms a tenth of that).

## License

Code: MIT OR Apache-2.0. Data: derived from [Wiktionary](https://www.wiktionary.org/) (via [kaikki.org](https://kaikki.org/)), CC BY-SA 4.0; the model bytes in `src/model.bin` are shared under the same license.
