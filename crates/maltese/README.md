# maltese

Maltese inflection for Rust, learned from Wiktionary by [morphe](https://github.com/gold-silver-copper/morphe).
Inflects any word, known or made up. The `_scored` functions give every candidate with a confidence.

Generated from the Kaikki Maltese dump of 2026-10-03. Do not edit by hand: change the config in morphe and regenerate.

## Example

```rust
use maltese::*;
println!("{}", noun_plural("art")); // artijiet
println!("{}", noun_possessive("art", Possessor::SecondSingular)); // artek
println!("{}", verb("amar", Tense::Imperfect, PersonNumber::FirstPlural)); // namru
println!("{}", adj("ferm", Form::Comparative)); // ifrem
```

Verbs are cited by their third person masculine singular perfect, as in
Wiktionary. Input may write h, z, g, c for ħ, ż, ġ, ċ.

## Coverage

| Part of speech | Entries | Cells | Edits | Classes | Tree nodes | Lexicon |
|---|---|---|---|---|---|---|
| noun | 4717 | 14 | 1426 | 1669 | 945 | 2548 |
| adj | 1044 | 4 | 255 | 264 | 186 | 465 |
| verb | 2914 | 18 | 2251 | 1534 | 1008 | 2242 |

Model size: 299810 bytes.

**noun** cells: `Plural`, `Paucal`, `Dual`, `Singulative`, `Collective`, `Diminutive`, `Construct`, `PossessiveFirstSingular`, `PossessiveSecondSingular`, `PossessiveThirdMasculine`, `PossessiveThirdFeminine`, `PossessiveFirstPlural`, `PossessiveSecondPlural`, `PossessiveThirdPlural`

Citation form: `Lemma, Plural` (comma-separated; parts you give are output exactly and narrow the prediction).

**adj** cells: `Feminine`, `Plural`, `Comparative`, `Diminutive`

Citation form: `Lemma, Feminine, Plural` (comma-separated; parts you give are output exactly and narrow the prediction).

**verb** cells: `PerfectFirstSingular`, `PerfectSecondSingular`, `PerfectThirdSingularMasculine`, `PerfectThirdSingularFeminine`, `PerfectFirstPlural`, `PerfectSecondPlural`, `PerfectThirdPlural`, `ImperfectFirstSingular`, `ImperfectSecondSingular`, `ImperfectThirdSingularMasculine`, `ImperfectThirdSingularFeminine`, `ImperfectFirstPlural`, `ImperfectSecondPlural`, `ImperfectThirdPlural`, `ImperativeSingular`, `ImperativePlural`, `ParticiplePast`, `VerbalNoun`

Citation form: `Lemma, ImperfectThirdSingularMasculine` (comma-separated; parts you give are output exactly and narrow the prediction).

## How it works

Each attested form is stored as an edit tree from the lemma. A lemma's edits across all cells make its inflection class. A suffix tree over the word, read from its end and pruned by a description-length test, predicts the class of any word. Known words the tree gets wrong are kept in a small lexicon. Confidences are smoothed counts; variant weights follow Wiktionary's listing order (1, 1/2, 1/3…, nonstandard forms a tenth of that).

## License

Code: MIT OR Apache-2.0. Data: derived from [Wiktionary](https://www.wiktionary.org/) (via [kaikki.org](https://kaikki.org/)), CC BY-SA 4.0; the model bytes in `src/model.bin` are shared under the same license.
