# albanian

Albanian inflection for Rust, learned from Wiktionary by [morphe](https://github.com/gold-silver-copper/morphe).
Inflects any word, known or made up. The `_scored` functions give every candidate with a confidence.

Generated from the Kaikki Albanian dump of 2026-10-03. Do not edit by hand: change the config in morphe and regenerate.

## Example

```rust
use albanian::*;
assert_eq!(noun("mal", Case::Nominative, Definiteness::Definite, Number::Plural), "malet");
println!("{}", noun("vajzë", Case::Dative, Definiteness::Definite, Number::Singular)); // vajzës
println!("{:?}", noun_gender("qytet"));
println!("{}", verb("punoj", Tense::Aorist, Person::Third, Number::Singular)); // punoi
println!("{}", verb("punoj", Tense::AdmirativePresent, Person::First, Number::Singular)); // punuakam
println!("{}", verb_participle("punoj")); // punuar
// Citations pin principal parts: present, aorist, participle.
println!("{}", verb("flas, fola, folur", Tense::Imperfect, Person::First, Number::Plural));
```

Nouns have four cases, definite and indefinite, in both numbers (the
genitive is the dative with a linking article). Verbs have every tense and
mood of the tables, including the periphrastic ones (kam punuar, do të
punoj), the admirative and the optative. Adjectives have their feminine and
plural forms from the headword line.

## Coverage

| Part of speech | Entries | Cells | Edits | Classes | Tree nodes | Lexicon |
|---|---|---|---|---|---|---|
| noun | 6140 | 16 | 1011 | 634 | 554 | 2738 |
| adj | 953 | 3 | 58 | 65 | 59 | 330 |
| verb | 1732 | 119 | 6648 | 308 | 112 | 525 |

Model size: 717139 bytes.

**noun** cells: `NominativeIndefiniteSingular`, `NominativeIndefinitePlural`, `NominativeDefiniteSingular`, `NominativeDefinitePlural`, `AccusativeIndefiniteSingular`, `AccusativeIndefinitePlural`, `AccusativeDefiniteSingular`, `AccusativeDefinitePlural`, `DativeIndefiniteSingular`, `DativeIndefinitePlural`, `DativeDefiniteSingular`, `DativeDefinitePlural`, `AblativeIndefiniteSingular`, `AblativeIndefinitePlural`, `AblativeDefiniteSingular`, `AblativeDefinitePlural`

Citation form: `NominativeIndefiniteSingular, NominativeDefiniteSingular, NominativeIndefinitePlural` (comma-separated; parts you give are output exactly and narrow the prediction).

**adj** cells: `Feminine`, `MasculinePlural`, `FemininePlural`

Citation form: `Lemma, Feminine` (comma-separated; parts you give are output exactly and narrow the prediction).

**verb** cells: `Participle`, `Gerund`, `Infinitive`, `PresentFirstSingular`, `PresentFirstPlural`, `PresentSecondSingular`, `PresentSecondPlural`, `PresentThirdSingular`, `PresentThirdPlural`, `ImperfectFirstSingular`, `ImperfectFirstPlural`, `ImperfectSecondSingular`, `ImperfectSecondPlural`, `ImperfectThirdSingular`, `ImperfectThirdPlural`, `AoristFirstSingular`, `AoristFirstPlural`, `AoristSecondSingular`, `AoristSecondPlural`, `AoristThirdSingular`, `AoristThirdPlural`, `PerfectFirstSingular`, `PerfectFirstPlural`, `PerfectSecondSingular`, `PerfectSecondPlural`, `PerfectThirdSingular`, `PerfectThirdPlural`, `PluperfectFirstSingular`, `PluperfectFirstPlural`, `PluperfectSecondSingular`, `PluperfectSecondPlural`, `PluperfectThirdSingular`, `PluperfectThirdPlural`, `FutureFirstSingular`, `FutureFirstPlural`, `FutureSecondSingular`, `FutureSecondPlural`, `FutureThirdSingular`, `FutureThirdPlural`, `FuturePerfectFirstSingular`, `FuturePerfectFirstPlural`, `FuturePerfectSecondSingular`, `FuturePerfectSecondPlural`, `FuturePerfectThirdSingular`, `FuturePerfectThirdPlural`, `PresentSubjunctiveFirstSingular`, `PresentSubjunctiveFirstPlural`, `PresentSubjunctiveSecondSingular`, `PresentSubjunctiveSecondPlural`, `PresentSubjunctiveThirdSingular`, `PresentSubjunctiveThirdPlural`, `ImperfectSubjunctiveFirstSingular`, `ImperfectSubjunctiveFirstPlural`, `ImperfectSubjunctiveSecondSingular`, `ImperfectSubjunctiveSecondPlural`, `ImperfectSubjunctiveThirdSingular`, `ImperfectSubjunctiveThirdPlural`, `PerfectSubjunctiveFirstSingular`, `PerfectSubjunctiveFirstPlural`, `PerfectSubjunctiveSecondSingular`, `PerfectSubjunctiveSecondPlural`, `PerfectSubjunctiveThirdSingular`, `PerfectSubjunctiveThirdPlural`, `PluperfectSubjunctiveFirstSingular`, `PluperfectSubjunctiveFirstPlural`, `PluperfectSubjunctiveSecondSingular`, `PluperfectSubjunctiveSecondPlural`, `PluperfectSubjunctiveThirdSingular`, `PluperfectSubjunctiveThirdPlural`, `ConditionalFirstSingular`, `ConditionalFirstPlural`, `ConditionalSecondSingular`, `ConditionalSecondPlural`, `ConditionalThirdSingular`, `ConditionalThirdPlural`, `ConditionalPerfectFirstSingular`, `ConditionalPerfectFirstPlural`, `ConditionalPerfectSecondSingular`, `ConditionalPerfectSecondPlural`, `ConditionalPerfectThirdSingular`, `ConditionalPerfectThirdPlural`, `OptativeFirstSingular`, `OptativeFirstPlural`, `OptativeSecondSingular`, `OptativeSecondPlural`, `OptativeThirdSingular`, `OptativeThirdPlural`, `OptativePerfectFirstSingular`, `OptativePerfectFirstPlural`, `OptativePerfectSecondSingular`, `OptativePerfectSecondPlural`, `OptativePerfectThirdSingular`, `OptativePerfectThirdPlural`, `AdmirativePresentFirstSingular`, `AdmirativePresentFirstPlural`, `AdmirativePresentSecondSingular`, `AdmirativePresentSecondPlural`, `AdmirativePresentThirdSingular`, `AdmirativePresentThirdPlural`, `AdmirativeImperfectFirstSingular`, `AdmirativeImperfectFirstPlural`, `AdmirativeImperfectSecondSingular`, `AdmirativeImperfectSecondPlural`, `AdmirativeImperfectThirdSingular`, `AdmirativeImperfectThirdPlural`, `AdmirativePerfectFirstSingular`, `AdmirativePerfectFirstPlural`, `AdmirativePerfectSecondSingular`, `AdmirativePerfectSecondPlural`, `AdmirativePerfectThirdSingular`, `AdmirativePerfectThirdPlural`, `AdmirativePluperfectFirstSingular`, `AdmirativePluperfectFirstPlural`, `AdmirativePluperfectSecondSingular`, `AdmirativePluperfectSecondPlural`, `AdmirativePluperfectThirdSingular`, `AdmirativePluperfectThirdPlural`, `ImperativeSingular`, `ImperativePlural`

Citation form: `PresentFirstSingular, AoristFirstSingular, Participle` (comma-separated; parts you give are output exactly and narrow the prediction).

## How it works

Each attested form is stored as an edit tree from the lemma. A lemma's edits across all cells make its inflection class. A suffix tree over the word, read from its end and pruned by a description-length test, predicts the class of any word. Known words the tree gets wrong are kept in a small lexicon. Confidences are smoothed counts; variant weights follow Wiktionary's listing order (1, 1/2, 1/3…, nonstandard forms a tenth of that).

## License

Code: MIT OR Apache-2.0. Data: derived from [Wiktionary](https://www.wiktionary.org/) (via [kaikki.org](https://kaikki.org/)), CC BY-SA 4.0; the model bytes in `src/model.bin` are shared under the same license.
