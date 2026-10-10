# kildin_sami

KildinSami inflection for Rust, learned from Wiktionary by [morphe](https://github.com/gold-silver-copper/morphe).
Inflects any word, known or made up. The `_scored` functions give every candidate with a confidence.

Generated from the Kaikki KildinSami dump of 2026-10-03. Do not edit by hand: change the config in morphe and regenerate.

## Example

```rust
use kildin_sami::*;
println!("{}", noun("кӣлл", Case::Locative, Number::Singular)); // кӣлэсьт
println!("{}", noun("мырр", Case::Comitative, Number::Plural)); // мырэгуэйм
println!("{}", noun_essive("мырр")); // мыррэнҍ
```

Nouns have seven cases in both numbers, plus the essive and the partitive.
Input matching ignores the long-vowel macron. Wiktionary has no Kildin
Sami conjugation tables.

## Coverage

| Part of speech | Entries | Cells | Edits | Classes | Tree nodes | Lexicon |
|---|---|---|---|---|---|---|
| noun | 350 | 16 | 2079 | 312 | 65 | 209 |

Model size: 68464 bytes.

**noun** cells: `NominativeSingular`, `NominativePlural`, `AccusativeSingular`, `AccusativePlural`, `GenitiveSingular`, `GenitivePlural`, `IllativeSingular`, `IllativePlural`, `LocativeSingular`, `LocativePlural`, `ComitativeSingular`, `ComitativePlural`, `AbessiveSingular`, `AbessivePlural`, `Essive`, `Partitive`

Citation form: `NominativeSingular, GenitiveSingular` (comma-separated; parts you give are output exactly and narrow the prediction).

## How it works

Each attested form is stored as an edit tree from the lemma. A lemma's edits across all cells make its inflection class. A suffix tree over the word, read from its end and pruned by a description-length test, predicts the class of any word. Known words the tree gets wrong are kept in a small lexicon. Confidences are smoothed counts; variant weights follow Wiktionary's listing order (1, 1/2, 1/3…, nonstandard forms a tenth of that).

## License

Code: MIT OR Apache-2.0. Data: derived from [Wiktionary](https://www.wiktionary.org/) (via [kaikki.org](https://kaikki.org/)), CC BY-SA 4.0; the model bytes in `src/model.bin` are shared under the same license.
