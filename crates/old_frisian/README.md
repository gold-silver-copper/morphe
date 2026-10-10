# old_frisian

OldFrisian inflection for Rust, learned from Wiktionary by [morphe](https://github.com/gold-silver-copper/morphe).
Inflects any word, known or made up. The `_scored` functions give every candidate with a confidence.

Generated from the Kaikki OldFrisian dump of 2026-10-03. Do not edit by hand: change the config in morphe and regenerate.

## Example

```rust
use old_frisian::*;
println!("{}", noun("abba", Case::Dative, Number::Plural)); // abbaum
println!("{:?}", noun_gender("brother"));
println!("{}", verb("luka", Tense::Past, PersonNumber::ThirdSingular)); // lāk
println!("{}", verb_participle("makia", Tense::Past)); // emakad
println!("{}", adj("al", Declension::Strong, Case::Accusative, Gender::Masculine)); // alene
```

Macrons are optional on input; output carries them.

## Coverage

| Part of speech | Entries | Cells | Edits | Classes | Tree nodes | Lexicon |
|---|---|---|---|---|---|---|
| noun | 113 | 8 | 66 | 47 | 22 | 62 |
| adj | 75 | 24 | 17 | 5 | 12 | 34 |
| verb | 68 | 17 | 247 | 78 | 23 | 50 |
| det | 6 | 24 | 18 | 5 | 2 | 3 |
| num | 2 | 12 | 13 | 4 | 2 | 1 |

Model size: 41409 bytes.

**noun** cells: `NominativeSingular`, `NominativePlural`, `AccusativeSingular`, `AccusativePlural`, `GenitiveSingular`, `GenitivePlural`, `DativeSingular`, `DativePlural`

Citation form: `NominativeSingular, GenitiveSingular, NominativePlural` (comma-separated; parts you give are output exactly and narrow the prediction).

**adj** cells: `StrongNominativeMasculine`, `StrongNominativeFeminine`, `StrongNominativeNeuter`, `StrongAccusativeMasculine`, `StrongAccusativeFeminine`, `StrongAccusativeNeuter`, `StrongGenitiveMasculine`, `StrongGenitiveFeminine`, `StrongGenitiveNeuter`, `StrongDativeMasculine`, `StrongDativeFeminine`, `StrongDativeNeuter`, `WeakNominativeMasculine`, `WeakNominativeFeminine`, `WeakNominativeNeuter`, `WeakAccusativeMasculine`, `WeakAccusativeFeminine`, `WeakAccusativeNeuter`, `WeakGenitiveMasculine`, `WeakGenitiveFeminine`, `WeakGenitiveNeuter`, `WeakDativeMasculine`, `WeakDativeFeminine`, `WeakDativeNeuter`

Citation form: `StrongNominativeMasculine` (comma-separated; parts you give are output exactly and narrow the prediction).

**verb** cells: `Infinitive`, `PresentFirstSingular`, `PresentSecondSingular`, `PresentThirdSingular`, `PresentPlural`, `PastFirstSingular`, `PastSecondSingular`, `PastThirdSingular`, `PastPlural`, `SubjunctivePresentSingular`, `SubjunctivePresentPlural`, `SubjunctivePastSingular`, `SubjunctivePastPlural`, `ImperativeSingular`, `ImperativePlural`, `ParticiplePresent`, `ParticiplePast`

Citation form: `Infinitive, PastThirdSingular, PastPlural, ParticiplePast` (comma-separated; parts you give are output exactly and narrow the prediction).

**det** cells: `StrongNominativeMasculine`, `StrongNominativeFeminine`, `StrongNominativeNeuter`, `StrongAccusativeMasculine`, `StrongAccusativeFeminine`, `StrongAccusativeNeuter`, `StrongGenitiveMasculine`, `StrongGenitiveFeminine`, `StrongGenitiveNeuter`, `StrongDativeMasculine`, `StrongDativeFeminine`, `StrongDativeNeuter`, `WeakNominativeMasculine`, `WeakNominativeFeminine`, `WeakNominativeNeuter`, `WeakAccusativeMasculine`, `WeakAccusativeFeminine`, `WeakAccusativeNeuter`, `WeakGenitiveMasculine`, `WeakGenitiveFeminine`, `WeakGenitiveNeuter`, `WeakDativeMasculine`, `WeakDativeFeminine`, `WeakDativeNeuter`

**num** cells: `NominativeMasculine`, `NominativeFeminine`, `NominativeNeuter`, `AccusativeMasculine`, `AccusativeFeminine`, `AccusativeNeuter`, `GenitiveMasculine`, `GenitiveFeminine`, `GenitiveNeuter`, `DativeMasculine`, `DativeFeminine`, `DativeNeuter`

## How it works

Each attested form is stored as an edit tree from the lemma. A lemma's edits across all cells make its inflection class. A suffix tree over the word, read from its end and pruned by a description-length test, predicts the class of any word. Known words the tree gets wrong are kept in a small lexicon. Confidences are smoothed counts; variant weights follow Wiktionary's listing order (1, 1/2, 1/3…, nonstandard forms a tenth of that).

## License

Code: MIT OR Apache-2.0. Data: derived from [Wiktionary](https://www.wiktionary.org/) (via [kaikki.org](https://kaikki.org/)), CC BY-SA 4.0; the model bytes in `src/model.bin` are shared under the same license.
