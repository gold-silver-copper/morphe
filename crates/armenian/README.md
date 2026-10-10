# armenian

Armenian inflection for Rust, learned from Wiktionary by [morphe](https://github.com/gold-silver-copper/morphe).
Inflects any word, known or made up. The `_scored` functions give every candidate with a confidence.

Generated from the Kaikki Armenian dump of 2026-10-03. Do not edit by hand: change the config in morphe and regenerate.

## Example

```rust
use armenian::*;
assert_eq!(noun("հայ", Case::Ablative, Number::Plural), "հայերից");
println!("{}", noun_definite("գիրք", Case::Nominative, Number::Singular)); // գիրքը
println!("{}", noun_possessive("տուն", Possessor::First, Case::Nominative, Number::Singular)); // տունս
println!("{}", verb("խոսել", Tense::Aorist, Person::First, Number::Singular)); // խոսեցի
println!("{}", verb("կարդալ", Tense::PresentSubjunctive, Person::Third, Number::Plural)); // կարդան
println!("{}", verb_converb("խոսել", Converb::Imperfective)); // խոսում
```

This is Eastern Armenian in reformed spelling; Wiktionary's Western Armenian
tables are left out. Nouns have five cases in both numbers, the definite
nominative and dative, and the first- and second-person possessive forms.
Verbs have the infinitive, converbs, participles, the synthetic tenses
(aorist, subjunctives, conditionals, imperative) and the periphrastic ones
built with the copula (խոսում եմ, խոսել էի). Negative forms are left out.
Input matching ignores the stress and emphasis marks.

## Coverage

| Part of speech | Entries | Cells | Edits | Classes | Tree nodes | Lexicon |
|---|---|---|---|---|---|---|
| noun | 10215 | 34 | 2043 | 624 | 757 | 3186 |
| adj | 3221 | 35 | 391 | 84 | 153 | 1054 |
| verb | 2011 | 79 | 876 | 150 | 82 | 551 |

Model size: 1216862 bytes.

**noun** cells: `NominativeSingular`, `NominativePlural`, `DativeSingular`, `DativePlural`, `AblativeSingular`, `AblativePlural`, `InstrumentalSingular`, `InstrumentalPlural`, `LocativeSingular`, `LocativePlural`, `DefiniteNominativeSingular`, `DefiniteNominativePlural`, `DefiniteDativeSingular`, `DefiniteDativePlural`, `PossessiveFirstNominativeSingular`, `PossessiveFirstNominativePlural`, `PossessiveFirstDativeSingular`, `PossessiveFirstDativePlural`, `PossessiveFirstAblativeSingular`, `PossessiveFirstAblativePlural`, `PossessiveFirstInstrumentalSingular`, `PossessiveFirstInstrumentalPlural`, `PossessiveFirstLocativeSingular`, `PossessiveFirstLocativePlural`, `PossessiveSecondNominativeSingular`, `PossessiveSecondNominativePlural`, `PossessiveSecondDativeSingular`, `PossessiveSecondDativePlural`, `PossessiveSecondAblativeSingular`, `PossessiveSecondAblativePlural`, `PossessiveSecondInstrumentalSingular`, `PossessiveSecondInstrumentalPlural`, `PossessiveSecondLocativeSingular`, `PossessiveSecondLocativePlural`

Citation form: `NominativeSingular, DativeSingular, NominativePlural` (comma-separated; parts you give are output exactly and narrow the prediction).

**adj** cells: `NominativeSingular`, `NominativePlural`, `DativeSingular`, `DativePlural`, `AblativeSingular`, `AblativePlural`, `InstrumentalSingular`, `InstrumentalPlural`, `LocativeSingular`, `LocativePlural`, `DefiniteNominativeSingular`, `DefiniteNominativePlural`, `DefiniteDativeSingular`, `DefiniteDativePlural`, `PossessiveFirstNominativeSingular`, `PossessiveFirstNominativePlural`, `PossessiveFirstDativeSingular`, `PossessiveFirstDativePlural`, `PossessiveFirstAblativeSingular`, `PossessiveFirstAblativePlural`, `PossessiveFirstInstrumentalSingular`, `PossessiveFirstInstrumentalPlural`, `PossessiveFirstLocativeSingular`, `PossessiveFirstLocativePlural`, `PossessiveSecondNominativeSingular`, `PossessiveSecondNominativePlural`, `PossessiveSecondDativeSingular`, `PossessiveSecondDativePlural`, `PossessiveSecondAblativeSingular`, `PossessiveSecondAblativePlural`, `PossessiveSecondInstrumentalSingular`, `PossessiveSecondInstrumentalPlural`, `PossessiveSecondLocativeSingular`, `PossessiveSecondLocativePlural`, `Superlative`

**verb** cells: `Infinitive`, `ConverbImperfective`, `ConverbSimultaneous`, `ConverbPerfective`, `ConverbFutureI`, `ConverbFutureII`, `ParticipleResultative`, `ParticipleSubjective`, `AoristStem`, `Passive`, `Causative`, `PresentFirstSingular`, `PresentFirstPlural`, `PresentSecondSingular`, `PresentSecondPlural`, `PresentThirdSingular`, `PresentThirdPlural`, `ImperfectFirstSingular`, `ImperfectFirstPlural`, `ImperfectSecondSingular`, `ImperfectSecondPlural`, `ImperfectThirdSingular`, `ImperfectThirdPlural`, `FutureFirstSingular`, `FutureFirstPlural`, `FutureSecondSingular`, `FutureSecondPlural`, `FutureThirdSingular`, `FutureThirdPlural`, `FuturePastFirstSingular`, `FuturePastFirstPlural`, `FuturePastSecondSingular`, `FuturePastSecondPlural`, `FuturePastThirdSingular`, `FuturePastThirdPlural`, `PerfectFirstSingular`, `PerfectFirstPlural`, `PerfectSecondSingular`, `PerfectSecondPlural`, `PerfectThirdSingular`, `PerfectThirdPlural`, `PluperfectFirstSingular`, `PluperfectFirstPlural`, `PluperfectSecondSingular`, `PluperfectSecondPlural`, `PluperfectThirdSingular`, `PluperfectThirdPlural`, `AoristFirstSingular`, `AoristFirstPlural`, `AoristSecondSingular`, `AoristSecondPlural`, `AoristThirdSingular`, `AoristThirdPlural`, `PresentSubjunctiveFirstSingular`, `PresentSubjunctiveFirstPlural`, `PresentSubjunctiveSecondSingular`, `PresentSubjunctiveSecondPlural`, `PresentSubjunctiveThirdSingular`, `PresentSubjunctiveThirdPlural`, `PastSubjunctiveFirstSingular`, `PastSubjunctiveFirstPlural`, `PastSubjunctiveSecondSingular`, `PastSubjunctiveSecondPlural`, `PastSubjunctiveThirdSingular`, `PastSubjunctiveThirdPlural`, `ConditionalFutureFirstSingular`, `ConditionalFutureFirstPlural`, `ConditionalFutureSecondSingular`, `ConditionalFutureSecondPlural`, `ConditionalFutureThirdSingular`, `ConditionalFutureThirdPlural`, `ConditionalPastFirstSingular`, `ConditionalPastFirstPlural`, `ConditionalPastSecondSingular`, `ConditionalPastSecondPlural`, `ConditionalPastThirdSingular`, `ConditionalPastThirdPlural`, `ImperativeSingular`, `ImperativePlural`

Citation form: `Infinitive, AoristThirdSingular` (comma-separated; parts you give are output exactly and narrow the prediction).

## How it works

Each attested form is stored as an edit tree from the lemma. A lemma's edits across all cells make its inflection class. A suffix tree over the word, read from its end and pruned by a description-length test, predicts the class of any word. Known words the tree gets wrong are kept in a small lexicon. Confidences are smoothed counts; variant weights follow Wiktionary's listing order (1, 1/2, 1/3…, nonstandard forms a tenth of that).

## License

Code: MIT OR Apache-2.0. Data: derived from [Wiktionary](https://www.wiktionary.org/) (via [kaikki.org](https://kaikki.org/)), CC BY-SA 4.0; the model bytes in `src/model.bin` are shared under the same license.
