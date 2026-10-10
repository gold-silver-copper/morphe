# mongolian

Mongolian inflection for Rust, learned from Wiktionary by [morphe](https://github.com/gold-silver-copper/morphe).
Inflects any word, known or made up. The `_scored` functions give every candidate with a confidence.

Generated from the Kaikki Mongolian dump of 2026-10-03. Do not edit by hand: change the config in morphe and regenerate.

## Example

```rust
use mongolian::*;
println!("{}", noun("зөгий", Case::Genitive, Number::Singular)); // зөгийн
println!("{}", noun("ном", Case::Ablative, Number::Plural));
println!("{}", noun_reflexive("гэр", Case::DativeLocative, Number::Singular)); // гэртээ
```

Nouns have nine cases and the substantive (зөгийнх, "the bee's") in both
numbers, the reflexive-possessive forms (-аа, "one's own"), and the
attributive form. Wiktionary has Mongolian tables for a few hundred nouns;
verbs have almost none and are left out.

## Coverage

| Part of speech | Entries | Cells | Edits | Classes | Tree nodes | Lexicon |
|---|---|---|---|---|---|---|
| noun | 435 | 39 | 2133 | 448 | 69 | 278 |

Model size: 242628 bytes.

**noun** cells: `NominativeSingular`, `NominativePlural`, `GenitiveSingular`, `GenitivePlural`, `AccusativeSingular`, `AccusativePlural`, `DativeLocativeSingular`, `DativeLocativePlural`, `AblativeSingular`, `AblativePlural`, `InstrumentalSingular`, `InstrumentalPlural`, `ComitativeSingular`, `ComitativePlural`, `PrivativeSingular`, `PrivativePlural`, `DirectiveSingular`, `DirectivePlural`, `SubstantiveSingular`, `SubstantivePlural`, `ReflexiveNominativeSingular`, `ReflexiveNominativePlural`, `ReflexiveGenitiveSingular`, `ReflexiveGenitivePlural`, `ReflexiveAccusativeSingular`, `ReflexiveAccusativePlural`, `ReflexiveDativeLocativeSingular`, `ReflexiveDativeLocativePlural`, `ReflexiveAblativeSingular`, `ReflexiveAblativePlural`, `ReflexiveInstrumentalSingular`, `ReflexiveInstrumentalPlural`, `ReflexiveComitativeSingular`, `ReflexiveComitativePlural`, `ReflexivePrivativeSingular`, `ReflexivePrivativePlural`, `ReflexiveDirectiveSingular`, `ReflexiveDirectivePlural`, `Attributive`

Citation form: `NominativeSingular, GenitiveSingular, NominativePlural` (comma-separated; parts you give are output exactly and narrow the prediction).

## How it works

Each attested form is stored as an edit tree from the lemma. A lemma's edits across all cells make its inflection class. A suffix tree over the word, read from its end and pruned by a description-length test, predicts the class of any word. Known words the tree gets wrong are kept in a small lexicon. Confidences are smoothed counts; variant weights follow Wiktionary's listing order (1, 1/2, 1/3…, nonstandard forms a tenth of that).

## License

Code: MIT OR Apache-2.0. Data: derived from [Wiktionary](https://www.wiktionary.org/) (via [kaikki.org](https://kaikki.org/)), CC BY-SA 4.0; the model bytes in `src/model.bin` are shared under the same license.
