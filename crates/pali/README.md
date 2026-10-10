# pali

Pali inflection for Rust, learned from Wiktionary by [morphe](https://github.com/gold-silver-copper/morphe).
Inflects any word, known or made up. The `_scored` functions give every candidate with a confidence.

Generated from the Kaikki Pali dump of 2026-10-03. Do not edit by hand: change the config in morphe and regenerate.

## Example

```rust
use pali::*;
assert_eq!(noun("dhamma", Case::Locative, Number::Plural), "dhammesu");
println!("{}", noun("citta", Case::Instrumental, Number::Singular)); // cittena
println!("{:?}", noun_gender("citta"));
println!("{}", adj("kusala", Gender::Feminine, Case::Nominative, Number::Singular)); // kusalā
println!("{}", verb("gacchati", Mood::Optative, Voice::Active, Person::Third, Number::Singular));
println!("{}", verb("gacchati", Mood::Imperative, Voice::Active, Person::Second, Number::Singular)); // gaccha
```

Pali in roman script; entries in other scripts are left out. Nouns are
looked up by their stem (dhamma) and have eight cases in both numbers;
adjectives decline in three genders. Verbs are looked up by the third
person singular present and have the present, imperative, optative,
aorist, imperfect, future and conditional, active and middle. Input
matching ignores the macrons and dots (ā ṃ ṭ ṇ…).

## Coverage

| Part of speech | Entries | Cells | Edits | Classes | Tree nodes | Lexicon |
|---|---|---|---|---|---|---|
| noun | 1413 | 16 | 3472 | 913 | 199 | 790 |
| adj | 450 | 48 | 2229 | 283 | 82 | 199 |
| verb | 80 | 84 | 1086 | 100 | 29 | 39 |
| pron | 20 | 48 | 134 | 32 | 2 | 12 |
| num | 28 | 48 | 203 | 36 | 6 | 19 |

Model size: 481845 bytes.

**noun** cells: `NominativeSingular`, `NominativePlural`, `AccusativeSingular`, `AccusativePlural`, `InstrumentalSingular`, `InstrumentalPlural`, `DativeSingular`, `DativePlural`, `AblativeSingular`, `AblativePlural`, `GenitiveSingular`, `GenitivePlural`, `LocativeSingular`, `LocativePlural`, `VocativeSingular`, `VocativePlural`

Citation form: `NominativeSingular, GenitiveSingular` (comma-separated; parts you give are output exactly and narrow the prediction).

**adj** cells: `MasculineNominativeSingular`, `MasculineNominativePlural`, `MasculineAccusativeSingular`, `MasculineAccusativePlural`, `MasculineInstrumentalSingular`, `MasculineInstrumentalPlural`, `MasculineDativeSingular`, `MasculineDativePlural`, `MasculineAblativeSingular`, `MasculineAblativePlural`, `MasculineGenitiveSingular`, `MasculineGenitivePlural`, `MasculineLocativeSingular`, `MasculineLocativePlural`, `MasculineVocativeSingular`, `MasculineVocativePlural`, `FeminineNominativeSingular`, `FeminineNominativePlural`, `FeminineAccusativeSingular`, `FeminineAccusativePlural`, `FeminineInstrumentalSingular`, `FeminineInstrumentalPlural`, `FeminineDativeSingular`, `FeminineDativePlural`, `FeminineAblativeSingular`, `FeminineAblativePlural`, `FeminineGenitiveSingular`, `FeminineGenitivePlural`, `FeminineLocativeSingular`, `FeminineLocativePlural`, `FeminineVocativeSingular`, `FeminineVocativePlural`, `NeuterNominativeSingular`, `NeuterNominativePlural`, `NeuterAccusativeSingular`, `NeuterAccusativePlural`, `NeuterInstrumentalSingular`, `NeuterInstrumentalPlural`, `NeuterDativeSingular`, `NeuterDativePlural`, `NeuterAblativeSingular`, `NeuterAblativePlural`, `NeuterGenitiveSingular`, `NeuterGenitivePlural`, `NeuterLocativeSingular`, `NeuterLocativePlural`, `NeuterVocativeSingular`, `NeuterVocativePlural`

**verb** cells: `PresentActiveFirstSingular`, `PresentActiveFirstPlural`, `PresentActiveSecondSingular`, `PresentActiveSecondPlural`, `PresentActiveThirdSingular`, `PresentActiveThirdPlural`, `PresentMiddleFirstSingular`, `PresentMiddleFirstPlural`, `PresentMiddleSecondSingular`, `PresentMiddleSecondPlural`, `PresentMiddleThirdSingular`, `PresentMiddleThirdPlural`, `ImperativeActiveFirstSingular`, `ImperativeActiveFirstPlural`, `ImperativeActiveSecondSingular`, `ImperativeActiveSecondPlural`, `ImperativeActiveThirdSingular`, `ImperativeActiveThirdPlural`, `ImperativeMiddleFirstSingular`, `ImperativeMiddleFirstPlural`, `ImperativeMiddleSecondSingular`, `ImperativeMiddleSecondPlural`, `ImperativeMiddleThirdSingular`, `ImperativeMiddleThirdPlural`, `OptativeActiveFirstSingular`, `OptativeActiveFirstPlural`, `OptativeActiveSecondSingular`, `OptativeActiveSecondPlural`, `OptativeActiveThirdSingular`, `OptativeActiveThirdPlural`, `OptativeMiddleFirstSingular`, `OptativeMiddleFirstPlural`, `OptativeMiddleSecondSingular`, `OptativeMiddleSecondPlural`, `OptativeMiddleThirdSingular`, `OptativeMiddleThirdPlural`, `AoristActiveFirstSingular`, `AoristActiveFirstPlural`, `AoristActiveSecondSingular`, `AoristActiveSecondPlural`, `AoristActiveThirdSingular`, `AoristActiveThirdPlural`, `AoristMiddleFirstSingular`, `AoristMiddleFirstPlural`, `AoristMiddleSecondSingular`, `AoristMiddleSecondPlural`, `AoristMiddleThirdSingular`, `AoristMiddleThirdPlural`, `ImperfectActiveFirstSingular`, `ImperfectActiveFirstPlural`, `ImperfectActiveSecondSingular`, `ImperfectActiveSecondPlural`, `ImperfectActiveThirdSingular`, `ImperfectActiveThirdPlural`, `ImperfectMiddleFirstSingular`, `ImperfectMiddleFirstPlural`, `ImperfectMiddleSecondSingular`, `ImperfectMiddleSecondPlural`, `ImperfectMiddleThirdSingular`, `ImperfectMiddleThirdPlural`, `FutureActiveFirstSingular`, `FutureActiveFirstPlural`, `FutureActiveSecondSingular`, `FutureActiveSecondPlural`, `FutureActiveThirdSingular`, `FutureActiveThirdPlural`, `FutureMiddleFirstSingular`, `FutureMiddleFirstPlural`, `FutureMiddleSecondSingular`, `FutureMiddleSecondPlural`, `FutureMiddleThirdSingular`, `FutureMiddleThirdPlural`, `ConditionalActiveFirstSingular`, `ConditionalActiveFirstPlural`, `ConditionalActiveSecondSingular`, `ConditionalActiveSecondPlural`, `ConditionalActiveThirdSingular`, `ConditionalActiveThirdPlural`, `ConditionalMiddleFirstSingular`, `ConditionalMiddleFirstPlural`, `ConditionalMiddleSecondSingular`, `ConditionalMiddleSecondPlural`, `ConditionalMiddleThirdSingular`, `ConditionalMiddleThirdPlural`

Citation form: `PresentActiveThirdSingular` (comma-separated; parts you give are output exactly and narrow the prediction).

**pron** cells: `MasculineNominativeSingular`, `MasculineNominativePlural`, `MasculineAccusativeSingular`, `MasculineAccusativePlural`, `MasculineInstrumentalSingular`, `MasculineInstrumentalPlural`, `MasculineDativeSingular`, `MasculineDativePlural`, `MasculineAblativeSingular`, `MasculineAblativePlural`, `MasculineGenitiveSingular`, `MasculineGenitivePlural`, `MasculineLocativeSingular`, `MasculineLocativePlural`, `MasculineVocativeSingular`, `MasculineVocativePlural`, `FeminineNominativeSingular`, `FeminineNominativePlural`, `FeminineAccusativeSingular`, `FeminineAccusativePlural`, `FeminineInstrumentalSingular`, `FeminineInstrumentalPlural`, `FeminineDativeSingular`, `FeminineDativePlural`, `FeminineAblativeSingular`, `FeminineAblativePlural`, `FeminineGenitiveSingular`, `FeminineGenitivePlural`, `FeminineLocativeSingular`, `FeminineLocativePlural`, `FeminineVocativeSingular`, `FeminineVocativePlural`, `NeuterNominativeSingular`, `NeuterNominativePlural`, `NeuterAccusativeSingular`, `NeuterAccusativePlural`, `NeuterInstrumentalSingular`, `NeuterInstrumentalPlural`, `NeuterDativeSingular`, `NeuterDativePlural`, `NeuterAblativeSingular`, `NeuterAblativePlural`, `NeuterGenitiveSingular`, `NeuterGenitivePlural`, `NeuterLocativeSingular`, `NeuterLocativePlural`, `NeuterVocativeSingular`, `NeuterVocativePlural`

**num** cells: `MasculineNominativeSingular`, `MasculineNominativePlural`, `MasculineAccusativeSingular`, `MasculineAccusativePlural`, `MasculineInstrumentalSingular`, `MasculineInstrumentalPlural`, `MasculineDativeSingular`, `MasculineDativePlural`, `MasculineAblativeSingular`, `MasculineAblativePlural`, `MasculineGenitiveSingular`, `MasculineGenitivePlural`, `MasculineLocativeSingular`, `MasculineLocativePlural`, `MasculineVocativeSingular`, `MasculineVocativePlural`, `FeminineNominativeSingular`, `FeminineNominativePlural`, `FeminineAccusativeSingular`, `FeminineAccusativePlural`, `FeminineInstrumentalSingular`, `FeminineInstrumentalPlural`, `FeminineDativeSingular`, `FeminineDativePlural`, `FeminineAblativeSingular`, `FeminineAblativePlural`, `FeminineGenitiveSingular`, `FeminineGenitivePlural`, `FeminineLocativeSingular`, `FeminineLocativePlural`, `FeminineVocativeSingular`, `FeminineVocativePlural`, `NeuterNominativeSingular`, `NeuterNominativePlural`, `NeuterAccusativeSingular`, `NeuterAccusativePlural`, `NeuterInstrumentalSingular`, `NeuterInstrumentalPlural`, `NeuterDativeSingular`, `NeuterDativePlural`, `NeuterAblativeSingular`, `NeuterAblativePlural`, `NeuterGenitiveSingular`, `NeuterGenitivePlural`, `NeuterLocativeSingular`, `NeuterLocativePlural`, `NeuterVocativeSingular`, `NeuterVocativePlural`

## How it works

Each attested form is stored as an edit tree from the lemma. A lemma's edits across all cells make its inflection class. A suffix tree over the word, read from its end and pruned by a description-length test, predicts the class of any word. Known words the tree gets wrong are kept in a small lexicon. Confidences are smoothed counts; variant weights follow Wiktionary's listing order (1, 1/2, 1/3…, nonstandard forms a tenth of that).

## License

Code: MIT OR Apache-2.0. Data: derived from [Wiktionary](https://www.wiktionary.org/) (via [kaikki.org](https://kaikki.org/)), CC BY-SA 4.0; the model bytes in `src/model.bin` are shared under the same license.
