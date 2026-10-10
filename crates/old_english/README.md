# old_english

OldEnglish inflection for Rust, learned from Wiktionary by [morphe](https://github.com/gold-silver-copper/morphe).
Inflects any word, known or made up. The `_scored` functions give every candidate with a confidence.

Generated from the Kaikki OldEnglish dump of 2026-10-03. Do not edit by hand: change the config in morphe and regenerate.

## Example

```rust
use old_english::*;
println!("{}", noun("stān", Case::Genitive, Number::Plural)); // stāna
println!("{}", noun("word", Case::Dative, Number::Singular)); // worde
println!("{:?}", noun_gender("giefu"));
println!("{}", verb("beran", Tense::Past, PersonNumber::FirstSingular)); // bær
println!("{}", verb_subjunctive("lufian", Tense::Present, Number::Plural)); // lufiġen
println!("{}", verb_participle("singan", Tense::Past)); // sungen
println!("{}", adj("gōd", Case::Dative, Gender::Masculine, Number::Singular)); // gōdum
println!("{}", adj_weak("gōd", Case::Nominative, Gender::Feminine, Number::Singular)); // gōde
// Citations pin principal parts: infinitive, past singular, past plural, past participle.
println!("{}", verb("drincan, dranc, druncon, druncen", Tense::Past, PersonNumber::Plural));
```

West Saxon spellings as Wiktionary normalizes them. Input may leave out the
editorial macrons and the dots of ċ and ġ; ƿ matches w and ð matches þ.
Output carries the marks (`marks(false)` removes them). The indicative has
one plural form for all persons, hence `PersonNumber`.

## Coverage

| Part of speech | Entries | Cells | Edits | Classes | Tree nodes | Lexicon |
|---|---|---|---|---|---|---|
| noun | 7375 | 8 | 999 | 686 | 1583 | 4345 |
| adj | 2329 | 62 | 724 | 431 | 538 | 1275 |
| verb | 4081 | 18 | 2625 | 1121 | 903 | 2696 |
| pron | 42 | 45 | 109 | 45 | 12 | 34 |
| det | 11 | 30 | 43 | 12 | 7 | 7 |
| num | 21 | 30 | 35 | 16 | 3 | 17 |

Model size: 595835 bytes.

**noun** cells: `NominativeSingular`, `NominativePlural`, `AccusativeSingular`, `AccusativePlural`, `GenitiveSingular`, `GenitivePlural`, `DativeSingular`, `DativePlural`

Citation form: `NominativeSingular, GenitiveSingular, NominativePlural` (comma-separated; parts you give are output exactly and narrow the prediction).

**adj** cells: `NominativeMasculineSingular`, `NominativeMasculinePlural`, `NominativeFeminineSingular`, `NominativeFemininePlural`, `NominativeNeuterSingular`, `NominativeNeuterPlural`, `AccusativeMasculineSingular`, `AccusativeMasculinePlural`, `AccusativeFeminineSingular`, `AccusativeFemininePlural`, `AccusativeNeuterSingular`, `AccusativeNeuterPlural`, `GenitiveMasculineSingular`, `GenitiveMasculinePlural`, `GenitiveFeminineSingular`, `GenitiveFemininePlural`, `GenitiveNeuterSingular`, `GenitiveNeuterPlural`, `DativeMasculineSingular`, `DativeMasculinePlural`, `DativeFeminineSingular`, `DativeFemininePlural`, `DativeNeuterSingular`, `DativeNeuterPlural`, `InstrumentalMasculineSingular`, `InstrumentalMasculinePlural`, `InstrumentalFeminineSingular`, `InstrumentalFemininePlural`, `InstrumentalNeuterSingular`, `InstrumentalNeuterPlural`, `WeakNominativeMasculineSingular`, `WeakNominativeMasculinePlural`, `WeakNominativeFeminineSingular`, `WeakNominativeFemininePlural`, `WeakNominativeNeuterSingular`, `WeakNominativeNeuterPlural`, `WeakAccusativeMasculineSingular`, `WeakAccusativeMasculinePlural`, `WeakAccusativeFeminineSingular`, `WeakAccusativeFemininePlural`, `WeakAccusativeNeuterSingular`, `WeakAccusativeNeuterPlural`, `WeakGenitiveMasculineSingular`, `WeakGenitiveMasculinePlural`, `WeakGenitiveFeminineSingular`, `WeakGenitiveFemininePlural`, `WeakGenitiveNeuterSingular`, `WeakGenitiveNeuterPlural`, `WeakDativeMasculineSingular`, `WeakDativeMasculinePlural`, `WeakDativeFeminineSingular`, `WeakDativeFemininePlural`, `WeakDativeNeuterSingular`, `WeakDativeNeuterPlural`, `WeakInstrumentalMasculineSingular`, `WeakInstrumentalMasculinePlural`, `WeakInstrumentalFeminineSingular`, `WeakInstrumentalFemininePlural`, `WeakInstrumentalNeuterSingular`, `WeakInstrumentalNeuterPlural`, `Comparative`, `Superlative`

**verb** cells: `Infinitive`, `InflectedInfinitive`, `PresentFirstSingular`, `PresentSecondSingular`, `PresentThirdSingular`, `PresentPlural`, `PastFirstSingular`, `PastSecondSingular`, `PastThirdSingular`, `PastPlural`, `SubjunctivePresentSingular`, `SubjunctivePresentPlural`, `SubjunctivePastSingular`, `SubjunctivePastPlural`, `ImperativeSingular`, `ImperativePlural`, `ParticiplePresent`, `ParticiplePast`

Citation form: `Infinitive, PastFirstSingular, PastPlural, ParticiplePast` (comma-separated; parts you give are output exactly and narrow the prediction).

**pron** cells: `NominativeMasculineSingular`, `NominativeMasculineDual`, `NominativeMasculinePlural`, `NominativeFeminineSingular`, `NominativeFeminineDual`, `NominativeFemininePlural`, `NominativeNeuterSingular`, `NominativeNeuterDual`, `NominativeNeuterPlural`, `AccusativeMasculineSingular`, `AccusativeMasculineDual`, `AccusativeMasculinePlural`, `AccusativeFeminineSingular`, `AccusativeFeminineDual`, `AccusativeFemininePlural`, `AccusativeNeuterSingular`, `AccusativeNeuterDual`, `AccusativeNeuterPlural`, `GenitiveMasculineSingular`, `GenitiveMasculineDual`, `GenitiveMasculinePlural`, `GenitiveFeminineSingular`, `GenitiveFeminineDual`, `GenitiveFemininePlural`, `GenitiveNeuterSingular`, `GenitiveNeuterDual`, `GenitiveNeuterPlural`, `DativeMasculineSingular`, `DativeMasculineDual`, `DativeMasculinePlural`, `DativeFeminineSingular`, `DativeFeminineDual`, `DativeFemininePlural`, `DativeNeuterSingular`, `DativeNeuterDual`, `DativeNeuterPlural`, `InstrumentalMasculineSingular`, `InstrumentalMasculineDual`, `InstrumentalMasculinePlural`, `InstrumentalFeminineSingular`, `InstrumentalFeminineDual`, `InstrumentalFemininePlural`, `InstrumentalNeuterSingular`, `InstrumentalNeuterDual`, `InstrumentalNeuterPlural`

**det** cells: `NominativeMasculineSingular`, `NominativeMasculinePlural`, `NominativeFeminineSingular`, `NominativeFemininePlural`, `NominativeNeuterSingular`, `NominativeNeuterPlural`, `AccusativeMasculineSingular`, `AccusativeMasculinePlural`, `AccusativeFeminineSingular`, `AccusativeFemininePlural`, `AccusativeNeuterSingular`, `AccusativeNeuterPlural`, `GenitiveMasculineSingular`, `GenitiveMasculinePlural`, `GenitiveFeminineSingular`, `GenitiveFemininePlural`, `GenitiveNeuterSingular`, `GenitiveNeuterPlural`, `DativeMasculineSingular`, `DativeMasculinePlural`, `DativeFeminineSingular`, `DativeFemininePlural`, `DativeNeuterSingular`, `DativeNeuterPlural`, `InstrumentalMasculineSingular`, `InstrumentalMasculinePlural`, `InstrumentalFeminineSingular`, `InstrumentalFemininePlural`, `InstrumentalNeuterSingular`, `InstrumentalNeuterPlural`

**num** cells: `NominativeMasculineSingular`, `NominativeMasculinePlural`, `NominativeFeminineSingular`, `NominativeFemininePlural`, `NominativeNeuterSingular`, `NominativeNeuterPlural`, `AccusativeMasculineSingular`, `AccusativeMasculinePlural`, `AccusativeFeminineSingular`, `AccusativeFemininePlural`, `AccusativeNeuterSingular`, `AccusativeNeuterPlural`, `GenitiveMasculineSingular`, `GenitiveMasculinePlural`, `GenitiveFeminineSingular`, `GenitiveFemininePlural`, `GenitiveNeuterSingular`, `GenitiveNeuterPlural`, `DativeMasculineSingular`, `DativeMasculinePlural`, `DativeFeminineSingular`, `DativeFemininePlural`, `DativeNeuterSingular`, `DativeNeuterPlural`, `InstrumentalMasculineSingular`, `InstrumentalMasculinePlural`, `InstrumentalFeminineSingular`, `InstrumentalFemininePlural`, `InstrumentalNeuterSingular`, `InstrumentalNeuterPlural`

## How it works

Each attested form is stored as an edit tree from the lemma. A lemma's edits across all cells make its inflection class. A suffix tree over the word, read from its end and pruned by a description-length test, predicts the class of any word. Known words the tree gets wrong are kept in a small lexicon. Confidences are smoothed counts; variant weights follow Wiktionary's listing order (1, 1/2, 1/3…, nonstandard forms a tenth of that).

## License

Code: MIT OR Apache-2.0. Data: derived from [Wiktionary](https://www.wiktionary.org/) (via [kaikki.org](https://kaikki.org/)), CC BY-SA 4.0; the model bytes in `src/model.bin` are shared under the same license.
