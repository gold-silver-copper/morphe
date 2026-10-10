# kazakh

Kazakh inflection for Rust, learned from Wiktionary by [morphe](https://github.com/gold-silver-copper/morphe).
Inflects any word, known or made up. The `_scored` functions give every candidate with a confidence.

Generated from the Kaikki Kazakh dump of 2026-10-03. Do not edit by hand: change the config in morphe and regenerate.

## Example

```rust
use kazakh::*;
assert_eq!(noun("кітап", Case::Dative, Number::Singular), "кітапқа");
println!("{}", noun("үй", Case::Locative, Number::Plural)); // үйлерде
println!("{}", noun("бала", Case::Equative, Number::Singular)); // баладай
println!("{}", verb("келу", Tense::Preterite, Person::First, Number::Singular)); // келдім
println!("{}", verb("келу", Tense::Imperative, Person::SecondFormal, Number::Singular)); // келіңіз
```

Nouns have eight cases, including the equative (-дай, "like"), in both
numbers. Verbs are looked up by their verbal noun (келу) and have eleven
tenses and moods for the first person, the informal and formal second
person, and the third person. Wiktionary has conjugation tables for a few
hundred verbs only.

## Coverage

| Part of speech | Entries | Cells | Edits | Classes | Tree nodes | Lexicon |
|---|---|---|---|---|---|---|
| noun | 4648 | 16 | 900 | 345 | 194 | 487 |
| verb | 168 | 88 | 1007 | 21 | 18 | 43 |

Model size: 324479 bytes.

**noun** cells: `NominativeSingular`, `NominativePlural`, `GenitiveSingular`, `GenitivePlural`, `DativeSingular`, `DativePlural`, `AccusativeSingular`, `AccusativePlural`, `LocativeSingular`, `LocativePlural`, `AblativeSingular`, `AblativePlural`, `InstrumentalSingular`, `InstrumentalPlural`, `EquativeSingular`, `EquativePlural`

Citation form: `NominativeSingular, NominativePlural` (comma-separated; parts you give are output exactly and narrow the prediction).

**verb** cells: `AoristFirstSingular`, `AoristFirstPlural`, `AoristSecondSingular`, `AoristSecondPlural`, `AoristSecondFormalSingular`, `AoristSecondFormalPlural`, `AoristThirdSingular`, `AoristThirdPlural`, `PreteriteFirstSingular`, `PreteriteFirstPlural`, `PreteriteSecondSingular`, `PreteriteSecondPlural`, `PreteriteSecondFormalSingular`, `PreteriteSecondFormalPlural`, `PreteriteThirdSingular`, `PreteriteThirdPlural`, `PerfectFirstSingular`, `PerfectFirstPlural`, `PerfectSecondSingular`, `PerfectSecondPlural`, `PerfectSecondFormalSingular`, `PerfectSecondFormalPlural`, `PerfectThirdSingular`, `PerfectThirdPlural`, `PluperfectFirstSingular`, `PluperfectFirstPlural`, `PluperfectSecondSingular`, `PluperfectSecondPlural`, `PluperfectSecondFormalSingular`, `PluperfectSecondFormalPlural`, `PluperfectThirdSingular`, `PluperfectThirdPlural`, `RenarrativeFirstSingular`, `RenarrativeFirstPlural`, `RenarrativeSecondSingular`, `RenarrativeSecondPlural`, `RenarrativeSecondFormalSingular`, `RenarrativeSecondFormalPlural`, `RenarrativeThirdSingular`, `RenarrativeThirdPlural`, `PastHabitualFirstSingular`, `PastHabitualFirstPlural`, `PastHabitualSecondSingular`, `PastHabitualSecondPlural`, `PastHabitualSecondFormalSingular`, `PastHabitualSecondFormalPlural`, `PastHabitualThirdSingular`, `PastHabitualThirdPlural`, `PresumptiveFirstSingular`, `PresumptiveFirstPlural`, `PresumptiveSecondSingular`, `PresumptiveSecondPlural`, `PresumptiveSecondFormalSingular`, `PresumptiveSecondFormalPlural`, `PresumptiveThirdSingular`, `PresumptiveThirdPlural`, `IntentiveFirstSingular`, `IntentiveFirstPlural`, `IntentiveSecondSingular`, `IntentiveSecondPlural`, `IntentiveSecondFormalSingular`, `IntentiveSecondFormalPlural`, `IntentiveThirdSingular`, `IntentiveThirdPlural`, `OptativeFirstSingular`, `OptativeFirstPlural`, `OptativeSecondSingular`, `OptativeSecondPlural`, `OptativeSecondFormalSingular`, `OptativeSecondFormalPlural`, `OptativeThirdSingular`, `OptativeThirdPlural`, `ConditionalFirstSingular`, `ConditionalFirstPlural`, `ConditionalSecondSingular`, `ConditionalSecondPlural`, `ConditionalSecondFormalSingular`, `ConditionalSecondFormalPlural`, `ConditionalThirdSingular`, `ConditionalThirdPlural`, `ImperativeFirstSingular`, `ImperativeFirstPlural`, `ImperativeSecondSingular`, `ImperativeSecondPlural`, `ImperativeSecondFormalSingular`, `ImperativeSecondFormalPlural`, `ImperativeThirdSingular`, `ImperativeThirdPlural`

## How it works

Each attested form is stored as an edit tree from the lemma. A lemma's edits across all cells make its inflection class. A suffix tree over the word, read from its end and pruned by a description-length test, predicts the class of any word. Known words the tree gets wrong are kept in a small lexicon. Confidences are smoothed counts; variant weights follow Wiktionary's listing order (1, 1/2, 1/3…, nonstandard forms a tenth of that).

## License

Code: MIT OR Apache-2.0. Data: derived from [Wiktionary](https://www.wiktionary.org/) (via [kaikki.org](https://kaikki.org/)), CC BY-SA 4.0; the model bytes in `src/model.bin` are shared under the same license.
