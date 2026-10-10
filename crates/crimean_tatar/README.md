# crimean_tatar

CrimeanTatar inflection for Rust, learned from Wiktionary by [morphe](https://github.com/gold-silver-copper/morphe).
Inflects any word, known or made up. The `_scored` functions give every candidate with a confidence.

Generated from the Kaikki CrimeanTatar dump of 2026-10-03. Do not edit by hand: change the config in morphe and regenerate.

## Example

```rust
use crimean_tatar::*;
println!("{}", noun("kitap", Case::Dative, Number::Singular)); // kitapqa
println!("{}", noun("ev", Case::Locative, Number::Plural)); // evlerde
println!("{}", verb("ekmek", Tense::Past, Person::First, Number::Singular)); // ektim
println!("{}", verb("ekmek", Tense::Future, Person::Third, Number::Plural)); // ekecekler
```

Nouns have six cases in both numbers. Verbs have the present, past,
perfect, future, two continuatives, the conditional, the optative and the
imperative for six persons, plus the infinitive, the participles, the
gerunds and the converb. Input matching ignores the circumflex (â).

## Coverage

| Part of speech | Entries | Cells | Edits | Classes | Tree nodes | Lexicon |
|---|---|---|---|---|---|---|
| noun | 2529 | 12 | 315 | 61 | 117 | 719 |
| verb | 422 | 61 | 320 | 23 | 31 | 65 |

Model size: 149400 bytes.

**noun** cells: `NominativeSingular`, `NominativePlural`, `GenitiveSingular`, `GenitivePlural`, `DativeSingular`, `DativePlural`, `AccusativeSingular`, `AccusativePlural`, `LocativeSingular`, `LocativePlural`, `AblativeSingular`, `AblativePlural`

Citation form: `NominativeSingular, NominativePlural` (comma-separated; parts you give are output exactly and narrow the prediction).

**verb** cells: `Infinitive`, `PresentFirstSingular`, `PresentFirstPlural`, `PresentSecondSingular`, `PresentSecondPlural`, `PresentThirdSingular`, `PresentThirdPlural`, `PastFirstSingular`, `PastFirstPlural`, `PastSecondSingular`, `PastSecondPlural`, `PastThirdSingular`, `PastThirdPlural`, `PerfectFirstSingular`, `PerfectFirstPlural`, `PerfectSecondSingular`, `PerfectSecondPlural`, `PerfectThirdSingular`, `PerfectThirdPlural`, `FutureFirstSingular`, `FutureFirstPlural`, `FutureSecondSingular`, `FutureSecondPlural`, `FutureThirdSingular`, `FutureThirdPlural`, `ContinuativeFirstSingular`, `ContinuativeFirstPlural`, `ContinuativeSecondSingular`, `ContinuativeSecondPlural`, `ContinuativeThirdSingular`, `ContinuativeThirdPlural`, `PastContinuativeFirstSingular`, `PastContinuativeFirstPlural`, `PastContinuativeSecondSingular`, `PastContinuativeSecondPlural`, `PastContinuativeThirdSingular`, `PastContinuativeThirdPlural`, `ConditionalFirstSingular`, `ConditionalFirstPlural`, `ConditionalSecondSingular`, `ConditionalSecondPlural`, `ConditionalThirdSingular`, `ConditionalThirdPlural`, `OptativeFirstSingular`, `OptativeFirstPlural`, `OptativeSecondSingular`, `OptativeSecondPlural`, `OptativeThirdSingular`, `OptativeThirdPlural`, `ImperativeFirstSingular`, `ImperativeFirstPlural`, `ImperativeSecondSingular`, `ImperativeSecondPlural`, `ImperativeThirdSingular`, `ImperativeThirdPlural`, `ParticiplePast`, `ParticipleFuture`, `Gerund`, `RepeatedGerund`, `TemporalGerund`, `Converb`

Citation form: `Infinitive, PresentThirdSingular, PastThirdSingular` (comma-separated; parts you give are output exactly and narrow the prediction).

## How it works

Each attested form is stored as an edit tree from the lemma. A lemma's edits across all cells make its inflection class. A suffix tree over the word, read from its end and pruned by a description-length test, predicts the class of any word. Known words the tree gets wrong are kept in a small lexicon. Confidences are smoothed counts; variant weights follow Wiktionary's listing order (1, 1/2, 1/3…, nonstandard forms a tenth of that).

## License

Code: MIT OR Apache-2.0. Data: derived from [Wiktionary](https://www.wiktionary.org/) (via [kaikki.org](https://kaikki.org/)), CC BY-SA 4.0; the model bytes in `src/model.bin` are shared under the same license.
