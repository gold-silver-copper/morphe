# upper_sorbian

UpperSorbian inflection for Rust, learned from Wiktionary by [morphe](https://github.com/gold-silver-copper/morphe).
Inflects any word, known or made up. The `_scored` functions give every candidate with a confidence.

Generated from the Kaikki UpperSorbian dump of 2026-10-03. Do not edit by hand: change the config in morphe and regenerate.

## Example

```rust
use upper_sorbian::*;
println!("{}", noun("woman", Case::Instrumental, Number::Dual)); // womanomaj
println!("{:?}", noun_gender("woman"));
println!("{}", verb("pić", Tense::Past, PersonNumber::ThirdPlural)); // pijachu
println!("{}", verb("pić", Tense::Present, PersonNumber::SecondDualNonvirile)); // pijetej
println!("{}", adj("daty", Case::Genitive, SingularGender::Feminine)); // dateje
```

Upper Sorbian keeps the dual; its second and third persons have virile and
nonvirile forms. Diacritics are optional on input.

## Coverage

| Part of speech | Entries | Cells | Edits | Classes | Tree nodes | Lexicon |
|---|---|---|---|---|---|---|
| noun | 549 | 22 | 438 | 305 | 142 | 365 |
| adj | 105 | 48 | 44 | 18 | 19 | 45 |
| verb | 91 | 61 | 324 | 45 | 25 | 57 |

Model size: 61977 bytes.

**noun** cells: `NominativeSingular`, `NominativeDual`, `NominativePlural`, `GenitiveSingular`, `GenitiveDual`, `GenitivePlural`, `DativeSingular`, `DativeDual`, `DativePlural`, `AccusativeSingular`, `AccusativeDual`, `AccusativePlural`, `InstrumentalSingular`, `InstrumentalDual`, `InstrumentalPlural`, `LocativeSingular`, `LocativeDual`, `LocativePlural`, `VocativeSingular`, `VocativeDual`, `VocativePlural`, `Diminutive`

Citation form: `NominativeSingular, GenitiveSingular` (comma-separated; parts you give are output exactly and narrow the prediction).

**adj** cells: `NominativeMasculineAnimate`, `NominativeMasculineInanimate`, `NominativeNeuter`, `NominativeFeminine`, `GenitiveMasculineAnimate`, `GenitiveMasculineInanimate`, `GenitiveNeuter`, `GenitiveFeminine`, `DativeMasculineAnimate`, `DativeMasculineInanimate`, `DativeNeuter`, `DativeFeminine`, `AccusativeMasculineAnimate`, `AccusativeMasculineInanimate`, `AccusativeNeuter`, `AccusativeFeminine`, `InstrumentalMasculineAnimate`, `InstrumentalMasculineInanimate`, `InstrumentalNeuter`, `InstrumentalFeminine`, `LocativeMasculineAnimate`, `LocativeMasculineInanimate`, `LocativeNeuter`, `LocativeFeminine`, `DualNominativeVirile`, `DualNominativeNonvirile`, `DualGenitiveVirile`, `DualGenitiveNonvirile`, `DualDativeVirile`, `DualDativeNonvirile`, `DualAccusativeVirile`, `DualAccusativeNonvirile`, `DualInstrumentalVirile`, `DualInstrumentalNonvirile`, `DualLocativeVirile`, `DualLocativeNonvirile`, `PluralNominativeVirile`, `PluralNominativeNonvirile`, `PluralGenitiveVirile`, `PluralGenitiveNonvirile`, `PluralDativeVirile`, `PluralDativeNonvirile`, `PluralAccusativeVirile`, `PluralAccusativeNonvirile`, `PluralInstrumentalVirile`, `PluralInstrumentalNonvirile`, `PluralLocativeVirile`, `PluralLocativeNonvirile`

Citation form: `NominativeMasculineInanimate, NominativeFeminine, NominativeNeuter` (comma-separated; parts you give are output exactly and narrow the prediction).

**verb** cells: `Infinitive`, `PresentFirstSingular`, `PresentSecondSingular`, `PresentThirdSingular`, `PresentFirstDual`, `PresentSecondDual`, `PresentSecondDualNonvirile`, `PresentThirdDual`, `PresentThirdDualNonvirile`, `PresentFirstPlural`, `PresentSecondPlural`, `PresentThirdPlural`, `PastFirstSingular`, `PastSecondSingular`, `PastThirdSingular`, `PastFirstDual`, `PastSecondDual`, `PastSecondDualNonvirile`, `PastThirdDual`, `PastThirdDualNonvirile`, `PastFirstPlural`, `PastSecondPlural`, `PastThirdPlural`, `FutureFirstSingular`, `FutureSecondSingular`, `FutureThirdSingular`, `FutureFirstDual`, `FutureSecondDual`, `FutureSecondDualNonvirile`, `FutureThirdDual`, `FutureThirdDualNonvirile`, `FutureFirstPlural`, `FutureSecondPlural`, `FutureThirdPlural`, `ConditionalFirstSingular`, `ConditionalSecondSingular`, `ConditionalThirdSingular`, `ConditionalFirstDual`, `ConditionalSecondDual`, `ConditionalSecondDualNonvirile`, `ConditionalThirdDual`, `ConditionalThirdDualNonvirile`, `ConditionalFirstPlural`, `ConditionalSecondPlural`, `ConditionalThirdPlural`, `ImperativeFirstSingular`, `ImperativeSecondSingular`, `ImperativeThirdSingular`, `ImperativeFirstDual`, `ImperativeSecondDual`, `ImperativeSecondDualNonvirile`, `ImperativeThirdDual`, `ImperativeThirdDualNonvirile`, `ImperativeFirstPlural`, `ImperativeSecondPlural`, `ImperativeThirdPlural`, `Transgressive`, `ActiveParticiple`, `PassiveParticiple`, `LParticiple`, `VerbalNoun`

Citation form: `Infinitive, PresentFirstSingular, PresentThirdSingular` (comma-separated; parts you give are output exactly and narrow the prediction).

## How it works

Each attested form is stored as an edit tree from the lemma. A lemma's edits across all cells make its inflection class. A suffix tree over the word, read from its end and pruned by a description-length test, predicts the class of any word. Known words the tree gets wrong are kept in a small lexicon. Confidences are smoothed counts; variant weights follow Wiktionary's listing order (1, 1/2, 1/3…, nonstandard forms a tenth of that).

## License

Code: MIT OR Apache-2.0. Data: derived from [Wiktionary](https://www.wiktionary.org/) (via [kaikki.org](https://kaikki.org/)), CC BY-SA 4.0; the model bytes in `src/model.bin` are shared under the same license.
