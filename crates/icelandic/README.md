# icelandic

Icelandic inflection for Rust, learned from Wiktionary by [morphe](https://github.com/gold-silver-copper/morphe).
Inflects any word, known or made up. The `_scored` functions give every candidate with a confidence.

Generated from the Kaikki Icelandic dump of 2026-10-03. Do not edit by hand: change the config in morphe and regenerate.

## Example

```rust
use icelandic::*;
assert_eq!(noun("hestur", Definiteness::Definite, Case::Dative, Number::Plural), "hestunum");
println!("{:?}", noun_gender("kona"));
assert_eq!(adj_strong("stór", Degree::Positive, Case::Accusative, Gender::Masculine, Number::Singular), "stóran");
assert_eq!(verb_past("kalla"), "kallaði");
assert_eq!(verb_supine("kalla"), "kallað");
// Citations pin principal parts: nominative, genitive, plural.
println!("{}", noun("maður, manns, menn", Definiteness::Indefinite, Case::Dative, Number::Plural));
```

Wiktionary's Icelandic conjugation tables don't say which form is which
person and tense, so the only finite verb form covered is the third-person
singular past. Personal pronouns are not covered.

## Coverage

| Part of speech | Entries | Cells | Edits | Classes | Tree nodes | Lexicon |
|---|---|---|---|---|---|---|
| noun | 11576 | 16 | 1623 | 825 | 1439 | 3309 |
| adj | 1933 | 120 | 1082 | 203 | 150 | 482 |
| verb | 2064 | 53 | 1152 | 278 | 227 | 951 |

Model size: 1738173 bytes.

**noun** cells: `IndefiniteNominativeSingular`, `IndefiniteNominativePlural`, `IndefiniteAccusativeSingular`, `IndefiniteAccusativePlural`, `IndefiniteDativeSingular`, `IndefiniteDativePlural`, `IndefiniteGenitiveSingular`, `IndefiniteGenitivePlural`, `DefiniteNominativeSingular`, `DefiniteNominativePlural`, `DefiniteAccusativeSingular`, `DefiniteAccusativePlural`, `DefiniteDativeSingular`, `DefiniteDativePlural`, `DefiniteGenitiveSingular`, `DefiniteGenitivePlural`

Citation form: `IndefiniteNominativeSingular, IndefiniteGenitiveSingular, IndefiniteNominativePlural` (comma-separated; parts you give are output exactly and narrow the prediction).

**adj** cells: `StrongPositiveNominativeMasculineSingular`, `StrongPositiveNominativeMasculinePlural`, `StrongPositiveNominativeFeminineSingular`, `StrongPositiveNominativeFemininePlural`, `StrongPositiveNominativeNeuterSingular`, `StrongPositiveNominativeNeuterPlural`, `StrongPositiveAccusativeMasculineSingular`, `StrongPositiveAccusativeMasculinePlural`, `StrongPositiveAccusativeFeminineSingular`, `StrongPositiveAccusativeFemininePlural`, `StrongPositiveAccusativeNeuterSingular`, `StrongPositiveAccusativeNeuterPlural`, `StrongPositiveDativeMasculineSingular`, `StrongPositiveDativeMasculinePlural`, `StrongPositiveDativeFeminineSingular`, `StrongPositiveDativeFemininePlural`, `StrongPositiveDativeNeuterSingular`, `StrongPositiveDativeNeuterPlural`, `StrongPositiveGenitiveMasculineSingular`, `StrongPositiveGenitiveMasculinePlural`, `StrongPositiveGenitiveFeminineSingular`, `StrongPositiveGenitiveFemininePlural`, `StrongPositiveGenitiveNeuterSingular`, `StrongPositiveGenitiveNeuterPlural`, `StrongSuperlativeNominativeMasculineSingular`, `StrongSuperlativeNominativeMasculinePlural`, `StrongSuperlativeNominativeFeminineSingular`, `StrongSuperlativeNominativeFemininePlural`, `StrongSuperlativeNominativeNeuterSingular`, `StrongSuperlativeNominativeNeuterPlural`, `StrongSuperlativeAccusativeMasculineSingular`, `StrongSuperlativeAccusativeMasculinePlural`, `StrongSuperlativeAccusativeFeminineSingular`, `StrongSuperlativeAccusativeFemininePlural`, `StrongSuperlativeAccusativeNeuterSingular`, `StrongSuperlativeAccusativeNeuterPlural`, `StrongSuperlativeDativeMasculineSingular`, `StrongSuperlativeDativeMasculinePlural`, `StrongSuperlativeDativeFeminineSingular`, `StrongSuperlativeDativeFemininePlural`, `StrongSuperlativeDativeNeuterSingular`, `StrongSuperlativeDativeNeuterPlural`, `StrongSuperlativeGenitiveMasculineSingular`, `StrongSuperlativeGenitiveMasculinePlural`, `StrongSuperlativeGenitiveFeminineSingular`, `StrongSuperlativeGenitiveFemininePlural`, `StrongSuperlativeGenitiveNeuterSingular`, `StrongSuperlativeGenitiveNeuterPlural`, `WeakPositiveNominativeMasculineSingular`, `WeakPositiveNominativeMasculinePlural`, `WeakPositiveNominativeFeminineSingular`, `WeakPositiveNominativeFemininePlural`, `WeakPositiveNominativeNeuterSingular`, `WeakPositiveNominativeNeuterPlural`, `WeakPositiveAccusativeMasculineSingular`, `WeakPositiveAccusativeMasculinePlural`, `WeakPositiveAccusativeFeminineSingular`, `WeakPositiveAccusativeFemininePlural`, `WeakPositiveAccusativeNeuterSingular`, `WeakPositiveAccusativeNeuterPlural`, `WeakPositiveDativeMasculineSingular`, `WeakPositiveDativeMasculinePlural`, `WeakPositiveDativeFeminineSingular`, `WeakPositiveDativeFemininePlural`, `WeakPositiveDativeNeuterSingular`, `WeakPositiveDativeNeuterPlural`, `WeakPositiveGenitiveMasculineSingular`, `WeakPositiveGenitiveMasculinePlural`, `WeakPositiveGenitiveFeminineSingular`, `WeakPositiveGenitiveFemininePlural`, `WeakPositiveGenitiveNeuterSingular`, `WeakPositiveGenitiveNeuterPlural`, `WeakComparativeNominativeMasculineSingular`, `WeakComparativeNominativeMasculinePlural`, `WeakComparativeNominativeFeminineSingular`, `WeakComparativeNominativeFemininePlural`, `WeakComparativeNominativeNeuterSingular`, `WeakComparativeNominativeNeuterPlural`, `WeakComparativeAccusativeMasculineSingular`, `WeakComparativeAccusativeMasculinePlural`, `WeakComparativeAccusativeFeminineSingular`, `WeakComparativeAccusativeFemininePlural`, `WeakComparativeAccusativeNeuterSingular`, `WeakComparativeAccusativeNeuterPlural`, `WeakComparativeDativeMasculineSingular`, `WeakComparativeDativeMasculinePlural`, `WeakComparativeDativeFeminineSingular`, `WeakComparativeDativeFemininePlural`, `WeakComparativeDativeNeuterSingular`, `WeakComparativeDativeNeuterPlural`, `WeakComparativeGenitiveMasculineSingular`, `WeakComparativeGenitiveMasculinePlural`, `WeakComparativeGenitiveFeminineSingular`, `WeakComparativeGenitiveFemininePlural`, `WeakComparativeGenitiveNeuterSingular`, `WeakComparativeGenitiveNeuterPlural`, `WeakSuperlativeNominativeMasculineSingular`, `WeakSuperlativeNominativeMasculinePlural`, `WeakSuperlativeNominativeFeminineSingular`, `WeakSuperlativeNominativeFemininePlural`, `WeakSuperlativeNominativeNeuterSingular`, `WeakSuperlativeNominativeNeuterPlural`, `WeakSuperlativeAccusativeMasculineSingular`, `WeakSuperlativeAccusativeMasculinePlural`, `WeakSuperlativeAccusativeFeminineSingular`, `WeakSuperlativeAccusativeFemininePlural`, `WeakSuperlativeAccusativeNeuterSingular`, `WeakSuperlativeAccusativeNeuterPlural`, `WeakSuperlativeDativeMasculineSingular`, `WeakSuperlativeDativeMasculinePlural`, `WeakSuperlativeDativeFeminineSingular`, `WeakSuperlativeDativeFemininePlural`, `WeakSuperlativeDativeNeuterSingular`, `WeakSuperlativeDativeNeuterPlural`, `WeakSuperlativeGenitiveMasculineSingular`, `WeakSuperlativeGenitiveMasculinePlural`, `WeakSuperlativeGenitiveFeminineSingular`, `WeakSuperlativeGenitiveFemininePlural`, `WeakSuperlativeGenitiveNeuterSingular`, `WeakSuperlativeGenitiveNeuterPlural`

**verb** cells: `Infinitive`, `InfinitiveMediopassive`, `Supine`, `Past`, `ParticiplePresent`, `ParticiplePastStrongNominativeMasculineSingular`, `ParticiplePastStrongNominativeMasculinePlural`, `ParticiplePastStrongNominativeFeminineSingular`, `ParticiplePastStrongNominativeFemininePlural`, `ParticiplePastStrongNominativeNeuterSingular`, `ParticiplePastStrongNominativeNeuterPlural`, `ParticiplePastStrongAccusativeMasculineSingular`, `ParticiplePastStrongAccusativeMasculinePlural`, `ParticiplePastStrongAccusativeFeminineSingular`, `ParticiplePastStrongAccusativeFemininePlural`, `ParticiplePastStrongAccusativeNeuterSingular`, `ParticiplePastStrongAccusativeNeuterPlural`, `ParticiplePastStrongDativeMasculineSingular`, `ParticiplePastStrongDativeMasculinePlural`, `ParticiplePastStrongDativeFeminineSingular`, `ParticiplePastStrongDativeFemininePlural`, `ParticiplePastStrongDativeNeuterSingular`, `ParticiplePastStrongDativeNeuterPlural`, `ParticiplePastStrongGenitiveMasculineSingular`, `ParticiplePastStrongGenitiveMasculinePlural`, `ParticiplePastStrongGenitiveFeminineSingular`, `ParticiplePastStrongGenitiveFemininePlural`, `ParticiplePastStrongGenitiveNeuterSingular`, `ParticiplePastStrongGenitiveNeuterPlural`, `ParticiplePastWeakNominativeMasculineSingular`, `ParticiplePastWeakNominativeMasculinePlural`, `ParticiplePastWeakNominativeFeminineSingular`, `ParticiplePastWeakNominativeFemininePlural`, `ParticiplePastWeakNominativeNeuterSingular`, `ParticiplePastWeakNominativeNeuterPlural`, `ParticiplePastWeakAccusativeMasculineSingular`, `ParticiplePastWeakAccusativeMasculinePlural`, `ParticiplePastWeakAccusativeFeminineSingular`, `ParticiplePastWeakAccusativeFemininePlural`, `ParticiplePastWeakAccusativeNeuterSingular`, `ParticiplePastWeakAccusativeNeuterPlural`, `ParticiplePastWeakDativeMasculineSingular`, `ParticiplePastWeakDativeMasculinePlural`, `ParticiplePastWeakDativeFeminineSingular`, `ParticiplePastWeakDativeFemininePlural`, `ParticiplePastWeakDativeNeuterSingular`, `ParticiplePastWeakDativeNeuterPlural`, `ParticiplePastWeakGenitiveMasculineSingular`, `ParticiplePastWeakGenitiveMasculinePlural`, `ParticiplePastWeakGenitiveFeminineSingular`, `ParticiplePastWeakGenitiveFemininePlural`, `ParticiplePastWeakGenitiveNeuterSingular`, `ParticiplePastWeakGenitiveNeuterPlural`

Citation form: `Lemma, Past, Supine` (comma-separated; parts you give are output exactly and narrow the prediction).

## How it works

Each attested form is stored as an edit tree from the lemma. A lemma's edits across all cells make its inflection class. A suffix tree over the word, read from its end and pruned by a description-length test, predicts the class of any word. Known words the tree gets wrong are kept in a small lexicon. Confidences are smoothed counts; variant weights follow Wiktionary's listing order (1, 1/2, 1/3…, nonstandard forms a tenth of that).

## License

Code: MIT OR Apache-2.0. Data: derived from [Wiktionary](https://www.wiktionary.org/) (via [kaikki.org](https://kaikki.org/)), CC BY-SA 4.0; the model bytes in `src/model.bin` are shared under the same license.
