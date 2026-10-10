# basque

Basque inflection for Rust, learned from Wiktionary by [morphe](https://github.com/gold-silver-copper/morphe).
Inflects any word, known or made up. The `_scored` functions give every candidate with a confidence.

Generated from the Kaikki Basque dump of 2026-10-03. Do not edit by hand: change the config in morphe and regenerate.

## Example

```rust
use basque::*;
assert_eq!(noun("etxe", Case::Inessive, Number::Singular), "etxean");
println!("{}", noun("mendi", Case::Ablative, Number::Plural)); // mendietatik
println!("{}", noun("gizon", Case::Ergative, Number::Singular)); // gizonak
println!("{}", noun("robot", Case::Inessive, Number::Indefinite)); // robotengan
println!("{}", verb_participle("hartu", Participle::Future)); // hartuko
println!("{}", verb_verbal_noun("hartu")); // hartze
```

Nouns, adjectives, pronouns and numerals have seventeen cases in four
forms: indefinite, definite singular, definite plural and proximal plural
(`Number::Proximal`, "these"). Animate nouns take -gan in the local cases.
Verbs are looked up by their perfect participle and have the imperfect and
future participles, the verbal noun and the short stem; Basque finite
verbs are mostly periphrastic, built on these with an auxiliary.

## Coverage

| Part of speech | Entries | Cells | Edits | Classes | Tree nodes | Lexicon |
|---|---|---|---|---|---|---|
| noun | 3042 | 68 | 265 | 47 | 133 | 590 |
| adj | 509 | 68 | 188 | 11 | 18 | 13 |
| verb | 523 | 4 | 39 | 21 | 25 | 52 |
| pron | 32 | 68 | 2538 | 61 | 2 | 23 |
| num | 108 | 68 | 137 | 8 | 7 | 19 |

Model size: 570873 bytes.

**noun** cells: `AbsolutiveIndefinite`, `AbsolutiveSingular`, `AbsolutivePlural`, `AbsolutiveProximal`, `ErgativeIndefinite`, `ErgativeSingular`, `ErgativePlural`, `ErgativeProximal`, `DativeIndefinite`, `DativeSingular`, `DativePlural`, `DativeProximal`, `GenitiveIndefinite`, `GenitiveSingular`, `GenitivePlural`, `GenitiveProximal`, `ComitativeIndefinite`, `ComitativeSingular`, `ComitativePlural`, `ComitativeProximal`, `CausativeIndefinite`, `CausativeSingular`, `CausativePlural`, `CausativeProximal`, `BenefactiveIndefinite`, `BenefactiveSingular`, `BenefactivePlural`, `BenefactiveProximal`, `InstrumentalIndefinite`, `InstrumentalSingular`, `InstrumentalPlural`, `InstrumentalProximal`, `InessiveIndefinite`, `InessiveSingular`, `InessivePlural`, `InessiveProximal`, `LocativeIndefinite`, `LocativeSingular`, `LocativePlural`, `LocativeProximal`, `AllativeIndefinite`, `AllativeSingular`, `AllativePlural`, `AllativeProximal`, `TerminativeIndefinite`, `TerminativeSingular`, `TerminativePlural`, `TerminativeProximal`, `DirectiveIndefinite`, `DirectiveSingular`, `DirectivePlural`, `DirectiveProximal`, `DestinativeIndefinite`, `DestinativeSingular`, `DestinativePlural`, `DestinativeProximal`, `AblativeIndefinite`, `AblativeSingular`, `AblativePlural`, `AblativeProximal`, `PartitiveIndefinite`, `PartitiveSingular`, `PartitivePlural`, `PartitiveProximal`, `ProlativeIndefinite`, `ProlativeSingular`, `ProlativePlural`, `ProlativeProximal`

Citation form: `AbsolutiveIndefinite, AbsolutiveSingular` (comma-separated; parts you give are output exactly and narrow the prediction).

**adj** cells: `AbsolutiveIndefinite`, `AbsolutiveSingular`, `AbsolutivePlural`, `AbsolutiveProximal`, `ErgativeIndefinite`, `ErgativeSingular`, `ErgativePlural`, `ErgativeProximal`, `DativeIndefinite`, `DativeSingular`, `DativePlural`, `DativeProximal`, `GenitiveIndefinite`, `GenitiveSingular`, `GenitivePlural`, `GenitiveProximal`, `ComitativeIndefinite`, `ComitativeSingular`, `ComitativePlural`, `ComitativeProximal`, `CausativeIndefinite`, `CausativeSingular`, `CausativePlural`, `CausativeProximal`, `BenefactiveIndefinite`, `BenefactiveSingular`, `BenefactivePlural`, `BenefactiveProximal`, `InstrumentalIndefinite`, `InstrumentalSingular`, `InstrumentalPlural`, `InstrumentalProximal`, `InessiveIndefinite`, `InessiveSingular`, `InessivePlural`, `InessiveProximal`, `LocativeIndefinite`, `LocativeSingular`, `LocativePlural`, `LocativeProximal`, `AllativeIndefinite`, `AllativeSingular`, `AllativePlural`, `AllativeProximal`, `TerminativeIndefinite`, `TerminativeSingular`, `TerminativePlural`, `TerminativeProximal`, `DirectiveIndefinite`, `DirectiveSingular`, `DirectivePlural`, `DirectiveProximal`, `DestinativeIndefinite`, `DestinativeSingular`, `DestinativePlural`, `DestinativeProximal`, `AblativeIndefinite`, `AblativeSingular`, `AblativePlural`, `AblativeProximal`, `PartitiveIndefinite`, `PartitiveSingular`, `PartitivePlural`, `PartitiveProximal`, `ProlativeIndefinite`, `ProlativeSingular`, `ProlativePlural`, `ProlativeProximal`

**verb** cells: `ParticipleImperfect`, `ParticipleFuture`, `VerbalNoun`, `Stem`

**pron** cells: `AbsolutiveIndefinite`, `AbsolutiveSingular`, `AbsolutivePlural`, `AbsolutiveProximal`, `ErgativeIndefinite`, `ErgativeSingular`, `ErgativePlural`, `ErgativeProximal`, `DativeIndefinite`, `DativeSingular`, `DativePlural`, `DativeProximal`, `GenitiveIndefinite`, `GenitiveSingular`, `GenitivePlural`, `GenitiveProximal`, `ComitativeIndefinite`, `ComitativeSingular`, `ComitativePlural`, `ComitativeProximal`, `CausativeIndefinite`, `CausativeSingular`, `CausativePlural`, `CausativeProximal`, `BenefactiveIndefinite`, `BenefactiveSingular`, `BenefactivePlural`, `BenefactiveProximal`, `InstrumentalIndefinite`, `InstrumentalSingular`, `InstrumentalPlural`, `InstrumentalProximal`, `InessiveIndefinite`, `InessiveSingular`, `InessivePlural`, `InessiveProximal`, `LocativeIndefinite`, `LocativeSingular`, `LocativePlural`, `LocativeProximal`, `AllativeIndefinite`, `AllativeSingular`, `AllativePlural`, `AllativeProximal`, `TerminativeIndefinite`, `TerminativeSingular`, `TerminativePlural`, `TerminativeProximal`, `DirectiveIndefinite`, `DirectiveSingular`, `DirectivePlural`, `DirectiveProximal`, `DestinativeIndefinite`, `DestinativeSingular`, `DestinativePlural`, `DestinativeProximal`, `AblativeIndefinite`, `AblativeSingular`, `AblativePlural`, `AblativeProximal`, `PartitiveIndefinite`, `PartitiveSingular`, `PartitivePlural`, `PartitiveProximal`, `ProlativeIndefinite`, `ProlativeSingular`, `ProlativePlural`, `ProlativeProximal`

**num** cells: `AbsolutiveIndefinite`, `AbsolutiveSingular`, `AbsolutivePlural`, `AbsolutiveProximal`, `ErgativeIndefinite`, `ErgativeSingular`, `ErgativePlural`, `ErgativeProximal`, `DativeIndefinite`, `DativeSingular`, `DativePlural`, `DativeProximal`, `GenitiveIndefinite`, `GenitiveSingular`, `GenitivePlural`, `GenitiveProximal`, `ComitativeIndefinite`, `ComitativeSingular`, `ComitativePlural`, `ComitativeProximal`, `CausativeIndefinite`, `CausativeSingular`, `CausativePlural`, `CausativeProximal`, `BenefactiveIndefinite`, `BenefactiveSingular`, `BenefactivePlural`, `BenefactiveProximal`, `InstrumentalIndefinite`, `InstrumentalSingular`, `InstrumentalPlural`, `InstrumentalProximal`, `InessiveIndefinite`, `InessiveSingular`, `InessivePlural`, `InessiveProximal`, `LocativeIndefinite`, `LocativeSingular`, `LocativePlural`, `LocativeProximal`, `AllativeIndefinite`, `AllativeSingular`, `AllativePlural`, `AllativeProximal`, `TerminativeIndefinite`, `TerminativeSingular`, `TerminativePlural`, `TerminativeProximal`, `DirectiveIndefinite`, `DirectiveSingular`, `DirectivePlural`, `DirectiveProximal`, `DestinativeIndefinite`, `DestinativeSingular`, `DestinativePlural`, `DestinativeProximal`, `AblativeIndefinite`, `AblativeSingular`, `AblativePlural`, `AblativeProximal`, `PartitiveIndefinite`, `PartitiveSingular`, `PartitivePlural`, `PartitiveProximal`, `ProlativeIndefinite`, `ProlativeSingular`, `ProlativePlural`, `ProlativeProximal`

## How it works

Each attested form is stored as an edit tree from the lemma. A lemma's edits across all cells make its inflection class. A suffix tree over the word, read from its end and pruned by a description-length test, predicts the class of any word. Known words the tree gets wrong are kept in a small lexicon. Confidences are smoothed counts; variant weights follow Wiktionary's listing order (1, 1/2, 1/3…, nonstandard forms a tenth of that).

## License

Code: MIT OR Apache-2.0. Data: derived from [Wiktionary](https://www.wiktionary.org/) (via [kaikki.org](https://kaikki.org/)), CC BY-SA 4.0; the model bytes in `src/model.bin` are shared under the same license.
