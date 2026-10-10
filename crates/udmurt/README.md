# udmurt

Udmurt inflection for Rust, learned from Wiktionary by [morphe](https://github.com/gold-silver-copper/morphe).
Inflects any word, known or made up. The `_scored` functions give every candidate with a confidence.

Generated from the Kaikki Udmurt dump of 2026-10-03. Do not edit by hand: change the config in morphe and regenerate.

## Example

```rust
use udmurt::*;
println!("{}", verb("мыныны", Tense::Present, Person::First, Number::Singular)); // мынӥсько
println!("{}", verb("мыныны", Tense::Future, Person::Third, Number::Plural)); // мынозы
println!("{}", verb_negative("мыныны", Tense::Present, Person::Third, Number::Singular)); // уг мыны
println!("{}", noun_possessive("ю", Case::Dative, Number::Singular)); // юзылы
```

Verbs have the present, the future, two preterites (the direct past and
the evidential), and the conditional for six persons, the negatives of the
first four (analytic, with уг/ӧй), the imperative, the participles, the
verbal noun and the gerunds (with their caritive forms). Wiktionary's
Udmurt noun tables come out of Kaikki with only the forms possessed by the
third person singular (юзы, юзылы), in six cases and both numbers.

## Coverage

| Part of speech | Entries | Cells | Edits | Classes | Tree nodes | Lexicon |
|---|---|---|---|---|---|---|
| noun | 737 | 12 | 30 | 11 | 33 | 46 |
| verb | 159 | 72 | 432 | 32 | 14 | 50 |

Model size: 31620 bytes.

**noun** cells: `PossessiveNominativeSingular`, `PossessiveNominativePlural`, `PossessiveAccusativeSingular`, `PossessiveAccusativePlural`, `PossessiveGenitiveSingular`, `PossessiveGenitivePlural`, `PossessiveDativeSingular`, `PossessiveDativePlural`, `PossessiveAblativeSingular`, `PossessiveAblativePlural`, `PossessiveInstrumentalSingular`, `PossessiveInstrumentalPlural`

**verb** cells: `Infinitive`, `PresentFirstSingular`, `PresentFirstPlural`, `PresentSecondSingular`, `PresentSecondPlural`, `PresentThirdSingular`, `PresentThirdPlural`, `FutureFirstSingular`, `FutureFirstPlural`, `FutureSecondSingular`, `FutureSecondPlural`, `FutureThirdSingular`, `FutureThirdPlural`, `PastFirstSingular`, `PastFirstPlural`, `PastSecondSingular`, `PastSecondPlural`, `PastThirdSingular`, `PastThirdPlural`, `EvidentialFirstSingular`, `EvidentialFirstPlural`, `EvidentialSecondSingular`, `EvidentialSecondPlural`, `EvidentialThirdSingular`, `EvidentialThirdPlural`, `ConditionalFirstSingular`, `ConditionalFirstPlural`, `ConditionalSecondSingular`, `ConditionalSecondPlural`, `ConditionalThirdSingular`, `ConditionalThirdPlural`, `NegativePresentFirstSingular`, `NegativePresentFirstPlural`, `NegativePresentSecondSingular`, `NegativePresentSecondPlural`, `NegativePresentThirdSingular`, `NegativePresentThirdPlural`, `NegativeFutureFirstSingular`, `NegativeFutureFirstPlural`, `NegativeFutureSecondSingular`, `NegativeFutureSecondPlural`, `NegativeFutureThirdSingular`, `NegativeFutureThirdPlural`, `NegativePastFirstSingular`, `NegativePastFirstPlural`, `NegativePastSecondSingular`, `NegativePastSecondPlural`, `NegativePastThirdSingular`, `NegativePastThirdPlural`, `NegativeEvidentialFirstSingular`, `NegativeEvidentialFirstPlural`, `NegativeEvidentialSecondSingular`, `NegativeEvidentialSecondPlural`, `NegativeEvidentialThirdSingular`, `NegativeEvidentialThirdPlural`, `ImperativeSingular`, `ImperativePlural`, `ParticiplePresentAffirmative`, `ParticiplePresentCaritive`, `ParticiplePastAffirmative`, `ParticiplePastCaritive`, `ParticipleFutureAffirmative`, `ParticipleFutureCaritive`, `VerbalNoun`, `GerundPlain`, `GerundInstrumental`, `GerundElative`, `GerundTemporal`, `GerundTerminative`, `CaritiveGerundPlain`, `CaritiveGerundInstrumental`, `CaritiveGerundElative`

Citation form: `Infinitive, PresentFirstSingular, PastFirstSingular` (comma-separated; parts you give are output exactly and narrow the prediction).

## How it works

Each attested form is stored as an edit tree from the lemma. A lemma's edits across all cells make its inflection class. A suffix tree over the word, read from its end and pruned by a description-length test, predicts the class of any word. Known words the tree gets wrong are kept in a small lexicon. Confidences are smoothed counts; variant weights follow Wiktionary's listing order (1, 1/2, 1/3…, nonstandard forms a tenth of that).

## License

Code: MIT OR Apache-2.0. Data: derived from [Wiktionary](https://www.wiktionary.org/) (via [kaikki.org](https://kaikki.org/)), CC BY-SA 4.0; the model bytes in `src/model.bin` are shared under the same license.
