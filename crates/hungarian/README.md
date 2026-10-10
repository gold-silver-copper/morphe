# hungarian

Hungarian inflection for Rust, learned from Wiktionary by [morphe](https://github.com/gold-silver-copper/morphe).
Inflects any word, known or made up. The `_scored` functions give every candidate with a confidence.

Generated from the Kaikki Hungarian dump of 2026-10-03. Do not edit by hand: change the config in morphe and regenerate.

## Example

```rust
use hungarian::*;
assert_eq!(noun("ház", Case::Inessive, Number::Singular), "házban");
println!("{}", noun("fa", Case::Accusative, Number::Plural)); // fákat
println!("{}", noun_possessive("kert", Possessor::FirstSingular, Possessed::One)); // kertem
println!("{}", verb("olvas", Mood::Present, Definiteness::Definite, Person::Third, Number::Singular)); // olvassa
println!("{}", verb_lak("lát", Mood::Present)); // látlak
println!("{}", adj_comparative("szép")); // szebb
// Citations pin principal parts: nominative, accusative, plural, possessed.
println!("{}", noun("ló, lovat", Case::Dative, Number::Plural)); // lovaknak
```

Cells cover the seventeen productive cases in both numbers, the possessed
forms (in the nominative), the indefinite and definite conjugations of the
present, past, conditional and subjunctive (the imperative is the
subjunctive), the `-lak/-lek` forms (I → you), the infinitive and its
personal forms, and the participles. Wiktionary's verb tables are read by
position; archaic tenses and the potential (`-hat/-het`) table are left
out. Verbs with an irregular table layout (`van`, `jön`…) are learned from
whatever parts fit and predicted for the rest.

## Coverage

| Part of speech | Entries | Cells | Edits | Classes | Tree nodes | Lexicon |
|---|---|---|---|---|---|---|
| noun | 21627 | 46 | 2796 | 724 | 1111 | 2550 |
| adj | 9630 | 36 | 863 | 394 | 416 | 2216 |
| verb | 4723 | 63 | 1689 | 504 | 358 | 1174 |
| pron | 235 | 34 | 327 | 47 | 38 | 53 |
| num | 334 | 34 | 379 | 49 | 46 | 36 |

Model size: 2722878 bytes.

**noun** cells: `NominativeSingular`, `NominativePlural`, `AccusativeSingular`, `AccusativePlural`, `DativeSingular`, `DativePlural`, `InstrumentalSingular`, `InstrumentalPlural`, `CausalFinalSingular`, `CausalFinalPlural`, `TranslativeSingular`, `TranslativePlural`, `TerminativeSingular`, `TerminativePlural`, `EssiveFormalSingular`, `EssiveFormalPlural`, `InessiveSingular`, `InessivePlural`, `SuperessiveSingular`, `SuperessivePlural`, `AdessiveSingular`, `AdessivePlural`, `IllativeSingular`, `IllativePlural`, `SublativeSingular`, `SublativePlural`, `AllativeSingular`, `AllativePlural`, `ElativeSingular`, `ElativePlural`, `DelativeSingular`, `DelativePlural`, `AblativeSingular`, `AblativePlural`, `PossessiveFirstSingularOne`, `PossessiveFirstSingularMany`, `PossessiveSecondSingularOne`, `PossessiveSecondSingularMany`, `PossessiveThirdSingularOne`, `PossessiveThirdSingularMany`, `PossessiveFirstPluralOne`, `PossessiveFirstPluralMany`, `PossessiveSecondPluralOne`, `PossessiveSecondPluralMany`, `PossessiveThirdPluralOne`, `PossessiveThirdPluralMany`

Citation form: `NominativeSingular, AccusativeSingular, NominativePlural, PossessiveThirdSingularOne` (comma-separated; parts you give are output exactly and narrow the prediction).

**adj** cells: `NominativeSingular`, `NominativePlural`, `AccusativeSingular`, `AccusativePlural`, `DativeSingular`, `DativePlural`, `InstrumentalSingular`, `InstrumentalPlural`, `CausalFinalSingular`, `CausalFinalPlural`, `TranslativeSingular`, `TranslativePlural`, `TerminativeSingular`, `TerminativePlural`, `EssiveFormalSingular`, `EssiveFormalPlural`, `InessiveSingular`, `InessivePlural`, `SuperessiveSingular`, `SuperessivePlural`, `AdessiveSingular`, `AdessivePlural`, `IllativeSingular`, `IllativePlural`, `SublativeSingular`, `SublativePlural`, `AllativeSingular`, `AllativePlural`, `ElativeSingular`, `ElativePlural`, `DelativeSingular`, `DelativePlural`, `AblativeSingular`, `AblativePlural`, `Comparative`, `Superlative`

Citation form: `NominativeSingular, AccusativeSingular, NominativePlural` (comma-separated; parts you give are output exactly and narrow the prediction).

**verb** cells: `Infinitive`, `PresentIndefiniteFirstSingular`, `PresentIndefiniteFirstPlural`, `PresentIndefiniteSecondSingular`, `PresentIndefiniteSecondPlural`, `PresentIndefiniteThirdSingular`, `PresentIndefiniteThirdPlural`, `PresentDefiniteFirstSingular`, `PresentDefiniteFirstPlural`, `PresentDefiniteSecondSingular`, `PresentDefiniteSecondPlural`, `PresentDefiniteThirdSingular`, `PresentDefiniteThirdPlural`, `PastIndefiniteFirstSingular`, `PastIndefiniteFirstPlural`, `PastIndefiniteSecondSingular`, `PastIndefiniteSecondPlural`, `PastIndefiniteThirdSingular`, `PastIndefiniteThirdPlural`, `PastDefiniteFirstSingular`, `PastDefiniteFirstPlural`, `PastDefiniteSecondSingular`, `PastDefiniteSecondPlural`, `PastDefiniteThirdSingular`, `PastDefiniteThirdPlural`, `ConditionalIndefiniteFirstSingular`, `ConditionalIndefiniteFirstPlural`, `ConditionalIndefiniteSecondSingular`, `ConditionalIndefiniteSecondPlural`, `ConditionalIndefiniteThirdSingular`, `ConditionalIndefiniteThirdPlural`, `ConditionalDefiniteFirstSingular`, `ConditionalDefiniteFirstPlural`, `ConditionalDefiniteSecondSingular`, `ConditionalDefiniteSecondPlural`, `ConditionalDefiniteThirdSingular`, `ConditionalDefiniteThirdPlural`, `SubjunctiveIndefiniteFirstSingular`, `SubjunctiveIndefiniteFirstPlural`, `SubjunctiveIndefiniteSecondSingular`, `SubjunctiveIndefiniteSecondPlural`, `SubjunctiveIndefiniteThirdSingular`, `SubjunctiveIndefiniteThirdPlural`, `SubjunctiveDefiniteFirstSingular`, `SubjunctiveDefiniteFirstPlural`, `SubjunctiveDefiniteSecondSingular`, `SubjunctiveDefiniteSecondPlural`, `SubjunctiveDefiniteThirdSingular`, `SubjunctiveDefiniteThirdPlural`, `LakPresent`, `LakPast`, `LakConditional`, `LakSubjunctive`, `PersonalInfinitiveFirstSingular`, `PersonalInfinitiveFirstPlural`, `PersonalInfinitiveSecondSingular`, `PersonalInfinitiveSecondPlural`, `PersonalInfinitiveThirdSingular`, `PersonalInfinitiveThirdPlural`, `ParticiplePresent`, `ParticiplePast`, `ParticipleFuture`, `AdverbialParticiple`

Citation form: `PresentIndefiniteThirdSingular, PresentIndefiniteSecondSingular, PastIndefiniteThirdSingular` (comma-separated; parts you give are output exactly and narrow the prediction).

**pron** cells: `NominativeSingular`, `NominativePlural`, `AccusativeSingular`, `AccusativePlural`, `DativeSingular`, `DativePlural`, `InstrumentalSingular`, `InstrumentalPlural`, `CausalFinalSingular`, `CausalFinalPlural`, `TranslativeSingular`, `TranslativePlural`, `TerminativeSingular`, `TerminativePlural`, `EssiveFormalSingular`, `EssiveFormalPlural`, `InessiveSingular`, `InessivePlural`, `SuperessiveSingular`, `SuperessivePlural`, `AdessiveSingular`, `AdessivePlural`, `IllativeSingular`, `IllativePlural`, `SublativeSingular`, `SublativePlural`, `AllativeSingular`, `AllativePlural`, `ElativeSingular`, `ElativePlural`, `DelativeSingular`, `DelativePlural`, `AblativeSingular`, `AblativePlural`

**num** cells: `NominativeSingular`, `NominativePlural`, `AccusativeSingular`, `AccusativePlural`, `DativeSingular`, `DativePlural`, `InstrumentalSingular`, `InstrumentalPlural`, `CausalFinalSingular`, `CausalFinalPlural`, `TranslativeSingular`, `TranslativePlural`, `TerminativeSingular`, `TerminativePlural`, `EssiveFormalSingular`, `EssiveFormalPlural`, `InessiveSingular`, `InessivePlural`, `SuperessiveSingular`, `SuperessivePlural`, `AdessiveSingular`, `AdessivePlural`, `IllativeSingular`, `IllativePlural`, `SublativeSingular`, `SublativePlural`, `AllativeSingular`, `AllativePlural`, `ElativeSingular`, `ElativePlural`, `DelativeSingular`, `DelativePlural`, `AblativeSingular`, `AblativePlural`

## How it works

Each attested form is stored as an edit tree from the lemma. A lemma's edits across all cells make its inflection class. A suffix tree over the word, read from its end and pruned by a description-length test, predicts the class of any word. Known words the tree gets wrong are kept in a small lexicon. Confidences are smoothed counts; variant weights follow Wiktionary's listing order (1, 1/2, 1/3…, nonstandard forms a tenth of that).

## License

Code: MIT OR Apache-2.0. Data: derived from [Wiktionary](https://www.wiktionary.org/) (via [kaikki.org](https://kaikki.org/)), CC BY-SA 4.0; the model bytes in `src/model.bin` are shared under the same license.
