# old_east_slavic

OldEastSlavic inflection for Rust, learned from Wiktionary by [morphe](https://github.com/gold-silver-copper/morphe).
Inflects any word, known or made up. The `_scored` functions give every candidate with a confidence.

Generated from the Kaikki OldEastSlavic dump of 2026-10-03. Do not edit by hand: change the config in morphe and regenerate.

## Example

```rust
use old_east_slavic::*;
println!("{}", noun("собака", Case::Dative, Number::Singular)); // собацѣ
println!("{:?}", noun_gender("собака"));
println!("{}", verb("правити", Tense::Aorist, Person::Third, Number::Plural)); // правиша
println!("{}", verb_l_participle("правити")); // правилъ
println!("{}", adj("свѧтъ", Case::Genitive, Gender::Feminine, Number::Singular)); // свѧтꙑ
println!("{}", adj_long("свѧтъ", Case::Nominative, Gender::Masculine, Number::Singular)); // свѧтꙑи
```

Cyrillic, as Wiktionary normalizes it. Input may leave out the titlo and
stress marks; і/ї match и, ѡ matches о, and ꙋ/ѹ match оу.

## Coverage

| Part of speech | Entries | Cells | Edits | Classes | Tree nodes | Lexicon |
|---|---|---|---|---|---|---|
| noun | 332 | 21 | 281 | 57 | 35 | 79 |
| adj | 68 | 108 | 139 | 6 | 7 | 4 |
| verb | 62 | 39 | 617 | 82 | 17 | 41 |

Model size: 62899 bytes.

**noun** cells: `NominativeSingular`, `NominativeDual`, `NominativePlural`, `AccusativeSingular`, `AccusativeDual`, `AccusativePlural`, `GenitiveSingular`, `GenitiveDual`, `GenitivePlural`, `DativeSingular`, `DativeDual`, `DativePlural`, `InstrumentalSingular`, `InstrumentalDual`, `InstrumentalPlural`, `LocativeSingular`, `LocativeDual`, `LocativePlural`, `VocativeSingular`, `VocativeDual`, `VocativePlural`

Citation form: `NominativeSingular, GenitiveSingular` (comma-separated; parts you give are output exactly and narrow the prediction).

**adj** cells: `NominativeMasculineSingular`, `NominativeMasculineDual`, `NominativeMasculinePlural`, `NominativeFeminineSingular`, `NominativeFeminineDual`, `NominativeFemininePlural`, `NominativeNeuterSingular`, `NominativeNeuterDual`, `NominativeNeuterPlural`, `AccusativeMasculineSingular`, `AccusativeMasculineDual`, `AccusativeMasculinePlural`, `AccusativeFeminineSingular`, `AccusativeFeminineDual`, `AccusativeFemininePlural`, `AccusativeNeuterSingular`, `AccusativeNeuterDual`, `AccusativeNeuterPlural`, `GenitiveMasculineSingular`, `GenitiveMasculineDual`, `GenitiveMasculinePlural`, `GenitiveFeminineSingular`, `GenitiveFeminineDual`, `GenitiveFemininePlural`, `GenitiveNeuterSingular`, `GenitiveNeuterDual`, `GenitiveNeuterPlural`, `DativeMasculineSingular`, `DativeMasculineDual`, `DativeMasculinePlural`, `DativeFeminineSingular`, `DativeFeminineDual`, `DativeFemininePlural`, `DativeNeuterSingular`, `DativeNeuterDual`, `DativeNeuterPlural`, `InstrumentalMasculineSingular`, `InstrumentalMasculineDual`, `InstrumentalMasculinePlural`, `InstrumentalFeminineSingular`, `InstrumentalFeminineDual`, `InstrumentalFemininePlural`, `InstrumentalNeuterSingular`, `InstrumentalNeuterDual`, `InstrumentalNeuterPlural`, `LocativeMasculineSingular`, `LocativeMasculineDual`, `LocativeMasculinePlural`, `LocativeFeminineSingular`, `LocativeFeminineDual`, `LocativeFemininePlural`, `LocativeNeuterSingular`, `LocativeNeuterDual`, `LocativeNeuterPlural`, `LongNominativeMasculineSingular`, `LongNominativeMasculineDual`, `LongNominativeMasculinePlural`, `LongNominativeFeminineSingular`, `LongNominativeFeminineDual`, `LongNominativeFemininePlural`, `LongNominativeNeuterSingular`, `LongNominativeNeuterDual`, `LongNominativeNeuterPlural`, `LongAccusativeMasculineSingular`, `LongAccusativeMasculineDual`, `LongAccusativeMasculinePlural`, `LongAccusativeFeminineSingular`, `LongAccusativeFeminineDual`, `LongAccusativeFemininePlural`, `LongAccusativeNeuterSingular`, `LongAccusativeNeuterDual`, `LongAccusativeNeuterPlural`, `LongGenitiveMasculineSingular`, `LongGenitiveMasculineDual`, `LongGenitiveMasculinePlural`, `LongGenitiveFeminineSingular`, `LongGenitiveFeminineDual`, `LongGenitiveFemininePlural`, `LongGenitiveNeuterSingular`, `LongGenitiveNeuterDual`, `LongGenitiveNeuterPlural`, `LongDativeMasculineSingular`, `LongDativeMasculineDual`, `LongDativeMasculinePlural`, `LongDativeFeminineSingular`, `LongDativeFeminineDual`, `LongDativeFemininePlural`, `LongDativeNeuterSingular`, `LongDativeNeuterDual`, `LongDativeNeuterPlural`, `LongInstrumentalMasculineSingular`, `LongInstrumentalMasculineDual`, `LongInstrumentalMasculinePlural`, `LongInstrumentalFeminineSingular`, `LongInstrumentalFeminineDual`, `LongInstrumentalFemininePlural`, `LongInstrumentalNeuterSingular`, `LongInstrumentalNeuterDual`, `LongInstrumentalNeuterPlural`, `LongLocativeMasculineSingular`, `LongLocativeMasculineDual`, `LongLocativeMasculinePlural`, `LongLocativeFeminineSingular`, `LongLocativeFeminineDual`, `LongLocativeFemininePlural`, `LongLocativeNeuterSingular`, `LongLocativeNeuterDual`, `LongLocativeNeuterPlural`

Citation form: `NominativeMasculineSingular, NominativeFeminineSingular, NominativeNeuterSingular` (comma-separated; parts you give are output exactly and narrow the prediction).

**verb** cells: `Infinitive`, `Supine`, `PresentFirstSingular`, `PresentFirstDual`, `PresentFirstPlural`, `PresentSecondSingular`, `PresentSecondDual`, `PresentSecondPlural`, `PresentThirdSingular`, `PresentThirdDual`, `PresentThirdPlural`, `ImperfectFirstSingular`, `ImperfectFirstDual`, `ImperfectFirstPlural`, `ImperfectSecondSingular`, `ImperfectSecondDual`, `ImperfectSecondPlural`, `ImperfectThirdSingular`, `ImperfectThirdDual`, `ImperfectThirdPlural`, `AoristFirstSingular`, `AoristFirstDual`, `AoristFirstPlural`, `AoristSecondSingular`, `AoristSecondDual`, `AoristSecondPlural`, `AoristThirdSingular`, `AoristThirdDual`, `AoristThirdPlural`, `ImperativeSingular`, `ImperativeDual`, `ImperativePlural`, `CohortativeDual`, `CohortativePlural`, `ParticipleActivePresent`, `ParticipleActivePast`, `ParticiplePassivePresent`, `ParticiplePassivePast`, `LParticiple`

Citation form: `Infinitive, PresentFirstSingular, AoristThirdSingular` (comma-separated; parts you give are output exactly and narrow the prediction).

## How it works

Each attested form is stored as an edit tree from the lemma. A lemma's edits across all cells make its inflection class. A suffix tree over the word, read from its end and pruned by a description-length test, predicts the class of any word. Known words the tree gets wrong are kept in a small lexicon. Confidences are smoothed counts; variant weights follow Wiktionary's listing order (1, 1/2, 1/3…, nonstandard forms a tenth of that).

## License

Code: MIT OR Apache-2.0. Data: derived from [Wiktionary](https://www.wiktionary.org/) (via [kaikki.org](https://kaikki.org/)), CC BY-SA 4.0; the model bytes in `src/model.bin` are shared under the same license.
