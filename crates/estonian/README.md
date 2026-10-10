# estonian

Estonian inflection for Rust, learned from Wiktionary by [morphe](https://github.com/gold-silver-copper/morphe).
Inflects any word, known or made up. The `_scored` functions give every candidate with a confidence.

Generated from the Kaikki Estonian dump of 2026-10-03. Do not edit by hand: change the config in morphe and regenerate.

## Example

```rust
use estonian::*;
assert_eq!(noun("maja", Case::Inessive, Number::Plural), "majades");
println!("{}", noun("jõgi", Case::Genitive, Number::Singular)); // jõe
println!("{}", noun("inimene", Case::Partitive, Number::Plural)); // inimesi
println!("{}", verb("tegema", Mood::IndicativePast, Person::First, Number::Singular)); // tegin
println!("{}", verb_infinitive_da("tegema")); // teha
println!("{}", verb_supine("tegema", SupineCase::Inessive)); // tegemas
println!("{}", verb_participle("tegema", Voice::Passive, Tense::Past)); // tehtud
// Citations pin principal parts: ma- and da-infinitive, present, past participle.
println!("{}", verb("lugema, lugeda, loeb, loetud", Mood::ConditionalPresent, Person::Third, Number::Plural));
```

Nouns, adjectives, pronouns and numerals have fourteen cases in both
numbers; adjectives also have the comparative and superlative. Verbs have
their one-word forms: the indicative present and past, the conditional
present and past, the imperative, the impersonal, the quotative, the ma-
and da-infinitives, the des-form and the participles. Compound tenses
(olen teinud) and negatives (ei tee) are built from auxiliaries and left out.

## Coverage

| Part of speech | Entries | Cells | Edits | Classes | Tree nodes | Lexicon |
|---|---|---|---|---|---|---|
| noun | 4312 | 28 | 4931 | 785 | 527 | 1493 |
| adj | 690 | 30 | 1588 | 266 | 77 | 329 |
| verb | 670 | 49 | 1767 | 126 | 47 | 179 |
| pron | 28 | 28 | 1554 | 51 | 3 | 26 |
| num | 34 | 28 | 447 | 42 | 12 | 21 |

Model size: 1646634 bytes.

**noun** cells: `NominativeSingular`, `NominativePlural`, `GenitiveSingular`, `GenitivePlural`, `PartitiveSingular`, `PartitivePlural`, `IllativeSingular`, `IllativePlural`, `InessiveSingular`, `InessivePlural`, `ElativeSingular`, `ElativePlural`, `AllativeSingular`, `AllativePlural`, `AdessiveSingular`, `AdessivePlural`, `AblativeSingular`, `AblativePlural`, `TranslativeSingular`, `TranslativePlural`, `TerminativeSingular`, `TerminativePlural`, `EssiveSingular`, `EssivePlural`, `AbessiveSingular`, `AbessivePlural`, `ComitativeSingular`, `ComitativePlural`

Citation form: `NominativeSingular, GenitiveSingular, PartitiveSingular` (comma-separated; parts you give are output exactly and narrow the prediction).

**adj** cells: `NominativeSingular`, `NominativePlural`, `GenitiveSingular`, `GenitivePlural`, `PartitiveSingular`, `PartitivePlural`, `IllativeSingular`, `IllativePlural`, `InessiveSingular`, `InessivePlural`, `ElativeSingular`, `ElativePlural`, `AllativeSingular`, `AllativePlural`, `AdessiveSingular`, `AdessivePlural`, `AblativeSingular`, `AblativePlural`, `TranslativeSingular`, `TranslativePlural`, `TerminativeSingular`, `TerminativePlural`, `EssiveSingular`, `EssivePlural`, `AbessiveSingular`, `AbessivePlural`, `ComitativeSingular`, `ComitativePlural`, `Comparative`, `Superlative`

Citation form: `NominativeSingular, GenitiveSingular, PartitiveSingular` (comma-separated; parts you give are output exactly and narrow the prediction).

**verb** cells: `InfinitiveMa`, `InfinitiveMaPassive`, `InfinitiveDa`, `SupineIllative`, `SupineInessive`, `SupineElative`, `SupineTranslative`, `SupineAbessive`, `Gerund`, `IndicativePresentFirstSingular`, `IndicativePresentFirstPlural`, `IndicativePresentSecondSingular`, `IndicativePresentSecondPlural`, `IndicativePresentThirdSingular`, `IndicativePresentThirdPlural`, `IndicativePastFirstSingular`, `IndicativePastFirstPlural`, `IndicativePastSecondSingular`, `IndicativePastSecondPlural`, `IndicativePastThirdSingular`, `IndicativePastThirdPlural`, `ConditionalPresentFirstSingular`, `ConditionalPresentFirstPlural`, `ConditionalPresentSecondSingular`, `ConditionalPresentSecondPlural`, `ConditionalPresentThirdSingular`, `ConditionalPresentThirdPlural`, `ConditionalPastFirstSingular`, `ConditionalPastFirstPlural`, `ConditionalPastSecondSingular`, `ConditionalPastSecondPlural`, `ConditionalPastThirdSingular`, `ConditionalPastThirdPlural`, `ImperativeFirstSingular`, `ImperativeFirstPlural`, `ImperativeSecondSingular`, `ImperativeSecondPlural`, `ImperativeThirdSingular`, `ImperativeThirdPlural`, `ImpersonalIndicativePresent`, `ImpersonalIndicativePast`, `ImpersonalConditionalPresent`, `ImpersonalImperative`, `QuotativeActive`, `QuotativePassive`, `ParticipleActivePresent`, `ParticipleActivePast`, `ParticiplePassivePresent`, `ParticiplePassivePast`

Citation form: `InfinitiveMa, InfinitiveDa, IndicativePresentThirdSingular, ParticiplePassivePast` (comma-separated; parts you give are output exactly and narrow the prediction).

**pron** cells: `NominativeSingular`, `NominativePlural`, `GenitiveSingular`, `GenitivePlural`, `PartitiveSingular`, `PartitivePlural`, `IllativeSingular`, `IllativePlural`, `InessiveSingular`, `InessivePlural`, `ElativeSingular`, `ElativePlural`, `AllativeSingular`, `AllativePlural`, `AdessiveSingular`, `AdessivePlural`, `AblativeSingular`, `AblativePlural`, `TranslativeSingular`, `TranslativePlural`, `TerminativeSingular`, `TerminativePlural`, `EssiveSingular`, `EssivePlural`, `AbessiveSingular`, `AbessivePlural`, `ComitativeSingular`, `ComitativePlural`

**num** cells: `NominativeSingular`, `NominativePlural`, `GenitiveSingular`, `GenitivePlural`, `PartitiveSingular`, `PartitivePlural`, `IllativeSingular`, `IllativePlural`, `InessiveSingular`, `InessivePlural`, `ElativeSingular`, `ElativePlural`, `AllativeSingular`, `AllativePlural`, `AdessiveSingular`, `AdessivePlural`, `AblativeSingular`, `AblativePlural`, `TranslativeSingular`, `TranslativePlural`, `TerminativeSingular`, `TerminativePlural`, `EssiveSingular`, `EssivePlural`, `AbessiveSingular`, `AbessivePlural`, `ComitativeSingular`, `ComitativePlural`

## How it works

Each attested form is stored as an edit tree from the lemma. A lemma's edits across all cells make its inflection class. A suffix tree over the word, read from its end and pruned by a description-length test, predicts the class of any word. Known words the tree gets wrong are kept in a small lexicon. Confidences are smoothed counts; variant weights follow Wiktionary's listing order (1, 1/2, 1/3…, nonstandard forms a tenth of that).

## License

Code: MIT OR Apache-2.0. Data: derived from [Wiktionary](https://www.wiktionary.org/) (via [kaikki.org](https://kaikki.org/)), CC BY-SA 4.0; the model bytes in `src/model.bin` are shared under the same license.
