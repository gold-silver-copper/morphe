# swedish

Swedish inflection for Rust, learned from Wiktionary by [morphe](https://github.com/gold-silver-copper/morphe).
Inflects any word, known or made up. The `_scored` functions give every candidate with a confidence.

Generated from the Kaikki Swedish dump of 2026-10-03. Do not edit by hand: change the config in morphe and regenerate.

## Example

```rust
use swedish::*;
assert_eq!(noun("bil", Definiteness::Definite, Case::Nominative, Number::Plural), "bilarna");
println!("{:?}", noun_gender("hus"));
assert_eq!(verb_past("skriva", Voice::Active), "skrev");
assert_eq!(verb_supine("skriva", Voice::Active), "skrivit");
assert_eq!(adj("stor", Degree::Positive, Form::Neuter), "stort");
assert_eq!(adj("stor", Degree::Comparative, Form::Common), "större");
assert_eq!(adj("liten", Degree::Positive, Form::Plural), "små");
// Citations pin principal parts: singular, definite singular, plural.
println!("{}", noun("mus, musen, möss", Definiteness::Definite, Case::Nominative, Number::Plural));
```

Subjunctive and plural verb forms are left out as dated. Personal pronouns
are not covered.

## Coverage

| Part of speech | Entries | Cells | Edits | Classes | Tree nodes | Lexicon |
|---|---|---|---|---|---|---|
| noun | 37182 | 8 | 1351 | 669 | 2510 | 6771 |
| adj | 7736 | 15 | 173 | 98 | 364 | 2499 |
| verb | 6911 | 12 | 1252 | 1090 | 441 | 2912 |
| pron | 30 | 2 | 29 | 23 | 6 | 20 |
| det | 18 | 2 | 14 | 11 | 4 | 12 |
| num | 9 | 8 | 8 | 1 | 2 | 0 |

Model size: 1542473 bytes.

**noun** cells: `IndefiniteNominativeSingular`, `IndefiniteNominativePlural`, `IndefiniteGenitiveSingular`, `IndefiniteGenitivePlural`, `DefiniteNominativeSingular`, `DefiniteNominativePlural`, `DefiniteGenitiveSingular`, `DefiniteGenitivePlural`

Citation form: `IndefiniteNominativeSingular, DefiniteNominativeSingular, IndefiniteNominativePlural` (comma-separated; parts you give are output exactly and narrow the prediction).

**adj** cells: `PositiveCommon`, `PositiveNeuter`, `PositivePlural`, `PositiveDefinite`, `PositiveDefiniteMasculine`, `ComparativeCommon`, `ComparativeNeuter`, `ComparativePlural`, `ComparativeDefinite`, `ComparativeDefiniteMasculine`, `SuperlativeCommon`, `SuperlativeNeuter`, `SuperlativePlural`, `SuperlativeDefinite`, `SuperlativeDefiniteMasculine`

Citation form: `Lemma, ComparativeCommon, SuperlativeCommon` (comma-separated; parts you give are output exactly and narrow the prediction).

**verb** cells: `InfinitiveActive`, `InfinitivePassive`, `PresentActive`, `PresentPassive`, `PastActive`, `PastPassive`, `SupineActive`, `SupinePassive`, `ImperativeActive`, `ImperativePassive`, `ParticiplePresent`, `ParticiplePast`

Citation form: `InfinitiveActive, PresentActive, PastActive, SupineActive` (comma-separated; parts you give are output exactly and narrow the prediction).

**pron** cells: `Neuter`, `Plural`

**det** cells: `Neuter`, `Plural`

**num** cells: `IndefiniteNominativeSingular`, `IndefiniteNominativePlural`, `IndefiniteGenitiveSingular`, `IndefiniteGenitivePlural`, `DefiniteNominativeSingular`, `DefiniteNominativePlural`, `DefiniteGenitiveSingular`, `DefiniteGenitivePlural`

## How it works

Each attested form is stored as an edit tree from the lemma. A lemma's edits across all cells make its inflection class. A suffix tree over the word, read from its end and pruned by a description-length test, predicts the class of any word. Known words the tree gets wrong are kept in a small lexicon. Confidences are smoothed counts; variant weights follow Wiktionary's listing order (1, 1/2, 1/3…, nonstandard forms a tenth of that).

## License

Code: MIT OR Apache-2.0. Data: derived from [Wiktionary](https://www.wiktionary.org/) (via [kaikki.org](https://kaikki.org/)), CC BY-SA 4.0; the model bytes in `src/model.bin` are shared under the same license.
