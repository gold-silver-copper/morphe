# dutch

Dutch inflection for Rust, learned from Wiktionary by [morphe](https://github.com/gold-silver-copper/morphe).
Inflects any word, known or made up. The `_scored` functions give every candidate with a confidence.

Generated from the Kaikki Dutch dump of 2026-10-03. Do not edit by hand: change the config in morphe and regenerate.

## Example

```rust
use dutch::*;
assert_eq!(noun("huis", Number::Plural), "huizen");
assert_eq!(noun_diminutive("huis"), "huisje");
println!("{:?}", noun_gender("huis"));
assert_eq!(verb_past("lopen", PersonNumber::ThirdSingular), "liep");
// Separable verbs split in main clauses and join in subordinate ones:
assert_eq!(verb_present("aanvallen", PersonNumber::ThirdSingular), "valt aan");
assert_eq!(verb_subordinate_present("aanvallen", PersonNumber::ThirdSingular), "aanvalt");
assert_eq!(adj("groot", Degree::Positive, Form::Definite), "grote");
assert_eq!(adj("goed", Degree::Comparative, Form::Predicative), "beter");
// Citations pin principal parts: infinitive, past, participle.
println!("{}", verb_past("vragen, vroeg, gevraagd", PersonNumber::Plural));
```

The superlative predicative is given without "het" ("snelst"). Subjunctive,
plural imperative and "gij" forms are left out as archaic or regional.

## Coverage

| Part of speech | Entries | Cells | Edits | Classes | Tree nodes | Lexicon |
|---|---|---|---|---|---|---|
| noun | 143919 | 2 | 1807 | 3335 | 5552 | 47770 |
| adj | 18219 | 18 | 327 | 441 | 1172 | 8097 |
| verb | 13622 | 24 | 4141 | 4265 | 1525 | 8658 |
| det | 23 | 6 | 6 | 7 | 4 | 13 |
| num | 14 | 6 | 3 | 1 | 2 | 0 |

Model size: 1641168 bytes.

**noun** cells: `Plural`, `Diminutive`

Citation form: `Lemma, Plural` (comma-separated; parts you give are output exactly and narrow the prediction).

**adj** cells: `PositivePredicative`, `PositiveCommon`, `PositiveNeuter`, `PositivePlural`, `PositiveDefinite`, `PositivePartitive`, `ComparativePredicative`, `ComparativeCommon`, `ComparativeNeuter`, `ComparativePlural`, `ComparativeDefinite`, `ComparativePartitive`, `SuperlativePredicative`, `SuperlativeCommon`, `SuperlativeNeuter`, `SuperlativePlural`, `SuperlativeDefinite`, `SuperlativePartitive`

Citation form: `Lemma, ComparativePredicative, SuperlativePredicative` (comma-separated; parts you give are output exactly and narrow the prediction).

**verb** cells: `Infinitive`, `ParticiplePresent`, `ParticiplePast`, `Imperative`, `PresentFirstSingular`, `PresentSecondSingular`, `PresentFormal`, `PresentThirdSingular`, `PresentPlural`, `PastFirstSingular`, `PastSecondSingular`, `PastFormal`, `PastThirdSingular`, `PastPlural`, `SubordinatePresentFirstSingular`, `SubordinatePresentSecondSingular`, `SubordinatePresentFormal`, `SubordinatePresentThirdSingular`, `SubordinatePresentPlural`, `SubordinatePastFirstSingular`, `SubordinatePastSecondSingular`, `SubordinatePastFormal`, `SubordinatePastThirdSingular`, `SubordinatePastPlural`

Citation form: `Infinitive, PastThirdSingular, ParticiplePast` (comma-separated; parts you give are output exactly and narrow the prediction).

**det** cells: `PositivePredicative`, `PositiveCommon`, `PositiveNeuter`, `PositivePlural`, `PositiveDefinite`, `PositivePartitive`

**num** cells: `PositivePredicative`, `PositiveCommon`, `PositiveNeuter`, `PositivePlural`, `PositiveDefinite`, `PositivePartitive`

## How it works

Each attested form is stored as an edit tree from the lemma. A lemma's edits across all cells make its inflection class. A suffix tree over the word, read from its end and pruned by a description-length test, predicts the class of any word. Known words the tree gets wrong are kept in a small lexicon. Confidences are smoothed counts; variant weights follow Wiktionary's listing order (1, 1/2, 1/3…, nonstandard forms a tenth of that).

## License

Code: MIT OR Apache-2.0. Data: derived from [Wiktionary](https://www.wiktionary.org/) (via [kaikki.org](https://kaikki.org/)), CC BY-SA 4.0; the model bytes in `src/model.bin` are shared under the same license.
