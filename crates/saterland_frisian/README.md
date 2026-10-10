# saterland_frisian

SaterlandFrisian inflection for Rust, learned from Wiktionary by [morphe](https://github.com/gold-silver-copper/morphe).
Inflects any word, known or made up. The `_scored` functions give every candidate with a confidence.

Generated from the Kaikki SaterlandFrisian dump of 2026-10-03. Do not edit by hand: change the config in morphe and regenerate.

## Example

```rust
use saterland_frisian::*;
println!("{}", noun_plural("wai")); // wege
println!("{}", verb_past("here", PersonNumber::ThirdSingular)); // heerde
println!("{}", verb_present("ouannerje", PersonNumber::ThirdSingular)); // annert ou
println!("{}", verb_subordinate_present("ouannerje", PersonNumber::ThirdSingular)); // ouannert
println!("{}", adj("breed", Form::Comparative)); // brader
```

Output is lowercase (nouns are capitalized in writing). Plurals are given
without the article "do". Umlauts and accents are optional on input.

## Coverage

| Part of speech | Entries | Cells | Edits | Classes | Tree nodes | Lexicon |
|---|---|---|---|---|---|---|
| noun | 450 | 1 | 61 | 67 | 98 | 189 |
| adj | 63 | 5 | 63 | 30 | 11 | 46 |
| verb | 138 | 22 | 502 | 122 | 54 | 80 |
| det | 12 | 5 | 15 | 9 | 6 | 7 |

Model size: 76528 bytes.

**noun** cells: `Plural`

Citation form: `Lemma, Plural` (comma-separated; parts you give are output exactly and narrow the prediction).

**adj** cells: `Inflected`, `Masculine`, `Neuter`, `Comparative`, `Superlative`

Citation form: `Lemma, Comparative, Superlative` (comma-separated; parts you give are output exactly and narrow the prediction).

**verb** cells: `Infinitive`, `LongInfinitive`, `ParticiplePresent`, `ParticiplePast`, `ImperativeSingular`, `ImperativePlural`, `PresentFirstSingular`, `PresentSecondSingular`, `PresentThirdSingular`, `PresentPlural`, `PastFirstSingular`, `PastSecondSingular`, `PastThirdSingular`, `PastPlural`, `SubordinatePresentFirstSingular`, `SubordinatePresentSecondSingular`, `SubordinatePresentThirdSingular`, `SubordinatePresentPlural`, `SubordinatePastFirstSingular`, `SubordinatePastSecondSingular`, `SubordinatePastThirdSingular`, `SubordinatePastPlural`

Citation form: `Infinitive, PastThirdSingular, ParticiplePast` (comma-separated; parts you give are output exactly and narrow the prediction).

**det** cells: `Feminine`, `Neuter`, `Plural`, `Predicative`, `Oblique`

## How it works

Each attested form is stored as an edit tree from the lemma. A lemma's edits across all cells make its inflection class. A suffix tree over the word, read from its end and pruned by a description-length test, predicts the class of any word. Known words the tree gets wrong are kept in a small lexicon. Confidences are smoothed counts; variant weights follow Wiktionary's listing order (1, 1/2, 1/3…, nonstandard forms a tenth of that).

## License

Code: MIT OR Apache-2.0. Data: derived from [Wiktionary](https://www.wiktionary.org/) (via [kaikki.org](https://kaikki.org/)), CC BY-SA 4.0; the model bytes in `src/model.bin` are shared under the same license.
