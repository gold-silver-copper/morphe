# faroese

Faroese inflection for Rust, learned from Wiktionary by [morphe](https://github.com/gold-silver-copper/morphe).
Inflects any word, known or made up. The `_scored` functions give every candidate with a confidence.

Generated from the Kaikki Faroese dump of 2026-10-03. Do not edit by hand: change the config in morphe and regenerate.

## Example

```rust
use faroese::*;
assert_eq!(noun("hestur", Definiteness::Definite, Case::Dative, Number::Plural), "hestunum");
println!("{:?}", noun_gender("bók"));
assert_eq!(verb_past("fella", PersonNumber::ThirdSingular), "feldi");
assert_eq!(verb_supine("fella"), "felt");
assert_eq!(adj("stórur", Case::Accusative, Number::Singular, Gender::Masculine), "stóran");
assert_eq!(adj_weak("stórur", Case::Nominative, Gender::Masculine), "stóri");
// Citations pin principal parts: lemma, genitive singular, plural.
println!("{}", noun("hestur, hests, hestar", Definiteness::Indefinite, Case::Dative, Number::Plural));
```

## Coverage

| Part of speech | Entries | Cells | Edits | Classes | Tree nodes | Lexicon |
|---|---|---|---|---|---|---|
| noun | 3463 | 16 | 988 | 415 | 427 | 983 |
| adj | 644 | 39 | 413 | 231 | 80 | 288 |
| verb | 798 | 13 | 512 | 215 | 103 | 282 |

Model size: 569940 bytes.

**noun** cells: `IndefiniteNominativeSingular`, `IndefiniteNominativePlural`, `IndefiniteAccusativeSingular`, `IndefiniteAccusativePlural`, `IndefiniteDativeSingular`, `IndefiniteDativePlural`, `IndefiniteGenitiveSingular`, `IndefiniteGenitivePlural`, `DefiniteNominativeSingular`, `DefiniteNominativePlural`, `DefiniteAccusativeSingular`, `DefiniteAccusativePlural`, `DefiniteDativeSingular`, `DefiniteDativePlural`, `DefiniteGenitiveSingular`, `DefiniteGenitivePlural`

Citation form: `Lemma, IndefiniteGenitiveSingular, IndefiniteNominativePlural` (comma-separated; parts you give are output exactly and narrow the prediction).

**adj** cells: `NominativeSingularMasculine`, `NominativeSingularFeminine`, `NominativeSingularNeuter`, `NominativePluralMasculine`, `NominativePluralFeminine`, `NominativePluralNeuter`, `AccusativeSingularMasculine`, `AccusativeSingularFeminine`, `AccusativeSingularNeuter`, `AccusativePluralMasculine`, `AccusativePluralFeminine`, `AccusativePluralNeuter`, `DativeSingularMasculine`, `DativeSingularFeminine`, `DativeSingularNeuter`, `DativePluralMasculine`, `DativePluralFeminine`, `DativePluralNeuter`, `GenitiveSingularMasculine`, `GenitiveSingularFeminine`, `GenitiveSingularNeuter`, `GenitivePluralMasculine`, `GenitivePluralFeminine`, `GenitivePluralNeuter`, `WeakNominativeMasculine`, `WeakNominativeFeminine`, `WeakNominativeNeuter`, `WeakAccusativeMasculine`, `WeakAccusativeFeminine`, `WeakAccusativeNeuter`, `WeakDativeMasculine`, `WeakDativeFeminine`, `WeakDativeNeuter`, `WeakGenitiveMasculine`, `WeakGenitiveFeminine`, `WeakGenitiveNeuter`, `WeakPlural`, `Comparative`, `Superlative`

Citation form: `Lemma, Comparative, Superlative` (comma-separated; parts you give are output exactly and narrow the prediction).

**verb** cells: `PresentFirstSingular`, `PresentSecondSingular`, `PresentThirdSingular`, `PresentPlural`, `PastFirstSingular`, `PastSecondSingular`, `PastThirdSingular`, `PastPlural`, `ImperativeSingular`, `ImperativePlural`, `Supine`, `ParticiplePresent`, `ParticiplePast`

Citation form: `Lemma, PastThirdSingular, Supine` (comma-separated; parts you give are output exactly and narrow the prediction).

## How it works

Each attested form is stored as an edit tree from the lemma. A lemma's edits across all cells make its inflection class. A suffix tree over the word, read from its end and pruned by a description-length test, predicts the class of any word. Known words the tree gets wrong are kept in a small lexicon. Confidences are smoothed counts; variant weights follow Wiktionary's listing order (1, 1/2, 1/3…, nonstandard forms a tenth of that).

## License

Code: MIT OR Apache-2.0. Data: derived from [Wiktionary](https://www.wiktionary.org/) (via [kaikki.org](https://kaikki.org/)), CC BY-SA 4.0; the model bytes in `src/model.bin` are shared under the same license.
