# middle_english

MiddleEnglish inflection for Rust, learned from Wiktionary by [morphe](https://github.com/gold-silver-copper/morphe).
Inflects any word, known or made up. The `_scored` functions give every candidate with a confidence.

Generated from the Kaikki MiddleEnglish dump of 2026-10-03. Do not edit by hand: change the config in morphe and regenerate.

## Example

```rust
use middle_english::*;
println!("{}", noun_plural("child")); // children
println!("{}", verb_present("loven", PersonNumber::ThirdSingular)); // loveth
println!("{}", verb_past("loven", PersonNumber::Plural)); // loveden
println!("{}", verb_participle_past("abreggen")); // abregged
```

Spelling varied widely; forms follow Wiktionary's headword spellings.
Macrons are optional on input, and th matches þ and ð.

## Coverage

| Part of speech | Entries | Cells | Edits | Classes | Tree nodes | Lexicon |
|---|---|---|---|---|---|---|
| noun | 3514 | 5 | 297 | 393 | 232 | 1110 |
| adj | 186 | 4 | 97 | 97 | 41 | 110 |
| verb | 1653 | 16 | 1070 | 690 | 174 | 727 |

Model size: 112286 bytes.

**noun** cells: `Plural`, `GenitiveSingular`, `GenitivePlural`, `DativeSingular`, `DativePlural`

Citation form: `Lemma, Plural` (comma-separated; parts you give are output exactly and narrow the prediction).

**adj** cells: `WeakSingular`, `WeakPlural`, `Comparative`, `Superlative`

**verb** cells: `PresentFirstSingular`, `PresentSecondSingular`, `PresentThirdSingular`, `PresentPlural`, `PastFirstSingular`, `PastSecondSingular`, `PastThirdSingular`, `PastPlural`, `SubjunctivePresentSingular`, `SubjunctivePresentPlural`, `SubjunctivePastSingular`, `SubjunctivePastPlural`, `ImperativeSingular`, `ImperativePlural`, `ParticiplePresent`, `ParticiplePast`

Citation form: `Lemma, PastThirdSingular, ParticiplePast` (comma-separated; parts you give are output exactly and narrow the prediction).

## How it works

Each attested form is stored as an edit tree from the lemma. A lemma's edits across all cells make its inflection class. A suffix tree over the word, read from its end and pruned by a description-length test, predicts the class of any word. Known words the tree gets wrong are kept in a small lexicon. Confidences are smoothed counts; variant weights follow Wiktionary's listing order (1, 1/2, 1/3…, nonstandard forms a tenth of that).

## License

Code: MIT OR Apache-2.0. Data: derived from [Wiktionary](https://www.wiktionary.org/) (via [kaikki.org](https://kaikki.org/)), CC BY-SA 4.0; the model bytes in `src/model.bin` are shared under the same license.
