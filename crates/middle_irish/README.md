# middle_irish

MiddleIrish inflection for Rust, learned from Wiktionary by [morphe](https://github.com/gold-silver-copper/morphe).
Inflects any word, known or made up. The `_scored` functions give every candidate with a confidence.

Generated from the Kaikki MiddleIrish dump of 2026-10-03. Do not edit by hand: change the config in morphe and regenerate.

## Example

```rust
use middle_irish::*;
println!("{}", noun("mac", Case::Nominative, Number::Plural)); // mic
println!("{}", noun_mutated("ab", Mutation::Nasalization)); // n-ab
println!("{}", verb_verbal_noun("beirid")); // breth
```

Most words have only their headword forms (genitive, plural) and initial
mutations; Wiktionary has full declension tables for a few. Length marks are
optional on input.

## Coverage

| Part of speech | Entries | Cells | Edits | Classes | Tree nodes | Lexicon |
|---|---|---|---|---|---|---|
| noun | 541 | 17 | 112 | 144 | 103 | 336 |
| adj | 77 | 35 | 14 | 14 | 13 | 30 |
| verb | 55 | 4 | 39 | 35 | 13 | 42 |

Model size: 13521 bytes.

**noun** cells: `NominativeSingular`, `NominativeDual`, `NominativePlural`, `VocativeSingular`, `VocativeDual`, `VocativePlural`, `AccusativeSingular`, `AccusativeDual`, `AccusativePlural`, `GenitiveSingular`, `GenitiveDual`, `GenitivePlural`, `DativeSingular`, `DativeDual`, `DativePlural`, `MutatedLenition`, `MutatedNasalization`

Citation form: `NominativeSingular, GenitiveSingular, NominativePlural` (comma-separated; parts you give are output exactly and narrow the prediction).

**adj** cells: `NominativeSingularMasculine`, `NominativeSingularFeminine`, `NominativeSingularNeuter`, `NominativePluralMasculine`, `NominativePluralFeminine`, `NominativePluralNeuter`, `VocativeSingularMasculine`, `VocativeSingularFeminine`, `VocativeSingularNeuter`, `VocativePluralMasculine`, `VocativePluralFeminine`, `VocativePluralNeuter`, `AccusativeSingularMasculine`, `AccusativeSingularFeminine`, `AccusativeSingularNeuter`, `AccusativePluralMasculine`, `AccusativePluralFeminine`, `AccusativePluralNeuter`, `GenitiveSingularMasculine`, `GenitiveSingularFeminine`, `GenitiveSingularNeuter`, `GenitivePluralMasculine`, `GenitivePluralFeminine`, `GenitivePluralNeuter`, `DativeSingularMasculine`, `DativeSingularFeminine`, `DativeSingularNeuter`, `DativePluralMasculine`, `DativePluralFeminine`, `DativePluralNeuter`, `DegreeEquative`, `DegreeComparative`, `DegreeSuperlative`, `MutatedLenition`, `MutatedNasalization`

**verb** cells: `VerbalNoun`, `Conjunct`, `MutatedLenition`, `MutatedNasalization`

Citation form: `Lemma, VerbalNoun` (comma-separated; parts you give are output exactly and narrow the prediction).

## How it works

Each attested form is stored as an edit tree from the lemma. A lemma's edits across all cells make its inflection class. A suffix tree over the word, read from its end and pruned by a description-length test, predicts the class of any word. Known words the tree gets wrong are kept in a small lexicon. Confidences are smoothed counts; variant weights follow Wiktionary's listing order (1, 1/2, 1/3…, nonstandard forms a tenth of that).

## License

Code: MIT OR Apache-2.0. Data: derived from [Wiktionary](https://www.wiktionary.org/) (via [kaikki.org](https://kaikki.org/)), CC BY-SA 4.0; the model bytes in `src/model.bin` are shared under the same license.
