# old_french

OldFrench inflection for Rust, learned from Wiktionary by [morphe](https://github.com/gold-silver-copper/morphe).
Inflects any word, known or made up. The `_scored` functions give every candidate with a confidence.

Generated from the Kaikki OldFrench dump of 2026-10-03. Do not edit by hand: change the config in morphe and regenerate.

## Example

```rust
use old_french::*;
println!("{}", noun("chevalier", Case::Nominative, Number::Singular)); // chevaliers
println!("{:?}", noun_gender("livre"));
assert_eq!(verb("abandoner", Tense::Preterite, Person::Third, Number::Singular), "abandona");
println!("{}", adj("bon", Case::Oblique, Number::Singular, Gender::Feminine)); // bone
```

Nouns and adjectives have two cases: the nominative (subject) and the
oblique (everything else, and the dictionary form). Compound tenses are left
to the caller. Editorial accents (é, ë, ï) are optional on input.

## Coverage

| Part of speech | Entries | Cells | Edits | Classes | Tree nodes | Lexicon |
|---|---|---|---|---|---|---|
| noun | 4583 | 4 | 112 | 158 | 270 | 1138 |
| adj | 786 | 13 | 87 | 105 | 65 | 299 |
| verb | 1960 | 47 | 7517 | 873 | 256 | 433 |

Model size: 574751 bytes.

**noun** cells: `NominativeSingular`, `NominativePlural`, `ObliqueSingular`, `ObliquePlural`

Citation form: `Lemma, NominativeSingular, ObliquePlural` (comma-separated; parts you give are output exactly and narrow the prediction).

**adj** cells: `NominativeSingularMasculine`, `NominativeSingularFeminine`, `NominativeSingularNeuter`, `NominativePluralMasculine`, `NominativePluralFeminine`, `NominativePluralNeuter`, `ObliqueSingularMasculine`, `ObliqueSingularFeminine`, `ObliqueSingularNeuter`, `ObliquePluralMasculine`, `ObliquePluralFeminine`, `ObliquePluralNeuter`, `Comparative`

**verb** cells: `PresentFirstSingular`, `PresentFirstPlural`, `PresentSecondSingular`, `PresentSecondPlural`, `PresentThirdSingular`, `PresentThirdPlural`, `ImperfectFirstSingular`, `ImperfectFirstPlural`, `ImperfectSecondSingular`, `ImperfectSecondPlural`, `ImperfectThirdSingular`, `ImperfectThirdPlural`, `PreteriteFirstSingular`, `PreteriteFirstPlural`, `PreteriteSecondSingular`, `PreteriteSecondPlural`, `PreteriteThirdSingular`, `PreteriteThirdPlural`, `FutureFirstSingular`, `FutureFirstPlural`, `FutureSecondSingular`, `FutureSecondPlural`, `FutureThirdSingular`, `FutureThirdPlural`, `ConditionalFirstSingular`, `ConditionalFirstPlural`, `ConditionalSecondSingular`, `ConditionalSecondPlural`, `ConditionalThirdSingular`, `ConditionalThirdPlural`, `SubjunctiveFirstSingular`, `SubjunctiveFirstPlural`, `SubjunctiveSecondSingular`, `SubjunctiveSecondPlural`, `SubjunctiveThirdSingular`, `SubjunctiveThirdPlural`, `ImperfectSubjunctiveFirstSingular`, `ImperfectSubjunctiveFirstPlural`, `ImperfectSubjunctiveSecondSingular`, `ImperfectSubjunctiveSecondPlural`, `ImperfectSubjunctiveThirdSingular`, `ImperfectSubjunctiveThirdPlural`, `ImperativeSecondSingular`, `ImperativeFirstPlural`, `ImperativeSecondPlural`, `ParticiplePresent`, `ParticiplePast`

Citation form: `Lemma, PresentThirdSingular, PreteriteThirdSingular, ParticiplePast` (comma-separated; parts you give are output exactly and narrow the prediction).

## How it works

Each attested form is stored as an edit tree from the lemma. A lemma's edits across all cells make its inflection class. A suffix tree over the word, read from its end and pruned by a description-length test, predicts the class of any word. Known words the tree gets wrong are kept in a small lexicon. Confidences are smoothed counts; variant weights follow Wiktionary's listing order (1, 1/2, 1/3…, nonstandard forms a tenth of that).

## License

Code: MIT OR Apache-2.0. Data: derived from [Wiktionary](https://www.wiktionary.org/) (via [kaikki.org](https://kaikki.org/)), CC BY-SA 4.0; the model bytes in `src/model.bin` are shared under the same license.
