# esperanto

Esperanto inflection for Rust, learned from Wiktionary by [morphe](https://github.com/gold-silver-copper/morphe).
Inflects any word, known or made up. The `_scored` functions give every candidate with a confidence.

Generated from the Kaikki Esperanto dump of 2026-10-03. Do not edit by hand: change the config in morphe and regenerate.

## Example

```rust
use esperanto::*;
assert_eq!(noun("ĉevalo", Case::Accusative, Number::Plural), "ĉevalojn");
assert_eq!(adj("bela", Case::Nominative, Number::Plural), "belaj");
assert_eq!(verb("ami", Tense::Past), "amis");
assert_eq!(verb_participle("ami", Voice::Passive, Tense::Present), "amata");
println!("{}", verb_participle_noun("lerni", Voice::Active, Tense::Present)); // lernanto
// Input without diacritics or in the x-system finds known words:
println!("{}", noun("cxevalo", Case::Accusative, Number::Singular));
```

Esperanto is regular, so unknown words inflect as reliably as known ones.

## Coverage

| Part of speech | Entries | Cells | Edits | Classes | Tree nodes | Lexicon |
|---|---|---|---|---|---|---|
| noun | 18943 | 4 | 145 | 55 | 214 | 4374 |
| adj | 10552 | 4 | 40 | 8 | 7 | 1633 |
| pron | 34 | 3 | 8 | 8 | 6 | 14 |
| verb | 2842 | 23 | 23 | 3 | 81 | 1167 |

Model size: 301130 bytes.

**noun** cells: `NominativeSingular`, `NominativePlural`, `AccusativeSingular`, `AccusativePlural`

**adj** cells: `NominativeSingular`, `NominativePlural`, `AccusativeSingular`, `AccusativePlural`

**pron** cells: `Nominative`, `Accusative`, `Possessive`

**verb** cells: `Present`, `Past`, `Future`, `Conditional`, `Volitive`, `ParticipleActivePresent`, `ParticipleActivePast`, `ParticipleActiveFuture`, `ParticiplePassivePresent`, `ParticiplePassivePast`, `ParticiplePassiveFuture`, `ParticipleNounActivePresent`, `ParticipleNounActivePast`, `ParticipleNounActiveFuture`, `ParticipleNounPassivePresent`, `ParticipleNounPassivePast`, `ParticipleNounPassiveFuture`, `ParticipleAdverbActivePresent`, `ParticipleAdverbActivePast`, `ParticipleAdverbActiveFuture`, `ParticipleAdverbPassivePresent`, `ParticipleAdverbPassivePast`, `ParticipleAdverbPassiveFuture`

## How it works

Each attested form is stored as an edit tree from the lemma. A lemma's edits across all cells make its inflection class. A suffix tree over the word, read from its end and pruned by a description-length test, predicts the class of any word. Known words the tree gets wrong are kept in a small lexicon. Confidences are smoothed counts; variant weights follow Wiktionary's listing order (1, 1/2, 1/3…, nonstandard forms a tenth of that).

## License

Code: MIT OR Apache-2.0. Data: derived from [Wiktionary](https://www.wiktionary.org/) (via [kaikki.org](https://kaikki.org/)), CC BY-SA 4.0; the model bytes in `src/model.bin` are shared under the same license.
