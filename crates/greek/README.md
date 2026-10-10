# greek

Greek inflection for Rust, learned from Wiktionary by [morphe](https://github.com/gold-silver-copper/morphe).
Inflects any word, known or made up. The `_scored` functions give every candidate with a confidence.

Generated from the Kaikki Greek dump of 2026-10-03. Do not edit by hand: change the config in morphe and regenerate.

## Example

```rust
use greek::*;
assert_eq!(noun("δρόμος", Case::Genitive, Number::Plural), "δρόμων");
println!("{:?}", noun_gender("θάλασσα"));
println!("{}", adj("καλός", Case::Accusative, Gender::Feminine, Number::Plural));
println!("{}", verb("γράφω", Voice::Active, Tense::Aorist, Person::Third, Number::Plural));
println!("{}", verb_imperative("γράφω", Voice::Active, Aspect::Perfective, Number::Plural));
// Accents are optional on input:
println!("{}", noun("θαλασσα", Case::Genitive, Number::Singular));
// Citations pin principal parts: nominative and genitive singular.
println!("{}", noun("γάλα, γάλακτος", Case::Genitive, Number::Plural));
```

Verbs give the synthetic forms; the future (θα + present or dependent), the
subjunctive (να + the same) and the perfect tenses (έχω + infinitive) are
built from them. Input matching ignores the tonos accent.

## Coverage

| Part of speech | Entries | Cells | Edits | Classes | Tree nodes | Lexicon |
|---|---|---|---|---|---|---|
| noun | 49633 | 8 | 4694 | 3475 | 7891 | 13322 |
| adj | 20735 | 48 | 2079 | 646 | 2443 | 6312 |
| verb | 6058 | 61 | 13020 | 4647 | 1118 | 2973 |
| pron | 51 | 24 | 290 | 66 | 21 | 39 |
| num | 19 | 24 | 64 | 18 | 4 | 11 |
| article | 13 | 24 | 149 | 26 | 2 | 3 |

Model size: 1634192 bytes.

**noun** cells: `NominativeSingular`, `NominativePlural`, `GenitiveSingular`, `GenitivePlural`, `AccusativeSingular`, `AccusativePlural`, `VocativeSingular`, `VocativePlural`

Citation form: `NominativeSingular, GenitiveSingular` (comma-separated; parts you give are output exactly and narrow the prediction).

**adj** cells: `NominativeMasculineSingular`, `NominativeMasculinePlural`, `NominativeFeminineSingular`, `NominativeFemininePlural`, `NominativeNeuterSingular`, `NominativeNeuterPlural`, `GenitiveMasculineSingular`, `GenitiveMasculinePlural`, `GenitiveFeminineSingular`, `GenitiveFemininePlural`, `GenitiveNeuterSingular`, `GenitiveNeuterPlural`, `AccusativeMasculineSingular`, `AccusativeMasculinePlural`, `AccusativeFeminineSingular`, `AccusativeFemininePlural`, `AccusativeNeuterSingular`, `AccusativeNeuterPlural`, `VocativeMasculineSingular`, `VocativeMasculinePlural`, `VocativeFeminineSingular`, `VocativeFemininePlural`, `VocativeNeuterSingular`, `VocativeNeuterPlural`, `ComparativeNominativeMasculineSingular`, `ComparativeNominativeMasculinePlural`, `ComparativeNominativeFeminineSingular`, `ComparativeNominativeFemininePlural`, `ComparativeNominativeNeuterSingular`, `ComparativeNominativeNeuterPlural`, `ComparativeGenitiveMasculineSingular`, `ComparativeGenitiveMasculinePlural`, `ComparativeGenitiveFeminineSingular`, `ComparativeGenitiveFemininePlural`, `ComparativeGenitiveNeuterSingular`, `ComparativeGenitiveNeuterPlural`, `ComparativeAccusativeMasculineSingular`, `ComparativeAccusativeMasculinePlural`, `ComparativeAccusativeFeminineSingular`, `ComparativeAccusativeFemininePlural`, `ComparativeAccusativeNeuterSingular`, `ComparativeAccusativeNeuterPlural`, `ComparativeVocativeMasculineSingular`, `ComparativeVocativeMasculinePlural`, `ComparativeVocativeFeminineSingular`, `ComparativeVocativeFemininePlural`, `ComparativeVocativeNeuterSingular`, `ComparativeVocativeNeuterPlural`

Citation form: `NominativeMasculineSingular, NominativeFeminineSingular, NominativeNeuterSingular` (comma-separated; parts you give are output exactly and narrow the prediction).

**verb** cells: `ActivePresentFirstSingular`, `ActivePresentFirstPlural`, `ActivePresentSecondSingular`, `ActivePresentSecondPlural`, `ActivePresentThirdSingular`, `ActivePresentThirdPlural`, `ActiveImperfectFirstSingular`, `ActiveImperfectFirstPlural`, `ActiveImperfectSecondSingular`, `ActiveImperfectSecondPlural`, `ActiveImperfectThirdSingular`, `ActiveImperfectThirdPlural`, `ActiveAoristFirstSingular`, `ActiveAoristFirstPlural`, `ActiveAoristSecondSingular`, `ActiveAoristSecondPlural`, `ActiveAoristThirdSingular`, `ActiveAoristThirdPlural`, `ActiveDependentFirstSingular`, `ActiveDependentFirstPlural`, `ActiveDependentSecondSingular`, `ActiveDependentSecondPlural`, `ActiveDependentThirdSingular`, `ActiveDependentThirdPlural`, `PassivePresentFirstSingular`, `PassivePresentFirstPlural`, `PassivePresentSecondSingular`, `PassivePresentSecondPlural`, `PassivePresentThirdSingular`, `PassivePresentThirdPlural`, `PassiveImperfectFirstSingular`, `PassiveImperfectFirstPlural`, `PassiveImperfectSecondSingular`, `PassiveImperfectSecondPlural`, `PassiveImperfectThirdSingular`, `PassiveImperfectThirdPlural`, `PassiveAoristFirstSingular`, `PassiveAoristFirstPlural`, `PassiveAoristSecondSingular`, `PassiveAoristSecondPlural`, `PassiveAoristThirdSingular`, `PassiveAoristThirdPlural`, `PassiveDependentFirstSingular`, `PassiveDependentFirstPlural`, `PassiveDependentSecondSingular`, `PassiveDependentSecondPlural`, `PassiveDependentThirdSingular`, `PassiveDependentThirdPlural`, `ImperativeActiveImperfectiveSingular`, `ImperativeActiveImperfectivePlural`, `ImperativeActivePerfectiveSingular`, `ImperativeActivePerfectivePlural`, `ImperativePassiveImperfectiveSingular`, `ImperativePassiveImperfectivePlural`, `ImperativePassivePerfectiveSingular`, `ImperativePassivePerfectivePlural`, `InfinitiveActive`, `InfinitivePassive`, `Gerund`, `ParticiplePresent`, `ParticiplePerfect`

Citation form: `ActivePresentFirstSingular|PassivePresentFirstSingular, ActiveAoristFirstSingular|PassiveAoristFirstSingular` (comma-separated; parts you give are output exactly and narrow the prediction).

**pron** cells: `NominativeMasculineSingular`, `NominativeMasculinePlural`, `NominativeFeminineSingular`, `NominativeFemininePlural`, `NominativeNeuterSingular`, `NominativeNeuterPlural`, `GenitiveMasculineSingular`, `GenitiveMasculinePlural`, `GenitiveFeminineSingular`, `GenitiveFemininePlural`, `GenitiveNeuterSingular`, `GenitiveNeuterPlural`, `AccusativeMasculineSingular`, `AccusativeMasculinePlural`, `AccusativeFeminineSingular`, `AccusativeFemininePlural`, `AccusativeNeuterSingular`, `AccusativeNeuterPlural`, `VocativeMasculineSingular`, `VocativeMasculinePlural`, `VocativeFeminineSingular`, `VocativeFemininePlural`, `VocativeNeuterSingular`, `VocativeNeuterPlural`

**num** cells: `NominativeMasculineSingular`, `NominativeMasculinePlural`, `NominativeFeminineSingular`, `NominativeFemininePlural`, `NominativeNeuterSingular`, `NominativeNeuterPlural`, `GenitiveMasculineSingular`, `GenitiveMasculinePlural`, `GenitiveFeminineSingular`, `GenitiveFemininePlural`, `GenitiveNeuterSingular`, `GenitiveNeuterPlural`, `AccusativeMasculineSingular`, `AccusativeMasculinePlural`, `AccusativeFeminineSingular`, `AccusativeFemininePlural`, `AccusativeNeuterSingular`, `AccusativeNeuterPlural`, `VocativeMasculineSingular`, `VocativeMasculinePlural`, `VocativeFeminineSingular`, `VocativeFemininePlural`, `VocativeNeuterSingular`, `VocativeNeuterPlural`

**article** cells: `NominativeMasculineSingular`, `NominativeMasculinePlural`, `NominativeFeminineSingular`, `NominativeFemininePlural`, `NominativeNeuterSingular`, `NominativeNeuterPlural`, `GenitiveMasculineSingular`, `GenitiveMasculinePlural`, `GenitiveFeminineSingular`, `GenitiveFemininePlural`, `GenitiveNeuterSingular`, `GenitiveNeuterPlural`, `AccusativeMasculineSingular`, `AccusativeMasculinePlural`, `AccusativeFeminineSingular`, `AccusativeFemininePlural`, `AccusativeNeuterSingular`, `AccusativeNeuterPlural`, `VocativeMasculineSingular`, `VocativeMasculinePlural`, `VocativeFeminineSingular`, `VocativeFemininePlural`, `VocativeNeuterSingular`, `VocativeNeuterPlural`

## How it works

Each attested form is stored as an edit tree from the lemma. A lemma's edits across all cells make its inflection class. A suffix tree over the word, read from its end and pruned by a description-length test, predicts the class of any word. Known words the tree gets wrong are kept in a small lexicon. Confidences are smoothed counts; variant weights follow Wiktionary's listing order (1, 1/2, 1/3…, nonstandard forms a tenth of that).

## License

Code: MIT OR Apache-2.0. Data: derived from [Wiktionary](https://www.wiktionary.org/) (via [kaikki.org](https://kaikki.org/)), CC BY-SA 4.0; the model bytes in `src/model.bin` are shared under the same license.
