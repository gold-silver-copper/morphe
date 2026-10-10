# portuguese

Portuguese inflection for Rust, learned from Wiktionary by [morphe](https://github.com/gold-silver-copper/morphe).
Inflects any word, known or made up. The `_scored` functions give every candidate with a confidence.

Generated from the Kaikki Portuguese dump of 2026-10-03. Do not edit by hand: change the config in morphe and regenerate.

## Example

```rust
use portuguese::*;
assert_eq!(noun("cidade", Number::Plural), "cidades");
assert_eq!(noun("pão", Number::Plural), "pães");
assert_eq!(adj("feliz", Gender::Masculine, Number::Plural), "felizes");
assert_eq!(verb_indicative("ter", Tense::Present, Person::First, Number::Singular), "tenho");
assert_eq!(verb_subjunctive("fazer", Tense::Future, Person::Third, Number::Plural), "fizerem");
assert_eq!(verb_personal_infinitive("falar", Person::First, Number::Plural), "falarmos");
println!("{:?}", noun_gender("mão"));
// Made-up words inflect by analogy:
println!("{}", verb_indicative("brugar", Tense::Preterite, Person::First, Number::Singular));
```

Spelling is matched as written, accents included. Both Brazilian and
European spellings of a form are kept where Wiktionary lists both
(`amamos`, `amámos`). Negative imperatives (`não fales`) are the
present subjunctive.

## Coverage

| Part of speech | Entries | Cells | Edits | Classes | Tree nodes | Lexicon |
|---|---|---|---|---|---|---|
| noun | 40837 | 5 | 1146 | 1111 | 1429 | 7668 |
| adj | 13400 | 5 | 680 | 658 | 267 | 1015 |
| verb | 7104 | 73 | 2892 | 458 | 167 | 1128 |
| pron | 70 | 4 | 46 | 42 | 7 | 41 |
| det | 38 | 4 | 28 | 16 | 4 | 13 |
| article | 3 | 4 | 7 | 4 | 2 | 1 |
| num | 13 | 4 | 7 | 6 | 2 | 8 |

Model size: 1442194 bytes.

**noun** cells: `Plural`, `CounterpartMasculineSingular`, `CounterpartMasculinePlural`, `CounterpartFeminineSingular`, `CounterpartFemininePlural`

Citation form: `Lemma, Plural` (comma-separated; parts you give are output exactly and narrow the prediction).

**adj** cells: `MasculineSingular`, `MasculinePlural`, `FeminineSingular`, `FemininePlural`, `Superlative`

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
