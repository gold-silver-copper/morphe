# northern_kurdish

NorthernKurdish inflection for Rust, learned from Wiktionary by [morphe](https://github.com/gold-silver-copper/morphe).
Inflects any word, known or made up. The `_scored` functions give every candidate with a confidence.

Generated from the Kaikki NorthernKurdish dump of 2026-10-03. Do not edit by hand: change the config in morphe and regenerate.

## Example

```rust
use northern_kurdish::*;
println!("{}", noun("av", Case::Oblique, Number::Singular)); // avê
println!("{}", noun("av", Case::Construct, Number::Plural)); // avên
println!("{}", noun_indefinite("av", Case::Nominative, Number::Singular)); // avek
println!("{:?}", noun_gender("av")); // Feminine
```

Nouns have the direct (nominative), construct (ezafe), oblique,
demonstrative and vocative forms in both numbers, the indefinite direct,
construct and oblique, and their gender. Wiktionary's Kurmanji
conjugation tables come out of Kaikki with their template slots unfilled,
so verbs are left out.

## Coverage

| Part of speech | Entries | Cells | Edits | Classes | Tree nodes | Lexicon |
|---|---|---|---|---|---|---|
| noun | 630 | 16 | 84 | 16 | 50 | 204 |

Model size: 17496 bytes.

**noun** cells: `NominativeSingular`, `NominativePlural`, `ConstructSingular`, `ConstructPlural`, `ObliqueSingular`, `ObliquePlural`, `DemonstrativeSingular`, `DemonstrativePlural`, `VocativeSingular`, `VocativePlural`, `IndefiniteNominativeSingular`, `IndefiniteNominativePlural`, `IndefiniteConstructSingular`, `IndefiniteConstructPlural`, `IndefiniteObliqueSingular`, `IndefiniteObliquePlural`

Citation form: `NominativeSingular, ObliqueSingular` (comma-separated; parts you give are output exactly and narrow the prediction).

## How it works

Each attested form is stored as an edit tree from the lemma. A lemma's edits across all cells make its inflection class. A suffix tree over the word, read from its end and pruned by a description-length test, predicts the class of any word. Known words the tree gets wrong are kept in a small lexicon. Confidences are smoothed counts; variant weights follow Wiktionary's listing order (1, 1/2, 1/3…, nonstandard forms a tenth of that).

## License

Code: MIT OR Apache-2.0. Data: derived from [Wiktionary](https://www.wiktionary.org/) (via [kaikki.org](https://kaikki.org/)), CC BY-SA 4.0; the model bytes in `src/model.bin` are shared under the same license.
