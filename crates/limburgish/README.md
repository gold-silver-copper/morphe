# limburgish

Limburgish inflection for Rust, learned from Wiktionary by [morphe](https://github.com/gold-silver-copper/morphe).
Inflects any word, known or made up. The `_scored` functions give every candidate with a confidence.

Generated from the Kaikki Limburgish dump of 2026-10-03. Do not edit by hand: change the config in morphe and regenerate.

## Example

```rust
use limburgish::*;
println!("{}", noun_plural("computer")); // computers
println!("{}", noun_diminutive("computer")); // computerke
println!("{}", verb("male", Mood::Past, Person::Third, Number::Singular)); // maledje
println!("{}", verb_inverted("male", Mood::Past, Person::First, Number::Singular)); // maledje-n
println!("{}", adj_simple("good", Form::Comparative)); // baeter
```

Forms follow Wiktionary's tables, which mostly give the Venlo/Mestreech
type of spelling. Accents are optional on input.

## Coverage

| Part of speech | Entries | Cells | Edits | Classes | Tree nodes | Lexicon |
|---|---|---|---|---|---|---|
| noun | 609 | 2 | 319 | 331 | 179 | 418 |
| adj | 93 | 35 | 93 | 66 | 25 | 76 |
| verb | 259 | 48 | 509 | 176 | 86 | 190 |

Model size: 107539 bytes.

**noun** cells: `Plural`, `Diminutive`

Citation form: `Lemma, Plural` (comma-separated; parts you give are output exactly and narrow the prediction).

**adj** cells: `NominativeMasculineSingular`, `NominativeMasculinePlural`, `NominativeFeminineSingular`, `NominativeFemininePlural`, `NominativeNeuterSingular`, `NominativeNeuterPlural`, `AccusativeMasculineSingular`, `AccusativeMasculinePlural`, `AccusativeFeminineSingular`, `AccusativeFemininePlural`, `AccusativeNeuterSingular`, `AccusativeNeuterPlural`, `GenitiveMasculineSingular`, `GenitiveMasculinePlural`, `GenitiveFeminineSingular`, `GenitiveFemininePlural`, `GenitiveNeuterSingular`, `GenitiveNeuterPlural`, `DativeMasculineSingular`, `DativeMasculinePlural`, `DativeFeminineSingular`, `DativeFemininePlural`, `DativeNeuterSingular`, `DativeNeuterPlural`, `LocativeMasculineSingular`, `LocativeMasculinePlural`, `LocativeFeminineSingular`, `LocativeFemininePlural`, `LocativeNeuterSingular`, `LocativeNeuterPlural`, `SimpleMasculine`, `SimpleFeminine`, `SimpleComparative`, `SimpleSuperlative`, `SimplePredicativeSuperlative`

Citation form: `Lemma, SimpleComparative, SimpleSuperlative` (comma-separated; parts you give are output exactly and narrow the prediction).

**verb** cells: `Infinitive`, `ParticiplePresent`, `ParticiplePast`, `PresentFirstSingular`, `PresentFirstPlural`, `PresentSecondSingular`, `PresentSecondPlural`, `PresentThirdSingular`, `PresentThirdPlural`, `PastFirstSingular`, `PastFirstPlural`, `PastSecondSingular`, `PastSecondPlural`, `PastThirdSingular`, `PastThirdPlural`, `SubjunctiveFirstSingular`, `SubjunctiveFirstPlural`, `SubjunctiveSecondSingular`, `SubjunctiveSecondPlural`, `SubjunctiveThirdSingular`, `SubjunctiveThirdPlural`, `ImperativeSingular`, `ImperativePolite`, `ImperativePlural`, `ImperativeDual`, `InvertedPresentFirstSingular`, `InvertedPresentFirstPlural`, `InvertedPresentSecondSingular`, `InvertedPresentSecondPlural`, `InvertedPresentThirdSingular`, `InvertedPresentThirdPlural`, `InvertedPastFirstSingular`, `InvertedPastFirstPlural`, `InvertedPastSecondSingular`, `InvertedPastSecondPlural`, `InvertedPastThirdSingular`, `InvertedPastThirdPlural`, `InvertedSubjunctiveFirstSingular`, `InvertedSubjunctiveFirstPlural`, `InvertedSubjunctiveSecondSingular`, `InvertedSubjunctiveSecondPlural`, `InvertedSubjunctiveThirdSingular`, `InvertedSubjunctiveThirdPlural`, `InvertedImperativeSingular`, `InvertedImperativePolite`, `InvertedImperativePlural`, `InvertedImperativeDual`, `Inclusive`

Citation form: `Infinitive, PastThirdSingular, ParticiplePast` (comma-separated; parts you give are output exactly and narrow the prediction).

## How it works

Each attested form is stored as an edit tree from the lemma. A lemma's edits across all cells make its inflection class. A suffix tree over the word, read from its end and pruned by a description-length test, predicts the class of any word. Known words the tree gets wrong are kept in a small lexicon. Confidences are smoothed counts; variant weights follow Wiktionary's listing order (1, 1/2, 1/3…, nonstandard forms a tenth of that).

## License

Code: MIT OR Apache-2.0. Data: derived from [Wiktionary](https://www.wiktionary.org/) (via [kaikki.org](https://kaikki.org/)), CC BY-SA 4.0; the model bytes in `src/model.bin` are shared under the same license.
