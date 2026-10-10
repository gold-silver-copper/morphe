# old_church_slavonic

OldChurchSlavonic inflection for Rust, learned from Wiktionary by [morphe](https://github.com/gold-silver-copper/morphe).
Inflects any word, known or made up. The `_scored` functions give every candidate with a confidence.

Generated from the Kaikki OldChurchSlavonic dump of 2026-10-03. Do not edit by hand: change the config in morphe and regenerate.

## Example

```rust
use old_church_slavonic::*;
println!("{}", noun("градъ", Case::Locative, Number::Singular)); // градѣ
println!("{:?}", noun_gender("рѫка"));
println!("{}", verb("правити", Tense::Aorist, Person::Third, Number::Plural)); // правишѧ
println!("{}", verb_imperative("правити", PersonNumber::SecondSingular)); // прави
println!("{}", verb_l_participle("правити", Gender::Feminine, Number::Singular)); // правила
println!("{}", adj("добръ", Case::Dative, Gender::Feminine, Number::Singular)); // добрѣ
println!("{}", adj_long("добръ", Case::Nominative, Gender::Masculine, Number::Singular)); // добрꙑи
println!("{:?}", verb_aspect("правити"));
```

Cyrillic, as Wiktionary normalizes it. Input may leave out the palatalization
hook (҄), titlo and stress marks; і/ї match и, ѡ matches о, and ꙋ/ѹ match оу.
Adjectives have short (indefinite) forms, `adj`, and long (definite) forms,
`adj_long`. Personal pronouns are not covered.

## Coverage

| Part of speech | Entries | Cells | Edits | Classes | Tree nodes | Lexicon |
|---|---|---|---|---|---|---|
| noun | 1977 | 21 | 731 | 151 | 118 | 401 |
| adj | 319 | 126 | 417 | 20 | 18 | 19 |
| verb | 712 | 46 | 1215 | 182 | 85 | 140 |
| pron | 29 | 54 | 355 | 36 | 4 | 19 |
| num | 17 | 54 | 62 | 13 | 4 | 6 |

Model size: 224880 bytes.

**noun** cells: `NominativeSingular`, `NominativeDual`, `NominativePlural`, `AccusativeSingular`, `AccusativeDual`, `AccusativePlural`, `GenitiveSingular`, `GenitiveDual`, `GenitivePlural`, `DativeSingular`, `DativeDual`, `DativePlural`, `InstrumentalSingular`, `InstrumentalDual`, `InstrumentalPlural`, `LocativeSingular`, `LocativeDual`, `LocativePlural`, `VocativeSingular`, `VocativeDual`, `VocativePlural`

Citation form: `NominativeSingular, GenitiveSingular` (comma-separated; parts you give are output exactly and narrow the prediction).

**adj** cells: `NominativeMasculineSingular`, `NominativeMasculineDual`, `NominativeMasculinePlural`, `NominativeFeminineSingular`, `NominativeFeminineDual`, `NominativeFemininePlural`, `NominativeNeuterSingular`, `NominativeNeuterDual`, `NominativeNeuterPlural`, `AccusativeMasculineSingular`, `AccusativeMasculineDual`, `AccusativeMasculinePlural`, `AccusativeFeminineSingular`, `AccusativeFeminineDual`, `AccusativeFemininePlural`, `AccusativeNeuterSingular`, `AccusativeNeuterDual`, `AccusativeNeuterPlural`, `GenitiveMasculineSingular`, `GenitiveMasculineDual`, `GenitiveMasculinePlural`, `GenitiveFeminineSingular`, `GenitiveFeminineDual`, `GenitiveFemininePlural`, `GenitiveNeuterSingular`, `GenitiveNeuterDual`, `GenitiveNeuterPlural`, `DativeMasculineSingular`, `DativeMasculineDual`, `DativeMasculinePlural`, `DativeFeminineSingular`, `DativeFeminineDual`, `DativeFemininePlural`, `DativeNeuterSingular`, `DativeNeuterDual`, `DativeNeuterPlural`, `InstrumentalMasculineSingular`, `InstrumentalMasculineDual`, `InstrumentalMasculinePlural`, `InstrumentalFeminineSingular`, `InstrumentalFeminineDual`, `InstrumentalFemininePlural`, `InstrumentalNeuterSingular`, `InstrumentalNeuterDual`, `InstrumentalNeuterPlural`, `LocativeMasculineSingular`, `LocativeMasculineDual`, `LocativeMasculinePlural`, `LocativeFeminineSingular`, `LocativeFeminineDual`, `LocativeFemininePlural`, `LocativeNeuterSingular`, `LocativeNeuterDual`, `LocativeNeuterPlural`, `VocativeMasculineSingular`, `VocativeMasculineDual`, `VocativeMasculinePlural`, `VocativeFeminineSingular`, `VocativeFeminineDual`, `VocativeFemininePlural`, `VocativeNeuterSingular`, `VocativeNeuterDual`, `VocativeNeuterPlural`, `LongNominativeMasculineSingular`, `LongNominativeMasculineDual`, `LongNominativeMasculinePlural`, `LongNominativeFeminineSingular`, `LongNominativeFeminineDual`, `LongNominativeFemininePlural`, `LongNominativeNeuterSingular`, `LongNominativeNeuterDual`, `LongNominativeNeuterPlural`, `LongAccusativeMasculineSingular`, `LongAccusativeMasculineDual`, `LongAccusativeMasculinePlural`, `LongAccusativeFeminineSingular`, `LongAccusativeFeminineDual`, `LongAccusativeFemininePlural`, `LongAccusativeNeuterSingular`, `LongAccusativeNeuterDual`, `LongAccusativeNeuterPlural`, `LongGenitiveMasculineSingular`, `LongGenitiveMasculineDual`, `LongGenitiveMasculinePlural`, `LongGenitiveFeminineSingular`, `LongGenitiveFeminineDual`, `LongGenitiveFemininePlural`, `LongGenitiveNeuterSingular`, `LongGenitiveNeuterDual`, `LongGenitiveNeuterPlural`, `LongDativeMasculineSingular`, `LongDativeMasculineDual`, `LongDativeMasculinePlural`, `LongDativeFeminineSingular`, `LongDativeFeminineDual`, `LongDativeFemininePlural`, `LongDativeNeuterSingular`, `LongDativeNeuterDual`, `LongDativeNeuterPlural`, `LongInstrumentalMasculineSingular`, `LongInstrumentalMasculineDual`, `LongInstrumentalMasculinePlural`, `LongInstrumentalFeminineSingular`, `LongInstrumentalFeminineDual`, `LongInstrumentalFemininePlural`, `LongInstrumentalNeuterSingular`, `LongInstrumentalNeuterDual`, `LongInstrumentalNeuterPlural`, `LongLocativeMasculineSingular`, `LongLocativeMasculineDual`, `LongLocativeMasculinePlural`, `LongLocativeFeminineSingular`, `LongLocativeFeminineDual`, `LongLocativeFemininePlural`, `LongLocativeNeuterSingular`, `LongLocativeNeuterDual`, `LongLocativeNeuterPlural`, `LongVocativeMasculineSingular`, `LongVocativeMasculineDual`, `LongVocativeMasculinePlural`, `LongVocativeFeminineSingular`, `LongVocativeFeminineDual`, `LongVocativeFemininePlural`, `LongVocativeNeuterSingular`, `LongVocativeNeuterDual`, `LongVocativeNeuterPlural`

**verb** cells: `PresentFirstSingular`, `PresentFirstDual`, `PresentFirstPlural`, `PresentSecondSingular`, `PresentSecondDual`, `PresentSecondPlural`, `PresentThirdSingular`, `PresentThirdDual`, `PresentThirdPlural`, `ImperfectFirstSingular`, `ImperfectFirstDual`, `ImperfectFirstPlural`, `ImperfectSecondSingular`, `ImperfectSecondDual`, `ImperfectSecondPlural`, `ImperfectThirdSingular`, `ImperfectThirdDual`, `ImperfectThirdPlural`, `AoristFirstSingular`, `AoristFirstDual`, `AoristFirstPlural`, `AoristSecondSingular`, `AoristSecondDual`, `AoristSecondPlural`, `AoristThirdSingular`, `AoristThirdDual`, `AoristThirdPlural`, `ImperativeSecondSingular`, `ImperativeThirdSingular`, `ImperativeFirstDual`, `ImperativeSecondDual`, `ImperativeFirstPlural`, `ImperativeSecondPlural`, `ParticipleActivePresent`, `ParticipleActivePast`, `ParticiplePassivePresent`, `ParticiplePassivePast`, `LParticipleMasculineSingular`, `LParticipleMasculineDual`, `LParticipleMasculinePlural`, `LParticipleFeminineSingular`, `LParticipleFeminineDual`, `LParticipleFemininePlural`, `LParticipleNeuterSingular`, `LParticipleNeuterDual`, `LParticipleNeuterPlural`

**pron** cells: `NominativeMasculineSingular`, `NominativeMasculineDual`, `NominativeMasculinePlural`, `NominativeFeminineSingular`, `NominativeFeminineDual`, `NominativeFemininePlural`, `NominativeNeuterSingular`, `NominativeNeuterDual`, `NominativeNeuterPlural`, `AccusativeMasculineSingular`, `AccusativeMasculineDual`, `AccusativeMasculinePlural`, `AccusativeFeminineSingular`, `AccusativeFeminineDual`, `AccusativeFemininePlural`, `AccusativeNeuterSingular`, `AccusativeNeuterDual`, `AccusativeNeuterPlural`, `GenitiveMasculineSingular`, `GenitiveMasculineDual`, `GenitiveMasculinePlural`, `GenitiveFeminineSingular`, `GenitiveFeminineDual`, `GenitiveFemininePlural`, `GenitiveNeuterSingular`, `GenitiveNeuterDual`, `GenitiveNeuterPlural`, `DativeMasculineSingular`, `DativeMasculineDual`, `DativeMasculinePlural`, `DativeFeminineSingular`, `DativeFeminineDual`, `DativeFemininePlural`, `DativeNeuterSingular`, `DativeNeuterDual`, `DativeNeuterPlural`, `InstrumentalMasculineSingular`, `InstrumentalMasculineDual`, `InstrumentalMasculinePlural`, `InstrumentalFeminineSingular`, `InstrumentalFeminineDual`, `InstrumentalFemininePlural`, `InstrumentalNeuterSingular`, `InstrumentalNeuterDual`, `InstrumentalNeuterPlural`, `LocativeMasculineSingular`, `LocativeMasculineDual`, `LocativeMasculinePlural`, `LocativeFeminineSingular`, `LocativeFeminineDual`, `LocativeFemininePlural`, `LocativeNeuterSingular`, `LocativeNeuterDual`, `LocativeNeuterPlural`

**num** cells: `NominativeMasculineSingular`, `NominativeMasculineDual`, `NominativeMasculinePlural`, `NominativeFeminineSingular`, `NominativeFeminineDual`, `NominativeFemininePlural`, `NominativeNeuterSingular`, `NominativeNeuterDual`, `NominativeNeuterPlural`, `AccusativeMasculineSingular`, `AccusativeMasculineDual`, `AccusativeMasculinePlural`, `AccusativeFeminineSingular`, `AccusativeFeminineDual`, `AccusativeFemininePlural`, `AccusativeNeuterSingular`, `AccusativeNeuterDual`, `AccusativeNeuterPlural`, `GenitiveMasculineSingular`, `GenitiveMasculineDual`, `GenitiveMasculinePlural`, `GenitiveFeminineSingular`, `GenitiveFeminineDual`, `GenitiveFemininePlural`, `GenitiveNeuterSingular`, `GenitiveNeuterDual`, `GenitiveNeuterPlural`, `DativeMasculineSingular`, `DativeMasculineDual`, `DativeMasculinePlural`, `DativeFeminineSingular`, `DativeFeminineDual`, `DativeFemininePlural`, `DativeNeuterSingular`, `DativeNeuterDual`, `DativeNeuterPlural`, `InstrumentalMasculineSingular`, `InstrumentalMasculineDual`, `InstrumentalMasculinePlural`, `InstrumentalFeminineSingular`, `InstrumentalFeminineDual`, `InstrumentalFemininePlural`, `InstrumentalNeuterSingular`, `InstrumentalNeuterDual`, `InstrumentalNeuterPlural`, `LocativeMasculineSingular`, `LocativeMasculineDual`, `LocativeMasculinePlural`, `LocativeFeminineSingular`, `LocativeFeminineDual`, `LocativeFemininePlural`, `LocativeNeuterSingular`, `LocativeNeuterDual`, `LocativeNeuterPlural`

## How it works

Each attested form is stored as an edit tree from the lemma. A lemma's edits across all cells make its inflection class. A suffix tree over the word, read from its end and pruned by a description-length test, predicts the class of any word. Known words the tree gets wrong are kept in a small lexicon. Confidences are smoothed counts; variant weights follow Wiktionary's listing order (1, 1/2, 1/3…, nonstandard forms a tenth of that).

## License

Code: MIT OR Apache-2.0. Data: derived from [Wiktionary](https://www.wiktionary.org/) (via [kaikki.org](https://kaikki.org/)), CC BY-SA 4.0; the model bytes in `src/model.bin` are shared under the same license.
