# old_saxon

OldSaxon inflection for Rust, learned from Wiktionary by [morphe](https://github.com/gold-silver-copper/morphe).
Inflects any word, known or made up. The `_scored` functions give every candidate with a confidence.

Generated from the Kaikki OldSaxon dump of 2026-10-03. Do not edit by hand: change the config in morphe and regenerate.

## Example

```rust
use old_saxon::*;
assert_eq!(noun("dag", Case::Dative, Number::Plural), "dagum");
println!("{:?}", noun_gender("word"));
println!("{}", verb("kritan", Mood::Indicative, Tense::Past, PersonNumber::ThirdSingular)); // krēt
println!("{}", verb("kritan", Mood::Subjunctive, Tense::Present, PersonNumber::Plural)); // krīten
```

Spelling follows Wiktionary's normalized Old Saxon. Length marks are
optional on input. The finite plural has one form for all persons.

## Coverage

| Part of speech | Entries | Cells | Edits | Classes | Tree nodes | Lexicon |
|---|---|---|---|---|---|---|
| noun | 311 | 10 | 224 | 90 | 67 | 193 |
| adj | 142 | 60 | 403 | 36 | 48 | 81 |
| verb | 664 | 19 | 1121 | 427 | 163 | 486 |

Model size: 129103 bytes.

**noun** cells: `NominativeSingular`, `NominativePlural`, `AccusativeSingular`, `AccusativePlural`, `GenitiveSingular`, `GenitivePlural`, `DativeSingular`, `DativePlural`, `InstrumentalSingular`, `InstrumentalPlural`

Citation form: `Lemma, GenitiveSingular, NominativePlural` (comma-separated; parts you give are output exactly and narrow the prediction).

**adj** cells: `NominativeSingularMasculine`, `NominativeSingularFeminine`, `NominativeSingularNeuter`, `NominativePluralMasculine`, `NominativePluralFeminine`, `NominativePluralNeuter`, `AccusativeSingularMasculine`, `AccusativeSingularFeminine`, `AccusativeSingularNeuter`, `AccusativePluralMasculine`, `AccusativePluralFeminine`, `AccusativePluralNeuter`, `GenitiveSingularMasculine`, `GenitiveSingularFeminine`, `GenitiveSingularNeuter`, `GenitivePluralMasculine`, `GenitivePluralFeminine`, `GenitivePluralNeuter`, `DativeSingularMasculine`, `DativeSingularFeminine`, `DativeSingularNeuter`, `DativePluralMasculine`, `DativePluralFeminine`, `DativePluralNeuter`, `InstrumentalSingularMasculine`, `InstrumentalSingularFeminine`, `InstrumentalSingularNeuter`, `InstrumentalPluralMasculine`, `InstrumentalPluralFeminine`, `InstrumentalPluralNeuter`, `WeakNominativeSingularMasculine`, `WeakNominativeSingularFeminine`, `WeakNominativeSingularNeuter`, `WeakNominativePluralMasculine`, `WeakNominativePluralFeminine`, `WeakNominativePluralNeuter`, `WeakAccusativeSingularMasculine`, `WeakAccusativeSingularFeminine`, `WeakAccusativeSingularNeuter`, `WeakAccusativePluralMasculine`, `WeakAccusativePluralFeminine`, `WeakAccusativePluralNeuter`, `WeakGenitiveSingularMasculine`, `WeakGenitiveSingularFeminine`, `WeakGenitiveSingularNeuter`, `WeakGenitivePluralMasculine`, `WeakGenitivePluralFeminine`, `WeakGenitivePluralNeuter`, `WeakDativeSingularMasculine`, `WeakDativeSingularFeminine`, `WeakDativeSingularNeuter`, `WeakDativePluralMasculine`, `WeakDativePluralFeminine`, `WeakDativePluralNeuter`, `WeakInstrumentalSingularMasculine`, `WeakInstrumentalSingularFeminine`, `WeakInstrumentalSingularNeuter`, `WeakInstrumentalPluralMasculine`, `WeakInstrumentalPluralFeminine`, `WeakInstrumentalPluralNeuter`

**verb** cells: `IndicativePresentFirstSingular`, `IndicativePresentSecondSingular`, `IndicativePresentThirdSingular`, `IndicativePresentPlural`, `IndicativePastFirstSingular`, `IndicativePastSecondSingular`, `IndicativePastThirdSingular`, `IndicativePastPlural`, `SubjunctivePresentFirstSingular`, `SubjunctivePresentSecondSingular`, `SubjunctivePresentThirdSingular`, `SubjunctivePresentPlural`, `SubjunctivePastFirstSingular`, `SubjunctivePastSecondSingular`, `SubjunctivePastThirdSingular`, `SubjunctivePastPlural`, `Imperative`, `ParticiplePresent`, `ParticiplePast`

Citation form: `Lemma, IndicativePastThirdSingular, IndicativePastPlural, ParticiplePast` (comma-separated; parts you give are output exactly and narrow the prediction).

## How it works

Each attested form is stored as an edit tree from the lemma. A lemma's edits across all cells make its inflection class. A suffix tree over the word, read from its end and pruned by a description-length test, predicts the class of any word. Known words the tree gets wrong are kept in a small lexicon. Confidences are smoothed counts; variant weights follow Wiktionary's listing order (1, 1/2, 1/3…, nonstandard forms a tenth of that).

## License

Code: MIT OR Apache-2.0. Data: derived from [Wiktionary](https://www.wiktionary.org/) (via [kaikki.org](https://kaikki.org/)), CC BY-SA 4.0; the model bytes in `src/model.bin` are shared under the same license.
