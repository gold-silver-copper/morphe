# manx

Manx inflection for Rust, learned from Wiktionary by [morphe](https://github.com/gold-silver-copper/morphe).
Inflects any word, known or made up. The `_scored` functions give every candidate with a confidence.

Generated from the Kaikki Manx dump of 2026-10-03. Do not edit by hand: change the config in morphe and regenerate.

## Example

```rust
use manx::*;
println!("{}", noun_plural("abb")); // abbyn
println!("{}", noun_mutated("fa", Mutation::Eclipsis)); // va
println!("{}", verb_past("mill", Clause::Independent)); // vill
println!("{}", verb_future("mill", Clause::Independent, Subject::Analytic)); // millee
println!("{}", adj_mutated("gear", Mutation::Lenition)); // ghear
```

The comparative is given without its particle ("s’buiggey" → "buiggey").

## Coverage

| Part of speech | Entries | Cells | Edits | Classes | Tree nodes | Lexicon |
|---|---|---|---|---|---|---|
| noun | 1479 | 4 | 240 | 539 | 208 | 792 |
| adj | 253 | 4 | 66 | 71 | 36 | 63 |
| verb | 280 | 22 | 209 | 227 | 60 | 182 |

Model size: 43447 bytes.

**noun** cells: `GenitiveSingular`, `Plural`, `MutatedLenition`, `MutatedEclipsis`

Citation form: `Lemma, GenitiveSingular, Plural` (comma-separated; parts you give are output exactly and narrow the prediction).

**adj** cells: `Plural`, `Comparative`, `MutatedLenition`, `MutatedEclipsis`

Citation form: `Lemma, Comparative` (comma-separated; parts you give are output exactly and narrow the prediction).

**verb** cells: `VerbalNoun`, `ParticiplePast`, `PresentIndependent`, `PresentDependent`, `PastIndependent`, `PastDependent`, `FutureIndependentFirstSingular`, `FutureIndependentFirstPlural`, `FutureIndependentAnalytic`, `FutureDependentFirstSingular`, `FutureDependentFirstPlural`, `FutureDependentAnalytic`, `ConditionalIndependentFirstSingular`, `ConditionalIndependentFirstPlural`, `ConditionalIndependentAnalytic`, `ConditionalDependentFirstSingular`, `ConditionalDependentFirstPlural`, `ConditionalDependentAnalytic`, `ImperativeSingular`, `ImperativePlural`, `MutatedLenition`, `MutatedEclipsis`

Citation form: `Lemma, VerbalNoun, FutureIndependentAnalytic, PastIndependent` (comma-separated; parts you give are output exactly and narrow the prediction).

## How it works

Each attested form is stored as an edit tree from the lemma. A lemma's edits across all cells make its inflection class. A suffix tree over the word, read from its end and pruned by a description-length test, predicts the class of any word. Known words the tree gets wrong are kept in a small lexicon. Confidences are smoothed counts; variant weights follow Wiktionary's listing order (1, 1/2, 1/3…, nonstandard forms a tenth of that).

## License

Code: MIT OR Apache-2.0. Data: derived from [Wiktionary](https://www.wiktionary.org/) (via [kaikki.org](https://kaikki.org/)), CC BY-SA 4.0; the model bytes in `src/model.bin` are shared under the same license.
