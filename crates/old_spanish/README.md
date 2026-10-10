# old_spanish

OldSpanish inflection for Rust, learned from Wiktionary by [morphe](https://github.com/gold-silver-copper/morphe).
Inflects any word, known or made up. The `_scored` functions give every candidate with a confidence.

Generated from the Kaikki OldSpanish dump of 2026-10-03. Do not edit by hand: change the config in morphe and regenerate.

## Example

```rust
use old_spanish::*;
println!("{}", noun_plural("pie")); // pies
println!("{}", adj("loco", Form::FemininePlural)); // locas
println!("{}", verb("ser", Tense::Imperfect, Person::Third, Number::Singular)); // era
println!("{}", verb("vender", Tense::Present, Person::Second, Number::Plural)); // vendedes
```

Accents are optional on input.

## Coverage

| Part of speech | Entries | Cells | Edits | Classes | Tree nodes | Lexicon |
|---|---|---|---|---|---|---|
| noun | 556 | 1 | 13 | 16 | 14 | 28 |
| adj | 53 | 3 | 9 | 11 | 8 | 10 |
| verb | 6 | 68 | 704 | 12 | 2 | 2 |

Model size: 23389 bytes.

**noun** cells: `Plural`

Citation form: `Lemma, Plural` (comma-separated; parts you give are output exactly and narrow the prediction).

**adj** cells: `Feminine`, `MasculinePlural`, `FemininePlural`

Citation form: `Lemma, Feminine, MasculinePlural` (comma-separated; parts you give are output exactly and narrow the prediction).

**verb** cells: `Infinitive`, `Gerund`, `ParticipleMasculine`, `ParticipleFeminine`, `ParticipleMasculinePlural`, `ParticipleFemininePlural`, `ParticiplePresentSingular`, `ParticiplePresentPlural`, `PresentFirstSingular`, `PresentFirstPlural`, `PresentSecondSingular`, `PresentSecondPlural`, `PresentThirdSingular`, `PresentThirdPlural`, `ImperfectFirstSingular`, `ImperfectFirstPlural`, `ImperfectSecondSingular`, `ImperfectSecondPlural`, `ImperfectThirdSingular`, `ImperfectThirdPlural`, `PreteriteFirstSingular`, `PreteriteFirstPlural`, `PreteriteSecondSingular`, `PreteriteSecondPlural`, `PreteriteThirdSingular`, `PreteriteThirdPlural`, `PluperfectFirstSingular`, `PluperfectFirstPlural`, `PluperfectSecondSingular`, `PluperfectSecondPlural`, `PluperfectThirdSingular`, `PluperfectThirdPlural`, `FutureFirstSingular`, `FutureFirstPlural`, `FutureSecondSingular`, `FutureSecondPlural`, `FutureThirdSingular`, `FutureThirdPlural`, `ConditionalFirstSingular`, `ConditionalFirstPlural`, `ConditionalSecondSingular`, `ConditionalSecondPlural`, `ConditionalThirdSingular`, `ConditionalThirdPlural`, `SubjunctiveFirstSingular`, `SubjunctiveFirstPlural`, `SubjunctiveSecondSingular`, `SubjunctiveSecondPlural`, `SubjunctiveThirdSingular`, `SubjunctiveThirdPlural`, `ImperfectSubjunctiveFirstSingular`, `ImperfectSubjunctiveFirstPlural`, `ImperfectSubjunctiveSecondSingular`, `ImperfectSubjunctiveSecondPlural`, `ImperfectSubjunctiveThirdSingular`, `ImperfectSubjunctiveThirdPlural`, `FutureSubjunctiveFirstSingular`, `FutureSubjunctiveFirstPlural`, `FutureSubjunctiveSecondSingular`, `FutureSubjunctiveSecondPlural`, `FutureSubjunctiveThirdSingular`, `FutureSubjunctiveThirdPlural`, `ImperativeFirstSingular`, `ImperativeFirstPlural`, `ImperativeSecondSingular`, `ImperativeSecondPlural`, `ImperativeThirdSingular`, `ImperativeThirdPlural`

Citation form: `Infinitive, PresentFirstSingular, ParticipleMasculine` (comma-separated; parts you give are output exactly and narrow the prediction).

## How it works

Each attested form is stored as an edit tree from the lemma. A lemma's edits across all cells make its inflection class. A suffix tree over the word, read from its end and pruned by a description-length test, predicts the class of any word. Known words the tree gets wrong are kept in a small lexicon. Confidences are smoothed counts; variant weights follow Wiktionary's listing order (1, 1/2, 1/3…, nonstandard forms a tenth of that).

## License

Code: MIT OR Apache-2.0. Data: derived from [Wiktionary](https://www.wiktionary.org/) (via [kaikki.org](https://kaikki.org/)), CC BY-SA 4.0; the model bytes in `src/model.bin` are shared under the same license.
