# cornish

Cornish inflection for Rust, learned from Wiktionary by [morphe](https://github.com/gold-silver-copper/morphe).
Inflects any word, known or made up. The `_scored` functions give every candidate with a confidence.

Generated from the Kaikki Cornish dump of 2026-10-03. Do not edit by hand: change the config in morphe and regenerate.

## Example

```rust
use cornish::*;
println!("{}", noun_plural("on")); // eyn
println!("{}", noun_mutated("pies", Mutation::Soft)); // bies
println!("{}", verb("kara", Tense::Preterite, Person::ThirdSingular)); // karas
println!("{}", verb_participle_past("kara")); // kerys
println!("{}", adj("moon", Degree::Comparative)); // monna
```

Spellings follow the Standard Written Form, as in Wiktionary. The present
participle is given without "ow" and the superlative without "an".

## Coverage

| Part of speech | Entries | Cells | Edits | Classes | Tree nodes | Lexicon |
|---|---|---|---|---|---|---|
| noun | 3622 | 7 | 647 | 1221 | 544 | 2235 |
| adj | 424 | 6 | 70 | 115 | 27 | 88 |
| verb | 1088 | 53 | 3048 | 660 | 218 | 590 |

Model size: 342299 bytes.

**noun** cells: `Plural`, `Singulative`, `Dual`, `MutatedSoft`, `MutatedAspirate`, `MutatedHard`, `MutatedMixed`

Citation form: `Lemma, Plural` (comma-separated; parts you give are output exactly and narrow the prediction).

**adj** cells: `Comparative`, `Superlative`, `MutatedSoft`, `MutatedAspirate`, `MutatedHard`, `MutatedMixed`

Citation form: `Lemma, Comparative` (comma-separated; parts you give are output exactly and narrow the prediction).

**verb** cells: `PresentFirstSingular`, `PresentSecondSingular`, `PresentThirdSingular`, `PresentFirstPlural`, `PresentSecondPlural`, `PresentThirdPlural`, `PresentImpersonal`, `PreteriteFirstSingular`, `PreteriteSecondSingular`, `PreteriteThirdSingular`, `PreteriteFirstPlural`, `PreteriteSecondPlural`, `PreteriteThirdPlural`, `PreteriteImpersonal`, `ImperfectFirstSingular`, `ImperfectSecondSingular`, `ImperfectThirdSingular`, `ImperfectFirstPlural`, `ImperfectSecondPlural`, `ImperfectThirdPlural`, `ImperfectImpersonal`, `PluperfectFirstSingular`, `PluperfectSecondSingular`, `PluperfectThirdSingular`, `PluperfectFirstPlural`, `PluperfectSecondPlural`, `PluperfectThirdPlural`, `PluperfectImpersonal`, `SubjunctiveFirstSingular`, `SubjunctiveSecondSingular`, `SubjunctiveThirdSingular`, `SubjunctiveFirstPlural`, `SubjunctiveSecondPlural`, `SubjunctiveThirdPlural`, `SubjunctiveImpersonal`, `ImperfectSubjunctiveFirstSingular`, `ImperfectSubjunctiveSecondSingular`, `ImperfectSubjunctiveThirdSingular`, `ImperfectSubjunctiveFirstPlural`, `ImperfectSubjunctiveSecondPlural`, `ImperfectSubjunctiveThirdPlural`, `ImperfectSubjunctiveImpersonal`, `ImperativeSecondSingular`, `ImperativeThirdSingular`, `ImperativeFirstPlural`, `ImperativeSecondPlural`, `ImperativeThirdPlural`, `ParticiplePresent`, `ParticiplePast`, `MutatedSoft`, `MutatedAspirate`, `MutatedHard`, `MutatedMixed`

Citation form: `Lemma, PresentThirdSingular, ParticiplePast` (comma-separated; parts you give are output exactly and narrow the prediction).

## How it works

Each attested form is stored as an edit tree from the lemma. A lemma's edits across all cells make its inflection class. A suffix tree over the word, read from its end and pruned by a description-length test, predicts the class of any word. Known words the tree gets wrong are kept in a small lexicon. Confidences are smoothed counts; variant weights follow Wiktionary's listing order (1, 1/2, 1/3…, nonstandard forms a tenth of that).

## License

Code: MIT OR Apache-2.0. Data: derived from [Wiktionary](https://www.wiktionary.org/) (via [kaikki.org](https://kaikki.org/)), CC BY-SA 4.0; the model bytes in `src/model.bin` are shared under the same license.
