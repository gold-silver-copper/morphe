# afrikaans

Afrikaans inflection for Rust, learned from Wiktionary by [morphe](https://github.com/gold-silver-copper/morphe).
Inflects any word, known or made up. The `_scored` functions give every candidate with a confidence.

Generated from the Kaikki Afrikaans dump of 2026-10-03. Do not edit by hand: change the config in morphe and regenerate.

## Example

```rust
use afrikaans::*;
assert_eq!(noun_plural("hond"), "honde");
assert_eq!(noun_diminutive("hond"), "hondjie");
assert_eq!(adj_attributive("arm"), "arme");
assert_eq!(adj_comparative("arm"), "armer");
println!("{}", verb_participle_past("skryf"));
```

Afrikaans verbs barely inflect: the past is formed with "het" and the past
participle. Nouns have no grammatical gender.

## Coverage

| Part of speech | Entries | Cells | Edits | Classes | Tree nodes | Lexicon |
|---|---|---|---|---|---|---|
| noun | 3444 | 2 | 142 | 245 | 316 | 1223 |
| adj | 727 | 5 | 99 | 107 | 58 | 173 |
| verb | 511 | 4 | 113 | 149 | 60 | 254 |

Model size: 93775 bytes.

**noun** cells: `Plural`, `Diminutive`

Citation form: `Lemma, Plural` (comma-separated; parts you give are output exactly and narrow the prediction).

**adj** cells: `Attributive`, `Comparative`, `ComparativeAttributive`, `Superlative`, `Partitive`

Citation form: `Lemma, Attributive, Comparative, Superlative` (comma-separated; parts you give are output exactly and narrow the prediction).

**verb** cells: `Present`, `Past`, `ParticiplePresent`, `ParticiplePast`

Citation form: `Lemma, ParticiplePast` (comma-separated; parts you give are output exactly and narrow the prediction).

## How it works

Each attested form is stored as an edit tree from the lemma. A lemma's edits across all cells make its inflection class. A suffix tree over the word, read from its end and pruned by a description-length test, predicts the class of any word. Known words the tree gets wrong are kept in a small lexicon. Confidences are smoothed counts; variant weights follow Wiktionary's listing order (1, 1/2, 1/3…, nonstandard forms a tenth of that).

## License

Code: MIT OR Apache-2.0. Data: derived from [Wiktionary](https://www.wiktionary.org/) (via [kaikki.org](https://kaikki.org/)), CC BY-SA 4.0; the model bytes in `src/model.bin` are shared under the same license.
