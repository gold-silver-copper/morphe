# telugu

Telugu inflection for Rust, learned from Wiktionary by [morphe](https://github.com/gold-silver-copper/morphe).
Inflects any word, known or made up. The `_scored` functions give every candidate with a confidence.

Generated from the Kaikki Telugu dump of 2026-10-03. Do not edit by hand: change the config in morphe and regenerate.

## Example

```rust
use telugu::*;
println!("{}", noun("ఇల్లు", Case::Dative, Number::Singular)); // ఇంటికి
println!("{}", noun("పుస్తకం", Case::Nominative, Number::Plural)); // పుస్తకాలు
println!("{}", verb("అరుచు", Tense::Past, Person::FirstSingular)); // అరిచాను
println!("{}", verb("అరుచు", Tense::Future, Person::ThirdMasculine)); // అరుస్తాడు
println!("{}", verb("విస్తరించు", Tense::Past, Person::SecondPlural)); // విస్తరించారు
```

Nouns have eight cases in both numbers, though Wiktionary gives most nouns
only their plural. Verbs are cited by the imperative singular, as in
Wiktionary, and have the past for eight persons; a few have full tables
with the future, the present progressive, the negative, the infinitive,
conditional, past participle, imperatives and hortative. The second person
plural also serves the human third person plural (వారు).

## Coverage

| Part of speech | Entries | Cells | Edits | Classes | Tree nodes | Lexicon |
|---|---|---|---|---|---|---|
| noun | 15891 | 16 | 112 | 83 | 235 | 1158 |
| verb | 2488 | 38 | 271 | 56 | 32 | 158 |

Model size: 93380 bytes.

**noun** cells: `NominativeSingular`, `NominativePlural`, `AccusativeSingular`, `AccusativePlural`, `DativeSingular`, `DativePlural`, `GenitiveSingular`, `GenitivePlural`, `InstrumentalSingular`, `InstrumentalPlural`, `LocativeSingular`, `LocativePlural`, `AblativeSingular`, `AblativePlural`, `VocativeSingular`, `VocativePlural`

Citation form: `NominativeSingular, NominativePlural` (comma-separated; parts you give are output exactly and narrow the prediction).

**verb** cells: `Infinitive`, `Conditional`, `PastParticiple`, `ImperativeSingular`, `ImperativePlural`, `Hortative`, `PastFirstSingular`, `PastSecondSingular`, `PastThirdMasculine`, `PastThirdFeminine`, `PastThirdNeuter`, `PastFirstPlural`, `PastSecondPlural`, `PastThirdNeuterPlural`, `FutureFirstSingular`, `FutureSecondSingular`, `FutureThirdMasculine`, `FutureThirdFeminine`, `FutureThirdNeuter`, `FutureFirstPlural`, `FutureSecondPlural`, `FutureThirdNeuterPlural`, `PresentFirstSingular`, `PresentSecondSingular`, `PresentThirdMasculine`, `PresentThirdFeminine`, `PresentThirdNeuter`, `PresentFirstPlural`, `PresentSecondPlural`, `PresentThirdNeuterPlural`, `NegativeFirstSingular`, `NegativeSecondSingular`, `NegativeThirdMasculine`, `NegativeThirdFeminine`, `NegativeThirdNeuter`, `NegativeFirstPlural`, `NegativeSecondPlural`, `NegativeThirdNeuterPlural`

Citation form: `ImperativeSingular, PastFirstSingular` (comma-separated; parts you give are output exactly and narrow the prediction).

## How it works

Each attested form is stored as an edit tree from the lemma. A lemma's edits across all cells make its inflection class. A suffix tree over the word, read from its end and pruned by a description-length test, predicts the class of any word. Known words the tree gets wrong are kept in a small lexicon. Confidences are smoothed counts; variant weights follow Wiktionary's listing order (1, 1/2, 1/3…, nonstandard forms a tenth of that).

## License

Code: MIT OR Apache-2.0. Data: derived from [Wiktionary](https://www.wiktionary.org/) (via [kaikki.org](https://kaikki.org/)), CC BY-SA 4.0; the model bytes in `src/model.bin` are shared under the same license.
