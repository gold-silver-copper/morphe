# moksha

Moksha inflection for Rust, learned from Wiktionary by [morphe](https://github.com/gold-silver-copper/morphe).
Inflects any word, known or made up. The `_scored` functions give every candidate with a confidence.

Generated from the Kaikki Moksha dump of 2026-10-03. Do not edit by hand: change the config in morphe and regenerate.

## Example

```rust
use moksha::*;
println!("{}", noun("он", Case::Inessive, Number::Singular)); // онца
println!("{}", noun_definite("он", Case::Genitive, Number::Plural)); // онтнень
println!("{}", verb("морамс", Tense::Past, Person::First, Number::Singular)); // морань
println!("{}", verb_negative("морамс", Tense::Present, Person::Third, Number::Plural)); // аф морайхть
println!("{}", verb_imperative("морамс", Number::Singular)); // морак
```

Nouns have thirteen cases in the indefinite singular and the indefinite
nominative plural, and the definite nominative, genitive and dative in
both numbers. Verbs have the present, two pasts, the periphrastic future,
the conditional, the conditional subjunctive, the desiderative and the
optative for six persons, positive and negative (the negatives are
analytic: аф моран), the imperative, the infinitives, the participles and
the gerunds.

## Coverage

| Part of speech | Entries | Cells | Edits | Classes | Tree nodes | Lexicon |
|---|---|---|---|---|---|---|
| noun | 176 | 26 | 118 | 50 | 26 | 44 |
| noun_definite | 180 | 6 | 62 | 16 | 19 | 58 |
| verb | 55 | 108 | 290 | 64 | 10 | 43 |

Model size: 28004 bytes.

**noun** cells: `NominativeSingular`, `NominativePlural`, `GenitiveSingular`, `GenitivePlural`, `DativeSingular`, `DativePlural`, `AblativeSingular`, `AblativePlural`, `InessiveSingular`, `InessivePlural`, `ElativeSingular`, `ElativePlural`, `IllativeSingular`, `IllativePlural`, `LativeSingular`, `LativePlural`, `ProlativeSingular`, `ProlativePlural`, `CausativeSingular`, `CausativePlural`, `TranslativeSingular`, `TranslativePlural`, `AbessiveSingular`, `AbessivePlural`, `ComparativeSingular`, `ComparativePlural`

Citation form: `NominativeSingular, NominativePlural` (comma-separated; parts you give are output exactly and narrow the prediction).

**noun_definite** cells: `NominativeSingular`, `NominativePlural`, `GenitiveSingular`, `GenitivePlural`, `DativeSingular`, `DativePlural`

**verb** cells: `Infinitive`, `InfinitiveMa`, `InfinitiveMda`, `PresentFirstSingular`, `PresentFirstPlural`, `PresentSecondSingular`, `PresentSecondPlural`, `PresentThirdSingular`, `PresentThirdPlural`, `PastFirstSingular`, `PastFirstPlural`, `PastSecondSingular`, `PastSecondPlural`, `PastThirdSingular`, `PastThirdPlural`, `PastIIFirstSingular`, `PastIIFirstPlural`, `PastIISecondSingular`, `PastIISecondPlural`, `PastIIThirdSingular`, `PastIIThirdPlural`, `FutureFirstSingular`, `FutureFirstPlural`, `FutureSecondSingular`, `FutureSecondPlural`, `FutureThirdSingular`, `FutureThirdPlural`, `ConditionalFirstSingular`, `ConditionalFirstPlural`, `ConditionalSecondSingular`, `ConditionalSecondPlural`, `ConditionalThirdSingular`, `ConditionalThirdPlural`, `ConditionalSubjunctiveFirstSingular`, `ConditionalSubjunctiveFirstPlural`, `ConditionalSubjunctiveSecondSingular`, `ConditionalSubjunctiveSecondPlural`, `ConditionalSubjunctiveThirdSingular`, `ConditionalSubjunctiveThirdPlural`, `DesiderativeFirstSingular`, `DesiderativeFirstPlural`, `DesiderativeSecondSingular`, `DesiderativeSecondPlural`, `DesiderativeThirdSingular`, `DesiderativeThirdPlural`, `OptativeFirstSingular`, `OptativeFirstPlural`, `OptativeSecondSingular`, `OptativeSecondPlural`, `OptativeThirdSingular`, `OptativeThirdPlural`, `NegativePresentFirstSingular`, `NegativePresentFirstPlural`, `NegativePresentSecondSingular`, `NegativePresentSecondPlural`, `NegativePresentThirdSingular`, `NegativePresentThirdPlural`, `NegativePastFirstSingular`, `NegativePastFirstPlural`, `NegativePastSecondSingular`, `NegativePastSecondPlural`, `NegativePastThirdSingular`, `NegativePastThirdPlural`, `NegativePastIIFirstSingular`, `NegativePastIIFirstPlural`, `NegativePastIISecondSingular`, `NegativePastIISecondPlural`, `NegativePastIIThirdSingular`, `NegativePastIIThirdPlural`, `NegativeFutureFirstSingular`, `NegativeFutureFirstPlural`, `NegativeFutureSecondSingular`, `NegativeFutureSecondPlural`, `NegativeFutureThirdSingular`, `NegativeFutureThirdPlural`, `NegativeConditionalFirstSingular`, `NegativeConditionalFirstPlural`, `NegativeConditionalSecondSingular`, `NegativeConditionalSecondPlural`, `NegativeConditionalThirdSingular`, `NegativeConditionalThirdPlural`, `NegativeConditionalSubjunctiveFirstSingular`, `NegativeConditionalSubjunctiveFirstPlural`, `NegativeConditionalSubjunctiveSecondSingular`, `NegativeConditionalSubjunctiveSecondPlural`, `NegativeConditionalSubjunctiveThirdSingular`, `NegativeConditionalSubjunctiveThirdPlural`, `NegativeDesiderativeFirstSingular`, `NegativeDesiderativeFirstPlural`, `NegativeDesiderativeSecondSingular`, `NegativeDesiderativeSecondPlural`, `NegativeDesiderativeThirdSingular`, `NegativeDesiderativeThirdPlural`, `NegativeOptativeFirstSingular`, `NegativeOptativeFirstPlural`, `NegativeOptativeSecondSingular`, `NegativeOptativeSecondPlural`, `NegativeOptativeThirdSingular`, `NegativeOptativeThirdPlural`, `ImperativeSingular`, `ImperativePlural`, `NegativeImperativeSingular`, `NegativeImperativePlural`, `ParticiplePassivePast`, `ParticiplePassivePresent`, `ParticipleActivePresent`, `Gerund`, `NegativeGerund`

Citation form: `Infinitive, PresentFirstSingular, PastThirdSingular` (comma-separated; parts you give are output exactly and narrow the prediction).

## How it works

Each attested form is stored as an edit tree from the lemma. A lemma's edits across all cells make its inflection class. A suffix tree over the word, read from its end and pruned by a description-length test, predicts the class of any word. Known words the tree gets wrong are kept in a small lexicon. Confidences are smoothed counts; variant weights follow Wiktionary's listing order (1, 1/2, 1/3…, nonstandard forms a tenth of that).

## License

Code: MIT OR Apache-2.0. Data: derived from [Wiktionary](https://www.wiktionary.org/) (via [kaikki.org](https://kaikki.org/)), CC BY-SA 4.0; the model bytes in `src/model.bin` are shared under the same license.
