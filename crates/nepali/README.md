# nepali

Nepali inflection for Rust, learned from Wiktionary by [morphe](https://github.com/gold-silver-copper/morphe).
Inflects any word, known or made up. The `_scored` functions give every candidate with a confidence.

Generated from the Kaikki Nepali dump of 2026-10-03. Do not edit by hand: change the config in morphe and regenerate.

## Example

```rust
use nepali::*;
println!("{}", noun("तीर", Case::Ergative, Number::Plural)); // तीरहरूले
println!("{}", adj("राम्रो", Gender::Feminine, Number::Singular)); // राम्री
println!("{}", verb("खानु", Tense::Present, Person::FirstSingular, Gender::Masculine)); // खान्छु
println!("{}", verb("खानु", Tense::Past, Person::ThirdLow, Gender::Feminine)); // खाई
println!("{}", verb("खानु", Tense::Present, Person::Honorific, Gender::Masculine)); // खानुहुन्छ
println!("{}", verb_imperative("खानु", Person::SecondMiddle)); // खाऊ
println!("{}", verb_perfect_participle("खानु", Agreement::FeminineSingular)); // खाएकी
```

Nouns have six cases in both numbers. Verbs have the stem, infinitives,
the derived passive and causative, the participles, and the affirmative
present, past, habitual past, evidential past, definite and probable
future and injunctive for seven persons (म; हामी; तँ; तिमी; ऊ; उनी;
तपाईं) and two genders, plus the imperative. Wiktionary's tables don't
tag the first and second persons; they are read by their order in the row.
Negative forms are left out.

## Coverage

| Part of speech | Entries | Cells | Edits | Classes | Tree nodes | Lexicon |
|---|---|---|---|---|---|---|
| noun | 264 | 12 | 12 | 3 | 4 | 22 |
| adj | 31 | 4 | 7 | 3 | 2 | 3 |
| verb | 244 | 115 | 586 | 99 | 17 | 108 |

Model size: 63545 bytes.

**noun** cells: `NominativeSingular`, `NominativePlural`, `AccusativeSingular`, `AccusativePlural`, `ErgativeSingular`, `ErgativePlural`, `AblativeSingular`, `AblativePlural`, `GenitiveSingular`, `GenitivePlural`, `LocativeSingular`, `LocativePlural`

Citation form: `NominativeSingular` (comma-separated; parts you give are output exactly and narrow the prediction).

**adj** cells: `MasculineSingular`, `MasculinePlural`, `FeminineSingular`, `FemininePlural`

Citation form: `MasculineSingular, FeminineSingular` (comma-separated; parts you give are output exactly and narrow the prediction).

**verb** cells: `Stem`, `Infinitive`, `Purposive`, `Passive`, `Causative`, `Conjunctive`, `PerfectParticipleMasculineSingular`, `PerfectParticipleFeminineSingular`, `PerfectParticiplePlural`, `Perfect`, `ImperfectParticipleSimultaneous`, `ImperfectParticipleProgressive`, `ImperfectParticipleAdjectival`, `Participle`, `PresentFirstSingularMasculine`, `PresentFirstSingularFeminine`, `PresentFirstPluralMasculine`, `PresentFirstPluralFeminine`, `PresentSecondLowMasculine`, `PresentSecondLowFeminine`, `PresentSecondMiddleMasculine`, `PresentSecondMiddleFeminine`, `PresentThirdLowMasculine`, `PresentThirdLowFeminine`, `PresentThirdMiddleMasculine`, `PresentThirdMiddleFeminine`, `PresentHonorificMasculine`, `PresentHonorificFeminine`, `PastFirstSingularMasculine`, `PastFirstSingularFeminine`, `PastFirstPluralMasculine`, `PastFirstPluralFeminine`, `PastSecondLowMasculine`, `PastSecondLowFeminine`, `PastSecondMiddleMasculine`, `PastSecondMiddleFeminine`, `PastThirdLowMasculine`, `PastThirdLowFeminine`, `PastThirdMiddleMasculine`, `PastThirdMiddleFeminine`, `PastHonorificMasculine`, `PastHonorificFeminine`, `HabitualPastFirstSingularMasculine`, `HabitualPastFirstSingularFeminine`, `HabitualPastFirstPluralMasculine`, `HabitualPastFirstPluralFeminine`, `HabitualPastSecondLowMasculine`, `HabitualPastSecondLowFeminine`, `HabitualPastSecondMiddleMasculine`, `HabitualPastSecondMiddleFeminine`, `HabitualPastThirdLowMasculine`, `HabitualPastThirdLowFeminine`, `HabitualPastThirdMiddleMasculine`, `HabitualPastThirdMiddleFeminine`, `HabitualPastHonorificMasculine`, `HabitualPastHonorificFeminine`, `EvidentialFirstSingularMasculine`, `EvidentialFirstSingularFeminine`, `EvidentialFirstPluralMasculine`, `EvidentialFirstPluralFeminine`, `EvidentialSecondLowMasculine`, `EvidentialSecondLowFeminine`, `EvidentialSecondMiddleMasculine`, `EvidentialSecondMiddleFeminine`, `EvidentialThirdLowMasculine`, `EvidentialThirdLowFeminine`, `EvidentialThirdMiddleMasculine`, `EvidentialThirdMiddleFeminine`, `EvidentialHonorificMasculine`, `EvidentialHonorificFeminine`, `DefiniteFutureFirstSingularMasculine`, `DefiniteFutureFirstSingularFeminine`, `DefiniteFutureFirstPluralMasculine`, `DefiniteFutureFirstPluralFeminine`, `DefiniteFutureSecondLowMasculine`, `DefiniteFutureSecondLowFeminine`, `DefiniteFutureSecondMiddleMasculine`, `DefiniteFutureSecondMiddleFeminine`, `DefiniteFutureThirdLowMasculine`, `DefiniteFutureThirdLowFeminine`, `DefiniteFutureThirdMiddleMasculine`, `DefiniteFutureThirdMiddleFeminine`, `DefiniteFutureHonorificMasculine`, `DefiniteFutureHonorificFeminine`, `ProbableFutureFirstSingularMasculine`, `ProbableFutureFirstSingularFeminine`, `ProbableFutureFirstPluralMasculine`, `ProbableFutureFirstPluralFeminine`, `ProbableFutureSecondLowMasculine`, `ProbableFutureSecondLowFeminine`, `ProbableFutureSecondMiddleMasculine`, `ProbableFutureSecondMiddleFeminine`, `ProbableFutureThirdLowMasculine`, `ProbableFutureThirdLowFeminine`, `ProbableFutureThirdMiddleMasculine`, `ProbableFutureThirdMiddleFeminine`, `ProbableFutureHonorificMasculine`, `ProbableFutureHonorificFeminine`, `InjunctiveFirstSingularMasculine`, `InjunctiveFirstSingularFeminine`, `InjunctiveFirstPluralMasculine`, `InjunctiveFirstPluralFeminine`, `InjunctiveSecondLowMasculine`, `InjunctiveSecondLowFeminine`, `InjunctiveSecondMiddleMasculine`, `InjunctiveSecondMiddleFeminine`, `InjunctiveThirdLowMasculine`, `InjunctiveThirdLowFeminine`, `InjunctiveThirdMiddleMasculine`, `InjunctiveThirdMiddleFeminine`, `InjunctiveHonorificMasculine`, `InjunctiveHonorificFeminine`, `ImperativeSecondLow`, `ImperativeSecondMiddle`, `ImperativeHonorific`

Citation form: `Infinitive, PastThirdLowMasculine` (comma-separated; parts you give are output exactly and narrow the prediction).

## How it works

Each attested form is stored as an edit tree from the lemma. A lemma's edits across all cells make its inflection class. A suffix tree over the word, read from its end and pruned by a description-length test, predicts the class of any word. Known words the tree gets wrong are kept in a small lexicon. Confidences are smoothed counts; variant weights follow Wiktionary's listing order (1, 1/2, 1/3…, nonstandard forms a tenth of that).

## License

Code: MIT OR Apache-2.0. Data: derived from [Wiktionary](https://www.wiktionary.org/) (via [kaikki.org](https://kaikki.org/)), CC BY-SA 4.0; the model bytes in `src/model.bin` are shared under the same license.
