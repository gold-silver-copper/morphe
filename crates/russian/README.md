# russian

Russian inflection for Rust, learned from Wiktionary by [morphe](https://github.com/gold-silver-copper/morphe).
Inflects any word, known or made up. The `_scored` functions give every candidate with a confidence.

Generated from the Kaikki Russian dump of 2026-10-03. Do not edit by hand: change the config in morphe and regenerate.

## Example

```rust
use russian::*;
assert_eq!(noun("книга", Case::Genitive, Number::Plural), "книг");
let plain = Russian::default().marks(false);
println!("{}", plain.noun("стол", Case::Instrumental, Number::Singular));
println!("{:?}", noun_gender("тетрадь"));
println!("{}", verb("писать", Tense::Present, Person::First, Number::Singular));
// Citations pin principal parts: nominative and genitive singular.
println!("{}", noun("путь, пути", Case::Instrumental, Number::Singular));
```

Input matching ignores stress marks and treats ё as е. Output carries stress
marks where Wiktionary has them (`marks(false)` removes them). Unknown words
get their stress predicted from similar words, which is often wrong.

## Coverage

| Part of speech | Entries | Cells | Edits | Classes | Tree nodes | Lexicon |
|---|---|---|---|---|---|---|
| noun | 28413 | 18 | 11067 | 3924 | 3920 | 13380 |
| adj | 9764 | 54 | 1466 | 649 | 1132 | 4128 |
| verb | 12824 | 25 | 4313 | 2259 | 2152 | 5487 |
| pron | 99 | 48 | 3005 | 139 | 53 | 71 |
| num | 151 | 36 | 308 | 131 | 31 | 105 |

Model size: 4472085 bytes.

**noun** cells: `NominativeSingular`, `NominativePlural`, `GenitiveSingular`, `GenitivePlural`, `DativeSingular`, `DativePlural`, `AccusativeSingular`, `AccusativePlural`, `InstrumentalSingular`, `InstrumentalPlural`, `PrepositionalSingular`, `PrepositionalPlural`, `PartitiveSingular`, `PartitivePlural`, `LocativeSingular`, `LocativePlural`, `VocativeSingular`, `VocativePlural`

Citation form: `NominativeSingular, GenitiveSingular` (comma-separated; parts you give are output exactly and narrow the prediction).

**adj** cells: `NominativeMasculineAnimate`, `NominativeMasculineInanimate`, `NominativeFeminineAnimate`, `NominativeFeminineInanimate`, `NominativeNeuterAnimate`, `NominativeNeuterInanimate`, `NominativePluralAnimate`, `NominativePluralInanimate`, `GenitiveMasculineAnimate`, `GenitiveMasculineInanimate`, `GenitiveFeminineAnimate`, `GenitiveFeminineInanimate`, `GenitiveNeuterAnimate`, `GenitiveNeuterInanimate`, `GenitivePluralAnimate`, `GenitivePluralInanimate`, `DativeMasculineAnimate`, `DativeMasculineInanimate`, `DativeFeminineAnimate`, `DativeFeminineInanimate`, `DativeNeuterAnimate`, `DativeNeuterInanimate`, `DativePluralAnimate`, `DativePluralInanimate`, `AccusativeMasculineAnimate`, `AccusativeMasculineInanimate`, `AccusativeFeminineAnimate`, `AccusativeFeminineInanimate`, `AccusativeNeuterAnimate`, `AccusativeNeuterInanimate`, `AccusativePluralAnimate`, `AccusativePluralInanimate`, `InstrumentalMasculineAnimate`, `InstrumentalMasculineInanimate`, `InstrumentalFeminineAnimate`, `InstrumentalFeminineInanimate`, `InstrumentalNeuterAnimate`, `InstrumentalNeuterInanimate`, `InstrumentalPluralAnimate`, `InstrumentalPluralInanimate`, `PrepositionalMasculineAnimate`, `PrepositionalMasculineInanimate`, `PrepositionalFeminineAnimate`, `PrepositionalFeminineInanimate`, `PrepositionalNeuterAnimate`, `PrepositionalNeuterInanimate`, `PrepositionalPluralAnimate`, `PrepositionalPluralInanimate`, `ShortMasculine`, `ShortFeminine`, `ShortNeuter`, `ShortPlural`, `Comparative`, `Superlative`

**verb** cells: `Infinitive`, `PresentFirstSingular`, `PresentFirstPlural`, `PresentSecondSingular`, `PresentSecondPlural`, `PresentThirdSingular`, `PresentThirdPlural`, `FutureFirstSingular`, `FutureFirstPlural`, `FutureSecondSingular`, `FutureSecondPlural`, `FutureThirdSingular`, `FutureThirdPlural`, `PastMasculine`, `PastFeminine`, `PastNeuter`, `PastPlural`, `ImperativeSingular`, `ImperativePlural`, `ParticipleActivePresent`, `ParticipleActivePast`, `ParticiplePassivePresent`, `ParticiplePassivePast`, `GerundPresent`, `GerundPast`

Citation form: `Infinitive, PresentFirstSingular|FutureFirstSingular, PresentSecondSingular|FutureSecondSingular` (comma-separated; parts you give are output exactly and narrow the prediction).

**pron** cells: `NominativeMasculineAnimate`, `NominativeMasculineInanimate`, `NominativeFeminineAnimate`, `NominativeFeminineInanimate`, `NominativeNeuterAnimate`, `NominativeNeuterInanimate`, `NominativePluralAnimate`, `NominativePluralInanimate`, `GenitiveMasculineAnimate`, `GenitiveMasculineInanimate`, `GenitiveFeminineAnimate`, `GenitiveFeminineInanimate`, `GenitiveNeuterAnimate`, `GenitiveNeuterInanimate`, `GenitivePluralAnimate`, `GenitivePluralInanimate`, `DativeMasculineAnimate`, `DativeMasculineInanimate`, `DativeFeminineAnimate`, `DativeFeminineInanimate`, `DativeNeuterAnimate`, `DativeNeuterInanimate`, `DativePluralAnimate`, `DativePluralInanimate`, `AccusativeMasculineAnimate`, `AccusativeMasculineInanimate`, `AccusativeFeminineAnimate`, `AccusativeFeminineInanimate`, `AccusativeNeuterAnimate`, `AccusativeNeuterInanimate`, `AccusativePluralAnimate`, `AccusativePluralInanimate`, `InstrumentalMasculineAnimate`, `InstrumentalMasculineInanimate`, `InstrumentalFeminineAnimate`, `InstrumentalFeminineInanimate`, `InstrumentalNeuterAnimate`, `InstrumentalNeuterInanimate`, `InstrumentalPluralAnimate`, `InstrumentalPluralInanimate`, `PrepositionalMasculineAnimate`, `PrepositionalMasculineInanimate`, `PrepositionalFeminineAnimate`, `PrepositionalFeminineInanimate`, `PrepositionalNeuterAnimate`, `PrepositionalNeuterInanimate`, `PrepositionalPluralAnimate`, `PrepositionalPluralInanimate`

**num** cells: `NominativeMasculineAnimate`, `NominativeMasculineInanimate`, `NominativeFeminineAnimate`, `NominativeFeminineInanimate`, `NominativeNeuterAnimate`, `NominativeNeuterInanimate`, `GenitiveMasculineAnimate`, `GenitiveMasculineInanimate`, `GenitiveFeminineAnimate`, `GenitiveFeminineInanimate`, `GenitiveNeuterAnimate`, `GenitiveNeuterInanimate`, `DativeMasculineAnimate`, `DativeMasculineInanimate`, `DativeFeminineAnimate`, `DativeFeminineInanimate`, `DativeNeuterAnimate`, `DativeNeuterInanimate`, `AccusativeMasculineAnimate`, `AccusativeMasculineInanimate`, `AccusativeFeminineAnimate`, `AccusativeFeminineInanimate`, `AccusativeNeuterAnimate`, `AccusativeNeuterInanimate`, `InstrumentalMasculineAnimate`, `InstrumentalMasculineInanimate`, `InstrumentalFeminineAnimate`, `InstrumentalFeminineInanimate`, `InstrumentalNeuterAnimate`, `InstrumentalNeuterInanimate`, `PrepositionalMasculineAnimate`, `PrepositionalMasculineInanimate`, `PrepositionalFeminineAnimate`, `PrepositionalFeminineInanimate`, `PrepositionalNeuterAnimate`, `PrepositionalNeuterInanimate`

## How it works

Each attested form is stored as an edit tree from the lemma. A lemma's edits across all cells make its inflection class. A suffix tree over the word, read from its end and pruned by a description-length test, predicts the class of any word. Known words the tree gets wrong are kept in a small lexicon. Confidences are smoothed counts; variant weights follow Wiktionary's listing order (1, 1/2, 1/3…, nonstandard forms a tenth of that).

## License

Code: MIT OR Apache-2.0. Data: derived from [Wiktionary](https://www.wiktionary.org/) (via [kaikki.org](https://kaikki.org/)), CC BY-SA 4.0; the model bytes in `src/model.bin` are shared under the same license.
