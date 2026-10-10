# kannada

Kannada inflection for Rust, learned from Wiktionary by [morphe](https://github.com/gold-silver-copper/morphe).
Inflects any word, known or made up. The `_scored` functions give every candidate with a confidence.

Generated from the Kaikki Kannada dump of 2026-10-03. Do not edit by hand: change the config in morphe and regenerate.

## Example

```rust
use kannada::*;
println!("{}", noun("ನುಡಿ", Case::Dative, Number::Singular)); // ನುಡಿಗೆ
println!("{}", noun("ಅನಿಲ", Case::Genitive, Number::Plural)); // ಅನಿಲಗಳ
println!("{}", verb("ಟೊಣೆ", Tense::Present, Person::FirstSingular)); // ಟೊಣೆಯುತ್ತೇನೆ
println!("{}", verb("ಟೊಣೆ", Tense::Future, Person::ThirdFeminine)); // ಟೊಣೆಯುವಳು
println!("{}", verb_infinitive("ಟೊಣೆ")); // ಟೊಣೆಯಲು
```

Nouns have five cases in both numbers. Verbs are cited by the imperative
singular, as in Wiktionary, and have the present, past, future, negative
and contingent for nine persons (the third person distinguishes masculine,
feminine and neuter singular, and human and neuter plural), the
infinitives, conditional, imperatives, cohortatives, optative and the
adverbial and adjectival participles.

## Coverage

| Part of speech | Entries | Cells | Edits | Classes | Tree nodes | Lexicon |
|---|---|---|---|---|---|---|
| noun | 1036 | 10 | 138 | 18 | 28 | 66 |
| verb | 251 | 60 | 615 | 64 | 22 | 153 |

Model size: 59817 bytes.

**noun** cells: `NominativeSingular`, `NominativePlural`, `AccusativeSingular`, `AccusativePlural`, `DativeSingular`, `DativePlural`, `GenitiveSingular`, `GenitivePlural`, `InstrumentalSingular`, `InstrumentalPlural`

Citation form: `NominativeSingular, NominativePlural` (comma-separated; parts you give are output exactly and narrow the prediction).

**verb** cells: `Infinitive`, `DativeInfinitive`, `Conditional`, `ImperativeSingular`, `ImperativePlural`, `Cohortative`, `CohortativeII`, `Optative`, `Hortative`, `AdverbialParticiplePresent`, `AdverbialParticiplePast`, `AdverbialParticipleNegative`, `AdjectivalParticipleNonPast`, `AdjectivalParticiplePast`, `AdjectivalParticipleNegative`, `PresentFirstSingular`, `PresentSecondSingular`, `PresentThirdMasculine`, `PresentThirdFeminine`, `PresentThirdNeuter`, `PresentFirstPlural`, `PresentSecondPlural`, `PresentThirdPlural`, `PresentThirdNeuterPlural`, `PastFirstSingular`, `PastSecondSingular`, `PastThirdMasculine`, `PastThirdFeminine`, `PastThirdNeuter`, `PastFirstPlural`, `PastSecondPlural`, `PastThirdPlural`, `PastThirdNeuterPlural`, `FutureFirstSingular`, `FutureSecondSingular`, `FutureThirdMasculine`, `FutureThirdFeminine`, `FutureThirdNeuter`, `FutureFirstPlural`, `FutureSecondPlural`, `FutureThirdPlural`, `FutureThirdNeuterPlural`, `NegativeFirstSingular`, `NegativeSecondSingular`, `NegativeThirdMasculine`, `NegativeThirdFeminine`, `NegativeThirdNeuter`, `NegativeFirstPlural`, `NegativeSecondPlural`, `NegativeThirdPlural`, `NegativeThirdNeuterPlural`, `ContingentFirstSingular`, `ContingentSecondSingular`, `ContingentThirdMasculine`, `ContingentThirdFeminine`, `ContingentThirdNeuter`, `ContingentFirstPlural`, `ContingentSecondPlural`, `ContingentThirdPlural`, `ContingentThirdNeuterPlural`

Citation form: `ImperativeSingular, PastFirstSingular` (comma-separated; parts you give are output exactly and narrow the prediction).

## How it works

Each attested form is stored as an edit tree from the lemma. A lemma's edits across all cells make its inflection class. A suffix tree over the word, read from its end and pruned by a description-length test, predicts the class of any word. Known words the tree gets wrong are kept in a small lexicon. Confidences are smoothed counts; variant weights follow Wiktionary's listing order (1, 1/2, 1/3…, nonstandard forms a tenth of that).

## License

Code: MIT OR Apache-2.0. Data: derived from [Wiktionary](https://www.wiktionary.org/) (via [kaikki.org](https://kaikki.org/)), CC BY-SA 4.0; the model bytes in `src/model.bin` are shared under the same license.
