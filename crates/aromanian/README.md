# aromanian

Aromanian inflection for Rust, learned from Wiktionary by [morphe](https://github.com/gold-silver-copper/morphe).
Inflects any word, known or made up. The `_scored` functions give every candidate with a confidence.

Generated from the Kaikki Aromanian dump of 2026-10-03. Do not edit by hand: change the config in morphe and regenerate.

## Example

```rust
use aromanian::*;
println!("{}", noun("frate", Definiteness::Definite, Case::Oblique, Number::Singular)); // fratelui
println!("{}", adj("alb", Form::FemininePlural)); // albi
println!("{}", verb("lucredzu", Tense::Imperfect, Person::Third, Number::Singular)); // lucra
println!("{}", verb_participle_past("lucredzu")); // lucratã
```

Verbs are cited by their first person singular present, as in Wiktionary.
Oblique noun forms are given without the particle "a". ã is optional on
input.

## Coverage

| Part of speech | Entries | Cells | Edits | Classes | Tree nodes | Lexicon |
|---|---|---|---|---|---|---|
| noun | 2321 | 8 | 461 | 530 | 502 | 1431 |
| adj | 328 | 3 | 104 | 143 | 75 | 171 |
| verb | 987 | 29 | 441 | 361 | 196 | 549 |

Model size: 79194 bytes.

**noun** cells: `IndefiniteDirectSingular`, `IndefiniteDirectPlural`, `IndefiniteObliqueSingular`, `IndefiniteObliquePlural`, `DefiniteDirectSingular`, `DefiniteDirectPlural`, `DefiniteObliqueSingular`, `DefiniteObliquePlural`

Citation form: `IndefiniteDirectSingular, IndefiniteDirectPlural` (comma-separated; parts you give are output exactly and narrow the prediction).

**adj** cells: `Feminine`, `MasculinePlural`, `FemininePlural`

Citation form: `Lemma, Feminine, MasculinePlural` (comma-separated; parts you give are output exactly and narrow the prediction).

**verb** cells: `Infinitive`, `Gerund`, `ParticiplePast`, `PresentFirstSingular`, `PresentFirstPlural`, `PresentSecondSingular`, `PresentSecondPlural`, `PresentThirdSingular`, `PresentThirdPlural`, `ImperfectFirstSingular`, `ImperfectFirstPlural`, `ImperfectSecondSingular`, `ImperfectSecondPlural`, `ImperfectThirdSingular`, `ImperfectThirdPlural`, `PerfectFirstSingular`, `PerfectFirstPlural`, `PerfectSecondSingular`, `PerfectSecondPlural`, `PerfectThirdSingular`, `PerfectThirdPlural`, `ConditionalFirstSingular`, `ConditionalFirstPlural`, `ConditionalSecondSingular`, `ConditionalSecondPlural`, `ConditionalThirdSingular`, `ConditionalThirdPlural`, `ImperativeSingular`, `ImperativePlural`

Citation form: `PresentFirstSingular, PerfectFirstSingular, ParticiplePast, Infinitive` (comma-separated; parts you give are output exactly and narrow the prediction).

## How it works

Each attested form is stored as an edit tree from the lemma. A lemma's edits across all cells make its inflection class. A suffix tree over the word, read from its end and pruned by a description-length test, predicts the class of any word. Known words the tree gets wrong are kept in a small lexicon. Confidences are smoothed counts; variant weights follow Wiktionary's listing order (1, 1/2, 1/3…, nonstandard forms a tenth of that).

## License

Code: MIT OR Apache-2.0. Data: derived from [Wiktionary](https://www.wiktionary.org/) (via [kaikki.org](https://kaikki.org/)), CC BY-SA 4.0; the model bytes in `src/model.bin` are shared under the same license.
