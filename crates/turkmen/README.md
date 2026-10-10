# turkmen

Turkmen inflection for Rust, learned from Wiktionary by [morphe](https://github.com/gold-silver-copper/morphe).
Inflects any word, known or made up. The `_scored` functions give every candidate with a confidence.

Generated from the Kaikki Turkmen dump of 2026-10-03. Do not edit by hand: change the config in morphe and regenerate.

## Example

```rust
use turkmen::*;
println!("{}", noun("kitap", Case::Dative, Number::Singular)); // kitaba
println!("{}", noun("at", Case::Ablative, Number::Plural)); // atlardan
println!("{}", verb("atmak", Tense::Present, Person::First, Number::Singular)); // atýaryn
println!("{}", verb("atmak", Tense::Past, Person::Third, Number::Plural)); // atdylar
```

Nouns have six cases in both numbers. Verbs have the present, three pasts
(proximal, distal, unwitnessed), the indefinite future, two past-futures,
the conditional, the imperative and the intentive and obligative pasts for
six persons, plus the infinitive, the participles and the gerund.
Wiktionary marks long vowels (āt, atýār) the standard spelling leaves out:
input matching ignores them, and `marks(false)` removes them from output.

## Coverage

| Part of speech | Entries | Cells | Edits | Classes | Tree nodes | Lexicon |
|---|---|---|---|---|---|---|
| noun | 948 | 12 | 251 | 64 | 85 | 183 |
| verb | 82 | 74 | 505 | 24 | 11 | 10 |
| pron | 5 | 12 | 42 | 5 | 2 | 3 |

Model size: 45220 bytes.

**noun** cells: `NominativeSingular`, `NominativePlural`, `GenitiveSingular`, `GenitivePlural`, `DativeSingular`, `DativePlural`, `AccusativeSingular`, `AccusativePlural`, `LocativeSingular`, `LocativePlural`, `AblativeSingular`, `AblativePlural`

Citation form: `NominativeSingular, AccusativeSingular, NominativePlural` (comma-separated; parts you give are output exactly and narrow the prediction).

**verb** cells: `Infinitive`, `PresentFirstSingular`, `PresentFirstPlural`, `PresentSecondSingular`, `PresentSecondPlural`, `PresentThirdSingular`, `PresentThirdPlural`, `PastFirstSingular`, `PastFirstPlural`, `PastSecondSingular`, `PastSecondPlural`, `PastThirdSingular`, `PastThirdPlural`, `DistalPastFirstSingular`, `DistalPastFirstPlural`, `DistalPastSecondSingular`, `DistalPastSecondPlural`, `DistalPastThirdSingular`, `DistalPastThirdPlural`, `UnwitnessedFirstSingular`, `UnwitnessedFirstPlural`, `UnwitnessedSecondSingular`, `UnwitnessedSecondPlural`, `UnwitnessedThirdSingular`, `UnwitnessedThirdPlural`, `FutureFirstSingular`, `FutureFirstPlural`, `FutureSecondSingular`, `FutureSecondPlural`, `FutureThirdSingular`, `FutureThirdPlural`, `PastFutureFirstSingular`, `PastFutureFirstPlural`, `PastFutureSecondSingular`, `PastFutureSecondPlural`, `PastFutureThirdSingular`, `PastFutureThirdPlural`, `DefinitePastFutureFirstSingular`, `DefinitePastFutureFirstPlural`, `DefinitePastFutureSecondSingular`, `DefinitePastFutureSecondPlural`, `DefinitePastFutureThirdSingular`, `DefinitePastFutureThirdPlural`, `ConditionalFirstSingular`, `ConditionalFirstPlural`, `ConditionalSecondSingular`, `ConditionalSecondPlural`, `ConditionalThirdSingular`, `ConditionalThirdPlural`, `ImperativeFirstSingular`, `ImperativeFirstPlural`, `ImperativeSecondSingular`, `ImperativeSecondPlural`, `ImperativeThirdSingular`, `ImperativeThirdPlural`, `IntentivePastFirstSingular`, `IntentivePastFirstPlural`, `IntentivePastSecondSingular`, `IntentivePastSecondPlural`, `IntentivePastThirdSingular`, `IntentivePastThirdPlural`, `ObligativePastFirstSingular`, `ObligativePastFirstPlural`, `ObligativePastSecondSingular`, `ObligativePastSecondPlural`, `ObligativePastThirdSingular`, `ObligativePastThirdPlural`, `DefiniteFuture`, `Intentive`, `Obligative`, `ParticiplePresent`, `ParticiplePast`, `ParticipleFuture`, `Gerund`

Citation form: `Infinitive, PresentThirdSingular, PastThirdSingular` (comma-separated; parts you give are output exactly and narrow the prediction).

**pron** cells: `NominativeSingular`, `NominativePlural`, `GenitiveSingular`, `GenitivePlural`, `DativeSingular`, `DativePlural`, `AccusativeSingular`, `AccusativePlural`, `LocativeSingular`, `LocativePlural`, `AblativeSingular`, `AblativePlural`

## How it works

Each attested form is stored as an edit tree from the lemma. A lemma's edits across all cells make its inflection class. A suffix tree over the word, read from its end and pruned by a description-length test, predicts the class of any word. Known words the tree gets wrong are kept in a small lexicon. Confidences are smoothed counts; variant weights follow Wiktionary's listing order (1, 1/2, 1/3…, nonstandard forms a tenth of that).

## License

Code: MIT OR Apache-2.0. Data: derived from [Wiktionary](https://www.wiktionary.org/) (via [kaikki.org](https://kaikki.org/)), CC BY-SA 4.0; the model bytes in `src/model.bin` are shared under the same license.
