# lower_sorbian

LowerSorbian inflection for Rust, learned from Wiktionary by [morphe](https://github.com/gold-silver-copper/morphe).
Inflects any word, known or made up. The `_scored` functions give every candidate with a confidence.

Generated from the Kaikki LowerSorbian dump of 2026-10-03. Do not edit by hand: change the config in morphe and regenerate.

## Example

```rust
use lower_sorbian::*;
println!("{}", noun("słownik", Case::Dative, Number::Dual)); // słownikoma
println!("{}", verb("sec", Tense::Preterite, PersonNumber::ThirdPlural)); // secechu
println!("{}", verb_l_participle("sec")); // sekł
println!("{}", adj("polski", Case::Genitive, SingularGender::Feminine)); // polskeje
println!("{}", adj_simple("dobry", Degree::Comparative)); // lěpšy
```

Lower Sorbian keeps the dual. Diacritics are optional on input.

## Coverage

| Part of speech | Entries | Cells | Edits | Classes | Tree nodes | Lexicon |
|---|---|---|---|---|---|---|
| noun | 932 | 19 | 527 | 325 | 184 | 632 |
| adj | 195 | 50 | 89 | 62 | 38 | 106 |
| verb | 212 | 33 | 528 | 104 | 63 | 124 |

Model size: 95411 bytes.

**noun** cells: `NominativeSingular`, `NominativeDual`, `NominativePlural`, `GenitiveSingular`, `GenitiveDual`, `GenitivePlural`, `DativeSingular`, `DativeDual`, `DativePlural`, `AccusativeSingular`, `AccusativeDual`, `AccusativePlural`, `InstrumentalSingular`, `InstrumentalDual`, `InstrumentalPlural`, `LocativeSingular`, `LocativeDual`, `LocativePlural`, `Diminutive`

Citation form: `NominativeSingular, GenitiveSingular` (comma-separated; parts you give are output exactly and narrow the prediction).

**adj** cells: `NominativeMasculine`, `NominativeMasculineAnimate`, `NominativeNeuter`, `NominativeFeminine`, `GenitiveMasculine`, `GenitiveMasculineAnimate`, `GenitiveNeuter`, `GenitiveFeminine`, `DativeMasculine`, `DativeMasculineAnimate`, `DativeNeuter`, `DativeFeminine`, `AccusativeMasculine`, `AccusativeMasculineAnimate`, `AccusativeNeuter`, `AccusativeFeminine`, `InstrumentalMasculine`, `InstrumentalMasculineAnimate`, `InstrumentalNeuter`, `InstrumentalFeminine`, `LocativeMasculine`, `LocativeMasculineAnimate`, `LocativeNeuter`, `LocativeFeminine`, `DualNominativeVirile`, `DualNominativeNonvirile`, `DualGenitiveVirile`, `DualGenitiveNonvirile`, `DualDativeVirile`, `DualDativeNonvirile`, `DualAccusativeVirile`, `DualAccusativeNonvirile`, `DualInstrumentalVirile`, `DualInstrumentalNonvirile`, `DualLocativeVirile`, `DualLocativeNonvirile`, `PluralNominativeVirile`, `PluralNominativeNonvirile`, `PluralGenitiveVirile`, `PluralGenitiveNonvirile`, `PluralDativeVirile`, `PluralDativeNonvirile`, `PluralAccusativeVirile`, `PluralAccusativeNonvirile`, `PluralInstrumentalVirile`, `PluralInstrumentalNonvirile`, `PluralLocativeVirile`, `PluralLocativeNonvirile`, `SimpleComparative`, `SimpleSuperlative`

Citation form: `NominativeMasculine, NominativeFeminine, NominativeNeuter` (comma-separated; parts you give are output exactly and narrow the prediction).

**verb** cells: `Infinitive`, `PresentFirstSingular`, `PresentSecondSingular`, `PresentThirdSingular`, `PresentFirstDual`, `PresentSecondDual`, `PresentThirdDual`, `PresentFirstPlural`, `PresentSecondPlural`, `PresentThirdPlural`, `PreteriteFirstSingular`, `PreteriteSecondSingular`, `PreteriteThirdSingular`, `PreteriteFirstDual`, `PreteriteSecondDual`, `PreteriteThirdDual`, `PreteriteFirstPlural`, `PreteriteSecondPlural`, `PreteriteThirdPlural`, `ImperativeFirstSingular`, `ImperativeSecondSingular`, `ImperativeThirdSingular`, `ImperativeFirstDual`, `ImperativeSecondDual`, `ImperativeThirdDual`, `ImperativeFirstPlural`, `ImperativeSecondPlural`, `ImperativeThirdPlural`, `Supine`, `ParticiplePresent`, `LParticiple`, `PassiveParticiple`, `VerbalNoun`

Citation form: `Infinitive, PresentFirstSingular, PresentThirdSingular` (comma-separated; parts you give are output exactly and narrow the prediction).

## How it works

Each attested form is stored as an edit tree from the lemma. A lemma's edits across all cells make its inflection class. A suffix tree over the word, read from its end and pruned by a description-length test, predicts the class of any word. Known words the tree gets wrong are kept in a small lexicon. Confidences are smoothed counts; variant weights follow Wiktionary's listing order (1, 1/2, 1/3…, nonstandard forms a tenth of that).

## License

Code: MIT OR Apache-2.0. Data: derived from [Wiktionary](https://www.wiktionary.org/) (via [kaikki.org](https://kaikki.org/)), CC BY-SA 4.0; the model bytes in `src/model.bin` are shared under the same license.
