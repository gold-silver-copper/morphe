# yiddish

Yiddish inflection for Rust, learned from Wiktionary by [morphe](https://github.com/gold-silver-copper/morphe).
Inflects any word, known or made up. The `_scored` functions give every candidate with a confidence.

Generated from the Kaikki Yiddish dump of 2026-10-03. Do not edit by hand: change the config in morphe and regenerate.

## Example

```rust
use yiddish::*;
println!("{}", noun_plural("הונט")); // הינט
println!("{:?}", noun_gender("הונט"));
println!("{}", verb_present("בענטשן", PersonNumber::SecondSingular)); // בענטשסט
println!("{:?}", verb_auxiliary("גיין")); // Be
println!("{}", adj("גוט", Case::Nominative, Form::Masculine)); // גוטער
```

Vowel points are optional on input, and ײ, װ, ױ match their two-letter
spellings. The past and future are compound tenses: `verb_auxiliary` plus
`verb_participle_past`, or וועלן plus the infinitive.

## Coverage

| Part of speech | Entries | Cells | Edits | Classes | Tree nodes | Lexicon |
|---|---|---|---|---|---|---|
| noun | 4528 | 2 | 412 | 579 | 904 | 3135 |
| adj | 1046 | 20 | 1438 | 336 | 158 | 594 |
| verb | 1280 | 10 | 2294 | 797 | 147 | 1002 |

Model size: 427983 bytes.

**noun** cells: `Plural`, `Diminutive`

Citation form: `Lemma, Plural` (comma-separated; parts you give are output exactly and narrow the prediction).

**adj** cells: `NominativeMasculine`, `NominativeFeminine`, `NominativeNeuter`, `NominativeNeuterDefinite`, `NominativeNeuterPostpositional`, `NominativePlural`, `AccusativeMasculine`, `AccusativeFeminine`, `AccusativeNeuter`, `AccusativeNeuterDefinite`, `AccusativeNeuterPostpositional`, `AccusativePlural`, `DativeMasculine`, `DativeFeminine`, `DativeNeuter`, `DativeNeuterDefinite`, `DativeNeuterPostpositional`, `DativePlural`, `Comparative`, `Superlative`

Citation form: `Lemma, Comparative, Superlative` (comma-separated; parts you give are output exactly and narrow the prediction).

**verb** cells: `PresentFirstSingular`, `PresentSecondSingular`, `PresentThirdSingular`, `PresentFirstPlural`, `PresentSecondPlural`, `PresentThirdPlural`, `ImperativeSingular`, `ImperativePlural`, `ParticiplePresent`, `ParticiplePast`

Citation form: `Lemma, ParticiplePast` (comma-separated; parts you give are output exactly and narrow the prediction).

## How it works

Each attested form is stored as an edit tree from the lemma. A lemma's edits across all cells make its inflection class. A suffix tree over the word, read from its end and pruned by a description-length test, predicts the class of any word. Known words the tree gets wrong are kept in a small lexicon. Confidences are smoothed counts; variant weights follow Wiktionary's listing order (1, 1/2, 1/3…, nonstandard forms a tenth of that).

## License

Code: MIT OR Apache-2.0. Data: derived from [Wiktionary](https://www.wiktionary.org/) (via [kaikki.org](https://kaikki.org/)), CC BY-SA 4.0; the model bytes in `src/model.bin` are shared under the same license.
