# west_frisian

WestFrisian inflection for Rust, learned from Wiktionary by [morphe](https://github.com/gold-silver-copper/morphe).
Inflects any word, known or made up. The `_scored` functions give every candidate with a confidence.

Generated from the Kaikki WestFrisian dump of 2026-10-03. Do not edit by hand: change the config in morphe and regenerate.

## Example

```rust
use west_frisian::*;
println!("{}", noun_plural("winter")); // winters
println!("{}", noun_diminutive("winter")); // winterke
println!("{:?}", noun_gender("hûs"));
println!("{}", verb_past("love", PersonNumber::ThirdSingular)); // loofde
// Separable verbs split in main clauses and join in subordinate ones:
println!("{}", verb_present("oersette", PersonNumber::ThirdSingular)); // set oer
println!("{}", verb_subordinate_present("oersette", PersonNumber::ThirdSingular)); // oerset
println!("{}", adj("bot", Degree::Comparative, Form::Definite)); // bottere
```

Circumflexes and acute accents are optional on input. The predicative
superlative is given without "it".

## Coverage

| Part of speech | Entries | Cells | Edits | Classes | Tree nodes | Lexicon |
|---|---|---|---|---|---|---|
| noun | 1593 | 2 | 133 | 211 | 217 | 543 |
| adj | 219 | 18 | 110 | 73 | 50 | 85 |
| verb | 260 | 25 | 694 | 187 | 70 | 133 |

Model size: 77623 bytes.

**noun** cells: `Plural`, `Diminutive`

Citation form: `Lemma, Plural` (comma-separated; parts you give are output exactly and narrow the prediction).

**adj** cells: `PositivePredicative`, `PositiveCommon`, `PositiveNeuter`, `PositivePlural`, `PositiveDefinite`, `PositivePartitive`, `ComparativePredicative`, `ComparativeCommon`, `ComparativeNeuter`, `ComparativePlural`, `ComparativeDefinite`, `ComparativePartitive`, `SuperlativePredicative`, `SuperlativeCommon`, `SuperlativeNeuter`, `SuperlativePlural`, `SuperlativeDefinite`, `SuperlativePartitive`

Citation form: `Lemma, ComparativePredicative, SuperlativePredicative` (comma-separated; parts you give are output exactly and narrow the prediction).

**verb** cells: `Infinitive`, `LongInfinitive`, `ParticiplePresent`, `ParticiplePast`, `Imperative`, `PresentFirstSingular`, `PresentSecondSingular`, `PresentThirdSingular`, `PresentPlural`, `PresentSecondClitic`, `PastFirstSingular`, `PastSecondSingular`, `PastThirdSingular`, `PastPlural`, `PastSecondClitic`, `SubordinatePresentFirstSingular`, `SubordinatePresentSecondSingular`, `SubordinatePresentThirdSingular`, `SubordinatePresentPlural`, `SubordinatePresentSecondClitic`, `SubordinatePastFirstSingular`, `SubordinatePastSecondSingular`, `SubordinatePastThirdSingular`, `SubordinatePastPlural`, `SubordinatePastSecondClitic`

Citation form: `Infinitive, PastThirdSingular, ParticiplePast` (comma-separated; parts you give are output exactly and narrow the prediction).

## How it works

Each attested form is stored as an edit tree from the lemma. A lemma's edits across all cells make its inflection class. A suffix tree over the word, read from its end and pruned by a description-length test, predicts the class of any word. Known words the tree gets wrong are kept in a small lexicon. Confidences are smoothed counts; variant weights follow Wiktionary's listing order (1, 1/2, 1/3…, nonstandard forms a tenth of that).

## License

Code: MIT OR Apache-2.0. Data: derived from [Wiktionary](https://www.wiktionary.org/) (via [kaikki.org](https://kaikki.org/)), CC BY-SA 4.0; the model bytes in `src/model.bin` are shared under the same license.
