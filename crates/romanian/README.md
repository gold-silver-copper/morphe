# romanian

Romanian inflection for Rust, learned from Wiktionary by [morphe](https://github.com/gold-silver-copper/morphe).
Inflects any word, known or made up. The `_scored` functions give every candidate with a confidence.

Generated from the Kaikki Romanian dump of 2026-10-03. Do not edit by hand: change the config in morphe and regenerate.

## Example

```rust
use romanian::*;
assert_eq!(noun("casă", Case::Nominative, Definiteness::Indefinite, Number::Plural), "case");
assert_eq!(noun("om", Case::Genitive, Definiteness::Definite, Number::Singular), "omului");
assert_eq!(noun_vocative("prieten", Number::Singular), "prietene");
assert_eq!(adj("frumos", Case::Nominative, Definiteness::Indefinite, Gender::Feminine, Number::Singular), "frumoasă");
assert_eq!(verb_indicative("face", Tense::Present, Person::First, Number::Singular), "fac");
assert_eq!(verb_subjunctive("face", Person::Third, Number::Singular), "facă");
println!("{:?}", noun_gender("tren"));
// Made-up words inflect by analogy:
println!("{}", noun("flătură", Case::Dative, Definiteness::Definite, Number::Plural));
```

Forms are given without their particles: the infinitive `face` (not
`a face`), the subjunctive `facă` (not `să facă`). Input written with
cedilla letters (ş ţ) is read as the comma letters (ș ț); output uses the
comma letters. Stress marks Wiktionary puts on a few forms are removed.

## Coverage

| Part of speech | Entries | Cells | Edits | Classes | Tree nodes | Lexicon |
|---|---|---|---|---|---|---|
| noun | 58295 | 20 | 1673 | 1248 | 2434 | 17760 |
| adj | 17776 | 48 | 272 | 119 | 165 | 381 |
| verb | 7344 | 37 | 1267 | 446 | 329 | 1452 |
| pron | 23 | 24 | 42 | 20 | 9 | 15 |
| det | 34 | 48 | 72 | 29 | 4 | 24 |
| article | 1 | 48 | 6 | 1 | 2 | 0 |
| num | 8 | 6 | 12 | 8 | 2 | 2 |

Model size: 1167091 bytes.

**noun** cells: `NominativeIndefiniteSingular`, `NominativeIndefinitePlural`, `NominativeDefiniteSingular`, `NominativeDefinitePlural`, `AccusativeIndefiniteSingular`, `AccusativeIndefinitePlural`, `AccusativeDefiniteSingular`, `AccusativeDefinitePlural`, `GenitiveIndefiniteSingular`, `GenitiveIndefinitePlural`, `GenitiveDefiniteSingular`, `GenitiveDefinitePlural`, `DativeIndefiniteSingular`, `DativeIndefinitePlural`, `DativeDefiniteSingular`, `DativeDefinitePlural`, `VocativeSingular`, `VocativePlural`, `CounterpartMasculine`, `CounterpartFeminine`

Citation form: `NominativeIndefiniteSingular, NominativeIndefinitePlural` (comma-separated; parts you give are output exactly and narrow the prediction).

**adj** cells: `NominativeIndefiniteMasculineSingular`, `NominativeIndefiniteMasculinePlural`, `NominativeIndefiniteFeminineSingular`, `NominativeIndefiniteFemininePlural`, `NominativeIndefiniteNeuterSingular`, `NominativeIndefiniteNeuterPlural`, `NominativeDefiniteMasculineSingular`, `NominativeDefiniteMasculinePlural`, `NominativeDefiniteFeminineSingular`, `NominativeDefiniteFemininePlural`, `NominativeDefiniteNeuterSingular`, `NominativeDefiniteNeuterPlural`, `AccusativeIndefiniteMasculineSingular`, `AccusativeIndefiniteMasculinePlural`, `AccusativeIndefiniteFeminineSingular`, `AccusativeIndefiniteFemininePlural`, `AccusativeIndefiniteNeuterSingular`, `AccusativeIndefiniteNeuterPlural`, `AccusativeDefiniteMasculineSingular`, `AccusativeDefiniteMasculinePlural`, `AccusativeDefiniteFeminineSingular`, `AccusativeDefiniteFemininePlural`, `AccusativeDefiniteNeuterSingular`, `AccusativeDefiniteNeuterPlural`, `GenitiveIndefiniteMasculineSingular`, `GenitiveIndefiniteMasculinePlural`, `GenitiveIndefiniteFeminineSingular`, `GenitiveIndefiniteFemininePlural`, `GenitiveIndefiniteNeuterSingular`, `GenitiveIndefiniteNeuterPlural`, `GenitiveDefiniteMasculineSingular`, `GenitiveDefiniteMasculinePlural`, `GenitiveDefiniteFeminineSingular`, `GenitiveDefiniteFemininePlural`, `GenitiveDefiniteNeuterSingular`, `GenitiveDefiniteNeuterPlural`, `DativeIndefiniteMasculineSingular`, `DativeIndefiniteMasculinePlural`, `DativeIndefiniteFeminineSingular`, `DativeIndefiniteFemininePlural`, `DativeIndefiniteNeuterSingular`, `DativeIndefiniteNeuterPlural`, `DativeDefiniteMasculineSingular`, `DativeDefiniteMasculinePlural`, `DativeDefiniteFeminineSingular`, `DativeDefiniteFemininePlural`, `DativeDefiniteNeuterSingular`, `DativeDefiniteNeuterPlural`

Citation form: `NominativeIndefiniteMasculineSingular, NominativeIndefiniteFeminineSingular` (comma-separated; parts you give are output exactly and narrow the prediction).

**verb** cells: `Infinitive`, `Gerund`, `Participle`, `IndicativePresentFirstSingular`, `IndicativePresentFirstPlural`, `IndicativePresentSecondSingular`, `IndicativePresentSecondPlural`, `IndicativePresentThirdSingular`, `IndicativePresentThirdPlural`, `IndicativeImperfectFirstSingular`, `IndicativeImperfectFirstPlural`, `IndicativeImperfectSecondSingular`, `IndicativeImperfectSecondPlural`, `IndicativeImperfectThirdSingular`, `IndicativeImperfectThirdPlural`, `IndicativeSimplePerfectFirstSingular`, `IndicativeSimplePerfectFirstPlural`, `IndicativeSimplePerfectSecondSingular`, `IndicativeSimplePerfectSecondPlural`, `IndicativeSimplePerfectThirdSingular`, `IndicativeSimplePerfectThirdPlural`, `IndicativePluperfectFirstSingular`, `IndicativePluperfectFirstPlural`, `IndicativePluperfectSecondSingular`, `IndicativePluperfectSecondPlural`, `IndicativePluperfectThirdSingular`, `IndicativePluperfectThirdPlural`, `SubjunctiveFirstSingular`, `SubjunctiveFirstPlural`, `SubjunctiveSecondSingular`, `SubjunctiveSecondPlural`, `SubjunctiveThirdSingular`, `SubjunctiveThirdPlural`, `ImperativeSingular`, `ImperativePlural`, `ImperativeNegativeSingular`, `ImperativeNegativePlural`

Citation form: `Infinitive, IndicativePresentThirdSingular, Participle` (comma-separated; parts you give are output exactly and narrow the prediction).

**pron** cells: `NominativeMasculineSingular`, `NominativeMasculinePlural`, `NominativeFeminineSingular`, `NominativeFemininePlural`, `NominativeNeuterSingular`, `NominativeNeuterPlural`, `AccusativeMasculineSingular`, `AccusativeMasculinePlural`, `AccusativeFeminineSingular`, `AccusativeFemininePlural`, `AccusativeNeuterSingular`, `AccusativeNeuterPlural`, `GenitiveMasculineSingular`, `GenitiveMasculinePlural`, `GenitiveFeminineSingular`, `GenitiveFemininePlural`, `GenitiveNeuterSingular`, `GenitiveNeuterPlural`, `DativeMasculineSingular`, `DativeMasculinePlural`, `DativeFeminineSingular`, `DativeFemininePlural`, `DativeNeuterSingular`, `DativeNeuterPlural`

**det** cells: `NominativeIndefiniteMasculineSingular`, `NominativeIndefiniteMasculinePlural`, `NominativeIndefiniteFeminineSingular`, `NominativeIndefiniteFemininePlural`, `NominativeIndefiniteNeuterSingular`, `NominativeIndefiniteNeuterPlural`, `NominativeDefiniteMasculineSingular`, `NominativeDefiniteMasculinePlural`, `NominativeDefiniteFeminineSingular`, `NominativeDefiniteFemininePlural`, `NominativeDefiniteNeuterSingular`, `NominativeDefiniteNeuterPlural`, `AccusativeIndefiniteMasculineSingular`, `AccusativeIndefiniteMasculinePlural`, `AccusativeIndefiniteFeminineSingular`, `AccusativeIndefiniteFemininePlural`, `AccusativeIndefiniteNeuterSingular`, `AccusativeIndefiniteNeuterPlural`, `AccusativeDefiniteMasculineSingular`, `AccusativeDefiniteMasculinePlural`, `AccusativeDefiniteFeminineSingular`, `AccusativeDefiniteFemininePlural`, `AccusativeDefiniteNeuterSingular`, `AccusativeDefiniteNeuterPlural`, `GenitiveIndefiniteMasculineSingular`, `GenitiveIndefiniteMasculinePlural`, `GenitiveIndefiniteFeminineSingular`, `GenitiveIndefiniteFemininePlural`, `GenitiveIndefiniteNeuterSingular`, `GenitiveIndefiniteNeuterPlural`, `GenitiveDefiniteMasculineSingular`, `GenitiveDefiniteMasculinePlural`, `GenitiveDefiniteFeminineSingular`, `GenitiveDefiniteFemininePlural`, `GenitiveDefiniteNeuterSingular`, `GenitiveDefiniteNeuterPlural`, `DativeIndefiniteMasculineSingular`, `DativeIndefiniteMasculinePlural`, `DativeIndefiniteFeminineSingular`, `DativeIndefiniteFemininePlural`, `DativeIndefiniteNeuterSingular`, `DativeIndefiniteNeuterPlural`, `DativeDefiniteMasculineSingular`, `DativeDefiniteMasculinePlural`, `DativeDefiniteFeminineSingular`, `DativeDefiniteFemininePlural`, `DativeDefiniteNeuterSingular`, `DativeDefiniteNeuterPlural`

**article** cells: `NominativeIndefiniteMasculineSingular`, `NominativeIndefiniteMasculinePlural`, `NominativeIndefiniteFeminineSingular`, `NominativeIndefiniteFemininePlural`, `NominativeIndefiniteNeuterSingular`, `NominativeIndefiniteNeuterPlural`, `NominativeDefiniteMasculineSingular`, `NominativeDefiniteMasculinePlural`, `NominativeDefiniteFeminineSingular`, `NominativeDefiniteFemininePlural`, `NominativeDefiniteNeuterSingular`, `NominativeDefiniteNeuterPlural`, `AccusativeIndefiniteMasculineSingular`, `AccusativeIndefiniteMasculinePlural`, `AccusativeIndefiniteFeminineSingular`, `AccusativeIndefiniteFemininePlural`, `AccusativeIndefiniteNeuterSingular`, `AccusativeIndefiniteNeuterPlural`, `AccusativeDefiniteMasculineSingular`, `AccusativeDefiniteMasculinePlural`, `AccusativeDefiniteFeminineSingular`, `AccusativeDefiniteFemininePlural`, `AccusativeDefiniteNeuterSingular`, `AccusativeDefiniteNeuterPlural`, `GenitiveIndefiniteMasculineSingular`, `GenitiveIndefiniteMasculinePlural`, `GenitiveIndefiniteFeminineSingular`, `GenitiveIndefiniteFemininePlural`, `GenitiveIndefiniteNeuterSingular`, `GenitiveIndefiniteNeuterPlural`, `GenitiveDefiniteMasculineSingular`, `GenitiveDefiniteMasculinePlural`, `GenitiveDefiniteFeminineSingular`, `GenitiveDefiniteFemininePlural`, `GenitiveDefiniteNeuterSingular`, `GenitiveDefiniteNeuterPlural`, `DativeIndefiniteMasculineSingular`, `DativeIndefiniteMasculinePlural`, `DativeIndefiniteFeminineSingular`, `DativeIndefiniteFemininePlural`, `DativeIndefiniteNeuterSingular`, `DativeIndefiniteNeuterPlural`, `DativeDefiniteMasculineSingular`, `DativeDefiniteMasculinePlural`, `DativeDefiniteFeminineSingular`, `DativeDefiniteFemininePlural`, `DativeDefiniteNeuterSingular`, `DativeDefiniteNeuterPlural`

**num** cells: `MasculineSingular`, `MasculinePlural`, `FeminineSingular`, `FemininePlural`, `NeuterSingular`, `NeuterPlural`

## How it works

Each attested form is stored as an edit tree from the lemma. A lemma's edits across all cells make its inflection class. A suffix tree over the word, read from its end and pruned by a description-length test, predicts the class of any word. Known words the tree gets wrong are kept in a small lexicon. Confidences are smoothed counts; variant weights follow Wiktionary's listing order (1, 1/2, 1/3…, nonstandard forms a tenth of that).

## License

Code: MIT OR Apache-2.0. Data: derived from [Wiktionary](https://www.wiktionary.org/) (via [kaikki.org](https://kaikki.org/)), CC BY-SA 4.0; the model bytes in `src/model.bin` are shared under the same license.
