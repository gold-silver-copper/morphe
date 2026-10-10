# scottish_gaelic

ScottishGaelic inflection for Rust, learned from Wiktionary by [morphe](https://github.com/gold-silver-copper/morphe).
Inflects any word, known or made up. The `_scored` functions give every candidate with a confidence.

Generated from the Kaikki ScottishGaelic dump of 2026-10-03. Do not edit by hand: change the config in morphe and regenerate.

## Example

```rust
use scottish_gaelic::*;
assert_eq!(noun("cat", Definiteness::Indefinite, Case::Genitive, Number::Singular), "cait");
assert_eq!(noun("cat", Definiteness::Definite, Case::Genitive, Number::Singular), "chait");
assert_eq!(noun_mutated("cat", Mutation::Lenition), "chat");
println!("{:?}", noun_gender("bean"));
assert_eq!(verb_past("can", Clause::Independent, Voice::Personal), "chan");
assert_eq!(verb_future("can", Clause::Independent, Voice::Personal), "canaidh");
assert_eq!(verb_conditional("can", Clause::Independent, Subject::FirstSingular), "chanainn");
assert_eq!(adj("bog", Case::Genitive, Gender::Feminine), "buige");
assert_eq!(prep("aig", Person::FirstSingular, Emphasis::Plain), "agam");
// Citations pin principal parts: lemma, genitive singular, plural.
println!("{}", noun("taigh, taighe, taighean", Definiteness::Indefinite, Case::Nominative, Number::Plural));
```

Definite forms are given without the article (an cat, a' chait → cat,
chait). Analytic verb forms are given without their pronoun. Comparatives
drop "nas"/"as". Accents are optional on input.

## Coverage

| Part of speech | Entries | Cells | Edits | Classes | Tree nodes | Lexicon |
|---|---|---|---|---|---|---|
| noun | 6003 | 18 | 2460 | 2785 | 1376 | 4231 |
| adj | 837 | 14 | 436 | 401 | 149 | 528 |
| verb | 863 | 32 | 1150 | 548 | 174 | 605 |
| prep | 40 | 14 | 447 | 43 | 5 | 36 |
| pron | 169 | 14 | 1378 | 154 | 58 | 92 |

Model size: 660170 bytes.

**noun** cells: `IndefiniteNominativeSingular`, `IndefiniteNominativePlural`, `IndefiniteGenitiveSingular`, `IndefiniteGenitivePlural`, `IndefiniteDativeSingular`, `IndefiniteDativePlural`, `DefiniteNominativeSingular`, `DefiniteNominativePlural`, `DefiniteGenitiveSingular`, `DefiniteGenitivePlural`, `DefiniteDativeSingular`, `DefiniteDativePlural`, `VocativeSingular`, `VocativePlural`, `MutatedLenition`, `MutatedEclipsis`, `MutatedHProthesis`, `MutatedTProthesis`

Citation form: `Lemma, IndefiniteGenitiveSingular, IndefiniteNominativePlural` (comma-separated; parts you give are output exactly and narrow the prediction).

**adj** cells: `NominativeMasculine`, `NominativeFeminine`, `GenitiveMasculine`, `GenitiveFeminine`, `DativeMasculine`, `DativeFeminine`, `VocativeMasculine`, `VocativeFeminine`, `Plural`, `Comparative`, `MutatedLenition`, `MutatedEclipsis`, `MutatedHProthesis`, `MutatedTProthesis`

**verb** cells: `PastIndependentPersonal`, `PastIndependentImpersonal`, `PastDependentPersonal`, `PastDependentImpersonal`, `FutureIndependentPersonal`, `FutureIndependentImpersonal`, `FutureDependentPersonal`, `FutureDependentImpersonal`, `RelativePersonal`, `RelativeImpersonal`, `ConditionalIndependentFirstSingular`, `ConditionalIndependentFirstPlural`, `ConditionalIndependentOther`, `ConditionalIndependentImpersonal`, `ConditionalDependentFirstSingular`, `ConditionalDependentFirstPlural`, `ConditionalDependentOther`, `ConditionalDependentImpersonal`, `ImperativeFirstSingular`, `ImperativeSecondSingular`, `ImperativeThirdSingular`, `ImperativeThirdSingularFeminine`, `ImperativeFirstPlural`, `ImperativeSecondPlural`, `ImperativeThirdPlural`, `ImperativeImpersonal`, `VerbalNoun`, `Participle`, `MutatedLenition`, `MutatedEclipsis`, `MutatedHProthesis`, `MutatedTProthesis`

Citation form: `Lemma, VerbalNoun` (comma-separated; parts you give are output exactly and narrow the prediction).

**prep** cells: `FirstSingularPlain`, `FirstSingularEmphatic`, `SecondSingularPlain`, `SecondSingularEmphatic`, `ThirdSingularPlain`, `ThirdSingularEmphatic`, `ThirdSingularFemininePlain`, `ThirdSingularFeminineEmphatic`, `FirstPluralPlain`, `FirstPluralEmphatic`, `SecondPluralPlain`, `SecondPluralEmphatic`, `ThirdPluralPlain`, `ThirdPluralEmphatic`

**pron** cells: `FirstSingularPlain`, `FirstSingularEmphatic`, `SecondSingularPlain`, `SecondSingularEmphatic`, `ThirdSingularPlain`, `ThirdSingularEmphatic`, `ThirdSingularFemininePlain`, `ThirdSingularFeminineEmphatic`, `FirstPluralPlain`, `FirstPluralEmphatic`, `SecondPluralPlain`, `SecondPluralEmphatic`, `ThirdPluralPlain`, `ThirdPluralEmphatic`

## How it works

Each attested form is stored as an edit tree from the lemma. A lemma's edits across all cells make its inflection class. A suffix tree over the word, read from its end and pruned by a description-length test, predicts the class of any word. Known words the tree gets wrong are kept in a small lexicon. Confidences are smoothed counts; variant weights follow Wiktionary's listing order (1, 1/2, 1/3…, nonstandard forms a tenth of that).

## License

Code: MIT OR Apache-2.0. Data: derived from [Wiktionary](https://www.wiktionary.org/) (via [kaikki.org](https://kaikki.org/)), CC BY-SA 4.0; the model bytes in `src/model.bin` are shared under the same license.
