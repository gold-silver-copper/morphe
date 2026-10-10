# malayalam

Malayalam inflection for Rust, learned from Wiktionary by [morphe](https://github.com/gold-silver-copper/morphe).
Inflects any word, known or made up. The `_scored` functions give every candidate with a confidence.

Generated from the Kaikki Malayalam dump of 2026-10-03. Do not edit by hand: change the config in morphe and regenerate.

## Example

```rust
use malayalam::*;
println!("{}", noun("അമ്മ", Case::Dative, Number::Singular)); // അമ്മയ്ക്ക്
println!("{}", noun("അമ്മ", Case::Nominative, Number::Plural)); // അമ്മമാർ
println!("{}", noun("അമ്മ", Case::Vocative, Number::Singular)); // അമ്മേ
println!("{}", verb_past("കരയുക")); // കരഞ്ഞു
```

Nouns and pronouns have eight cases in both numbers. Verbs have only their
past tense, which is all Wiktionary gives for most of them. Input matching
ignores zero-width non-joiners and accepts chillu letters written with a
joiner.

## Coverage

| Part of speech | Entries | Cells | Edits | Classes | Tree nodes | Lexicon |
|---|---|---|---|---|---|---|
| noun | 7557 | 16 | 313 | 79 | 107 | 377 |
| verb | 826 | 1 | 18 | 19 | 26 | 19 |
| pron | 100 | 16 | 65 | 7 | 2 | 5 |

Model size: 70409 bytes.

**noun** cells: `NominativeSingular`, `NominativePlural`, `AccusativeSingular`, `AccusativePlural`, `DativeSingular`, `DativePlural`, `SociativeSingular`, `SociativePlural`, `GenitiveSingular`, `GenitivePlural`, `InstrumentalSingular`, `InstrumentalPlural`, `LocativeSingular`, `LocativePlural`, `VocativeSingular`, `VocativePlural`

Citation form: `NominativeSingular, NominativePlural` (comma-separated; parts you give are output exactly and narrow the prediction).

**verb** cells: `Past`

Citation form: `Lemma, Past` (comma-separated; parts you give are output exactly and narrow the prediction).

**pron** cells: `NominativeSingular`, `NominativePlural`, `AccusativeSingular`, `AccusativePlural`, `DativeSingular`, `DativePlural`, `SociativeSingular`, `SociativePlural`, `GenitiveSingular`, `GenitivePlural`, `InstrumentalSingular`, `InstrumentalPlural`, `LocativeSingular`, `LocativePlural`, `VocativeSingular`, `VocativePlural`

## How it works

Each attested form is stored as an edit tree from the lemma. A lemma's edits across all cells make its inflection class. A suffix tree over the word, read from its end and pruned by a description-length test, predicts the class of any word. Known words the tree gets wrong are kept in a small lexicon. Confidences are smoothed counts; variant weights follow Wiktionary's listing order (1, 1/2, 1/3…, nonstandard forms a tenth of that).

## License

Code: MIT OR Apache-2.0. Data: derived from [Wiktionary](https://www.wiktionary.org/) (via [kaikki.org](https://kaikki.org/)), CC BY-SA 4.0; the model bytes in `src/model.bin` are shared under the same license.
