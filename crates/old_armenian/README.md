# old_armenian

OldArmenian inflection for Rust, learned from Wiktionary by [morphe](https://github.com/gold-silver-copper/morphe).
Inflects any word, known or made up. The `_scored` functions give every candidate with a confidence.

Generated from the Kaikki OldArmenian dump of 2026-10-03. Do not edit by hand: change the config in morphe and regenerate.

## Example

```rust
use old_armenian::*;
println!("{}", noun("գիր", Case::Genitive, Number::Singular)); // գրոյ
println!("{}", noun("գիր", Case::Instrumental, Number::Plural)); // գրովք
println!("{}", verb("խնդրեմ", Voice::Active, Tense::Aorist, Person::Third, Number::Singular)); // խնդրեաց
println!("{}", verb("տեսանեմ", Voice::Mediopassive, Tense::Present, Person::First, Number::Singular)); // տեսանիմ
// The aorist stem is lexical; a citation can give it.
println!("{}", verb("տեսանեմ, տեսի", Voice::Active, Tense::AoristSubjunctive, Person::Third, Number::Singular));
```

Nouns, adjectives, pronouns and numerals have seven cases in both
numbers. Verbs are looked up by the first person present (խնդրեմ) and
have the present, imperfect and aorist indicative and the present and
aorist subjunctive, active and mediopassive, for six persons, plus the
imperative, cohortative and prohibitive, the infinitives and the
participle. Many verbs are known from their aorist alone. Input matching
ignores the stress and question marks (՛ ՜ ՞) and reads և as եւ.

## Coverage

| Part of speech | Entries | Cells | Edits | Classes | Tree nodes | Lexicon |
|---|---|---|---|---|---|---|
| noun | 3063 | 14 | 1189 | 745 | 348 | 1357 |
| adj | 812 | 14 | 369 | 203 | 101 | 309 |
| verb | 927 | 75 | 1217 | 231 | 63 | 294 |
| pron | 48 | 14 | 225 | 54 | 8 | 32 |
| num | 123 | 14 | 121 | 44 | 16 | 45 |

Model size: 265312 bytes.

**noun** cells: `NominativeSingular`, `NominativePlural`, `AccusativeSingular`, `AccusativePlural`, `GenitiveSingular`, `GenitivePlural`, `DativeSingular`, `DativePlural`, `LocativeSingular`, `LocativePlural`, `AblativeSingular`, `AblativePlural`, `InstrumentalSingular`, `InstrumentalPlural`

Citation form: `NominativeSingular, GenitiveSingular` (comma-separated; parts you give are output exactly and narrow the prediction).

**adj** cells: `NominativeSingular`, `NominativePlural`, `AccusativeSingular`, `AccusativePlural`, `GenitiveSingular`, `GenitivePlural`, `DativeSingular`, `DativePlural`, `LocativeSingular`, `LocativePlural`, `AblativeSingular`, `AblativePlural`, `InstrumentalSingular`, `InstrumentalPlural`

**verb** cells: `InfinitiveActive`, `InfinitiveMediopassive`, `Participle`, `ActivePresentFirstSingular`, `ActivePresentFirstPlural`, `ActivePresentSecondSingular`, `ActivePresentSecondPlural`, `ActivePresentThirdSingular`, `ActivePresentThirdPlural`, `ActiveImperfectFirstSingular`, `ActiveImperfectFirstPlural`, `ActiveImperfectSecondSingular`, `ActiveImperfectSecondPlural`, `ActiveImperfectThirdSingular`, `ActiveImperfectThirdPlural`, `ActiveAoristFirstSingular`, `ActiveAoristFirstPlural`, `ActiveAoristSecondSingular`, `ActiveAoristSecondPlural`, `ActiveAoristThirdSingular`, `ActiveAoristThirdPlural`, `ActivePresentSubjunctiveFirstSingular`, `ActivePresentSubjunctiveFirstPlural`, `ActivePresentSubjunctiveSecondSingular`, `ActivePresentSubjunctiveSecondPlural`, `ActivePresentSubjunctiveThirdSingular`, `ActivePresentSubjunctiveThirdPlural`, `ActiveAoristSubjunctiveFirstSingular`, `ActiveAoristSubjunctiveFirstPlural`, `ActiveAoristSubjunctiveSecondSingular`, `ActiveAoristSubjunctiveSecondPlural`, `ActiveAoristSubjunctiveThirdSingular`, `ActiveAoristSubjunctiveThirdPlural`, `MediopassivePresentFirstSingular`, `MediopassivePresentFirstPlural`, `MediopassivePresentSecondSingular`, `MediopassivePresentSecondPlural`, `MediopassivePresentThirdSingular`, `MediopassivePresentThirdPlural`, `MediopassiveImperfectFirstSingular`, `MediopassiveImperfectFirstPlural`, `MediopassiveImperfectSecondSingular`, `MediopassiveImperfectSecondPlural`, `MediopassiveImperfectThirdSingular`, `MediopassiveImperfectThirdPlural`, `MediopassiveAoristFirstSingular`, `MediopassiveAoristFirstPlural`, `MediopassiveAoristSecondSingular`, `MediopassiveAoristSecondPlural`, `MediopassiveAoristThirdSingular`, `MediopassiveAoristThirdPlural`, `MediopassivePresentSubjunctiveFirstSingular`, `MediopassivePresentSubjunctiveFirstPlural`, `MediopassivePresentSubjunctiveSecondSingular`, `MediopassivePresentSubjunctiveSecondPlural`, `MediopassivePresentSubjunctiveThirdSingular`, `MediopassivePresentSubjunctiveThirdPlural`, `MediopassiveAoristSubjunctiveFirstSingular`, `MediopassiveAoristSubjunctiveFirstPlural`, `MediopassiveAoristSubjunctiveSecondSingular`, `MediopassiveAoristSubjunctiveSecondPlural`, `MediopassiveAoristSubjunctiveThirdSingular`, `MediopassiveAoristSubjunctiveThirdPlural`, `ImperativeActivePlainSingular`, `ImperativeActivePlainPlural`, `ImperativeActiveCohortativeSingular`, `ImperativeActiveCohortativePlural`, `ImperativeActiveProhibitiveSingular`, `ImperativeActiveProhibitivePlural`, `ImperativeMediopassivePlainSingular`, `ImperativeMediopassivePlainPlural`, `ImperativeMediopassiveCohortativeSingular`, `ImperativeMediopassiveCohortativePlural`, `ImperativeMediopassiveProhibitiveSingular`, `ImperativeMediopassiveProhibitivePlural`

Citation form: `Lemma, ActiveAoristFirstSingular|MediopassiveAoristFirstSingular` (comma-separated; parts you give are output exactly and narrow the prediction).

**pron** cells: `NominativeSingular`, `NominativePlural`, `AccusativeSingular`, `AccusativePlural`, `GenitiveSingular`, `GenitivePlural`, `DativeSingular`, `DativePlural`, `LocativeSingular`, `LocativePlural`, `AblativeSingular`, `AblativePlural`, `InstrumentalSingular`, `InstrumentalPlural`

**num** cells: `NominativeSingular`, `NominativePlural`, `AccusativeSingular`, `AccusativePlural`, `GenitiveSingular`, `GenitivePlural`, `DativeSingular`, `DativePlural`, `LocativeSingular`, `LocativePlural`, `AblativeSingular`, `AblativePlural`, `InstrumentalSingular`, `InstrumentalPlural`

## How it works

Each attested form is stored as an edit tree from the lemma. A lemma's edits across all cells make its inflection class. A suffix tree over the word, read from its end and pruned by a description-length test, predicts the class of any word. Known words the tree gets wrong are kept in a small lexicon. Confidences are smoothed counts; variant weights follow Wiktionary's listing order (1, 1/2, 1/3…, nonstandard forms a tenth of that).

## License

Code: MIT OR Apache-2.0. Data: derived from [Wiktionary](https://www.wiktionary.org/) (via [kaikki.org](https://kaikki.org/)), CC BY-SA 4.0; the model bytes in `src/model.bin` are shared under the same license.
