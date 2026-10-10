# erzya

Erzya inflection for Rust, learned from Wiktionary by [morphe](https://github.com/gold-silver-copper/morphe).
Inflects any word, known or made up. The `_scored` functions give every candidate with a confidence.

Generated from the Kaikki Erzya dump of 2026-10-03. Do not edit by hand: change the config in morphe and regenerate.

## Example

```rust
use erzya::*;
println!("{}", noun("он", Case::Inessive, Number::Singular)); // онсо
println!("{}", noun("он", Case::Nominative, Number::Plural)); // онт
println!("{}", noun_definite("он", Case::Genitive, Number::Plural)); // онтнэнь
```

Nouns have eleven cases in the indefinite singular and the indefinite
nominative plural (the other indefinite plural cases are not used, and
have no form), and ten cases of the definite declension in both numbers. Wiktionary has no
Erzya conjugation tables.

## Coverage

| Part of speech | Entries | Cells | Edits | Classes | Tree nodes | Lexicon |
|---|---|---|---|---|---|---|
| noun | 108 | 22 | 38 | 10 | 13 | 15 |
| noun_definite | 29 | 20 | 59 | 5 | 6 | 8 |

Model size: 8036 bytes.

**noun** cells: `NominativeSingular`, `NominativePlural`, `GenitiveSingular`, `GenitivePlural`, `DativeSingular`, `DativePlural`, `AblativeSingular`, `AblativePlural`, `InessiveSingular`, `InessivePlural`, `ElativeSingular`, `ElativePlural`, `IllativeSingular`, `IllativePlural`, `ProlativeSingular`, `ProlativePlural`, `TranslativeSingular`, `TranslativePlural`, `AbessiveSingular`, `AbessivePlural`, `ComparativeSingular`, `ComparativePlural`

Citation form: `NominativeSingular, NominativePlural` (comma-separated; parts you give are output exactly and narrow the prediction).

**noun_definite** cells: `NominativeSingular`, `NominativePlural`, `GenitiveSingular`, `GenitivePlural`, `DativeSingular`, `DativePlural`, `AblativeSingular`, `AblativePlural`, `InessiveSingular`, `InessivePlural`, `ElativeSingular`, `ElativePlural`, `IllativeSingular`, `IllativePlural`, `ProlativeSingular`, `ProlativePlural`, `AbessiveSingular`, `AbessivePlural`, `ComparativeSingular`, `ComparativePlural`

## How it works

Each attested form is stored as an edit tree from the lemma. A lemma's edits across all cells make its inflection class. A suffix tree over the word, read from its end and pruned by a description-length test, predicts the class of any word. Known words the tree gets wrong are kept in a small lexicon. Confidences are smoothed counts; variant weights follow Wiktionary's listing order (1, 1/2, 1/3…, nonstandard forms a tenth of that).

## License

Code: MIT OR Apache-2.0. Data: derived from [Wiktionary](https://www.wiktionary.org/) (via [kaikki.org](https://kaikki.org/)), CC BY-SA 4.0; the model bytes in `src/model.bin` are shared under the same license.
