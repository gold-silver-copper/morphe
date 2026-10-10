# czech

Czech inflection for Rust, learned from Wiktionary by [morphe](https://github.com/gold-silver-copper/morphe).
Inflects any word, known or made up. The `_scored` functions give every candidate with a confidence.

Generated from the Kaikki Czech dump of 2026-10-03. Do not edit by hand: change the config in morphe and regenerate.

## Example

```rust
use czech::*;
println!("{}", noun("žena", Case::Instrumental, Number::Plural));
println!("{}", noun("hrad", Case::Locative, Number::Singular));
println!("{:?}", noun_gender("pes"));
println!("{}", verb_present("dělat", Person::Third, Number::Plural));
println!("{}", verb_past("být", Number::Plural, Gender::MasculineAnimate));
println!("{}", adj("mladý", Case::Genitive, Number::Singular, Gender::Feminine));
// Citations pin principal parts: nominative and genitive singular.
println!("{}", noun("pes, psa", Case::Dative, Number::Singular));
```

Input is matched as written: Czech diacritics (á, č, ě, ů…) are letters, not
optional marks, so they are not ignored.

## Coverage

| Part of speech | Entries | Cells | Edits | Classes | Tree nodes | Lexicon |
|---|---|---|---|---|---|---|
| noun | 31549 | 14 | 4732 | 2819 | 1396 | 9075 |
| adj | 14062 | 50 | 530 | 249 | 588 | 5594 |
| verb | 11757 | 35 | 3080 | 1719 | 698 | 3023 |
| pron | 66 | 56 | 318 | 68 | 10 | 26 |
| det | 25 | 48 | 82 | 25 | 5 | 9 |
| num | 145 | 48 | 73 | 31 | 8 | 76 |

Model size: 1081869 bytes.

**noun** cells: `NominativeSingular`, `NominativePlural`, `GenitiveSingular`, `GenitivePlural`, `DativeSingular`, `DativePlural`, `AccusativeSingular`, `AccusativePlural`, `InstrumentalSingular`, `InstrumentalPlural`, `LocativeSingular`, `LocativePlural`, `VocativeSingular`, `VocativePlural`

Citation form: `NominativeSingular, GenitiveSingular` (comma-separated; parts you give are output exactly and narrow the prediction).

**adj** cells: `NominativeSingularMasculineAnimate`, `NominativeSingularMasculineInanimate`, `NominativeSingularFeminine`, `NominativeSingularNeuter`, `NominativePluralMasculineAnimate`, `NominativePluralMasculineInanimate`, `NominativePluralFeminine`, `NominativePluralNeuter`, `GenitiveSingularMasculineAnimate`, `GenitiveSingularMasculineInanimate`, `GenitiveSingularFeminine`, `GenitiveSingularNeuter`, `GenitivePluralMasculineAnimate`, `GenitivePluralMasculineInanimate`, `GenitivePluralFeminine`, `GenitivePluralNeuter`, `DativeSingularMasculineAnimate`, `DativeSingularMasculineInanimate`, `DativeSingularFeminine`, `DativeSingularNeuter`, `DativePluralMasculineAnimate`, `DativePluralMasculineInanimate`, `DativePluralFeminine`, `DativePluralNeuter`, `AccusativeSingularMasculineAnimate`, `AccusativeSingularMasculineInanimate`, `AccusativeSingularFeminine`, `AccusativeSingularNeuter`, `AccusativePluralMasculineAnimate`, `AccusativePluralMasculineInanimate`, `AccusativePluralFeminine`, `AccusativePluralNeuter`, `InstrumentalSingularMasculineAnimate`, `InstrumentalSingularMasculineInanimate`, `InstrumentalSingularFeminine`, `InstrumentalSingularNeuter`, `InstrumentalPluralMasculineAnimate`, `InstrumentalPluralMasculineInanimate`, `InstrumentalPluralFeminine`, `InstrumentalPluralNeuter`, `LocativeSingularMasculineAnimate`, `LocativeSingularMasculineInanimate`, `LocativeSingularFeminine`, `LocativeSingularNeuter`, `LocativePluralMasculineAnimate`, `LocativePluralMasculineInanimate`, `LocativePluralFeminine`, `LocativePluralNeuter`, `Comparative`, `Superlative`

**verb** cells: `Infinitive`, `PresentFirstSingular`, `PresentFirstPlural`, `PresentSecondSingular`, `PresentSecondPlural`, `PresentThirdSingular`, `PresentThirdPlural`, `ImperativeSingular`, `ImperativePlural`, `ImperativeFirstPlural`, `PastSingularMasculineAnimate`, `PastSingularMasculineInanimate`, `PastSingularFeminine`, `PastSingularNeuter`, `PastPluralMasculineAnimate`, `PastPluralMasculineInanimate`, `PastPluralFeminine`, `PastPluralNeuter`, `PassiveSingularMasculineAnimate`, `PassiveSingularMasculineInanimate`, `PassiveSingularFeminine`, `PassiveSingularNeuter`, `PassivePluralMasculineAnimate`, `PassivePluralMasculineInanimate`, `PassivePluralFeminine`, `PassivePluralNeuter`, `TransgressivePresentMasculineSingular`, `TransgressivePresentFeminineNeuterSingular`, `TransgressivePresentPlural`, `TransgressivePastMasculineSingular`, `TransgressivePastFeminineNeuterSingular`, `TransgressivePastPlural`, `ParticipleActive`, `ParticiplePassive`, `VerbalNoun`

Citation form: `Infinitive, PresentFirstSingular, PastSingularMasculineAnimate` (comma-separated; parts you give are output exactly and narrow the prediction).

**pron** cells: `NominativeSingularMasculineAnimate`, `NominativeSingularMasculineInanimate`, `NominativeSingularFeminine`, `NominativeSingularNeuter`, `NominativePluralMasculineAnimate`, `NominativePluralMasculineInanimate`, `NominativePluralFeminine`, `NominativePluralNeuter`, `GenitiveSingularMasculineAnimate`, `GenitiveSingularMasculineInanimate`, `GenitiveSingularFeminine`, `GenitiveSingularNeuter`, `GenitivePluralMasculineAnimate`, `GenitivePluralMasculineInanimate`, `GenitivePluralFeminine`, `GenitivePluralNeuter`, `DativeSingularMasculineAnimate`, `DativeSingularMasculineInanimate`, `DativeSingularFeminine`, `DativeSingularNeuter`, `DativePluralMasculineAnimate`, `DativePluralMasculineInanimate`, `DativePluralFeminine`, `DativePluralNeuter`, `AccusativeSingularMasculineAnimate`, `AccusativeSingularMasculineInanimate`, `AccusativeSingularFeminine`, `AccusativeSingularNeuter`, `AccusativePluralMasculineAnimate`, `AccusativePluralMasculineInanimate`, `AccusativePluralFeminine`, `AccusativePluralNeuter`, `InstrumentalSingularMasculineAnimate`, `InstrumentalSingularMasculineInanimate`, `InstrumentalSingularFeminine`, `InstrumentalSingularNeuter`, `InstrumentalPluralMasculineAnimate`, `InstrumentalPluralMasculineInanimate`, `InstrumentalPluralFeminine`, `InstrumentalPluralNeuter`, `LocativeSingularMasculineAnimate`, `LocativeSingularMasculineInanimate`, `LocativeSingularFeminine`, `LocativeSingularNeuter`, `LocativePluralMasculineAnimate`, `LocativePluralMasculineInanimate`, `LocativePluralFeminine`, `LocativePluralNeuter`, `VocativeSingularMasculineAnimate`, `VocativeSingularMasculineInanimate`, `VocativeSingularFeminine`, `VocativeSingularNeuter`, `VocativePluralMasculineAnimate`, `VocativePluralMasculineInanimate`, `VocativePluralFeminine`, `VocativePluralNeuter`

**det** cells: `NominativeSingularMasculineAnimate`, `NominativeSingularMasculineInanimate`, `NominativeSingularFeminine`, `NominativeSingularNeuter`, `NominativePluralMasculineAnimate`, `NominativePluralMasculineInanimate`, `NominativePluralFeminine`, `NominativePluralNeuter`, `GenitiveSingularMasculineAnimate`, `GenitiveSingularMasculineInanimate`, `GenitiveSingularFeminine`, `GenitiveSingularNeuter`, `GenitivePluralMasculineAnimate`, `GenitivePluralMasculineInanimate`, `GenitivePluralFeminine`, `GenitivePluralNeuter`, `DativeSingularMasculineAnimate`, `DativeSingularMasculineInanimate`, `DativeSingularFeminine`, `DativeSingularNeuter`, `DativePluralMasculineAnimate`, `DativePluralMasculineInanimate`, `DativePluralFeminine`, `DativePluralNeuter`, `AccusativeSingularMasculineAnimate`, `AccusativeSingularMasculineInanimate`, `AccusativeSingularFeminine`, `AccusativeSingularNeuter`, `AccusativePluralMasculineAnimate`, `AccusativePluralMasculineInanimate`, `AccusativePluralFeminine`, `AccusativePluralNeuter`, `InstrumentalSingularMasculineAnimate`, `InstrumentalSingularMasculineInanimate`, `InstrumentalSingularFeminine`, `InstrumentalSingularNeuter`, `InstrumentalPluralMasculineAnimate`, `InstrumentalPluralMasculineInanimate`, `InstrumentalPluralFeminine`, `InstrumentalPluralNeuter`, `LocativeSingularMasculineAnimate`, `LocativeSingularMasculineInanimate`, `LocativeSingularFeminine`, `LocativeSingularNeuter`, `LocativePluralMasculineAnimate`, `LocativePluralMasculineInanimate`, `LocativePluralFeminine`, `LocativePluralNeuter`

**num** cells: `NominativeSingularMasculineAnimate`, `NominativeSingularMasculineInanimate`, `NominativeSingularFeminine`, `NominativeSingularNeuter`, `NominativePluralMasculineAnimate`, `NominativePluralMasculineInanimate`, `NominativePluralFeminine`, `NominativePluralNeuter`, `GenitiveSingularMasculineAnimate`, `GenitiveSingularMasculineInanimate`, `GenitiveSingularFeminine`, `GenitiveSingularNeuter`, `GenitivePluralMasculineAnimate`, `GenitivePluralMasculineInanimate`, `GenitivePluralFeminine`, `GenitivePluralNeuter`, `DativeSingularMasculineAnimate`, `DativeSingularMasculineInanimate`, `DativeSingularFeminine`, `DativeSingularNeuter`, `DativePluralMasculineAnimate`, `DativePluralMasculineInanimate`, `DativePluralFeminine`, `DativePluralNeuter`, `AccusativeSingularMasculineAnimate`, `AccusativeSingularMasculineInanimate`, `AccusativeSingularFeminine`, `AccusativeSingularNeuter`, `AccusativePluralMasculineAnimate`, `AccusativePluralMasculineInanimate`, `AccusativePluralFeminine`, `AccusativePluralNeuter`, `InstrumentalSingularMasculineAnimate`, `InstrumentalSingularMasculineInanimate`, `InstrumentalSingularFeminine`, `InstrumentalSingularNeuter`, `InstrumentalPluralMasculineAnimate`, `InstrumentalPluralMasculineInanimate`, `InstrumentalPluralFeminine`, `InstrumentalPluralNeuter`, `LocativeSingularMasculineAnimate`, `LocativeSingularMasculineInanimate`, `LocativeSingularFeminine`, `LocativeSingularNeuter`, `LocativePluralMasculineAnimate`, `LocativePluralMasculineInanimate`, `LocativePluralFeminine`, `LocativePluralNeuter`

## How it works

Each attested form is stored as an edit tree from the lemma. A lemma's edits across all cells make its inflection class. A suffix tree over the word, read from its end and pruned by a description-length test, predicts the class of any word. Known words the tree gets wrong are kept in a small lexicon. Confidences are smoothed counts; variant weights follow Wiktionary's listing order (1, 1/2, 1/3…, nonstandard forms a tenth of that).

## License

Code: MIT OR Apache-2.0. Data: derived from [Wiktionary](https://www.wiktionary.org/) (via [kaikki.org](https://kaikki.org/)), CC BY-SA 4.0; the model bytes in `src/model.bin` are shared under the same license.
