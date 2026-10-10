# norwegian_bokmal

NorwegianBokmål inflection for Rust, learned from Wiktionary by [morphe](https://github.com/gold-silver-copper/morphe).
Inflects any word, known or made up. The `_scored` functions give every candidate with a confidence.

Generated from the Kaikki NorwegianBokmål dump of 2026-10-03. Do not edit by hand: change the config in morphe and regenerate.

## Example

```rust
use norwegian_bokmal::*;
assert_eq!(noun("gutt", Definiteness::Definite, Number::Plural), "guttene");
println!("{:?}", noun_gender("hus"));
assert_eq!(verb_past("skrive"), "skrev");
assert_eq!(verb_participle_past("skrive"), "skrevet");
assert_eq!(adj_neuter("stor"), "stort");
assert_eq!(adj_plural("liten"), "små");
assert_eq!(adj_superlative_definite("stor"), "største");
// Citations pin principal parts: lemma, definite singular, plural.
println!("{}", noun("bok, boka, bøker", Definiteness::Definite, Number::Plural));
```

Bokmål often allows several forms (kasta/kastet, boka/boken); all are
given with confidences. Forms are given without "å" or "har".

## Coverage

| Part of speech | Entries | Cells | Edits | Classes | Tree nodes | Lexicon |
|---|---|---|---|---|---|---|
| noun | 14440 | 4 | 285 | 399 | 1383 | 3523 |
| adj | 3241 | 6 | 150 | 175 | 196 | 1019 |
| verb | 2961 | 7 | 349 | 613 | 361 | 1347 |

Model size: 589266 bytes.

**noun** cells: `IndefiniteSingular`, `IndefinitePlural`, `DefiniteSingular`, `DefinitePlural`

Citation form: `Lemma, DefiniteSingular, IndefinitePlural` (comma-separated; parts you give are output exactly and narrow the prediction).

**adj** cells: `Neuter`, `Definite`, `Plural`, `Comparative`, `Superlative`, `SuperlativeDefinite`

Citation form: `Lemma, Neuter, Plural` (comma-separated; parts you give are output exactly and narrow the prediction).

**verb** cells: `Present`, `Past`, `Imperative`, `Passive`, `ParticiplePresent`, `ParticiplePast`, `ParticiplePastDefinite`

Citation form: `Lemma, Present, Past, ParticiplePast` (comma-separated; parts you give are output exactly and narrow the prediction).

## How it works

Each attested form is stored as an edit tree from the lemma. A lemma's edits across all cells make its inflection class. A suffix tree over the word, read from its end and pruned by a description-length test, predicts the class of any word. Known words the tree gets wrong are kept in a small lexicon. Confidences are smoothed counts; variant weights follow Wiktionary's listing order (1, 1/2, 1/3…, nonstandard forms a tenth of that).

## License

Code: MIT OR Apache-2.0. Data: derived from [Wiktionary](https://www.wiktionary.org/) (via [kaikki.org](https://kaikki.org/)), CC BY-SA 4.0; the model bytes in `src/model.bin` are shared under the same license.
