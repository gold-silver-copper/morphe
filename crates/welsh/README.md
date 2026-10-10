# welsh

Welsh inflection for Rust, learned from Wiktionary by [morphe](https://github.com/gold-silver-copper/morphe).
Inflects any word, known or made up. The `_scored` functions give every candidate with a confidence.

Generated from the Kaikki Welsh dump of 2026-10-03. Do not edit by hand: change the config in morphe and regenerate.

## Example

```rust
use welsh::*;
assert_eq!(noun("cath", Number::Plural), "cathod");
assert_eq!(noun_mutated("cath", Mutation::Soft), "gath");
assert_eq!(noun_mutated("cath", Mutation::Nasal), "nghath");
println!("{:?}", noun_gender("ysgol"));
assert_eq!(verb("canu", Tense::Preterite, PersonNumber::FirstSingular), "canais");
assert_eq!(adj("gwyn", Form::Feminine), "gwen");
assert_eq!(prep("ar", Person::FirstSingular), "arnaf");
```

Verb forms are the literary standard ones; `Tense::Future` is the colloquial
future (canith). Mutations are given for the base form only; mutate an
inflected form by passing it as the word. The tree reads a word's first two
letters, so mutation follows the initial consonant. Circumflexes are optional
on input.

## Coverage

| Part of speech | Entries | Cells | Edits | Classes | Tree nodes | Lexicon |
|---|---|---|---|---|---|---|
| noun | 15184 | 6 | 1797 | 3568 | 1960 | 7773 |
| adj | 3613 | 9 | 409 | 432 | 356 | 1954 |
| verb | 4055 | 61 | 5436 | 1746 | 569 | 1910 |
| prep | 25 | 7 | 68 | 16 | 4 | 18 |
| num | 222 | 4 | 35 | 22 | 14 | 26 |
| pron | 13 | 4 | 23 | 13 | 2 | 5 |

Model size: 883296 bytes.

**noun** cells: `Plural`, `Singulative`, `MutatedSoft`, `MutatedNasal`, `MutatedAspirate`, `MutatedHProthesis`

Citation form: `Lemma, Plural` (comma-separated; parts you give are output exactly and narrow the prediction).

**adj** cells: `Feminine`, `Plural`, `Equative`, `Comparative`, `Superlative`, `MutatedSoft`, `MutatedNasal`, `MutatedAspirate`, `MutatedHProthesis`

Citation form: `Lemma, Feminine, Plural` (comma-separated; parts you give are output exactly and narrow the prediction).

**verb** cells: `PresentFirstSingular`, `PresentSecondSingular`, `PresentThirdSingular`, `PresentFirstPlural`, `PresentSecondPlural`, `PresentThirdPlural`, `PresentImpersonal`, `FutureFirstSingular`, `FutureSecondSingular`, `FutureThirdSingular`, `FutureFirstPlural`, `FutureSecondPlural`, `FutureThirdPlural`, `FutureImpersonal`, `PreteriteFirstSingular`, `PreteriteSecondSingular`, `PreteriteThirdSingular`, `PreteriteFirstPlural`, `PreteriteSecondPlural`, `PreteriteThirdPlural`, `PreteriteImpersonal`, `ImperfectFirstSingular`, `ImperfectSecondSingular`, `ImperfectThirdSingular`, `ImperfectFirstPlural`, `ImperfectSecondPlural`, `ImperfectThirdPlural`, `ImperfectImpersonal`, `ConditionalFirstSingular`, `ConditionalSecondSingular`, `ConditionalThirdSingular`, `ConditionalFirstPlural`, `ConditionalSecondPlural`, `ConditionalThirdPlural`, `ConditionalImpersonal`, `PluperfectFirstSingular`, `PluperfectSecondSingular`, `PluperfectThirdSingular`, `PluperfectFirstPlural`, `PluperfectSecondPlural`, `PluperfectThirdPlural`, `PluperfectImpersonal`, `SubjunctiveFirstSingular`, `SubjunctiveSecondSingular`, `SubjunctiveThirdSingular`, `SubjunctiveFirstPlural`, `SubjunctiveSecondPlural`, `SubjunctiveThirdPlural`, `SubjunctiveImpersonal`, `ImperativeFirstSingular`, `ImperativeSecondSingular`, `ImperativeThirdSingular`, `ImperativeFirstPlural`, `ImperativeSecondPlural`, `ImperativeThirdPlural`, `ImperativeImpersonal`, `Participle`, `MutatedSoft`, `MutatedNasal`, `MutatedAspirate`, `MutatedHProthesis`

Citation form: `Lemma, PresentFirstSingular` (comma-separated; parts you give are output exactly and narrow the prediction).

**prep** cells: `FirstSingular`, `SecondSingular`, `ThirdSingularMasculine`, `ThirdSingularFeminine`, `FirstPlural`, `SecondPlural`, `ThirdPlural`

**num** cells: `MutatedSoft`, `MutatedNasal`, `MutatedAspirate`, `MutatedHProthesis`

**pron** cells: `MutatedSoft`, `MutatedNasal`, `MutatedAspirate`, `MutatedHProthesis`

## How it works

Each attested form is stored as an edit tree from the lemma. A lemma's edits across all cells make its inflection class. A suffix tree over the word, read from its end and pruned by a description-length test, predicts the class of any word. Known words the tree gets wrong are kept in a small lexicon. Confidences are smoothed counts; variant weights follow Wiktionary's listing order (1, 1/2, 1/3…, nonstandard forms a tenth of that).

## License

Code: MIT OR Apache-2.0. Data: derived from [Wiktionary](https://www.wiktionary.org/) (via [kaikki.org](https://kaikki.org/)), CC BY-SA 4.0; the model bytes in `src/model.bin` are shared under the same license.
