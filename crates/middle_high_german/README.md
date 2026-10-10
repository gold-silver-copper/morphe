# middle_high_german

MiddleHighGerman inflection for Rust, learned from Wiktionary by [morphe](https://github.com/gold-silver-copper/morphe).
Inflects any word, known or made up. The `_scored` functions give every candidate with a confidence.

Generated from the Kaikki MiddleHighGerman dump of 2026-10-03. Do not edit by hand: change the config in morphe and regenerate.

## Example

```rust
use middle_high_german::*;
assert_eq!(noun("tac", Case::Dative, Number::Plural), "tagen");
println!("{:?}", noun_gender("name"));
assert_eq!(verb("geben", Mood::Indicative, Tense::Past, Person::Third, Number::Singular), "gap");
println!("{}", verb("geben", Mood::Subjunctive, Tense::Past, Person::First, Number::Singular)); // gæbe
println!("{}", adj_weak("guot", Case::Nominative, Number::Singular, Gender::Masculine)); // guote
```

Spelling follows Wiktionary's normalized Middle High German. Length marks
and ë are optional on input, and z matches ȥ. Weak adjective forms are given
without their article. Compound tenses are left to the caller.

## Coverage

| Part of speech | Entries | Cells | Edits | Classes | Tree nodes | Lexicon |
|---|---|---|---|---|---|---|
| noun | 345 | 8 | 183 | 140 | 58 | 216 |
| adj | 38 | 50 | 142 | 36 | 9 | 19 |
| verb | 180 | 28 | 821 | 157 | 69 | 125 |

Model size: 153781 bytes.

**noun** cells: `NominativeSingular`, `NominativePlural`, `GenitiveSingular`, `GenitivePlural`, `DativeSingular`, `DativePlural`, `AccusativeSingular`, `AccusativePlural`

Citation form: `Lemma, GenitiveSingular, NominativePlural` (comma-separated; parts you give are output exactly and narrow the prediction).

**adj** cells: `NominativeSingularMasculine`, `NominativeSingularFeminine`, `NominativeSingularNeuter`, `NominativePluralMasculine`, `NominativePluralFeminine`, `NominativePluralNeuter`, `GenitiveSingularMasculine`, `GenitiveSingularFeminine`, `GenitiveSingularNeuter`, `GenitivePluralMasculine`, `GenitivePluralFeminine`, `GenitivePluralNeuter`, `DativeSingularMasculine`, `DativeSingularFeminine`, `DativeSingularNeuter`, `DativePluralMasculine`, `DativePluralFeminine`, `DativePluralNeuter`, `AccusativeSingularMasculine`, `AccusativeSingularFeminine`, `AccusativeSingularNeuter`, `AccusativePluralMasculine`, `AccusativePluralFeminine`, `AccusativePluralNeuter`, `WeakNominativeSingularMasculine`, `WeakNominativeSingularFeminine`, `WeakNominativeSingularNeuter`, `WeakNominativePluralMasculine`, `WeakNominativePluralFeminine`, `WeakNominativePluralNeuter`, `WeakGenitiveSingularMasculine`, `WeakGenitiveSingularFeminine`, `WeakGenitiveSingularNeuter`, `WeakGenitivePluralMasculine`, `WeakGenitivePluralFeminine`, `WeakGenitivePluralNeuter`, `WeakDativeSingularMasculine`, `WeakDativeSingularFeminine`, `WeakDativeSingularNeuter`, `WeakDativePluralMasculine`, `WeakDativePluralFeminine`, `WeakDativePluralNeuter`, `WeakAccusativeSingularMasculine`, `WeakAccusativeSingularFeminine`, `WeakAccusativeSingularNeuter`, `WeakAccusativePluralMasculine`, `WeakAccusativePluralFeminine`, `WeakAccusativePluralNeuter`, `Comparative`, `Superlative`

Citation form: `Lemma, Comparative, Superlative` (comma-separated; parts you give are output exactly and narrow the prediction).

**verb** cells: `IndicativePresentFirstSingular`, `IndicativePresentFirstPlural`, `IndicativePresentSecondSingular`, `IndicativePresentSecondPlural`, `IndicativePresentThirdSingular`, `IndicativePresentThirdPlural`, `IndicativePastFirstSingular`, `IndicativePastFirstPlural`, `IndicativePastSecondSingular`, `IndicativePastSecondPlural`, `IndicativePastThirdSingular`, `IndicativePastThirdPlural`, `SubjunctivePresentFirstSingular`, `SubjunctivePresentFirstPlural`, `SubjunctivePresentSecondSingular`, `SubjunctivePresentSecondPlural`, `SubjunctivePresentThirdSingular`, `SubjunctivePresentThirdPlural`, `SubjunctivePastFirstSingular`, `SubjunctivePastFirstPlural`, `SubjunctivePastSecondSingular`, `SubjunctivePastSecondPlural`, `SubjunctivePastThirdSingular`, `SubjunctivePastThirdPlural`, `ImperativeSingular`, `ImperativePlural`, `ParticiplePresent`, `ParticiplePast`

Citation form: `Lemma, IndicativePastThirdSingular, IndicativePastFirstPlural, ParticiplePast` (comma-separated; parts you give are output exactly and narrow the prediction).

## How it works

Each attested form is stored as an edit tree from the lemma. A lemma's edits across all cells make its inflection class. A suffix tree over the word, read from its end and pruned by a description-length test, predicts the class of any word. Known words the tree gets wrong are kept in a small lexicon. Confidences are smoothed counts; variant weights follow Wiktionary's listing order (1, 1/2, 1/3…, nonstandard forms a tenth of that).

## License

Code: MIT OR Apache-2.0. Data: derived from [Wiktionary](https://www.wiktionary.org/) (via [kaikki.org](https://kaikki.org/)), CC BY-SA 4.0; the model bytes in `src/model.bin` are shared under the same license.
