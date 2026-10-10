# low_german

LowGerman inflection for Rust, learned from Wiktionary by [morphe](https://github.com/gold-silver-copper/morphe).
Inflects any word, known or made up. The `_scored` functions give every candidate with a confidence.

Generated from the Kaikki LowGerman dump of 2026-10-03. Do not edit by hand: change the config in morphe and regenerate.

## Example

```rust
use low_german::*;
println!("{}", noun_plural("Kind"));
println!("{}", verb_preterite("fallen", PersonNumber::ThirdSingular)); // full
println!("{}", verb_present("afdingen", PersonNumber::ThirdSingular)); // dingt af
println!("{}", verb_subordinate_present("afdingen", PersonNumber::ThirdSingular)); // afdingt
println!("{:?}", verb_auxiliary("fallen")); // Wesen
println!("{}", adj("free", Degree::Positive, Declension::Weak, Case::Oblique, GenderNumber::Masculine)); // fre'en
```

Adjective forms are given without the article or predicate the tables show
them with ("den fre'en" → "fre'en", "he is de Freeste" → "freeste").
Output is lowercase. Umlauts are optional on input, and ß matches ss.

## Coverage

| Part of speech | Entries | Cells | Edits | Classes | Tree nodes | Lexicon |
|---|---|---|---|---|---|---|
| noun | 661 | 2 | 170 | 178 | 149 | 296 |
| adj | 93 | 98 | 311 | 96 | 25 | 62 |
| verb | 319 | 21 | 801 | 291 | 81 | 200 |

Model size: 87121 bytes.

**noun** cells: `Plural`, `Diminutive`

Citation form: `Lemma, Plural` (comma-separated; parts you give are output exactly and narrow the prediction).

**adj** cells: `PositiveStrongNominativeMasculine`, `PositiveStrongNominativeFeminine`, `PositiveStrongNominativeNeuter`, `PositiveStrongNominativePlural`, `PositiveStrongObliqueMasculine`, `PositiveStrongObliqueFeminine`, `PositiveStrongObliqueNeuter`, `PositiveStrongObliquePlural`, `PositiveWeakNominativeMasculine`, `PositiveWeakNominativeFeminine`, `PositiveWeakNominativeNeuter`, `PositiveWeakNominativePlural`, `PositiveWeakObliqueMasculine`, `PositiveWeakObliqueFeminine`, `PositiveWeakObliqueNeuter`, `PositiveWeakObliquePlural`, `PositiveMixedNominativeMasculine`, `PositiveMixedNominativeFeminine`, `PositiveMixedNominativeNeuter`, `PositiveMixedNominativePlural`, `PositiveMixedObliqueMasculine`, `PositiveMixedObliqueFeminine`, `PositiveMixedObliqueNeuter`, `PositiveMixedObliquePlural`, `ComparativeStrongNominativeMasculine`, `ComparativeStrongNominativeFeminine`, `ComparativeStrongNominativeNeuter`, `ComparativeStrongNominativePlural`, `ComparativeStrongObliqueMasculine`, `ComparativeStrongObliqueFeminine`, `ComparativeStrongObliqueNeuter`, `ComparativeStrongObliquePlural`, `ComparativeWeakNominativeMasculine`, `ComparativeWeakNominativeFeminine`, `ComparativeWeakNominativeNeuter`, `ComparativeWeakNominativePlural`, `ComparativeWeakObliqueMasculine`, `ComparativeWeakObliqueFeminine`, `ComparativeWeakObliqueNeuter`, `ComparativeWeakObliquePlural`, `ComparativeMixedNominativeMasculine`, `ComparativeMixedNominativeFeminine`, `ComparativeMixedNominativeNeuter`, `ComparativeMixedNominativePlural`, `ComparativeMixedObliqueMasculine`, `ComparativeMixedObliqueFeminine`, `ComparativeMixedObliqueNeuter`, `ComparativeMixedObliquePlural`, `SuperlativeStrongNominativeMasculine`, `SuperlativeStrongNominativeFeminine`, `SuperlativeStrongNominativeNeuter`, `SuperlativeStrongNominativePlural`, `SuperlativeStrongObliqueMasculine`, `SuperlativeStrongObliqueFeminine`, `SuperlativeStrongObliqueNeuter`, `SuperlativeStrongObliquePlural`, `SuperlativeWeakNominativeMasculine`, `SuperlativeWeakNominativeFeminine`, `SuperlativeWeakNominativeNeuter`, `SuperlativeWeakNominativePlural`, `SuperlativeWeakObliqueMasculine`, `SuperlativeWeakObliqueFeminine`, `SuperlativeWeakObliqueNeuter`, `SuperlativeWeakObliquePlural`, `SuperlativeMixedNominativeMasculine`, `SuperlativeMixedNominativeFeminine`, `SuperlativeMixedNominativeNeuter`, `SuperlativeMixedNominativePlural`, `SuperlativeMixedObliqueMasculine`, `SuperlativeMixedObliqueFeminine`, `SuperlativeMixedObliqueNeuter`, `SuperlativeMixedObliquePlural`, `PredicativePositiveMasculine`, `PredicativePositiveFeminine`, `PredicativePositiveNeuter`, `PredicativePositivePlural`, `PredicativeComparativeMasculine`, `PredicativeComparativeFeminine`, `PredicativeComparativeNeuter`, `PredicativeComparativePlural`, `PredicativeSuperlativeMasculine`, `PredicativeSuperlativeFeminine`, `PredicativeSuperlativeNeuter`, `PredicativeSuperlativePlural`, `PartitivePositiveMasculine`, `PartitivePositiveFeminine`, `PartitivePositiveNeuter`, `PartitivePositivePlural`, `PartitiveComparativeMasculine`, `PartitiveComparativeFeminine`, `PartitiveComparativeNeuter`, `PartitiveComparativePlural`, `PartitiveSuperlativeMasculine`, `PartitiveSuperlativeFeminine`, `PartitiveSuperlativeNeuter`, `PartitiveSuperlativePlural`, `SimpleComparative`, `SimpleSuperlative`

Citation form: `Lemma, SimpleComparative, SimpleSuperlative` (comma-separated; parts you give are output exactly and narrow the prediction).

**verb** cells: `Infinitive`, `ParticiplePresent`, `ParticiplePast`, `ImperativeSingular`, `ImperativePlural`, `PresentFirstSingular`, `PresentSecondSingular`, `PresentThirdSingular`, `PresentPlural`, `PreteriteFirstSingular`, `PreteriteSecondSingular`, `PreteriteThirdSingular`, `PreteritePlural`, `SubordinatePresentFirstSingular`, `SubordinatePresentSecondSingular`, `SubordinatePresentThirdSingular`, `SubordinatePresentPlural`, `SubordinatePreteriteFirstSingular`, `SubordinatePreteriteSecondSingular`, `SubordinatePreteriteThirdSingular`, `SubordinatePreteritePlural`

Citation form: `Infinitive, PreteriteThirdSingular, ParticiplePast` (comma-separated; parts you give are output exactly and narrow the prediction).

## How it works

Each attested form is stored as an edit tree from the lemma. A lemma's edits across all cells make its inflection class. A suffix tree over the word, read from its end and pruned by a description-length test, predicts the class of any word. Known words the tree gets wrong are kept in a small lexicon. Confidences are smoothed counts; variant weights follow Wiktionary's listing order (1, 1/2, 1/3…, nonstandard forms a tenth of that).

## License

Code: MIT OR Apache-2.0. Data: derived from [Wiktionary](https://www.wiktionary.org/) (via [kaikki.org](https://kaikki.org/)), CC BY-SA 4.0; the model bytes in `src/model.bin` are shared under the same license.
