# macedonian

Macedonian inflection for Rust, learned from Wiktionary by [morphe](https://github.com/gold-silver-copper/morphe).
Inflects any word, known or made up. The `_scored` functions give every candidate with a confidence.

Generated from the Kaikki Macedonian dump of 2026-10-03. Do not edit by hand: change the config in morphe and regenerate.

## Example

```rust
use macedonian::*;
assert_eq!(noun("град", Article::Definite, Number::Singular), "градот");
println!("{}", noun("жена", Article::Proximal, Number::Plural)); // жениве
println!("{:?}", noun_gender("дете"));
println!("{}", adj("стар", Article::Indefinite, GenderNumber::Feminine)); // стара
println!("{}", verb("пишува", Tense::Imperfect, Person::First, Number::Plural)); // пишувавме
println!("{}", verb("напише", Tense::Aorist, Person::Third, Number::Singular)); // напиша
println!("{}", verb_l_participle("напише", Tense::Aorist, GenderNumber::Feminine)); // напишала
println!("{:?}", verb_aspect("пишува"));
```

Verbs are cited, as in Macedonian dictionaries, by the third person
singular present. They have the present, imperfect and aorist, the
imperative, the l-participles, the passive and adverbial participles, the
perfect participle and the verbal noun; compound tenses (ќе пишувам, сум
пишувал) are built from these. Nouns have the indefinite form and the three
definite articles in both numbers, the vocative and the count form. Input
matching ignores stress marks.

## Coverage

| Part of speech | Entries | Cells | Edits | Classes | Tree nodes | Lexicon |
|---|---|---|---|---|---|---|
| noun | 25792 | 11 | 1977 | 611 | 989 | 6210 |
| adj | 9253 | 18 | 797 | 101 | 532 | 2457 |
| verb | 9793 | 32 | 982 | 251 | 341 | 3952 |

Model size: 1513653 bytes.

**noun** cells: `IndefiniteSingular`, `IndefinitePlural`, `DefiniteSingular`, `DefinitePlural`, `ProximalSingular`, `ProximalPlural`, `DistalSingular`, `DistalPlural`, `VocativeSingular`, `VocativePlural`, `Count`

Citation form: `IndefiniteSingular, IndefinitePlural` (comma-separated; parts you give are output exactly and narrow the prediction).

**adj** cells: `IndefiniteMasculine`, `IndefiniteFeminine`, `IndefiniteNeuter`, `IndefinitePlural`, `DefiniteMasculine`, `DefiniteFeminine`, `DefiniteNeuter`, `DefinitePlural`, `ProximalMasculine`, `ProximalFeminine`, `ProximalNeuter`, `ProximalPlural`, `DistalMasculine`, `DistalFeminine`, `DistalNeuter`, `DistalPlural`, `Comparative`, `Superlative`

**verb** cells: `PresentFirstSingular`, `PresentFirstPlural`, `PresentSecondSingular`, `PresentSecondPlural`, `PresentThirdSingular`, `PresentThirdPlural`, `ImperfectFirstSingular`, `ImperfectFirstPlural`, `ImperfectSecondSingular`, `ImperfectSecondPlural`, `ImperfectThirdSingular`, `ImperfectThirdPlural`, `AoristFirstSingular`, `AoristFirstPlural`, `AoristSecondSingular`, `AoristSecondPlural`, `AoristThirdSingular`, `AoristThirdPlural`, `ImperativeSingular`, `ImperativePlural`, `LParticipleImperfectMasculine`, `LParticipleImperfectFeminine`, `LParticipleImperfectNeuter`, `LParticipleImperfectPlural`, `LParticipleAoristMasculine`, `LParticipleAoristFeminine`, `LParticipleAoristNeuter`, `LParticipleAoristPlural`, `PassiveParticiple`, `AdverbialParticiple`, `PerfectParticiple`, `VerbalNoun`

Citation form: `PresentThirdSingular, PresentFirstSingular` (comma-separated; parts you give are output exactly and narrow the prediction).

## How it works

Each attested form is stored as an edit tree from the lemma. A lemma's edits across all cells make its inflection class. A suffix tree over the word, read from its end and pruned by a description-length test, predicts the class of any word. Known words the tree gets wrong are kept in a small lexicon. Confidences are smoothed counts; variant weights follow Wiktionary's listing order (1, 1/2, 1/3…, nonstandard forms a tenth of that).

## License

Code: MIT OR Apache-2.0. Data: derived from [Wiktionary](https://www.wiktionary.org/) (via [kaikki.org](https://kaikki.org/)), CC BY-SA 4.0; the model bytes in `src/model.bin` are shared under the same license.
