# english

English inflection for Rust, learned from Wiktionary by [morphe](https://github.com/gold-silver-copper/morphe).
Inflects any word, known or made up. The `_scored` functions give every candidate with a confidence.

Generated from the Kaikki English dump of 2026-10-03. Do not edit by hand: change the config in morphe and regenerate.

## Example

```rust
use english::*;
assert_eq!(noun("cat", Number::Plural), "cats");
assert_eq!(noun("child", Number::Plural), "children");
assert_eq!(verb("steal", Tense::Past, Person::Third, Number::Singular), "stole");
assert_eq!(verb_participle("go", Tense::Past), "gone");
assert_eq!(adj("big", Degree::Comparative), "bigger");
assert_eq!(count("potato", 7), "potatoes");
// Every candidate, with confidence:
println!("{:?}", noun_scored("octopus", Number::Plural));
// Homographs give one honest distribution:
println!("{:?}", verb_scored("lie", Tense::Past, Person::Third, Number::Singular));
// Citations pin principal parts: present, past, past participle.
assert_eq!(verb("lie, lied", Tense::Past, Person::Third, Number::Singular), "lied");
```

### Before and after (english 0.4 → 0.5)

```rust
// 0.4
English::noun("die_2", &Number::Plural);                // "dice"
English::verb("walk", &Person::Third, &Number::Singular, &Tense::Past, &Form::Finite);
// 0.5: no sense keys; homographs are scored, citations disambiguate
noun_scored("die", Number::Plural);                      // [("dies", …), ("dice", …)]
verb("walk", Tense::Past, Person::Third, Number::Singular);
verb_participle("walk", Tense::Present);                 // "walking"
```

## Coverage

| Part of speech | Entries | Cells | Edits | Classes | Tree nodes | Lexicon |
|---|---|---|---|---|---|---|
| noun | 353007 | 1 | 1308 | 2091 | 3870 | 19019 |
| verb | 58767 | 14 | 1425 | 1839 | 979 | 17292 |
| adj | 69087 | 2 | 196 | 211 | 683 | 2925 |
| adv | 13228 | 2 | 72 | 51 | 89 | 202 |

Model size: 3597213 bytes.

**noun** cells: `Plural`

Citation form: `Lemma, Plural` (comma-separated; parts you give are output exactly and narrow the prediction).

**verb** cells: `PresentFirstSingular`, `PresentFirstPlural`, `PresentSecondSingular`, `PresentSecondPlural`, `PresentThirdSingular`, `PresentThirdPlural`, `PastFirstSingular`, `PastFirstPlural`, `PastSecondSingular`, `PastSecondPlural`, `PastThirdSingular`, `PastThirdPlural`, `ParticiplePresent`, `ParticiplePast`

Citation form: `Lemma, PastFirstSingular|PastSecondSingular|PastThirdSingular|PastFirstPlural|PastSecondPlural|PastThirdPlural, ParticiplePast` (comma-separated; parts you give are output exactly and narrow the prediction).

**adj** cells: `Comparative`, `Superlative`

Citation form: `Lemma, Comparative, Superlative` (comma-separated; parts you give are output exactly and narrow the prediction).

**adv** cells: `Comparative`, `Superlative`

## How it works

Each attested form is stored as an edit tree from the lemma. A lemma's edits across all cells make its inflection class. A suffix tree over the word, read from its end and pruned by a description-length test, predicts the class of any word. Known words the tree gets wrong are kept in a small lexicon. Confidences are smoothed counts; variant weights follow Wiktionary's listing order (1, 1/2, 1/3…, nonstandard forms a tenth of that).

## License

Code: MIT OR Apache-2.0. Data: derived from [Wiktionary](https://www.wiktionary.org/) (via [kaikki.org](https://kaikki.org/)), CC BY-SA 4.0; the model bytes in `src/model.bin` are shared under the same license.
