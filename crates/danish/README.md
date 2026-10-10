# danish

Danish inflection for Rust, learned from Wiktionary by [morphe](https://github.com/gold-silver-copper/morphe).
Inflects any word, known or made up. The `_scored` functions give every candidate with a confidence.

Generated from the Kaikki Danish dump of 2026-10-03. Do not edit by hand: change the config in morphe and regenerate.

## Example

```rust
use danish::*;
assert_eq!(noun("dreng", Definiteness::Definite, Case::Nominative, Number::Plural), "drengene");
println!("{:?}", noun_gender("hus"));
assert_eq!(verb_past("give", Voice::Active), "gav");
assert_eq!(verb_participle_past("give"), "givet");
assert_eq!(adj_neuter("stor"), "stort");
assert_eq!(adj_superlative_definite("stor"), "største");
// Citations pin principal parts: singular, definite singular, plural.
println!("{}", noun("mand, manden, mænd", Definiteness::Definite, Case::Nominative, Number::Plural));
```

Forms are given without "at", "har" or articles ("give", "givet", "ansatte").

## Coverage

| Part of speech | Entries | Cells | Edits | Classes | Tree nodes | Lexicon |
|---|---|---|---|---|---|---|
| noun | 14042 | 8 | 657 | 700 | 1357 | 4146 |
| adj | 3510 | 6 | 161 | 167 | 334 | 1785 |
| verb | 3009 | 11 | 551 | 694 | 350 | 1444 |

Model size: 318109 bytes.

**noun** cells: `IndefiniteNominativeSingular`, `IndefiniteNominativePlural`, `IndefiniteGenitiveSingular`, `IndefiniteGenitivePlural`, `DefiniteNominativeSingular`, `DefiniteNominativePlural`, `DefiniteGenitiveSingular`, `DefiniteGenitivePlural`

Citation form: `IndefiniteNominativeSingular, DefiniteNominativeSingular, IndefiniteNominativePlural` (comma-separated; parts you give are output exactly and narrow the prediction).

**adj** cells: `Neuter`, `Definite`, `Plural`, `Comparative`, `Superlative`, `SuperlativeDefinite`

Citation form: `Lemma, Neuter, Definite` (comma-separated; parts you give are output exactly and narrow the prediction).

**verb** cells: `InfinitiveActive`, `InfinitivePassive`, `PresentActive`, `PresentPassive`, `PastActive`, `PastPassive`, `ImperativeActive`, `ParticiplePresent`, `ParticiplePast`, `ParticiplePastCommon`, `ParticiplePastPlural`

Citation form: `InfinitiveActive, PresentActive, PastActive, ParticiplePast` (comma-separated; parts you give are output exactly and narrow the prediction).

## How it works

Each attested form is stored as an edit tree from the lemma. A lemma's edits across all cells make its inflection class. A suffix tree over the word, read from its end and pruned by a description-length test, predicts the class of any word. Known words the tree gets wrong are kept in a small lexicon. Confidences are smoothed counts; variant weights follow Wiktionary's listing order (1, 1/2, 1/3…, nonstandard forms a tenth of that).

## License

Code: MIT OR Apache-2.0. Data: derived from [Wiktionary](https://www.wiktionary.org/) (via [kaikki.org](https://kaikki.org/)), CC BY-SA 4.0; the model bytes in `src/model.bin` are shared under the same license.
