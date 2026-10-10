# aramaic

Aramaic inflection for Rust, learned from Wiktionary by [morphe](https://github.com/gold-silver-copper/morphe).
Inflects any word, known or made up. The `_scored` functions give every candidate with a confidence.

Generated from the Kaikki Aramaic dump of 2026-10-03. Do not edit by hand: change the config in morphe and regenerate.

## Example

```rust
use aramaic::*;
println!("{}", noun("ברא", State::Absolute, Number::Plural)); // בנין
println!("{}", noun("ברא", State::Construct, Number::Plural)); // בני
println!("{:?}", noun_gender("מלכא"));
println!("{}", verb("כתב", Tense::Past, Person::FirstSingular)); // כתבת
println!("{}", verb("כתב", Tense::Future, Person::ThirdMasculinePlural)); // יכתבון
println!("{}", verb_imperative("כתב", Imperative::MasculineSingular)); // כתב
```

Nouns have the absolute, construct and emphatic states in both numbers,
and their gender; adjectives agree in gender too. Verbs have the past and
future for ten persons, the imperative and the active participle. Input
matching ignores vowel points, and forms are given unpointed, as Aramaic is
usually written. Words are in Hebrew square script.

## Coverage

| Part of speech | Entries | Cells | Edits | Classes | Tree nodes | Lexicon |
|---|---|---|---|---|---|---|
| noun | 1066 | 6 | 94 | 115 | 70 | 504 |
| adj | 236 | 12 | 25 | 25 | 15 | 78 |
| verb | 8 | 28 | 87 | 9 | 2 | 4 |

Model size: 15873 bytes.

**noun** cells: `AbsoluteSingular`, `AbsolutePlural`, `ConstructSingular`, `ConstructPlural`, `EmphaticSingular`, `EmphaticPlural`

Citation form: `EmphaticSingular, EmphaticPlural` (comma-separated; parts you give are output exactly and narrow the prediction).

**adj** cells: `AbsoluteMasculineSingular`, `AbsoluteMasculinePlural`, `AbsoluteFeminineSingular`, `AbsoluteFemininePlural`, `ConstructMasculineSingular`, `ConstructMasculinePlural`, `ConstructFeminineSingular`, `ConstructFemininePlural`, `EmphaticMasculineSingular`, `EmphaticMasculinePlural`, `EmphaticFeminineSingular`, `EmphaticFemininePlural`

Citation form: `AbsoluteMasculineSingular, AbsoluteFeminineSingular, AbsoluteMasculinePlural` (comma-separated; parts you give are output exactly and narrow the prediction).

**verb** cells: `PastFirstSingular`, `PastSecondMasculineSingular`, `PastSecondFeminineSingular`, `PastThirdMasculineSingular`, `PastThirdFeminineSingular`, `PastFirstPlural`, `PastSecondMasculinePlural`, `PastSecondFemininePlural`, `PastThirdMasculinePlural`, `PastThirdFemininePlural`, `FutureFirstSingular`, `FutureSecondMasculineSingular`, `FutureSecondFeminineSingular`, `FutureThirdMasculineSingular`, `FutureThirdFeminineSingular`, `FutureFirstPlural`, `FutureSecondMasculinePlural`, `FutureSecondFemininePlural`, `FutureThirdMasculinePlural`, `FutureThirdFemininePlural`, `ImperativeMasculineSingular`, `ImperativeFeminineSingular`, `ImperativeMasculinePlural`, `ImperativeFemininePlural`, `ParticipleMasculineSingular`, `ParticipleFeminineSingular`, `ParticipleMasculinePlural`, `ParticipleFemininePlural`

Citation form: `PastThirdMasculineSingular, FutureThirdMasculineSingular` (comma-separated; parts you give are output exactly and narrow the prediction).

## How it works

Each attested form is stored as an edit tree from the lemma. A lemma's edits across all cells make its inflection class. A suffix tree over the word, read from its end and pruned by a description-length test, predicts the class of any word. Known words the tree gets wrong are kept in a small lexicon. Confidences are smoothed counts; variant weights follow Wiktionary's listing order (1, 1/2, 1/3…, nonstandard forms a tenth of that).

## License

Code: MIT OR Apache-2.0. Data: derived from [Wiktionary](https://www.wiktionary.org/) (via [kaikki.org](https://kaikki.org/)), CC BY-SA 4.0; the model bytes in `src/model.bin` are shared under the same license.
