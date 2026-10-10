# occitan

Occitan inflection for Rust, learned from Wiktionary by [morphe](https://github.com/gold-silver-copper/morphe).
Inflects any word, known or made up. The `_scored` functions give every candidate with a confidence.

Generated from the Kaikki Occitan dump of 2026-10-03. Do not edit by hand: change the config in morphe and regenerate.

## Example

```rust
use occitan::*;
println!("{}", noun_plural("ostal")); // ostals
assert_eq!(adj("gratuit", Form::Feminine), "gratuita");
assert_eq!(verb("abandonar", Tense::Preterite, Person::Third, Number::Singular), "abandonèt");
assert_eq!(verb("abandonar", Tense::Present, Person::First, Number::Singular), "abandoni");
```

Forms follow the Languedocien standard as in Wiktionary's tables. Accents
are optional on input.

## Coverage

| Part of speech | Entries | Cells | Edits | Classes | Tree nodes | Lexicon |
|---|---|---|---|---|---|---|
| noun | 2462 | 1 | 121 | 126 | 160 | 448 |
| adj | 598 | 3 | 114 | 65 | 41 | 116 |
| verb | 620 | 53 | 1602 | 86 | 25 | 59 |

Model size: 264018 bytes.

**noun** cells: `Plural`

Citation form: `Lemma, Plural` (comma-separated; parts you give are output exactly and narrow the prediction).

**adj** cells: `Feminine`, `MasculinePlural`, `FemininePlural`

Citation form: `Lemma, Feminine, MasculinePlural` (comma-separated; parts you give are output exactly and narrow the prediction).

**verb** cells: `PresentFirstSingular`, `PresentFirstPlural`, `PresentSecondSingular`, `PresentSecondPlural`, `PresentThirdSingular`, `PresentThirdPlural`, `ImperfectFirstSingular`, `ImperfectFirstPlural`, `ImperfectSecondSingular`, `ImperfectSecondPlural`, `ImperfectThirdSingular`, `ImperfectThirdPlural`, `PreteriteFirstSingular`, `PreteriteFirstPlural`, `PreteriteSecondSingular`, `PreteriteSecondPlural`, `PreteriteThirdSingular`, `PreteriteThirdPlural`, `FutureFirstSingular`, `FutureFirstPlural`, `FutureSecondSingular`, `FutureSecondPlural`, `FutureThirdSingular`, `FutureThirdPlural`, `ConditionalFirstSingular`, `ConditionalFirstPlural`, `ConditionalSecondSingular`, `ConditionalSecondPlural`, `ConditionalThirdSingular`, `ConditionalThirdPlural`, `ConditionalIIFirstSingular`, `ConditionalIIFirstPlural`, `ConditionalIISecondSingular`, `ConditionalIISecondPlural`, `ConditionalIIThirdSingular`, `ConditionalIIThirdPlural`, `SubjunctiveFirstSingular`, `SubjunctiveFirstPlural`, `SubjunctiveSecondSingular`, `SubjunctiveSecondPlural`, `SubjunctiveThirdSingular`, `SubjunctiveThirdPlural`, `ImperfectSubjunctiveFirstSingular`, `ImperfectSubjunctiveFirstPlural`, `ImperfectSubjunctiveSecondSingular`, `ImperfectSubjunctiveSecondPlural`, `ImperfectSubjunctiveThirdSingular`, `ImperfectSubjunctiveThirdPlural`, `ImperativeSecondSingular`, `ImperativeFirstPlural`, `ImperativeSecondPlural`, `Gerund`, `ParticiplePast`

Citation form: `Lemma, PresentFirstSingular, PreteriteThirdSingular, ParticiplePast` (comma-separated; parts you give are output exactly and narrow the prediction).

## How it works

Each attested form is stored as an edit tree from the lemma. A lemma's edits across all cells make its inflection class. A suffix tree over the word, read from its end and pruned by a description-length test, predicts the class of any word. Known words the tree gets wrong are kept in a small lexicon. Confidences are smoothed counts; variant weights follow Wiktionary's listing order (1, 1/2, 1/3…, nonstandard forms a tenth of that).

## License

Code: MIT OR Apache-2.0. Data: derived from [Wiktionary](https://www.wiktionary.org/) (via [kaikki.org](https://kaikki.org/)), CC BY-SA 4.0; the model bytes in `src/model.bin` are shared under the same license.
