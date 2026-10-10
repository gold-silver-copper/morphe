# lithuanian

Lithuanian inflection for Rust, learned from Wiktionary by [morphe](https://github.com/gold-silver-copper/morphe).
Inflects any word, known or made up. The `_scored` functions give every candidate with a confidence.

Generated from the Kaikki Lithuanian dump of 2026-10-03. Do not edit by hand: change the config in morphe and regenerate.

## Example

```rust
use lithuanian::*;
let plain = Lithuanian::default().marks(false);
assert_eq!(plain.noun("namas", Case::Locative, Number::Plural), "namuose");
println!("{}", noun("pupa", Case::Genitive, Number::Singular)); // pùpos
println!("{:?}", noun_gender("pupa"));
println!("{}", adj("geras", Gender::Feminine, Number::Plural, Case::Dative)); // geróms
println!("{}", adj_definite("geras", Gender::Masculine, Number::Singular, Case::Nominative)); // geràsis
println!("{}", verb("rasti", Tense::Past, PersonNumber::FirstSingular)); // radaũ
// Citations pin principal parts: infinitive, present and past third person.
println!("{}", verb("arti, ãria, ãrė", Tense::Future, PersonNumber::Third)); // ar̃s
```

Input matching ignores accent marks; output carries them where Wiktionary
has them (`marks(false)` removes them). The third person is one cell, as
Lithuanian uses the same form for singular and plural. Adjective cells cover
the plain forms in all three degrees, the definite (pronominal) positive
forms and the neuter; definite comparatives and superlatives are left out.
Wiktionary's verb tables are read by row, as their person labels are lost.

## Coverage

| Part of speech | Entries | Cells | Edits | Classes | Tree nodes | Lexicon |
|---|---|---|---|---|---|---|
| noun | 3887 | 14 | 4274 | 965 | 999 | 2493 |
| adj | 624 | 99 | 7966 | 274 | 188 | 390 |
| verb | 2140 | 40 | 8709 | 1137 | 426 | 885 |
| pron | 36 | 36 | 714 | 49 | 11 | 28 |
| num | 11 | 12 | 63 | 10 | 5 | 8 |

Model size: 1450605 bytes.

**noun** cells: `NominativeSingular`, `NominativePlural`, `GenitiveSingular`, `GenitivePlural`, `DativeSingular`, `DativePlural`, `AccusativeSingular`, `AccusativePlural`, `InstrumentalSingular`, `InstrumentalPlural`, `LocativeSingular`, `LocativePlural`, `VocativeSingular`, `VocativePlural`

Citation form: `NominativeSingular, GenitiveSingular` (comma-separated; parts you give are output exactly and narrow the prediction).

**adj** cells: `MasculineSingularNominative`, `MasculineSingularGenitive`, `MasculineSingularDative`, `MasculineSingularAccusative`, `MasculineSingularInstrumental`, `MasculineSingularLocative`, `MasculinePluralNominative`, `MasculinePluralGenitive`, `MasculinePluralDative`, `MasculinePluralAccusative`, `MasculinePluralInstrumental`, `MasculinePluralLocative`, `FeminineSingularNominative`, `FeminineSingularGenitive`, `FeminineSingularDative`, `FeminineSingularAccusative`, `FeminineSingularInstrumental`, `FeminineSingularLocative`, `FemininePluralNominative`, `FemininePluralGenitive`, `FemininePluralDative`, `FemininePluralAccusative`, `FemininePluralInstrumental`, `FemininePluralLocative`, `ComparativeMasculineSingularNominative`, `ComparativeMasculineSingularGenitive`, `ComparativeMasculineSingularDative`, `ComparativeMasculineSingularAccusative`, `ComparativeMasculineSingularInstrumental`, `ComparativeMasculineSingularLocative`, `ComparativeMasculinePluralNominative`, `ComparativeMasculinePluralGenitive`, `ComparativeMasculinePluralDative`, `ComparativeMasculinePluralAccusative`, `ComparativeMasculinePluralInstrumental`, `ComparativeMasculinePluralLocative`, `ComparativeFeminineSingularNominative`, `ComparativeFeminineSingularGenitive`, `ComparativeFeminineSingularDative`, `ComparativeFeminineSingularAccusative`, `ComparativeFeminineSingularInstrumental`, `ComparativeFeminineSingularLocative`, `ComparativeFemininePluralNominative`, `ComparativeFemininePluralGenitive`, `ComparativeFemininePluralDative`, `ComparativeFemininePluralAccusative`, `ComparativeFemininePluralInstrumental`, `ComparativeFemininePluralLocative`, `SuperlativeMasculineSingularNominative`, `SuperlativeMasculineSingularGenitive`, `SuperlativeMasculineSingularDative`, `SuperlativeMasculineSingularAccusative`, `SuperlativeMasculineSingularInstrumental`, `SuperlativeMasculineSingularLocative`, `SuperlativeMasculinePluralNominative`, `SuperlativeMasculinePluralGenitive`, `SuperlativeMasculinePluralDative`, `SuperlativeMasculinePluralAccusative`, `SuperlativeMasculinePluralInstrumental`, `SuperlativeMasculinePluralLocative`, `SuperlativeFeminineSingularNominative`, `SuperlativeFeminineSingularGenitive`, `SuperlativeFeminineSingularDative`, `SuperlativeFeminineSingularAccusative`, `SuperlativeFeminineSingularInstrumental`, `SuperlativeFeminineSingularLocative`, `SuperlativeFemininePluralNominative`, `SuperlativeFemininePluralGenitive`, `SuperlativeFemininePluralDative`, `SuperlativeFemininePluralAccusative`, `SuperlativeFemininePluralInstrumental`, `SuperlativeFemininePluralLocative`, `DefiniteMasculineSingularNominative`, `DefiniteMasculineSingularGenitive`, `DefiniteMasculineSingularDative`, `DefiniteMasculineSingularAccusative`, `DefiniteMasculineSingularInstrumental`, `DefiniteMasculineSingularLocative`, `DefiniteMasculinePluralNominative`, `DefiniteMasculinePluralGenitive`, `DefiniteMasculinePluralDative`, `DefiniteMasculinePluralAccusative`, `DefiniteMasculinePluralInstrumental`, `DefiniteMasculinePluralLocative`, `DefiniteFeminineSingularNominative`, `DefiniteFeminineSingularGenitive`, `DefiniteFeminineSingularDative`, `DefiniteFeminineSingularAccusative`, `DefiniteFeminineSingularInstrumental`, `DefiniteFeminineSingularLocative`, `DefiniteFemininePluralNominative`, `DefiniteFemininePluralGenitive`, `DefiniteFemininePluralDative`, `DefiniteFemininePluralAccusative`, `DefiniteFemininePluralInstrumental`, `DefiniteFemininePluralLocative`, `NeuterPositive`, `NeuterComparative`, `NeuterSuperlative`

Citation form: `Lemma, FeminineSingularNominative` (comma-separated; parts you give are output exactly and narrow the prediction).

**verb** cells: `PresentFirstSingular`, `PresentSecondSingular`, `PresentThird`, `PresentFirstPlural`, `PresentSecondPlural`, `PastFirstSingular`, `PastSecondSingular`, `PastThird`, `PastFirstPlural`, `PastSecondPlural`, `PastFrequentativeFirstSingular`, `PastFrequentativeSecondSingular`, `PastFrequentativeThird`, `PastFrequentativeFirstPlural`, `PastFrequentativeSecondPlural`, `FutureFirstSingular`, `FutureSecondSingular`, `FutureThird`, `FutureFirstPlural`, `FutureSecondPlural`, `ConditionalFirstSingular`, `ConditionalSecondSingular`, `ConditionalThird`, `ConditionalFirstPlural`, `ConditionalSecondPlural`, `ImperativeSecondSingular`, `ImperativeThird`, `ImperativeFirstPlural`, `ImperativeSecondPlural`, `ParticipleActivePresent`, `ParticipleActivePast`, `ParticipleActiveFuture`, `ParticiplePassivePresent`, `ParticiplePassivePast`, `ParticiplePassiveFuture`, `HalfParticiple`, `GerundPresent`, `GerundPast`, `GerundFuture`, `Budinys`

Citation form: `Lemma, PresentThird, PastThird` (comma-separated; parts you give are output exactly and narrow the prediction).

**pron** cells: `NominativeSingularMasculine`, `NominativeSingularFeminine`, `NominativePluralMasculine`, `NominativePluralFeminine`, `NominativeDualMasculine`, `NominativeDualFeminine`, `GenitiveSingularMasculine`, `GenitiveSingularFeminine`, `GenitivePluralMasculine`, `GenitivePluralFeminine`, `GenitiveDualMasculine`, `GenitiveDualFeminine`, `DativeSingularMasculine`, `DativeSingularFeminine`, `DativePluralMasculine`, `DativePluralFeminine`, `DativeDualMasculine`, `DativeDualFeminine`, `AccusativeSingularMasculine`, `AccusativeSingularFeminine`, `AccusativePluralMasculine`, `AccusativePluralFeminine`, `AccusativeDualMasculine`, `AccusativeDualFeminine`, `InstrumentalSingularMasculine`, `InstrumentalSingularFeminine`, `InstrumentalPluralMasculine`, `InstrumentalPluralFeminine`, `InstrumentalDualMasculine`, `InstrumentalDualFeminine`, `LocativeSingularMasculine`, `LocativeSingularFeminine`, `LocativePluralMasculine`, `LocativePluralFeminine`, `LocativeDualMasculine`, `LocativeDualFeminine`

**num** cells: `NominativeMasculine`, `NominativeFeminine`, `GenitiveMasculine`, `GenitiveFeminine`, `DativeMasculine`, `DativeFeminine`, `AccusativeMasculine`, `AccusativeFeminine`, `InstrumentalMasculine`, `InstrumentalFeminine`, `LocativeMasculine`, `LocativeFeminine`

## How it works

Each attested form is stored as an edit tree from the lemma. A lemma's edits across all cells make its inflection class. A suffix tree over the word, read from its end and pruned by a description-length test, predicts the class of any word. Known words the tree gets wrong are kept in a small lexicon. Confidences are smoothed counts; variant weights follow Wiktionary's listing order (1, 1/2, 1/3…, nonstandard forms a tenth of that).

## License

Code: MIT OR Apache-2.0. Data: derived from [Wiktionary](https://www.wiktionary.org/) (via [kaikki.org](https://kaikki.org/)), CC BY-SA 4.0; the model bytes in `src/model.bin` are shared under the same license.
