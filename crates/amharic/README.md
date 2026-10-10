# amharic

Amharic inflection for Rust, learned from Wiktionary by [morphe](https://github.com/gold-silver-copper/morphe).
Inflects any word, known or made up. The `_scored` functions give every candidate with a confidence.

Generated from the Kaikki Amharic dump of 2026-10-03. Do not edit by hand: change the config in morphe and regenerate.

## Example

```rust
use amharic::*;
println!("{}", noun_plural("ጓደኛ")); // ጓደኞች
println!("{}", noun_definite("ቤት", Gender::Masculine)); // ቤቱ
println!("{}", noun_definite_object("ቤት", Gender::Masculine)); // ቤቱን
println!("{}", noun_possessive("ቤት", Possessor::FirstSingular)); // ቤቴ
println!("{}", noun_possessive("ቤት", Possessor::SecondPlural)); // ቤታችሁ
```

Nouns have the plural, the definite (masculine -ው, feminine -ዋ/-ይቱ),
the definite and indefinite object (-ን), and the possessed forms for ten
possessors; adjectives the same without the plural. Wiktionary has no
Amharic verb tables, so verbs are left out.

## Coverage

| Part of speech | Entries | Cells | Edits | Classes | Tree nodes | Lexicon |
|---|---|---|---|---|---|---|
| noun | 369 | 16 | 290 | 152 | 44 | 202 |
| adj | 51 | 15 | 105 | 26 | 8 | 7 |

Model size: 24300 bytes.

**noun** cells: `Plural`, `DefiniteMasculine`, `DefiniteFeminine`, `DefiniteObjectMasculine`, `DefiniteObjectFeminine`, `Object`, `PossessiveFirstSingular`, `PossessiveFirstPlural`, `PossessiveSecondMasculine`, `PossessiveSecondFeminine`, `PossessiveSecondPlural`, `PossessiveSecondFormal`, `PossessiveThirdMasculine`, `PossessiveThirdFeminine`, `PossessiveThirdPlural`, `PossessiveThirdFormal`

Citation form: `Lemma, Plural` (comma-separated; parts you give are output exactly and narrow the prediction).

**adj** cells: `DefiniteMasculine`, `DefiniteFeminine`, `DefiniteObjectMasculine`, `DefiniteObjectFeminine`, `Object`, `PossessiveFirstSingular`, `PossessiveFirstPlural`, `PossessiveSecondMasculine`, `PossessiveSecondFeminine`, `PossessiveSecondPlural`, `PossessiveSecondFormal`, `PossessiveThirdMasculine`, `PossessiveThirdFeminine`, `PossessiveThirdPlural`, `PossessiveThirdFormal`

## How it works

Each attested form is stored as an edit tree from the lemma. A lemma's edits across all cells make its inflection class. A suffix tree over the word, read from its end and pruned by a description-length test, predicts the class of any word. Known words the tree gets wrong are kept in a small lexicon. Confidences are smoothed counts; variant weights follow Wiktionary's listing order (1, 1/2, 1/3…, nonstandard forms a tenth of that).

## License

Code: MIT OR Apache-2.0. Data: derived from [Wiktionary](https://www.wiktionary.org/) (via [kaikki.org](https://kaikki.org/)), CC BY-SA 4.0; the model bytes in `src/model.bin` are shared under the same license.
