# irish

Irish inflection for Rust, learned from Wiktionary by [morphe](https://github.com/gold-silver-copper/morphe).
Inflects any word, known or made up. The `_scored` functions give every candidate with a confidence.

Generated from the Kaikki Irish dump of 2026-10-03. Do not edit by hand: change the config in morphe and regenerate.

## Example

```rust
use irish::*;
assert_eq!(noun_genitive_singular("cat"), "cait");
println!("{}", noun_plural("bean")); // mná
println!("{}", noun_definite("bean", Case::Genitive, Number::Plural)); // na mban
println!("{}", noun("bord", Mutation::Eclipsis)); // mbord
println!("{:?}", noun_gender("fuinneog"));
println!("{}", adj("mór", Case::Nominative, Gender::Feminine)); // mhór
println!("{}", adj_mutated("mór", Mutation::Lenition)); // mhór
println!("{}", verb("can", Mood::Past, Subject::ThirdSingular)); // chan
println!("{}", verb("can", Mood::Conditional, Subject::FirstPlural)); // chanfaimis
println!("{}", verb_verbal_noun("ól"));
// Citations pin principal parts: lemma, genitive singular, plural.
println!("{}", noun_plural("leabhar, leabhair, leabhair"));
```

Initial mutations (lenition, eclipsis) are part of the forms: the past
lenites (chan), the definite genitive plural eclipses (na gcat). Analytic
verb forms are given without their pronoun (canann for canann sé); subject
pronouns are separate words. Definite forms include the article.

## Coverage

| Part of speech | Entries | Cells | Edits | Classes | Tree nodes | Lexicon |
|---|---|---|---|---|---|---|
| noun | 19340 | 10 | 4117 | 7329 | 2935 | 10459 |
| adj | 4255 | 15 | 650 | 1454 | 872 | 1728 |
| verb | 1558 | 63 | 5334 | 1005 | 391 | 810 |

Model size: 715173 bytes.

**noun** cells: `GenitiveSingular`, `Plural`, `DefiniteNominativeSingular`, `DefiniteNominativePlural`, `DefiniteGenitiveSingular`, `DefiniteGenitivePlural`, `DefiniteDativeSingular`, `DefiniteDativePlural`, `Lenition`, `Eclipsis`

Citation form: `Lemma, GenitiveSingular, Plural` (comma-separated; parts you give are output exactly and narrow the prediction).

**adj** cells: `NominativeMasculine`, `NominativeFeminine`, `VocativeMasculine`, `VocativeFeminine`, `GenitiveMasculine`, `GenitiveFeminine`, `DativeMasculine`, `DativeFeminine`, `PluralNominative`, `PluralVocative`, `PluralGenitive`, `PluralDative`, `Comparative`, `MutatedLenition`, `MutatedEclipsis`

**verb** cells: `PresentFirstSingular`, `PresentSecondSingular`, `PresentThirdSingular`, `PresentFirstPlural`, `PresentSecondPlural`, `PresentThirdPlural`, `PresentAutonomous`, `PastFirstSingular`, `PastSecondSingular`, `PastThirdSingular`, `PastFirstPlural`, `PastSecondPlural`, `PastThirdPlural`, `PastAutonomous`, `HabitualFirstSingular`, `HabitualSecondSingular`, `HabitualThirdSingular`, `HabitualFirstPlural`, `HabitualSecondPlural`, `HabitualThirdPlural`, `HabitualAutonomous`, `FutureFirstSingular`, `FutureSecondSingular`, `FutureThirdSingular`, `FutureFirstPlural`, `FutureSecondPlural`, `FutureThirdPlural`, `FutureAutonomous`, `ConditionalFirstSingular`, `ConditionalSecondSingular`, `ConditionalThirdSingular`, `ConditionalFirstPlural`, `ConditionalSecondPlural`, `ConditionalThirdPlural`, `ConditionalAutonomous`, `SubjunctiveFirstSingular`, `SubjunctiveSecondSingular`, `SubjunctiveThirdSingular`, `SubjunctiveFirstPlural`, `SubjunctiveSecondPlural`, `SubjunctiveThirdPlural`, `SubjunctiveAutonomous`, `PastSubjunctiveFirstSingular`, `PastSubjunctiveSecondSingular`, `PastSubjunctiveThirdSingular`, `PastSubjunctiveFirstPlural`, `PastSubjunctiveSecondPlural`, `PastSubjunctiveThirdPlural`, `PastSubjunctiveAutonomous`, `ImperativeFirstSingular`, `ImperativeSecondSingular`, `ImperativeThirdSingular`, `ImperativeFirstPlural`, `ImperativeSecondPlural`, `ImperativeThirdPlural`, `ImperativeAutonomous`, `RelativePresent`, `RelativePast`, `RelativeHabitual`, `RelativeFuture`, `RelativeConditional`, `VerbalNoun`, `Participle`

Citation form: `Lemma, VerbalNoun` (comma-separated; parts you give are output exactly and narrow the prediction).

## How it works

Each attested form is stored as an edit tree from the lemma. A lemma's edits across all cells make its inflection class. A suffix tree over the word, read from its end and pruned by a description-length test, predicts the class of any word. Known words the tree gets wrong are kept in a small lexicon. Confidences are smoothed counts; variant weights follow Wiktionary's listing order (1, 1/2, 1/3…, nonstandard forms a tenth of that).

## License

Code: MIT OR Apache-2.0. Data: derived from [Wiktionary](https://www.wiktionary.org/) (via [kaikki.org](https://kaikki.org/)), CC BY-SA 4.0; the model bytes in `src/model.bin` are shared under the same license.
