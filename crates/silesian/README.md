# silesian

Silesian inflection for Rust, learned from Wiktionary by [morphe](https://github.com/gold-silver-copper/morphe).
Inflects any word, known or made up. The `_scored` functions give every candidate with a confidence.

Generated from the Kaikki Silesian dump of 2026-10-03. Do not edit by hand: change the config in morphe and regenerate.

## Example

```rust
use silesian::*;
println!("{}", noun("pies", Case::Instrumental, Number::Singular)); // psym
println!("{}", verb_present("czekać", PersonNumber::SecondSingular)); // czekŏsz
println!("{}", verb_past("czekać", Person::First, GenderNumber::Feminine)); // czekałach
println!("{}", adj("ruby", Case::Nominative, SingularGender::Feminine)); // rubŏ
```

Periphrastic forms (future, conditional, the "niych" imperative) are left
out. Diacritics are optional on input.

## Coverage

| Part of speech | Entries | Cells | Edits | Classes | Tree nodes | Lexicon |
|---|---|---|---|---|---|---|
| noun | 346 | 16 | 437 | 374 | 100 | 262 |
| adj | 241 | 44 | 86 | 51 | 50 | 109 |
| verb | 290 | 45 | 691 | 148 | 65 | 150 |

Model size: 77225 bytes.

**noun** cells: `NominativeSingular`, `NominativePlural`, `GenitiveSingular`, `GenitivePlural`, `DativeSingular`, `DativePlural`, `AccusativeSingular`, `AccusativePlural`, `InstrumentalSingular`, `InstrumentalPlural`, `LocativeSingular`, `LocativePlural`, `VocativeSingular`, `VocativePlural`, `Diminutive`, `Augmentative`

Citation form: `NominativeSingular, GenitiveSingular` (comma-separated; parts you give are output exactly and narrow the prediction).

**adj** cells: `NominativeMasculine`, `NominativeMasculineAnimate`, `NominativeNeuter`, `NominativeFeminine`, `GenitiveMasculine`, `GenitiveMasculineAnimate`, `GenitiveNeuter`, `GenitiveFeminine`, `DativeMasculine`, `DativeMasculineAnimate`, `DativeNeuter`, `DativeFeminine`, `AccusativeMasculine`, `AccusativeMasculineAnimate`, `AccusativeNeuter`, `AccusativeFeminine`, `InstrumentalMasculine`, `InstrumentalMasculineAnimate`, `InstrumentalNeuter`, `InstrumentalFeminine`, `LocativeMasculine`, `LocativeMasculineAnimate`, `LocativeNeuter`, `LocativeFeminine`, `VocativeMasculine`, `VocativeMasculineAnimate`, `VocativeNeuter`, `VocativeFeminine`, `PluralNominativeVirile`, `PluralNominativeNonvirile`, `PluralGenitiveVirile`, `PluralGenitiveNonvirile`, `PluralDativeVirile`, `PluralDativeNonvirile`, `PluralAccusativeVirile`, `PluralAccusativeNonvirile`, `PluralInstrumentalVirile`, `PluralInstrumentalNonvirile`, `PluralLocativeVirile`, `PluralLocativeNonvirile`, `PluralVocativeVirile`, `PluralVocativeNonvirile`, `SimpleComparative`, `SimpleSuperlative`

Citation form: `NominativeMasculine, NominativeFeminine, NominativeNeuter` (comma-separated; parts you give are output exactly and narrow the prediction).

**verb** cells: `Infinitive`, `PresentFirstSingular`, `PresentSecondSingular`, `PresentThirdSingular`, `PresentFirstPlural`, `PresentSecondPlural`, `PresentThirdPlural`, `PastFirstMasculine`, `PastFirstFeminine`, `PastFirstNeuter`, `PastFirstVirile`, `PastFirstNonvirile`, `PastSecondMasculine`, `PastSecondFeminine`, `PastSecondNeuter`, `PastSecondVirile`, `PastSecondNonvirile`, `PastThirdMasculine`, `PastThirdFeminine`, `PastThirdNeuter`, `PastThirdVirile`, `PastThirdNonvirile`, `FutureFirstSingular`, `FutureSecondSingular`, `FutureThirdSingular`, `FutureFirstPlural`, `FutureSecondPlural`, `FutureThirdPlural`, `ImperativeFirstSingular`, `ImperativeSecondSingular`, `ImperativeThirdSingular`, `ImperativeFirstPlural`, `ImperativeSecondPlural`, `ImperativeThirdPlural`, `ActiveParticipleMasculine`, `ActiveParticipleFeminine`, `ActiveParticipleNeuter`, `ActiveParticipleVirile`, `ActiveParticipleNonvirile`, `PassiveParticipleMasculine`, `PassiveParticipleFeminine`, `PassiveParticipleNeuter`, `PassiveParticipleVirile`, `PassiveParticipleNonvirile`, `VerbalNoun`

Citation form: `Infinitive, PresentFirstSingular, PastThirdMasculine` (comma-separated; parts you give are output exactly and narrow the prediction).

## How it works

Each attested form is stored as an edit tree from the lemma. A lemma's edits across all cells make its inflection class. A suffix tree over the word, read from its end and pruned by a description-length test, predicts the class of any word. Known words the tree gets wrong are kept in a small lexicon. Confidences are smoothed counts; variant weights follow Wiktionary's listing order (1, 1/2, 1/3…, nonstandard forms a tenth of that).

## License

Code: MIT OR Apache-2.0. Data: derived from [Wiktionary](https://www.wiktionary.org/) (via [kaikki.org](https://kaikki.org/)), CC BY-SA 4.0; the model bytes in `src/model.bin` are shared under the same license.
