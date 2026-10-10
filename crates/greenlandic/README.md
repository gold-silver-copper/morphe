# greenlandic

Greenlandic inflection for Rust, learned from Wiktionary by [morphe](https://github.com/gold-silver-copper/morphe).
Inflects any word, known or made up. The `_scored` functions give every candidate with a confidence.

Generated from the Kaikki Greenlandic dump of 2026-10-03. Do not edit by hand: change the config in morphe and regenerate.

## Example

```rust
use greenlandic::*;
println!("{}", noun("illu", Case::Locative, Number::Singular)); // illumi
println!("{}", noun("radio", Case::Ablative, Number::Plural)); // radioniit
println!("{}", noun_equative("radio")); // radiotut
```

Nouns have seven cases (absolutive, ergative, instrumental, allative,
ablative, locative, prolative) in both numbers, plus the equative.
Wiktionary has conjugation tables for a handful of verbs only, too few to
learn from; verbs are left out.

## Coverage

| Part of speech | Entries | Cells | Edits | Classes | Tree nodes | Lexicon |
|---|---|---|---|---|---|---|
| noun | 643 | 15 | 498 | 76 | 42 | 94 |

Model size: 32544 bytes.

**noun** cells: `AbsolutiveSingular`, `AbsolutivePlural`, `ErgativeSingular`, `ErgativePlural`, `InstrumentalSingular`, `InstrumentalPlural`, `AllativeSingular`, `AllativePlural`, `AblativeSingular`, `AblativePlural`, `LocativeSingular`, `LocativePlural`, `ProlativeSingular`, `ProlativePlural`, `Equative`

Citation form: `AbsolutiveSingular, AbsolutivePlural` (comma-separated; parts you give are output exactly and narrow the prediction).

## How it works

Each attested form is stored as an edit tree from the lemma. A lemma's edits across all cells make its inflection class. A suffix tree over the word, read from its end and pruned by a description-length test, predicts the class of any word. Known words the tree gets wrong are kept in a small lexicon. Confidences are smoothed counts; variant weights follow Wiktionary's listing order (1, 1/2, 1/3…, nonstandard forms a tenth of that).

## License

Code: MIT OR Apache-2.0. Data: derived from [Wiktionary](https://www.wiktionary.org/) (via [kaikki.org](https://kaikki.org/)), CC BY-SA 4.0; the model bytes in `src/model.bin` are shared under the same license.
