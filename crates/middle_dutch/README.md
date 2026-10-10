# middle_dutch

MiddleDutch inflection for Rust, learned from Wiktionary by [morphe](https://github.com/gold-silver-copper/morphe).
Inflects any word, known or made up. The `_scored` functions give every candidate with a confidence.

Generated from the Kaikki MiddleDutch dump of 2026-10-03. Do not edit by hand: change the config in morphe and regenerate.

## Example

```rust
use middle_dutch::*;
println!("{}", noun("raven", Case::Genitive, Number::Singular)); // rāvens
println!("{:?}", noun_gender("raven"));
println!("{}", verb("driven", Mood::Indicative, Tense::Past, Person::Third, Number::Singular)); // drêef
println!("{}", verb_participle("driven", Tense::Past)); // gedrēven
println!("{}", adj("arm", Definiteness::Indefinite, Case::Dative, GenderNumber::Masculine)); // armen
```

The editorial length marks (ē, ê, ā…) are optional on input; output carries
them.

## Coverage

| Part of speech | Entries | Cells | Edits | Classes | Tree nodes | Lexicon |
|---|---|---|---|---|---|---|
| noun | 538 | 8 | 189 | 225 | 145 | 330 |
| adj | 85 | 32 | 83 | 62 | 23 | 48 |
| verb | 259 | 31 | 456 | 243 | 70 | 164 |

Model size: 147430 bytes.

**noun** cells: `NominativeSingular`, `NominativePlural`, `AccusativeSingular`, `AccusativePlural`, `GenitiveSingular`, `GenitivePlural`, `DativeSingular`, `DativePlural`

Citation form: `NominativeSingular, GenitiveSingular, NominativePlural` (comma-separated; parts you give are output exactly and narrow the prediction).

**adj** cells: `IndefiniteNominativeMasculine`, `IndefiniteNominativeFeminine`, `IndefiniteNominativeNeuter`, `IndefiniteNominativePlural`, `IndefiniteAccusativeMasculine`, `IndefiniteAccusativeFeminine`, `IndefiniteAccusativeNeuter`, `IndefiniteAccusativePlural`, `IndefiniteGenitiveMasculine`, `IndefiniteGenitiveFeminine`, `IndefiniteGenitiveNeuter`, `IndefiniteGenitivePlural`, `IndefiniteDativeMasculine`, `IndefiniteDativeFeminine`, `IndefiniteDativeNeuter`, `IndefiniteDativePlural`, `DefiniteNominativeMasculine`, `DefiniteNominativeFeminine`, `DefiniteNominativeNeuter`, `DefiniteNominativePlural`, `DefiniteAccusativeMasculine`, `DefiniteAccusativeFeminine`, `DefiniteAccusativeNeuter`, `DefiniteAccusativePlural`, `DefiniteGenitiveMasculine`, `DefiniteGenitiveFeminine`, `DefiniteGenitiveNeuter`, `DefiniteGenitivePlural`, `DefiniteDativeMasculine`, `DefiniteDativeFeminine`, `DefiniteDativeNeuter`, `DefiniteDativePlural`

Citation form: `IndefiniteNominativeMasculine` (comma-separated; parts you give are output exactly and narrow the prediction).

**verb** cells: `Infinitive`, `InflectedInfinitiveGenitive`, `InflectedInfinitiveDative`, `IndicativePresentFirstSingular`, `IndicativePresentFirstPlural`, `IndicativePresentSecondSingular`, `IndicativePresentSecondPlural`, `IndicativePresentThirdSingular`, `IndicativePresentThirdPlural`, `IndicativePastFirstSingular`, `IndicativePastFirstPlural`, `IndicativePastSecondSingular`, `IndicativePastSecondPlural`, `IndicativePastThirdSingular`, `IndicativePastThirdPlural`, `SubjunctivePresentFirstSingular`, `SubjunctivePresentFirstPlural`, `SubjunctivePresentSecondSingular`, `SubjunctivePresentSecondPlural`, `SubjunctivePresentThirdSingular`, `SubjunctivePresentThirdPlural`, `SubjunctivePastFirstSingular`, `SubjunctivePastFirstPlural`, `SubjunctivePastSecondSingular`, `SubjunctivePastSecondPlural`, `SubjunctivePastThirdSingular`, `SubjunctivePastThirdPlural`, `ImperativeSingular`, `ImperativePlural`, `ParticiplePresent`, `ParticiplePast`

Citation form: `Infinitive, IndicativePastThirdSingular, IndicativePastThirdPlural, ParticiplePast` (comma-separated; parts you give are output exactly and narrow the prediction).

## How it works

Each attested form is stored as an edit tree from the lemma. A lemma's edits across all cells make its inflection class. A suffix tree over the word, read from its end and pruned by a description-length test, predicts the class of any word. Known words the tree gets wrong are kept in a small lexicon. Confidences are smoothed counts; variant weights follow Wiktionary's listing order (1, 1/2, 1/3…, nonstandard forms a tenth of that).

## License

Code: MIT OR Apache-2.0. Data: derived from [Wiktionary](https://www.wiktionary.org/) (via [kaikki.org](https://kaikki.org/)), CC BY-SA 4.0; the model bytes in `src/model.bin` are shared under the same license.
