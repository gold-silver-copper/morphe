# german

German inflection for Rust, learned from Wiktionary by [morphe](https://github.com/gold-silver-copper/morphe).
Inflects any word, known or made up. The `_scored` functions give every candidate with a confidence.

Generated from the Kaikki German dump of 2026-10-03. Do not edit by hand: change the config in morphe and regenerate.

## Example

```rust
use german::*;
assert_eq!(noun("Haus", Case::Dative, Number::Plural), "Häusern");
println!("{:?}", noun_gender("Haus"));
assert_eq!(verb_present("sehen", Person::Third, Number::Singular), "sieht");
// Separable verbs split in main clauses and join in subordinate ones:
assert_eq!(verb_present("ansehen", Person::Third, Number::Singular), "sieht an");
assert_eq!(verb_subordinate_present("ansehen", Person::Third, Number::Singular), "ansieht");
assert_eq!(adj_weak("alt", Degree::Comparative, Case::Dative, GenderNumber::Plural), "älteren");
assert_eq!(adj_predicative("gut", Degree::Superlative), "am besten");
// Citations pin principal parts: infinitive, present, preterite, participle.
assert_eq!(verb_preterite("backen, bäckt, buk, gebacken", Person::Third, Number::Singular), "buk");
```

Weak and mixed adjective forms are given without their article ("freien",
not "den freien"). Compound tenses (perfect, future) are left to the caller:
auxiliary plus `verb_participle_past`. Personal pronouns are not covered.

## Coverage

| Part of speech | Entries | Cells | Edits | Classes | Tree nodes | Lexicon |
|---|---|---|---|---|---|---|
| noun | 63142 | 9 | 1127 | 1948 | 4326 | 14277 |
| adj | 15385 | 146 | 565 | 127 | 690 | 4894 |
| verb | 10632 | 54 | 15242 | 5638 | 1179 | 5581 |
| pron | 83 | 16 | 110 | 89 | 44 | 44 |
| det | 20 | 16 | 35 | 13 | 5 | 9 |
| article | 12 | 16 | 50 | 13 | 2 | 11 |
| num | 19 | 8 | 3 | 3 | 4 | 1 |

Model size: 4071937 bytes.

**noun** cells: `NominativeSingular`, `NominativePlural`, `GenitiveSingular`, `GenitivePlural`, `DativeSingular`, `DativePlural`, `AccusativeSingular`, `AccusativePlural`, `Diminutive`

Citation form: `NominativeSingular, GenitiveSingular, NominativePlural` (comma-separated; parts you give are output exactly and narrow the prediction).

**adj** cells: `StrongPositiveNominativeMasculine`, `StrongPositiveNominativeFeminine`, `StrongPositiveNominativeNeuter`, `StrongPositiveNominativePlural`, `StrongPositiveGenitiveMasculine`, `StrongPositiveGenitiveFeminine`, `StrongPositiveGenitiveNeuter`, `StrongPositiveGenitivePlural`, `StrongPositiveDativeMasculine`, `StrongPositiveDativeFeminine`, `StrongPositiveDativeNeuter`, `StrongPositiveDativePlural`, `StrongPositiveAccusativeMasculine`, `StrongPositiveAccusativeFeminine`, `StrongPositiveAccusativeNeuter`, `StrongPositiveAccusativePlural`, `StrongComparativeNominativeMasculine`, `StrongComparativeNominativeFeminine`, `StrongComparativeNominativeNeuter`, `StrongComparativeNominativePlural`, `StrongComparativeGenitiveMasculine`, `StrongComparativeGenitiveFeminine`, `StrongComparativeGenitiveNeuter`, `StrongComparativeGenitivePlural`, `StrongComparativeDativeMasculine`, `StrongComparativeDativeFeminine`, `StrongComparativeDativeNeuter`, `StrongComparativeDativePlural`, `StrongComparativeAccusativeMasculine`, `StrongComparativeAccusativeFeminine`, `StrongComparativeAccusativeNeuter`, `StrongComparativeAccusativePlural`, `StrongSuperlativeNominativeMasculine`, `StrongSuperlativeNominativeFeminine`, `StrongSuperlativeNominativeNeuter`, `StrongSuperlativeNominativePlural`, `StrongSuperlativeGenitiveMasculine`, `StrongSuperlativeGenitiveFeminine`, `StrongSuperlativeGenitiveNeuter`, `StrongSuperlativeGenitivePlural`, `StrongSuperlativeDativeMasculine`, `StrongSuperlativeDativeFeminine`, `StrongSuperlativeDativeNeuter`, `StrongSuperlativeDativePlural`, `StrongSuperlativeAccusativeMasculine`, `StrongSuperlativeAccusativeFeminine`, `StrongSuperlativeAccusativeNeuter`, `StrongSuperlativeAccusativePlural`, `WeakPositiveNominativeMasculine`, `WeakPositiveNominativeFeminine`, `WeakPositiveNominativeNeuter`, `WeakPositiveNominativePlural`, `WeakPositiveGenitiveMasculine`, `WeakPositiveGenitiveFeminine`, `WeakPositiveGenitiveNeuter`, `WeakPositiveGenitivePlural`, `WeakPositiveDativeMasculine`, `WeakPositiveDativeFeminine`, `WeakPositiveDativeNeuter`, `WeakPositiveDativePlural`, `WeakPositiveAccusativeMasculine`, `WeakPositiveAccusativeFeminine`, `WeakPositiveAccusativeNeuter`, `WeakPositiveAccusativePlural`, `WeakComparativeNominativeMasculine`, `WeakComparativeNominativeFeminine`, `WeakComparativeNominativeNeuter`, `WeakComparativeNominativePlural`, `WeakComparativeGenitiveMasculine`, `WeakComparativeGenitiveFeminine`, `WeakComparativeGenitiveNeuter`, `WeakComparativeGenitivePlural`, `WeakComparativeDativeMasculine`, `WeakComparativeDativeFeminine`, `WeakComparativeDativeNeuter`, `WeakComparativeDativePlural`, `WeakComparativeAccusativeMasculine`, `WeakComparativeAccusativeFeminine`, `WeakComparativeAccusativeNeuter`, `WeakComparativeAccusativePlural`, `WeakSuperlativeNominativeMasculine`, `WeakSuperlativeNominativeFeminine`, `WeakSuperlativeNominativeNeuter`, `WeakSuperlativeNominativePlural`, `WeakSuperlativeGenitiveMasculine`, `WeakSuperlativeGenitiveFeminine`, `WeakSuperlativeGenitiveNeuter`, `WeakSuperlativeGenitivePlural`, `WeakSuperlativeDativeMasculine`, `WeakSuperlativeDativeFeminine`, `WeakSuperlativeDativeNeuter`, `WeakSuperlativeDativePlural`, `WeakSuperlativeAccusativeMasculine`, `WeakSuperlativeAccusativeFeminine`, `WeakSuperlativeAccusativeNeuter`, `WeakSuperlativeAccusativePlural`, `MixedPositiveNominativeMasculine`, `MixedPositiveNominativeFeminine`, `MixedPositiveNominativeNeuter`, `MixedPositiveNominativePlural`, `MixedPositiveGenitiveMasculine`, `MixedPositiveGenitiveFeminine`, `MixedPositiveGenitiveNeuter`, `MixedPositiveGenitivePlural`, `MixedPositiveDativeMasculine`, `MixedPositiveDativeFeminine`, `MixedPositiveDativeNeuter`, `MixedPositiveDativePlural`, `MixedPositiveAccusativeMasculine`, `MixedPositiveAccusativeFeminine`, `MixedPositiveAccusativeNeuter`, `MixedPositiveAccusativePlural`, `MixedComparativeNominativeMasculine`, `MixedComparativeNominativeFeminine`, `MixedComparativeNominativeNeuter`, `MixedComparativeNominativePlural`, `MixedComparativeGenitiveMasculine`, `MixedComparativeGenitiveFeminine`, `MixedComparativeGenitiveNeuter`, `MixedComparativeGenitivePlural`, `MixedComparativeDativeMasculine`, `MixedComparativeDativeFeminine`, `MixedComparativeDativeNeuter`, `MixedComparativeDativePlural`, `MixedComparativeAccusativeMasculine`, `MixedComparativeAccusativeFeminine`, `MixedComparativeAccusativeNeuter`, `MixedComparativeAccusativePlural`, `MixedSuperlativeNominativeMasculine`, `MixedSuperlativeNominativeFeminine`, `MixedSuperlativeNominativeNeuter`, `MixedSuperlativeNominativePlural`, `MixedSuperlativeGenitiveMasculine`, `MixedSuperlativeGenitiveFeminine`, `MixedSuperlativeGenitiveNeuter`, `MixedSuperlativeGenitivePlural`, `MixedSuperlativeDativeMasculine`, `MixedSuperlativeDativeFeminine`, `MixedSuperlativeDativeNeuter`, `MixedSuperlativeDativePlural`, `MixedSuperlativeAccusativeMasculine`, `MixedSuperlativeAccusativeFeminine`, `MixedSuperlativeAccusativeNeuter`, `MixedSuperlativeAccusativePlural`, `PredicativeComparative`, `PredicativeSuperlative`

Citation form: `Lemma, PredicativeComparative, PredicativeSuperlative` (comma-separated; parts you give are output exactly and narrow the prediction).

**verb** cells: `Infinitive`, `ZuInfinitive`, `ParticiplePresent`, `ParticiplePast`, `ImperativeSingular`, `ImperativePlural`, `PresentFirstSingular`, `PresentFirstPlural`, `PresentSecondSingular`, `PresentSecondPlural`, `PresentThirdSingular`, `PresentThirdPlural`, `PreteriteFirstSingular`, `PreteriteFirstPlural`, `PreteriteSecondSingular`, `PreteriteSecondPlural`, `PreteriteThirdSingular`, `PreteriteThirdPlural`, `SubjunctiveFirstSingular`, `SubjunctiveFirstPlural`, `SubjunctiveSecondSingular`, `SubjunctiveSecondPlural`, `SubjunctiveThirdSingular`, `SubjunctiveThirdPlural`, `PastSubjunctiveFirstSingular`, `PastSubjunctiveFirstPlural`, `PastSubjunctiveSecondSingular`, `PastSubjunctiveSecondPlural`, `PastSubjunctiveThirdSingular`, `PastSubjunctiveThirdPlural`, `SubordinatePresentFirstSingular`, `SubordinatePresentFirstPlural`, `SubordinatePresentSecondSingular`, `SubordinatePresentSecondPlural`, `SubordinatePresentThirdSingular`, `SubordinatePresentThirdPlural`, `SubordinatePreteriteFirstSingular`, `SubordinatePreteriteFirstPlural`, `SubordinatePreteriteSecondSingular`, `SubordinatePreteriteSecondPlural`, `SubordinatePreteriteThirdSingular`, `SubordinatePreteriteThirdPlural`, `SubordinateSubjunctiveFirstSingular`, `SubordinateSubjunctiveFirstPlural`, `SubordinateSubjunctiveSecondSingular`, `SubordinateSubjunctiveSecondPlural`, `SubordinateSubjunctiveThirdSingular`, `SubordinateSubjunctiveThirdPlural`, `SubordinatePastSubjunctiveFirstSingular`, `SubordinatePastSubjunctiveFirstPlural`, `SubordinatePastSubjunctiveSecondSingular`, `SubordinatePastSubjunctiveSecondPlural`, `SubordinatePastSubjunctiveThirdSingular`, `SubordinatePastSubjunctiveThirdPlural`

Citation form: `Infinitive, PresentThirdSingular, PreteriteThirdSingular, ParticiplePast` (comma-separated; parts you give are output exactly and narrow the prediction).

**pron** cells: `NominativeMasculine`, `NominativeFeminine`, `NominativeNeuter`, `NominativePlural`, `GenitiveMasculine`, `GenitiveFeminine`, `GenitiveNeuter`, `GenitivePlural`, `DativeMasculine`, `DativeFeminine`, `DativeNeuter`, `DativePlural`, `AccusativeMasculine`, `AccusativeFeminine`, `AccusativeNeuter`, `AccusativePlural`

**det** cells: `NominativeMasculine`, `NominativeFeminine`, `NominativeNeuter`, `NominativePlural`, `GenitiveMasculine`, `GenitiveFeminine`, `GenitiveNeuter`, `GenitivePlural`, `DativeMasculine`, `DativeFeminine`, `DativeNeuter`, `DativePlural`, `AccusativeMasculine`, `AccusativeFeminine`, `AccusativeNeuter`, `AccusativePlural`

**article** cells: `NominativeMasculine`, `NominativeFeminine`, `NominativeNeuter`, `NominativePlural`, `GenitiveMasculine`, `GenitiveFeminine`, `GenitiveNeuter`, `GenitivePlural`, `DativeMasculine`, `DativeFeminine`, `DativeNeuter`, `DativePlural`, `AccusativeMasculine`, `AccusativeFeminine`, `AccusativeNeuter`, `AccusativePlural`

**num** cells: `NominativeSingular`, `NominativePlural`, `GenitiveSingular`, `GenitivePlural`, `DativeSingular`, `DativePlural`, `AccusativeSingular`, `AccusativePlural`

## How it works

Each attested form is stored as an edit tree from the lemma. A lemma's edits across all cells make its inflection class. A suffix tree over the word, read from its end and pruned by a description-length test, predicts the class of any word. Known words the tree gets wrong are kept in a small lexicon. Confidences are smoothed counts; variant weights follow Wiktionary's listing order (1, 1/2, 1/3…, nonstandard forms a tenth of that).

## License

Code: MIT OR Apache-2.0. Data: derived from [Wiktionary](https://www.wiktionary.org/) (via [kaikki.org](https://kaikki.org/)), CC BY-SA 4.0; the model bytes in `src/model.bin` are shared under the same license.
