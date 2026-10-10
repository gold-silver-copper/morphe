# galician

Galician inflection for Rust, learned from Wiktionary by [morphe](https://github.com/gold-silver-copper/morphe).
Inflects any word, known or made up. The `_scored` functions give every candidate with a confidence.

Generated from the Kaikki Galician dump of 2026-10-03. Do not edit by hand: change the config in morphe and regenerate.

## Example

```rust
use galician::*;
assert_eq!(noun("can", Number::Plural), "cans");
assert_eq!(noun("animal", Number::Plural), "animais");
assert_eq!(adj("frío", Gender::Feminine, Number::Plural), "frías");
assert_eq!(verb_indicative("abater", Tense::Preterite, Person::First, Number::Singular), "abatín");
assert_eq!(verb_indicative("abater", Tense::Present, Person::Third, Number::Plural), "abaten");
assert_eq!(verb_personal_infinitive("falar", Person::First, Number::Plural), "falarmos");
println!("{:?}", noun_gender("mel"));
// Made-up words inflect by analogy:
println!("{}", verb_indicative("brugar", Tense::Preterite, Person::First, Number::Singular));
```

Forms follow the official (RAG) norm; the reintegrationist conjugations
Wiktionary also gives are left out. Negative imperatives (`non fales`) are
the present subjunctive.

## Coverage

| Part of speech | Entries | Cells | Edits | Classes | Tree nodes | Lexicon |
|---|---|---|---|---|---|---|
| noun | 10895 | 5 | 231 | 158 | 163 | 1035 |
| adj | 2932 | 4 | 65 | 45 | 28 | 73 |
| verb | 2573 | 73 | 1548 | 164 | 99 | 223 |
| pron | 40 | 4 | 38 | 28 | 10 | 21 |
| det | 36 | 4 | 45 | 26 | 14 | 15 |
| article | 10 | 4 | 22 | 10 | 2 | 9 |
| num | 16 | 4 | 11 | 10 | 4 | 7 |

Model size: 532511 bytes.

**noun** cells: `Plural`, `CounterpartMasculineSingular`, `CounterpartMasculinePlural`, `CounterpartFeminineSingular`, `CounterpartFemininePlural`

Citation form: `Lemma, Plural` (comma-separated; parts you give are output exactly and narrow the prediction).

**adj** cells: `MasculineSingular`, `MasculinePlural`, `FeminineSingular`, `FemininePlural`

**verb** cells: `Infinitive`, `PersonalInfinitiveFirstSingular`, `PersonalInfinitiveFirstPlural`, `PersonalInfinitiveSecondSingular`, `PersonalInfinitiveSecondPlural`, `PersonalInfinitiveThirdSingular`, `PersonalInfinitiveThirdPlural`, `Gerund`, `ParticipleMasculineSingular`, `ParticipleMasculinePlural`, `ParticipleFeminineSingular`, `ParticipleFemininePlural`, `ShortParticiple`, `IndicativePresentFirstSingular`, `IndicativePresentFirstPlural`, `IndicativePresentSecondSingular`, `IndicativePresentSecondPlural`, `IndicativePresentThirdSingular`, `IndicativePresentThirdPlural`, `IndicativeImperfectFirstSingular`, `IndicativeImperfectFirstPlural`, `IndicativeImperfectSecondSingular`, `IndicativeImperfectSecondPlural`, `IndicativeImperfectThirdSingular`, `IndicativeImperfectThirdPlural`, `IndicativePreteriteFirstSingular`, `IndicativePreteriteFirstPlural`, `IndicativePreteriteSecondSingular`, `IndicativePreteriteSecondPlural`, `IndicativePreteriteThirdSingular`, `IndicativePreteriteThirdPlural`, `IndicativePluperfectFirstSingular`, `IndicativePluperfectFirstPlural`, `IndicativePluperfectSecondSingular`, `IndicativePluperfectSecondPlural`, `IndicativePluperfectThirdSingular`, `IndicativePluperfectThirdPlural`, `IndicativeFutureFirstSingular`, `IndicativeFutureFirstPlural`, `IndicativeFutureSecondSingular`, `IndicativeFutureSecondPlural`, `IndicativeFutureThirdSingular`, `IndicativeFutureThirdPlural`, `ConditionalFirstSingular`, `ConditionalFirstPlural`, `ConditionalSecondSingular`, `ConditionalSecondPlural`, `ConditionalThirdSingular`, `ConditionalThirdPlural`, `SubjunctivePresentFirstSingular`, `SubjunctivePresentFirstPlural`, `SubjunctivePresentSecondSingular`, `SubjunctivePresentSecondPlural`, `SubjunctivePresentThirdSingular`, `SubjunctivePresentThirdPlural`, `SubjunctiveImperfectFirstSingular`, `SubjunctiveImperfectFirstPlural`, `SubjunctiveImperfectSecondSingular`, `SubjunctiveImperfectSecondPlural`, `SubjunctiveImperfectThirdSingular`, `SubjunctiveImperfectThirdPlural`, `SubjunctiveFutureFirstSingular`, `SubjunctiveFutureFirstPlural`, `SubjunctiveFutureSecondSingular`, `SubjunctiveFutureSecondPlural`, `SubjunctiveFutureThirdSingular`, `SubjunctiveFutureThirdPlural`, `ImperativeFirstSingular`, `ImperativeFirstPlural`, `ImperativeSecondSingular`, `ImperativeSecondPlural`, `ImperativeThirdSingular`, `ImperativeThirdPlural`

Citation form: `Infinitive, IndicativePresentFirstSingular, IndicativePreteriteFirstSingular, ParticipleMasculineSingular` (comma-separated; parts you give are output exactly and narrow the prediction).

**pron** cells: `MasculineSingular`, `MasculinePlural`, `FeminineSingular`, `FemininePlural`

**det** cells: `MasculineSingular`, `MasculinePlural`, `FeminineSingular`, `FemininePlural`

**article** cells: `MasculineSingular`, `MasculinePlural`, `FeminineSingular`, `FemininePlural`

**num** cells: `MasculineSingular`, `MasculinePlural`, `FeminineSingular`, `FemininePlural`

## How it works

Each attested form is stored as an edit tree from the lemma. A lemma's edits across all cells make its inflection class. A suffix tree over the word, read from its end and pruned by a description-length test, predicts the class of any word. Known words the tree gets wrong are kept in a small lexicon. Confidences are smoothed counts; variant weights follow Wiktionary's listing order (1, 1/2, 1/3…, nonstandard forms a tenth of that).

## License

Code: MIT OR Apache-2.0. Data: derived from [Wiktionary](https://www.wiktionary.org/) (via [kaikki.org](https://kaikki.org/)), CC BY-SA 4.0; the model bytes in `src/model.bin` are shared under the same license.
