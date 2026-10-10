# luxembourgish

Luxembourgish inflection for Rust, learned from Wiktionary by [morphe](https://github.com/gold-silver-copper/morphe).
Inflects any word, known or made up. The `_scored` functions give every candidate with a confidence.

Generated from the Kaikki Luxembourgish dump of 2026-10-03. Do not edit by hand: change the config in morphe and regenerate.

## Example

```rust
use luxembourgish::*;
println!("{}", noun_plural("Hond")); // Hënn
println!("{:?}", noun_gender("Buch"));
assert_eq!(adj("grouss", GenderNumber::Masculine), "groussen");
assert_eq!(adj_strong("grouss", GenderNumber::Neuter), "grousses");
println!("{}", verb_present("hidden", PersonNumber::SecondSingular)); // hitts
println!("{:?}", verb_auxiliary("goen")); // Be
```

Compound tenses are left to the caller: `verb_auxiliary` plus
`verb_participle`. Comparatives and superlatives are given as Wiktionary
writes them ("méi grouss", "am gréissten").

## Coverage

| Part of speech | Entries | Cells | Edits | Classes | Tree nodes | Lexicon |
|---|---|---|---|---|---|---|
| noun | 3332 | 2 | 232 | 302 | 337 | 879 |
| adj | 680 | 14 | 82 | 80 | 49 | 227 |
| verb | 1471 | 21 | 927 | 397 | 146 | 661 |

Model size: 217400 bytes.

**noun** cells: `Plural`, `Diminutive`

Citation form: `Lemma, Plural` (comma-separated; parts you give are output exactly and narrow the prediction).

**adj** cells: `Masculine`, `Feminine`, `Neuter`, `Plural`, `StrongMasculine`, `StrongFeminine`, `StrongNeuter`, `StrongPlural`, `DativeMasculine`, `DativeFeminine`, `DativeNeuter`, `DativePlural`, `Comparative`, `Superlative`

Citation form: `Lemma, Masculine, Neuter` (comma-separated; parts you give are output exactly and narrow the prediction).

**verb** cells: `PresentFirstSingular`, `PresentSecondSingular`, `PresentThirdSingular`, `PresentFirstPlural`, `PresentSecondPlural`, `PresentThirdPlural`, `PastFirstSingular`, `PastSecondSingular`, `PastThirdSingular`, `PastFirstPlural`, `PastSecondPlural`, `PastThirdPlural`, `ConditionalFirstSingular`, `ConditionalSecondSingular`, `ConditionalThirdSingular`, `ConditionalFirstPlural`, `ConditionalSecondPlural`, `ConditionalThirdPlural`, `ImperativeSingular`, `ImperativePlural`, `Participle`

Citation form: `Lemma, Participle` (comma-separated; parts you give are output exactly and narrow the prediction).

## How it works

Each attested form is stored as an edit tree from the lemma. A lemma's edits across all cells make its inflection class. A suffix tree over the word, read from its end and pruned by a description-length test, predicts the class of any word. Known words the tree gets wrong are kept in a small lexicon. Confidences are smoothed counts; variant weights follow Wiktionary's listing order (1, 1/2, 1/3…, nonstandard forms a tenth of that).

## License

Code: MIT OR Apache-2.0. Data: derived from [Wiktionary](https://www.wiktionary.org/) (via [kaikki.org](https://kaikki.org/)), CC BY-SA 4.0; the model bytes in `src/model.bin` are shared under the same license.
