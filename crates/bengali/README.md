# bengali

Bengali inflection for Rust, learned from Wiktionary by [morphe](https://github.com/gold-silver-copper/morphe).
Inflects any word, known or made up. The `_scored` functions give every candidate with a confidence.

Generated from the Kaikki Bengali dump of 2026-10-03. Do not edit by hand: change the config in morphe and regenerate.

## Example

```rust
use bengali::*;
println!("{}", noun_indefinite("চোখ", Case::Locative)); // চোখে
println!("{}", noun_definite("ছেলে", Case::Nominative, Number::Plural)); // ছেলেরা
println!("{}", noun_definite("মাস", Case::Genitive, Number::Singular)); // মাসটার
println!("{}", verb("করা", Tense::Past, Person::First)); // করলাম
println!("{}", verb("করা", Tense::Future, Person::Polite)); // করবেন
println!("{}", verb("করা", Tense::Present, Person::SecondIntimate)); // করিস
println!("{}", verb_participle("করা", Participle::Perfect)); // করে
```

Nouns have the indefinite nominative, objective, genitive and locative,
and the definite (with the classifier -টা/-টি, plural -গুলো/-রা) in both
numbers. Verbs are cited by the verbal noun (করা) and have eight tenses
for the five persons (আমি; তুই; তুমি; সে; আপনি/তিনি), the infinitive and
the perfect and conditional participles. The reduplicated participles
(করতে-করতে, করে-করে) are the infinitive and the perfect participle doubled. Forms are the standard colloquial language; the literary
sadhu tables are left out.

## Coverage

| Part of speech | Entries | Cells | Edits | Classes | Tree nodes | Lexicon |
|---|---|---|---|---|---|---|
| noun | 6312 | 12 | 684 | 368 | 323 | 1643 |
| verb | 313 | 44 | 1241 | 181 | 44 | 140 |
| pron | 184 | 28 | 273 | 58 | 20 | 44 |

Model size: 409063 bytes.

**noun** cells: `IndefiniteNominative`, `IndefiniteObjective`, `IndefiniteGenitive`, `IndefiniteLocative`, `DefiniteNominativeSingular`, `DefiniteNominativePlural`, `DefiniteObjectiveSingular`, `DefiniteObjectivePlural`, `DefiniteGenitiveSingular`, `DefiniteGenitivePlural`, `DefiniteLocativeSingular`, `DefiniteLocativePlural`

Citation form: `IndefiniteNominative, IndefiniteObjective` (comma-separated; parts you give are output exactly and narrow the prediction).

**verb** cells: `VerbalNoun`, `Infinitive`, `ParticiplePerfect`, `ParticipleConditional`, `PresentFirst`, `PresentSecondIntimate`, `PresentSecondFamiliar`, `PresentThird`, `PresentPolite`, `PresentContinuousFirst`, `PresentContinuousSecondIntimate`, `PresentContinuousSecondFamiliar`, `PresentContinuousThird`, `PresentContinuousPolite`, `PresentPerfectFirst`, `PresentPerfectSecondIntimate`, `PresentPerfectSecondFamiliar`, `PresentPerfectThird`, `PresentPerfectPolite`, `PastFirst`, `PastSecondIntimate`, `PastSecondFamiliar`, `PastThird`, `PastPolite`, `PastContinuousFirst`, `PastContinuousSecondIntimate`, `PastContinuousSecondFamiliar`, `PastContinuousThird`, `PastContinuousPolite`, `PastPerfectFirst`, `PastPerfectSecondIntimate`, `PastPerfectSecondFamiliar`, `PastPerfectThird`, `PastPerfectPolite`, `PastHabitualFirst`, `PastHabitualSecondIntimate`, `PastHabitualSecondFamiliar`, `PastHabitualThird`, `PastHabitualPolite`, `FutureFirst`, `FutureSecondIntimate`, `FutureSecondFamiliar`, `FutureThird`, `FuturePolite`

**pron** cells: `Nominative`, `Objective`, `Genitive`, `Locative`, `DefiniteNominativeSingular`, `DefiniteNominativePlural`, `DefiniteObjectiveSingular`, `DefiniteObjectivePlural`, `DefiniteGenitiveSingular`, `DefiniteGenitivePlural`, `DefiniteLocativeSingular`, `DefiniteLocativePlural`, `FormalNominativeSingular`, `FormalNominativePlural`, `FormalObjectiveSingular`, `FormalObjectivePlural`, `FormalGenitiveSingular`, `FormalGenitivePlural`, `FormalLocativeSingular`, `FormalLocativePlural`, `InformalNominativeSingular`, `InformalNominativePlural`, `InformalObjectiveSingular`, `InformalObjectivePlural`, `InformalGenitiveSingular`, `InformalGenitivePlural`, `InformalLocativeSingular`, `InformalLocativePlural`

## How it works

Each attested form is stored as an edit tree from the lemma. A lemma's edits across all cells make its inflection class. A suffix tree over the word, read from its end and pruned by a description-length test, predicts the class of any word. Known words the tree gets wrong are kept in a small lexicon. Confidences are smoothed counts; variant weights follow Wiktionary's listing order (1, 1/2, 1/3…, nonstandard forms a tenth of that).

## License

Code: MIT OR Apache-2.0. Data: derived from [Wiktionary](https://www.wiktionary.org/) (via [kaikki.org](https://kaikki.org/)), CC BY-SA 4.0; the model bytes in `src/model.bin` are shared under the same license.
