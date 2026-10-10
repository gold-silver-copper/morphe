# georgian

Georgian inflection for Rust, learned from Wiktionary by [morphe](https://github.com/gold-silver-copper/morphe).
Inflects any word, known or made up. The `_scored` functions give every candidate with a confidence.

Generated from the Kaikki Georgian dump of 2026-10-03. Do not edit by hand: change the config in morphe and regenerate.

## Example

```rust
use georgian::*;
assert_eq!(noun("მეგობარი", Case::Genitive, Number::Singular), "მეგობრის");
println!("{}", noun("კაცი", Case::Ergative, Number::Plural)); // კაცებმა
println!("{}", noun_postposition("სახლი", Postposition::In, Number::Singular)); // სახლში
println!("{}", adj_attributive("ქართული", Case::Ergative)); // ქართულმა
println!("{}", verb("ნიშნავს", Screeve::Aorist, Person::First, Number::Singular)); // დავნიშნე
println!("{}", verb("ნიშნავს", Screeve::Future, Person::Third, Number::Plural)); // დანიშნავენ
```

Nouns have the seven cases and the twelve postpositional forms (-ში, -ზე,
-თან…) in both numbers; archaic plurals in -ნი/-თა are left out. Adjectives
have their attributive forms, and the full declension where Wiktionary
gives one. Verbs have the eleven screeves for the subject's person and
number (the tables give forms with a third-person object), the imperative,
the verbal nouns and the participles. Verbs are looked up by their
third-person singular present (ნიშნავს), as in Wiktionary. A verb's preverb
(და-, გა-, მო-…) can't be told from its present form, so the future and
aorist of unknown verbs are often wrong.

## Coverage

| Part of speech | Entries | Cells | Edits | Classes | Tree nodes | Lexicon |
|---|---|---|---|---|---|---|
| noun | 11417 | 38 | 3122 | 496 | 279 | 3395 |
| adj | 3249 | 21 | 104 | 24 | 43 | 167 |
| verb | 445 | 78 | 6016 | 368 | 97 | 212 |

Model size: 2178253 bytes.

**noun** cells: `NominativeSingular`, `NominativePlural`, `ErgativeSingular`, `ErgativePlural`, `DativeSingular`, `DativePlural`, `GenitiveSingular`, `GenitivePlural`, `InstrumentalSingular`, `InstrumentalPlural`, `AdverbialSingular`, `AdverbialPlural`, `VocativeSingular`, `VocativePlural`, `PostpositionOnSingular`, `PostpositionOnPlural`, `PostpositionNearSingular`, `PostpositionNearPlural`, `PostpositionInSingular`, `PostpositionInPlural`, `PostpositionLikeSingular`, `PostpositionLikePlural`, `PostpositionForSingular`, `PostpositionForPlural`, `PostpositionFromSingular`, `PostpositionFromPlural`, `PostpositionTowardsSingular`, `PostpositionTowardsPlural`, `PostpositionRegardingSingular`, `PostpositionRegardingPlural`, `PostpositionSimilarSingular`, `PostpositionSimilarPlural`, `PostpositionSinceSingular`, `PostpositionSincePlural`, `PostpositionWithSingular`, `PostpositionWithPlural`, `PostpositionUpToSingular`, `PostpositionUpToPlural`

Citation form: `NominativeSingular, GenitiveSingular` (comma-separated; parts you give are output exactly and narrow the prediction).

**adj** cells: `AttributiveNominative`, `AttributiveErgative`, `AttributiveDative`, `AttributiveGenitive`, `AttributiveInstrumental`, `AttributiveAdverbial`, `AttributiveVocative`, `NominativeSingular`, `NominativePlural`, `ErgativeSingular`, `ErgativePlural`, `DativeSingular`, `DativePlural`, `GenitiveSingular`, `GenitivePlural`, `InstrumentalSingular`, `InstrumentalPlural`, `AdverbialSingular`, `AdverbialPlural`, `VocativeSingular`, `VocativePlural`

**verb** cells: `PresentFirstSingular`, `PresentFirstPlural`, `PresentSecondSingular`, `PresentSecondPlural`, `PresentThirdSingular`, `PresentThirdPlural`, `ImperfectFirstSingular`, `ImperfectFirstPlural`, `ImperfectSecondSingular`, `ImperfectSecondPlural`, `ImperfectThirdSingular`, `ImperfectThirdPlural`, `PresentSubjunctiveFirstSingular`, `PresentSubjunctiveFirstPlural`, `PresentSubjunctiveSecondSingular`, `PresentSubjunctiveSecondPlural`, `PresentSubjunctiveThirdSingular`, `PresentSubjunctiveThirdPlural`, `FutureFirstSingular`, `FutureFirstPlural`, `FutureSecondSingular`, `FutureSecondPlural`, `FutureThirdSingular`, `FutureThirdPlural`, `ConditionalFirstSingular`, `ConditionalFirstPlural`, `ConditionalSecondSingular`, `ConditionalSecondPlural`, `ConditionalThirdSingular`, `ConditionalThirdPlural`, `FutureSubjunctiveFirstSingular`, `FutureSubjunctiveFirstPlural`, `FutureSubjunctiveSecondSingular`, `FutureSubjunctiveSecondPlural`, `FutureSubjunctiveThirdSingular`, `FutureSubjunctiveThirdPlural`, `AoristFirstSingular`, `AoristFirstPlural`, `AoristSecondSingular`, `AoristSecondPlural`, `AoristThirdSingular`, `AoristThirdPlural`, `OptativeFirstSingular`, `OptativeFirstPlural`, `OptativeSecondSingular`, `OptativeSecondPlural`, `OptativeThirdSingular`, `OptativeThirdPlural`, `PerfectFirstSingular`, `PerfectFirstPlural`, `PerfectSecondSingular`, `PerfectSecondPlural`, `PerfectThirdSingular`, `PerfectThirdPlural`, `PluperfectFirstSingular`, `PluperfectFirstPlural`, `PluperfectSecondSingular`, `PluperfectSecondPlural`, `PluperfectThirdSingular`, `PluperfectThirdPlural`, `PerfectSubjunctiveFirstSingular`, `PerfectSubjunctiveFirstPlural`, `PerfectSubjunctiveSecondSingular`, `PerfectSubjunctiveSecondPlural`, `PerfectSubjunctiveThirdSingular`, `PerfectSubjunctiveThirdPlural`, `ImperativeFirstSingular`, `ImperativeFirstPlural`, `ImperativeSecondSingular`, `ImperativeSecondPlural`, `VerbalNounImperfective`, `VerbalNounPerfective`, `ParticiplePresentImperfective`, `ParticiplePresentPerfective`, `ParticipleFutureImperfective`, `ParticipleFuturePerfective`, `ParticiplePastImperfective`, `ParticiplePastPerfective`

Citation form: `PresentThirdSingular, FutureThirdSingular, AoristThirdSingular` (comma-separated; parts you give are output exactly and narrow the prediction).

## How it works

Each attested form is stored as an edit tree from the lemma. A lemma's edits across all cells make its inflection class. A suffix tree over the word, read from its end and pruned by a description-length test, predicts the class of any word. Known words the tree gets wrong are kept in a small lexicon. Confidences are smoothed counts; variant weights follow Wiktionary's listing order (1, 1/2, 1/3…, nonstandard forms a tenth of that).

## License

Code: MIT OR Apache-2.0. Data: derived from [Wiktionary](https://www.wiktionary.org/) (via [kaikki.org](https://kaikki.org/)), CC BY-SA 4.0; the model bytes in `src/model.bin` are shared under the same license.
