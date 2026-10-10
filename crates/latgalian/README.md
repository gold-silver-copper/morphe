# latgalian

Latgalian inflection for Rust, learned from Wiktionary by [morphe](https://github.com/gold-silver-copper/morphe).
Inflects any word, known or made up. The `_scored` functions give every candidate with a confidence.

Generated from the Kaikki Latgalian dump of 2026-10-03. Do not edit by hand: change the config in morphe and regenerate.

## Example

```rust
use latgalian::*;
println!("{}", noun("mama", Case::Dative, Number::Singular)); // mamai
println!("{}", adj("rats", Definiteness::Definite, Gender::Feminine, Case::Nominative, Number::Singular)); // ratuo
```

Nouns have seven cases in both numbers; dialectal forms are kept as
low-weight alternatives. Adjectives decline in both genders, definite and
indefinite. Wiktionary has Latgalian tables for a few hundred words; verbs
are too few to learn from and are left out.

## Coverage

| Part of speech | Entries | Cells | Edits | Classes | Tree nodes | Lexicon |
|---|---|---|---|---|---|---|
| noun | 289 | 14 | 182 | 78 | 31 | 56 |
| adj | 63 | 56 | 87 | 7 | 4 | 1 |
| pron | 22 | 28 | 86 | 19 | 7 | 8 |
| num | 9 | 28 | 29 | 5 | 2 | 4 |

Model size: 28084 bytes.

**noun** cells: `NominativeSingular`, `NominativePlural`, `GenitiveSingular`, `GenitivePlural`, `DativeSingular`, `DativePlural`, `AccusativeSingular`, `AccusativePlural`, `InstrumentalSingular`, `InstrumentalPlural`, `LocativeSingular`, `LocativePlural`, `VocativeSingular`, `VocativePlural`

Citation form: `NominativeSingular, GenitiveSingular` (comma-separated; parts you give are output exactly and narrow the prediction).

**adj** cells: `IndefiniteMasculineNominativeSingular`, `IndefiniteMasculineNominativePlural`, `IndefiniteMasculineGenitiveSingular`, `IndefiniteMasculineGenitivePlural`, `IndefiniteMasculineDativeSingular`, `IndefiniteMasculineDativePlural`, `IndefiniteMasculineAccusativeSingular`, `IndefiniteMasculineAccusativePlural`, `IndefiniteMasculineInstrumentalSingular`, `IndefiniteMasculineInstrumentalPlural`, `IndefiniteMasculineLocativeSingular`, `IndefiniteMasculineLocativePlural`, `IndefiniteMasculineVocativeSingular`, `IndefiniteMasculineVocativePlural`, `IndefiniteFeminineNominativeSingular`, `IndefiniteFeminineNominativePlural`, `IndefiniteFeminineGenitiveSingular`, `IndefiniteFeminineGenitivePlural`, `IndefiniteFeminineDativeSingular`, `IndefiniteFeminineDativePlural`, `IndefiniteFeminineAccusativeSingular`, `IndefiniteFeminineAccusativePlural`, `IndefiniteFeminineInstrumentalSingular`, `IndefiniteFeminineInstrumentalPlural`, `IndefiniteFeminineLocativeSingular`, `IndefiniteFeminineLocativePlural`, `IndefiniteFeminineVocativeSingular`, `IndefiniteFeminineVocativePlural`, `DefiniteMasculineNominativeSingular`, `DefiniteMasculineNominativePlural`, `DefiniteMasculineGenitiveSingular`, `DefiniteMasculineGenitivePlural`, `DefiniteMasculineDativeSingular`, `DefiniteMasculineDativePlural`, `DefiniteMasculineAccusativeSingular`, `DefiniteMasculineAccusativePlural`, `DefiniteMasculineInstrumentalSingular`, `DefiniteMasculineInstrumentalPlural`, `DefiniteMasculineLocativeSingular`, `DefiniteMasculineLocativePlural`, `DefiniteMasculineVocativeSingular`, `DefiniteMasculineVocativePlural`, `DefiniteFeminineNominativeSingular`, `DefiniteFeminineNominativePlural`, `DefiniteFeminineGenitiveSingular`, `DefiniteFeminineGenitivePlural`, `DefiniteFeminineDativeSingular`, `DefiniteFeminineDativePlural`, `DefiniteFeminineAccusativeSingular`, `DefiniteFeminineAccusativePlural`, `DefiniteFeminineInstrumentalSingular`, `DefiniteFeminineInstrumentalPlural`, `DefiniteFeminineLocativeSingular`, `DefiniteFeminineLocativePlural`, `DefiniteFeminineVocativeSingular`, `DefiniteFeminineVocativePlural`

**pron** cells: `MasculineNominativeSingular`, `MasculineNominativePlural`, `MasculineGenitiveSingular`, `MasculineGenitivePlural`, `MasculineDativeSingular`, `MasculineDativePlural`, `MasculineAccusativeSingular`, `MasculineAccusativePlural`, `MasculineInstrumentalSingular`, `MasculineInstrumentalPlural`, `MasculineLocativeSingular`, `MasculineLocativePlural`, `MasculineVocativeSingular`, `MasculineVocativePlural`, `FeminineNominativeSingular`, `FeminineNominativePlural`, `FeminineGenitiveSingular`, `FeminineGenitivePlural`, `FeminineDativeSingular`, `FeminineDativePlural`, `FeminineAccusativeSingular`, `FeminineAccusativePlural`, `FeminineInstrumentalSingular`, `FeminineInstrumentalPlural`, `FeminineLocativeSingular`, `FeminineLocativePlural`, `FeminineVocativeSingular`, `FeminineVocativePlural`

**num** cells: `MasculineNominativeSingular`, `MasculineNominativePlural`, `MasculineGenitiveSingular`, `MasculineGenitivePlural`, `MasculineDativeSingular`, `MasculineDativePlural`, `MasculineAccusativeSingular`, `MasculineAccusativePlural`, `MasculineInstrumentalSingular`, `MasculineInstrumentalPlural`, `MasculineLocativeSingular`, `MasculineLocativePlural`, `MasculineVocativeSingular`, `MasculineVocativePlural`, `FeminineNominativeSingular`, `FeminineNominativePlural`, `FeminineGenitiveSingular`, `FeminineGenitivePlural`, `FeminineDativeSingular`, `FeminineDativePlural`, `FeminineAccusativeSingular`, `FeminineAccusativePlural`, `FeminineInstrumentalSingular`, `FeminineInstrumentalPlural`, `FeminineLocativeSingular`, `FeminineLocativePlural`, `FeminineVocativeSingular`, `FeminineVocativePlural`

## How it works

Each attested form is stored as an edit tree from the lemma. A lemma's edits across all cells make its inflection class. A suffix tree over the word, read from its end and pruned by a description-length test, predicts the class of any word. Known words the tree gets wrong are kept in a small lexicon. Confidences are smoothed counts; variant weights follow Wiktionary's listing order (1, 1/2, 1/3…, nonstandard forms a tenth of that).

## License

Code: MIT OR Apache-2.0. Data: derived from [Wiktionary](https://www.wiktionary.org/) (via [kaikki.org](https://kaikki.org/)), CC BY-SA 4.0; the model bytes in `src/model.bin` are shared under the same license.
