# middle_welsh

MiddleWelsh inflection for Rust, learned from Wiktionary by [morphe](https://github.com/gold-silver-copper/morphe).
Inflects any word, known or made up. The `_scored` functions give every candidate with a confidence.

Generated from the Kaikki MiddleWelsh dump of 2026-10-03. Do not edit by hand: change the config in morphe and regenerate.

## Example

```rust
use middle_welsh::*;
println!("{}", noun_plural("ber")); // bereu
println!("{}", noun_mutated("arch", Mutation::HProthesis)); // harch
println!("{}", verb("bwrw", Tense::Preterite, Person::ThirdSingular)); // byriawð
println!("{}", adj("da", Degree::Superlative)); // goreu
```

Spellings follow Wiktionary's normalized Middle Welsh, with ð for dd; d
matches ð on input.

## Coverage

| Part of speech | Entries | Cells | Edits | Classes | Tree nodes | Lexicon |
|---|---|---|---|---|---|---|
| noun | 252 | 5 | 94 | 123 | 59 | 114 |
| adj | 65 | 8 | 41 | 35 | 20 | 24 |
| verb | 389 | 68 | 1490 | 148 | 62 | 127 |

Model size: 53563 bytes.

**noun** cells: `Plural`, `MutatedSoft`, `MutatedNasal`, `MutatedAspirate`, `MutatedHProthesis`

Citation form: `Lemma, Plural` (comma-separated; parts you give are output exactly and narrow the prediction).

**adj** cells: `Plural`, `Equative`, `Comparative`, `Superlative`, `MutatedSoft`, `MutatedNasal`, `MutatedAspirate`, `MutatedHProthesis`

Citation form: `Lemma, Plural` (comma-separated; parts you give are output exactly and narrow the prediction).

**verb** cells: `PresentFirstSingular`, `PresentSecondSingular`, `PresentThirdSingular`, `PresentFirstPlural`, `PresentSecondPlural`, `PresentThirdPlural`, `PresentImpersonal`, `FutureFirstSingular`, `FutureSecondSingular`, `FutureThirdSingular`, `FutureFirstPlural`, `FutureSecondPlural`, `FutureThirdPlural`, `FutureImpersonal`, `ImperfectFirstSingular`, `ImperfectSecondSingular`, `ImperfectThirdSingular`, `ImperfectFirstPlural`, `ImperfectSecondPlural`, `ImperfectThirdPlural`, `ImperfectImpersonal`, `PreteriteFirstSingular`, `PreteriteSecondSingular`, `PreteriteThirdSingular`, `PreteriteFirstPlural`, `PreteriteSecondPlural`, `PreteriteThirdPlural`, `PreteriteImpersonal`, `PerfectFirstSingular`, `PerfectSecondSingular`, `PerfectThirdSingular`, `PerfectFirstPlural`, `PerfectSecondPlural`, `PerfectThirdPlural`, `PerfectImpersonal`, `PluperfectFirstSingular`, `PluperfectSecondSingular`, `PluperfectThirdSingular`, `PluperfectFirstPlural`, `PluperfectSecondPlural`, `PluperfectThirdPlural`, `PluperfectImpersonal`, `SubjunctiveFirstSingular`, `SubjunctiveSecondSingular`, `SubjunctiveThirdSingular`, `SubjunctiveFirstPlural`, `SubjunctiveSecondPlural`, `SubjunctiveThirdPlural`, `SubjunctiveImpersonal`, `ImperfectSubjunctiveFirstSingular`, `ImperfectSubjunctiveSecondSingular`, `ImperfectSubjunctiveThirdSingular`, `ImperfectSubjunctiveFirstPlural`, `ImperfectSubjunctiveSecondPlural`, `ImperfectSubjunctiveThirdPlural`, `ImperfectSubjunctiveImpersonal`, `ImperativeSecondSingular`, `ImperativeThirdSingular`, `ImperativeFirstPlural`, `ImperativeSecondPlural`, `ImperativeThirdPlural`, `ImperativeImpersonal`, `VerbalNoun`, `Participle`, `MutatedSoft`, `MutatedNasal`, `MutatedAspirate`, `MutatedHProthesis`

Citation form: `VerbalNoun, PresentThirdSingular, PreteriteThirdSingular` (comma-separated; parts you give are output exactly and narrow the prediction).

## How it works

Each attested form is stored as an edit tree from the lemma. A lemma's edits across all cells make its inflection class. A suffix tree over the word, read from its end and pruned by a description-length test, predicts the class of any word. Known words the tree gets wrong are kept in a small lexicon. Confidences are smoothed counts; variant weights follow Wiktionary's listing order (1, 1/2, 1/3…, nonstandard forms a tenth of that).

## License

Code: MIT OR Apache-2.0. Data: derived from [Wiktionary](https://www.wiktionary.org/) (via [kaikki.org](https://kaikki.org/)), CC BY-SA 4.0; the model bytes in `src/model.bin` are shared under the same license.
