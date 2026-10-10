# spanish

Spanish inflection for Rust, learned from Wiktionary by [morphe](https://github.com/gold-silver-copper/morphe).
Inflects any word, known or made up. The `_scored` functions give every candidate with a confidence.

Generated from the Kaikki Spanish dump of 2026-10-03. Do not edit by hand: change the config in morphe and regenerate.

## Example

```rust
use spanish::*;
assert_eq!(noun("ciudad", Number::Plural), "ciudades");
assert_eq!(noun_counterpart("profesor", Gender::Feminine, Number::Singular), "profesora");
assert_eq!(adj("rojo", Gender::Feminine, Number::Plural), "rojas");
assert_eq!(verb_indicative("tener", Tense::Present, Person::First, Number::Singular), "tengo");
assert_eq!(verb_subjunctive("hablar", Tense::ImperfectSe, Person::Third, Number::Plural), "hablasen");
assert_eq!(verb_vos("tener", Mood::Indicative), "tenés");
println!("{:?}", noun_gender("mano"));
// Made-up words inflect by analogy:
println!("{}", verb_indicative("flurgar", Tense::Preterite, Person::First, Number::Singular));
```

Spelling is matched as written, accents included (`papa` and `papá` are
different words). Clitic-attached forms (`dámelo`) are not modelled.

## Coverage

| Part of speech | Entries | Cells | Edits | Classes | Tree nodes | Lexicon |
|---|---|---|---|---|---|---|
| noun | 69795 | 5 | 768 | 748 | 878 | 12730 |
| adj | 24493 | 5 | 200 | 165 | 136 | 558 |
| verb | 8930 | 75 | 3724 | 442 | 368 | 608 |
| pron | 45 | 6 | 34 | 22 | 8 | 23 |
| det | 36 | 6 | 13 | 8 | 5 | 15 |
| article | 6 | 6 | 11 | 6 | 2 | 4 |
| num | 28 | 2 | 4 | 3 | 4 | 7 |

Model size: 2116666 bytes.

**noun** cells: `Plural`, `CounterpartMasculineSingular`, `CounterpartMasculinePlural`, `CounterpartFeminineSingular`, `CounterpartFemininePlural`

Citation form: `Lemma, Plural` (comma-separated; parts you give are output exactly and narrow the prediction).

**adj** cells: `MasculineSingular`, `MasculinePlural`, `FeminineSingular`, `FemininePlural`, `Superlative`

**verb** cells: `Infinitive`, `Gerund`, `ParticipleMasculineSingular`, `ParticipleMasculinePlural`, `ParticipleFeminineSingular`, `ParticipleFemininePlural`, `IndicativePresentFirstSingular`, `IndicativePresentFirstPlural`, `IndicativePresentSecondSingular`, `IndicativePresentSecondPlural`, `IndicativePresentThirdSingular`, `IndicativePresentThirdPlural`, `IndicativeImperfectFirstSingular`, `IndicativeImperfectFirstPlural`, `IndicativeImperfectSecondSingular`, `IndicativeImperfectSecondPlural`, `IndicativeImperfectThirdSingular`, `IndicativeImperfectThirdPlural`, `IndicativePreteriteFirstSingular`, `IndicativePreteriteFirstPlural`, `IndicativePreteriteSecondSingular`, `IndicativePreteriteSecondPlural`, `IndicativePreteriteThirdSingular`, `IndicativePreteriteThirdPlural`, `IndicativeFutureFirstSingular`, `IndicativeFutureFirstPlural`, `IndicativeFutureSecondSingular`, `IndicativeFutureSecondPlural`, `IndicativeFutureThirdSingular`, `IndicativeFutureThirdPlural`, `IndicativeConditionalFirstSingular`, `IndicativeConditionalFirstPlural`, `IndicativeConditionalSecondSingular`, `IndicativeConditionalSecondPlural`, `IndicativeConditionalThirdSingular`, `IndicativeConditionalThirdPlural`, `SubjunctivePresentFirstSingular`, `SubjunctivePresentFirstPlural`, `SubjunctivePresentSecondSingular`, `SubjunctivePresentSecondPlural`, `SubjunctivePresentThirdSingular`, `SubjunctivePresentThirdPlural`, `SubjunctiveImperfectFirstSingular`, `SubjunctiveImperfectFirstPlural`, `SubjunctiveImperfectSecondSingular`, `SubjunctiveImperfectSecondPlural`, `SubjunctiveImperfectThirdSingular`, `SubjunctiveImperfectThirdPlural`, `SubjunctiveImperfectSeFirstSingular`, `SubjunctiveImperfectSeFirstPlural`, `SubjunctiveImperfectSeSecondSingular`, `SubjunctiveImperfectSeSecondPlural`, `SubjunctiveImperfectSeThirdSingular`, `SubjunctiveImperfectSeThirdPlural`, `SubjunctiveFutureFirstSingular`, `SubjunctiveFutureFirstPlural`, `SubjunctiveFutureSecondSingular`, `SubjunctiveFutureSecondPlural`, `SubjunctiveFutureThirdSingular`, `SubjunctiveFutureThirdPlural`, `ImperativeFirstSingular`, `ImperativeFirstPlural`, `ImperativeSecondSingular`, `ImperativeSecondPlural`, `ImperativeThirdSingular`, `ImperativeThirdPlural`, `ImperativeNegativeFirstSingular`, `ImperativeNegativeFirstPlural`, `ImperativeNegativeSecondSingular`, `ImperativeNegativeSecondPlural`, `ImperativeNegativeThirdSingular`, `ImperativeNegativeThirdPlural`, `VosIndicative`, `VosSubjunctive`, `VosImperative`

Citation form: `Infinitive, IndicativePresentFirstSingular, IndicativePreteriteFirstSingular, ParticipleMasculineSingular` (comma-separated; parts you give are output exactly and narrow the prediction).

**pron** cells: `MasculineSingular`, `MasculinePlural`, `FeminineSingular`, `FemininePlural`, `NeuterSingular`, `NeuterPlural`

**det** cells: `MasculineSingular`, `MasculinePlural`, `FeminineSingular`, `FemininePlural`, `NeuterSingular`, `NeuterPlural`

**article** cells: `MasculineSingular`, `MasculinePlural`, `FeminineSingular`, `FemininePlural`, `NeuterSingular`, `NeuterPlural`

**num** cells: `Masculine`, `Feminine`

## How it works

Each attested form is stored as an edit tree from the lemma. A lemma's edits across all cells make its inflection class. A suffix tree over the word, read from its end and pruned by a description-length test, predicts the class of any word. Known words the tree gets wrong are kept in a small lexicon. Confidences are smoothed counts; variant weights follow Wiktionary's listing order (1, 1/2, 1/3…, nonstandard forms a tenth of that).

## License

Code: MIT OR Apache-2.0. Data: derived from [Wiktionary](https://www.wiktionary.org/) (via [kaikki.org](https://kaikki.org/)), CC BY-SA 4.0; the model bytes in `src/model.bin` are shared under the same license.
