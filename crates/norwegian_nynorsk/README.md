# norwegian_nynorsk

NorwegianNynorsk inflection for Rust, learned from Wiktionary by [morphe](https://github.com/gold-silver-copper/morphe).
Inflects any word, known or made up. The `_scored` functions give every candidate with a confidence.

Generated from the Kaikki NorwegianNynorsk dump of 2026-10-03. Do not edit by hand: change the config in morphe and regenerate.

## Example

```rust
use norwegian_nynorsk::*;
assert_eq!(noun("gut", Definiteness::Definite, Number::Plural), "gutane");
println!("{:?}", noun_gender("hus"));
assert_eq!(verb_past("skrive"), "skreiv");
assert_eq!(verb_supine("skrive"), "skrive");
assert_eq!(verb_participle_past("skrive"), "skriven");
assert_eq!(adj_neuter("stor"), "stort");
assert_eq!(adj_superlative_definite("stor"), "største");
// Citations pin principal parts: lemma, definite singular, plural.
println!("{}", noun("bok, boka, bøker", Definiteness::Definite, Number::Plural));
```

Nynorsk often allows several forms (kaste/kast); all are given with
confidences. Forms are given without "å" or "har". Dative and subjunctive
forms are left out as dialectal or archaic.

## Coverage

| Part of speech | Entries | Cells | Edits | Classes | Tree nodes | Lexicon |
|---|---|---|---|---|---|---|
| noun | 15279 | 4 | 472 | 531 | 1521 | 4384 |
| adj | 3137 | 6 | 208 | 218 | 210 | 1113 |
| verb | 3993 | 7 | 1049 | 1196 | 506 | 1435 |

Model size: 304583 bytes.

**noun** cells: `IndefiniteSingular`, `IndefinitePlural`, `DefiniteSingular`, `DefinitePlural`

Citation form: `Lemma, DefiniteSingular, IndefinitePlural` (comma-separated; parts you give are output exactly and narrow the prediction).

**adj** cells: `Neuter`, `Definite`, `Plural`, `Comparative`, `Superlative`, `SuperlativeDefinite`

Citation form: `Lemma, Neuter, Plural` (comma-separated; parts you give are output exactly and narrow the prediction).

**verb** cells: `Present`, `Past`, `Supine`, `Imperative`, `Passive`, `ParticiplePresent`, `ParticiplePast`

Citation form: `Lemma, Present, Past, Supine` (comma-separated; parts you give are output exactly and narrow the prediction).

## How it works

Each attested form is stored as an edit tree from the lemma. A lemma's edits across all cells make its inflection class. A suffix tree over the word, read from its end and pruned by a description-length test, predicts the class of any word. Known words the tree gets wrong are kept in a small lexicon. Confidences are smoothed counts; variant weights follow Wiktionary's listing order (1, 1/2, 1/3…, nonstandard forms a tenth of that).

## License

Code: MIT OR Apache-2.0. Data: derived from [Wiktionary](https://www.wiktionary.org/) (via [kaikki.org](https://kaikki.org/)), CC BY-SA 4.0; the model bytes in `src/model.bin` are shared under the same license.
