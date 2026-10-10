# ossetian

Ossetian inflection for Rust, learned from Wiktionary by [morphe](https://github.com/gold-silver-copper/morphe).
Inflects any word, known or made up. The `_scored` functions give every candidate with a confidence.

Generated from the Kaikki Ossetian dump of 2026-10-03. Do not edit by hand: change the config in morphe and regenerate.

## Example

```rust
use ossetian::*;
println!("{}", noun("сӕр", Case::Dative, Number::Singular)); // сӕрӕн
println!("{}", noun("адӕймаг", Case::Nominative, Number::Plural)); // адӕймӕгтӕ
println!("{}", noun("хур", Case::Comitative, Number::Plural)); // хуртимӕ
```

Nouns have nine cases in both numbers. Wiktionary has declension tables
for a few dozen Ossetian nouns only (the declension is regular), and
conjugation tables for too few verbs to learn from.

## Coverage

| Part of speech | Entries | Cells | Edits | Classes | Tree nodes | Lexicon |
|---|---|---|---|---|---|---|
| noun | 11 | 18 | 83 | 8 | 2 | 8 |

Model size: 2073 bytes.

**noun** cells: `NominativeSingular`, `NominativePlural`, `GenitiveSingular`, `GenitivePlural`, `DativeSingular`, `DativePlural`, `AllativeSingular`, `AllativePlural`, `AblativeSingular`, `AblativePlural`, `InessiveSingular`, `InessivePlural`, `AdessiveSingular`, `AdessivePlural`, `EquativeSingular`, `EquativePlural`, `ComitativeSingular`, `ComitativePlural`

Citation form: `NominativeSingular, NominativePlural` (comma-separated; parts you give are output exactly and narrow the prediction).

## How it works

Each attested form is stored as an edit tree from the lemma. A lemma's edits across all cells make its inflection class. A suffix tree over the word, read from its end and pruned by a description-length test, predicts the class of any word. Known words the tree gets wrong are kept in a small lexicon. Confidences are smoothed counts; variant weights follow Wiktionary's listing order (1, 1/2, 1/3…, nonstandard forms a tenth of that).

## License

Code: MIT OR Apache-2.0. Data: derived from [Wiktionary](https://www.wiktionary.org/) (via [kaikki.org](https://kaikki.org/)), CC BY-SA 4.0; the model bytes in `src/model.bin` are shared under the same license.
