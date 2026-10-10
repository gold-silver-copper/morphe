# old_norse

OldNorse inflection for Rust, learned from Wiktionary by [morphe](https://github.com/gold-silver-copper/morphe).
Inflects any word, known or made up. The `_scored` functions give every candidate with a confidence.

Generated from the Kaikki OldNorse dump of 2026-10-03. Do not edit by hand: change the config in morphe and regenerate.

## Example

```rust
use old_norse::*;
assert_eq!(noun("hestr", Definiteness::Definite, Case::Dative, Number::Plural), "hestunum");
println!("{:?}", noun_gender("kona"));
assert_eq!(verb("taka", Mood::Indicative, Tense::Past, Person::Third, Number::Singular, Voice::Active), "tók");
assert_eq!(verb_infinitive("taka", Voice::Mediopassive), "takask");
assert_eq!(adj("blár", Case::Accusative, Number::Singular, Gender::Masculine), "bláan");
// Citations pin principal parts: lemma, genitive singular, plural.
println!("{}", noun("hestr, hests, hestar", Definiteness::Indefinite, Case::Dative, Number::Plural));
```

Spelling follows Wiktionary's normalized Old Norse. Length marks are
optional on input, and ö may be typed for ǫ.

## Coverage

| Part of speech | Entries | Cells | Edits | Classes | Tree nodes | Lexicon |
|---|---|---|---|---|---|---|
| noun | 2122 | 16 | 3441 | 823 | 366 | 1099 |
| adj | 163 | 50 | 234 | 99 | 37 | 86 |
| verb | 644 | 60 | 3596 | 474 | 101 | 428 |

Model size: 788241 bytes.

**noun** cells: `IndefiniteNominativeSingular`, `IndefiniteNominativePlural`, `IndefiniteAccusativeSingular`, `IndefiniteAccusativePlural`, `IndefiniteDativeSingular`, `IndefiniteDativePlural`, `IndefiniteGenitiveSingular`, `IndefiniteGenitivePlural`, `DefiniteNominativeSingular`, `DefiniteNominativePlural`, `DefiniteAccusativeSingular`, `DefiniteAccusativePlural`, `DefiniteDativeSingular`, `DefiniteDativePlural`, `DefiniteGenitiveSingular`, `DefiniteGenitivePlural`

Citation form: `Lemma, IndefiniteGenitiveSingular, IndefiniteNominativePlural` (comma-separated; parts you give are output exactly and narrow the prediction).

**adj** cells: `NominativeSingularMasculine`, `NominativeSingularFeminine`, `NominativeSingularNeuter`, `NominativePluralMasculine`, `NominativePluralFeminine`, `NominativePluralNeuter`, `AccusativeSingularMasculine`, `AccusativeSingularFeminine`, `AccusativeSingularNeuter`, `AccusativePluralMasculine`, `AccusativePluralFeminine`, `AccusativePluralNeuter`, `DativeSingularMasculine`, `DativeSingularFeminine`, `DativeSingularNeuter`, `DativePluralMasculine`, `DativePluralFeminine`, `DativePluralNeuter`, `GenitiveSingularMasculine`, `GenitiveSingularFeminine`, `GenitiveSingularNeuter`, `GenitivePluralMasculine`, `GenitivePluralFeminine`, `GenitivePluralNeuter`, `WeakNominativeSingularMasculine`, `WeakNominativeSingularFeminine`, `WeakNominativeSingularNeuter`, `WeakNominativePluralMasculine`, `WeakNominativePluralFeminine`, `WeakNominativePluralNeuter`, `WeakAccusativeSingularMasculine`, `WeakAccusativeSingularFeminine`, `WeakAccusativeSingularNeuter`, `WeakAccusativePluralMasculine`, `WeakAccusativePluralFeminine`, `WeakAccusativePluralNeuter`, `WeakDativeSingularMasculine`, `WeakDativeSingularFeminine`, `WeakDativeSingularNeuter`, `WeakDativePluralMasculine`, `WeakDativePluralFeminine`, `WeakDativePluralNeuter`, `WeakGenitiveSingularMasculine`, `WeakGenitiveSingularFeminine`, `WeakGenitiveSingularNeuter`, `WeakGenitivePluralMasculine`, `WeakGenitivePluralFeminine`, `WeakGenitivePluralNeuter`, `Comparative`, `Superlative`

Citation form: `Lemma, Comparative, Superlative` (comma-separated; parts you give are output exactly and narrow the prediction).

**verb** cells: `InfinitiveActive`, `InfinitiveMediopassive`, `IndicativePresentFirstSingularActive`, `IndicativePresentFirstSingularMediopassive`, `IndicativePresentFirstPluralActive`, `IndicativePresentFirstPluralMediopassive`, `IndicativePresentSecondSingularActive`, `IndicativePresentSecondSingularMediopassive`, `IndicativePresentSecondPluralActive`, `IndicativePresentSecondPluralMediopassive`, `IndicativePresentThirdSingularActive`, `IndicativePresentThirdSingularMediopassive`, `IndicativePresentThirdPluralActive`, `IndicativePresentThirdPluralMediopassive`, `IndicativePastFirstSingularActive`, `IndicativePastFirstSingularMediopassive`, `IndicativePastFirstPluralActive`, `IndicativePastFirstPluralMediopassive`, `IndicativePastSecondSingularActive`, `IndicativePastSecondSingularMediopassive`, `IndicativePastSecondPluralActive`, `IndicativePastSecondPluralMediopassive`, `IndicativePastThirdSingularActive`, `IndicativePastThirdSingularMediopassive`, `IndicativePastThirdPluralActive`, `IndicativePastThirdPluralMediopassive`, `SubjunctivePresentFirstSingularActive`, `SubjunctivePresentFirstSingularMediopassive`, `SubjunctivePresentFirstPluralActive`, `SubjunctivePresentFirstPluralMediopassive`, `SubjunctivePresentSecondSingularActive`, `SubjunctivePresentSecondSingularMediopassive`, `SubjunctivePresentSecondPluralActive`, `SubjunctivePresentSecondPluralMediopassive`, `SubjunctivePresentThirdSingularActive`, `SubjunctivePresentThirdSingularMediopassive`, `SubjunctivePresentThirdPluralActive`, `SubjunctivePresentThirdPluralMediopassive`, `SubjunctivePastFirstSingularActive`, `SubjunctivePastFirstSingularMediopassive`, `SubjunctivePastFirstPluralActive`, `SubjunctivePastFirstPluralMediopassive`, `SubjunctivePastSecondSingularActive`, `SubjunctivePastSecondSingularMediopassive`, `SubjunctivePastSecondPluralActive`, `SubjunctivePastSecondPluralMediopassive`, `SubjunctivePastThirdSingularActive`, `SubjunctivePastThirdSingularMediopassive`, `SubjunctivePastThirdPluralActive`, `SubjunctivePastThirdPluralMediopassive`, `ImperativeSecondSingularActive`, `ImperativeSecondSingularMediopassive`, `ImperativeFirstPluralActive`, `ImperativeFirstPluralMediopassive`, `ImperativeSecondPluralActive`, `ImperativeSecondPluralMediopassive`, `ParticiplePresentActive`, `ParticiplePresentMediopassive`, `ParticiplePastActive`, `ParticiplePastMediopassive`

Citation form: `InfinitiveActive, IndicativePresentThirdSingularActive, IndicativePastThirdSingularActive, ParticiplePastActive` (comma-separated; parts you give are output exactly and narrow the prediction).

## How it works

Each attested form is stored as an edit tree from the lemma. A lemma's edits across all cells make its inflection class. A suffix tree over the word, read from its end and pruned by a description-length test, predicts the class of any word. Known words the tree gets wrong are kept in a small lexicon. Confidences are smoothed counts; variant weights follow Wiktionary's listing order (1, 1/2, 1/3…, nonstandard forms a tenth of that).

## License

Code: MIT OR Apache-2.0. Data: derived from [Wiktionary](https://www.wiktionary.org/) (via [kaikki.org](https://kaikki.org/)), CC BY-SA 4.0; the model bytes in `src/model.bin` are shared under the same license.
