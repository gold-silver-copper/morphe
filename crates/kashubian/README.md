# kashubian

Kashubian inflection for Rust, learned from Wiktionary by [morphe](https://github.com/gold-silver-copper/morphe).
Inflects any word, known or made up. The `_scored` functions give every candidate with a confidence.

Generated from the Kaikki Kashubian dump of 2026-10-03. Do not edit by hand: change the config in morphe and regenerate.

## Example

```rust
use kashubian::*;
println!("{}", noun("abstinent", Case::Dative, Number::Singular)); // abstinentowi
println!("{}", noun_diminutive("pies")); // piesk
println!("{}", adj("chłodny", Case::Genitive, SingularGender::Masculine)); // chłodnégò
println!("{}", adj_simple("chłodny", Degree::Comparative)); // chłodniészi
```

Diacritics are optional on input. Verbs are left out: Wiktionary has
conjugation tables for only a handful.

## Coverage

| Part of speech | Entries | Cells | Edits | Classes | Tree nodes | Lexicon |
|---|---|---|---|---|---|---|
| noun | 554 | 16 | 665 | 548 | 170 | 482 |
| adj | 216 | 38 | 123 | 61 | 57 | 159 |

Model size: 51961 bytes.

**noun** cells: `NominativeSingular`, `NominativePlural`, `GenitiveSingular`, `GenitivePlural`, `DativeSingular`, `DativePlural`, `AccusativeSingular`, `AccusativePlural`, `InstrumentalSingular`, `InstrumentalPlural`, `LocativeSingular`, `LocativePlural`, `VocativeSingular`, `VocativePlural`, `Diminutive`, `Augmentative`

Citation form: `NominativeSingular, GenitiveSingular` (comma-separated; parts you give are output exactly and narrow the prediction).

**adj** cells: `NominativeMasculine`, `NominativeMasculineAnimate`, `NominativeNeuter`, `NominativeFeminine`, `GenitiveMasculine`, `GenitiveMasculineAnimate`, `GenitiveNeuter`, `GenitiveFeminine`, `DativeMasculine`, `DativeMasculineAnimate`, `DativeNeuter`, `DativeFeminine`, `AccusativeMasculine`, `AccusativeMasculineAnimate`, `AccusativeNeuter`, `AccusativeFeminine`, `InstrumentalMasculine`, `InstrumentalMasculineAnimate`, `InstrumentalNeuter`, `InstrumentalFeminine`, `LocativeMasculine`, `LocativeMasculineAnimate`, `LocativeNeuter`, `LocativeFeminine`, `PluralNominativeVirile`, `PluralNominativeNonvirile`, `PluralGenitiveVirile`, `PluralGenitiveNonvirile`, `PluralDativeVirile`, `PluralDativeNonvirile`, `PluralAccusativeVirile`, `PluralAccusativeNonvirile`, `PluralInstrumentalVirile`, `PluralInstrumentalNonvirile`, `PluralLocativeVirile`, `PluralLocativeNonvirile`, `SimpleComparative`, `SimpleSuperlative`

Citation form: `NominativeMasculine, NominativeFeminine, NominativeNeuter` (comma-separated; parts you give are output exactly and narrow the prediction).

## How it works

Each attested form is stored as an edit tree from the lemma. A lemma's edits across all cells make its inflection class. A suffix tree over the word, read from its end and pruned by a description-length test, predicts the class of any word. Known words the tree gets wrong are kept in a small lexicon. Confidences are smoothed counts; variant weights follow Wiktionary's listing order (1, 1/2, 1/3…, nonstandard forms a tenth of that).

## License

Code: MIT OR Apache-2.0. Data: derived from [Wiktionary](https://www.wiktionary.org/) (via [kaikki.org](https://kaikki.org/)), CC BY-SA 4.0; the model bytes in `src/model.bin` are shared under the same license.
