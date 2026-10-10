# classical_nahuatl

ClassicalNahuatl inflection for Rust, learned from Wiktionary by [morphe](https://github.com/gold-silver-copper/morphe).
Inflects any word, known or made up. The `_scored` functions give every candidate with a confidence.

Generated from the Kaikki ClassicalNahuatl dump of 2026-10-03. Do not edit by hand: change the config in morphe and regenerate.

## Example

```rust
use classical_nahuatl::*;
println!("{}", noun_plural("chichi")); // chichimeh
println!("{}", noun_possessed("milli", Possessor::FirstSingular)); // nomīl
println!("{:?}", noun_animacy("pochtecatl")); // Animate
println!("{}", verb("ami", Tense::Preterite, Person::FirstSingular)); // nān
println!("{}", verb("ami", Tense::Future, Person::ThirdPlural)); // āmizqueh
```

Nouns have the plural, the possessed forms for six persons and the
impersonal possessor (tē-), and their animacy. Verbs have the present,
customary, imperfect, preterite, pluperfect, future, near future,
conditional and the optatives for six persons. Input matching ignores
vowel length marks.

## Coverage

| Part of speech | Entries | Cells | Edits | Classes | Tree nodes | Lexicon |
|---|---|---|---|---|---|---|
| noun | 320 | 8 | 219 | 119 | 114 | 247 |
| verb | 519 | 60 | 334 | 14 | 117 | 179 |

Model size: 22484 bytes.

**noun** cells: `Plural`, `PossessedFirstSingular`, `PossessedSecondSingular`, `PossessedThirdSingular`, `PossessedFirstPlural`, `PossessedSecondPlural`, `PossessedThirdPlural`, `PossessedImpersonal`

Citation form: `Lemma, Plural` (comma-separated; parts you give are output exactly and narrow the prediction).

**verb** cells: `PresentFirstSingular`, `PresentSecondSingular`, `PresentThirdSingular`, `PresentFirstPlural`, `PresentSecondPlural`, `PresentThirdPlural`, `CustomaryFirstSingular`, `CustomarySecondSingular`, `CustomaryThirdSingular`, `CustomaryFirstPlural`, `CustomarySecondPlural`, `CustomaryThirdPlural`, `ImperfectFirstSingular`, `ImperfectSecondSingular`, `ImperfectThirdSingular`, `ImperfectFirstPlural`, `ImperfectSecondPlural`, `ImperfectThirdPlural`, `PreteriteFirstSingular`, `PreteriteSecondSingular`, `PreteriteThirdSingular`, `PreteriteFirstPlural`, `PreteriteSecondPlural`, `PreteriteThirdPlural`, `PluperfectFirstSingular`, `PluperfectSecondSingular`, `PluperfectThirdSingular`, `PluperfectFirstPlural`, `PluperfectSecondPlural`, `PluperfectThirdPlural`, `FutureFirstSingular`, `FutureSecondSingular`, `FutureThirdSingular`, `FutureFirstPlural`, `FutureSecondPlural`, `FutureThirdPlural`, `NearFutureFirstSingular`, `NearFutureSecondSingular`, `NearFutureThirdSingular`, `NearFutureFirstPlural`, `NearFutureSecondPlural`, `NearFutureThirdPlural`, `ConditionalFirstSingular`, `ConditionalSecondSingular`, `ConditionalThirdSingular`, `ConditionalFirstPlural`, `ConditionalSecondPlural`, `ConditionalThirdPlural`, `OptativeFirstSingular`, `OptativeSecondSingular`, `OptativeThirdSingular`, `OptativeFirstPlural`, `OptativeSecondPlural`, `OptativeThirdPlural`, `PastOptativeFirstSingular`, `PastOptativeSecondSingular`, `PastOptativeThirdSingular`, `PastOptativeFirstPlural`, `PastOptativeSecondPlural`, `PastOptativeThirdPlural`

Citation form: `PresentThirdSingular, PreteriteThirdSingular` (comma-separated; parts you give are output exactly and narrow the prediction).

## How it works

Each attested form is stored as an edit tree from the lemma. A lemma's edits across all cells make its inflection class. A suffix tree over the word, read from its end and pruned by a description-length test, predicts the class of any word. Known words the tree gets wrong are kept in a small lexicon. Confidences are smoothed counts; variant weights follow Wiktionary's listing order (1, 1/2, 1/3…, nonstandard forms a tenth of that).

## License

Code: MIT OR Apache-2.0. Data: derived from [Wiktionary](https://www.wiktionary.org/) (via [kaikki.org](https://kaikki.org/)), CC BY-SA 4.0; the model bytes in `src/model.bin` are shared under the same license.
