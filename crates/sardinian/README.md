# sardinian

Sardinian inflection for Rust, learned from Wiktionary by [morphe](https://github.com/gold-silver-copper/morphe).
Inflects any word, known or made up. The `_scored` functions give every candidate with a confidence.

Generated from the Kaikki Sardinian dump of 2026-10-03. Do not edit by hand: change the config in morphe and regenerate.

## Example

```rust
use sardinian::*;
println!("{}", noun_plural("abate")); // abates
println!("{}", adj("malu", Form::FemininePlural)); // malas
println!("{}", verb("cantare", Tense::Imperfect, Person::Third, Number::Singular)); // cantaiat
println!("{}", verb_participle_past("cantare")); // cantadu
```

Forms follow the Logudorese-based standard of Wiktionary's tables. The
future and conditional are periphrastic (apo a cantare, dia cantare) and are
left out. Accents are optional on input.

## Coverage

| Part of speech | Entries | Cells | Edits | Classes | Tree nodes | Lexicon |
|---|---|---|---|---|---|---|
| noun | 520 | 1 | 35 | 37 | 30 | 80 |
| adj | 96 | 3 | 8 | 8 | 10 | 9 |
| verb | 37 | 33 | 450 | 41 | 7 | 27 |

Model size: 19867 bytes.

**noun** cells: `Plural`

Citation form: `Lemma, Plural` (comma-separated; parts you give are output exactly and narrow the prediction).

**adj** cells: `Feminine`, `MasculinePlural`, `FemininePlural`

Citation form: `Lemma, Feminine, MasculinePlural` (comma-separated; parts you give are output exactly and narrow the prediction).

**verb** cells: `Infinitive`, `Gerund`, `ParticiplePast`, `PresentFirstSingular`, `PresentFirstPlural`, `PresentSecondSingular`, `PresentSecondPlural`, `PresentThirdSingular`, `PresentThirdPlural`, `ImperfectFirstSingular`, `ImperfectFirstPlural`, `ImperfectSecondSingular`, `ImperfectSecondPlural`, `ImperfectThirdSingular`, `ImperfectThirdPlural`, `SubjunctiveFirstSingular`, `SubjunctiveFirstPlural`, `SubjunctiveSecondSingular`, `SubjunctiveSecondPlural`, `SubjunctiveThirdSingular`, `SubjunctiveThirdPlural`, `ImperfectSubjunctiveFirstSingular`, `ImperfectSubjunctiveFirstPlural`, `ImperfectSubjunctiveSecondSingular`, `ImperfectSubjunctiveSecondPlural`, `ImperfectSubjunctiveThirdSingular`, `ImperfectSubjunctiveThirdPlural`, `ImperativeFirstSingular`, `ImperativeFirstPlural`, `ImperativeSecondSingular`, `ImperativeSecondPlural`, `ImperativeThirdSingular`, `ImperativeThirdPlural`

Citation form: `Infinitive, PresentFirstSingular, ParticiplePast` (comma-separated; parts you give are output exactly and narrow the prediction).

## How it works

Each attested form is stored as an edit tree from the lemma. A lemma's edits across all cells make its inflection class. A suffix tree over the word, read from its end and pruned by a description-length test, predicts the class of any word. Known words the tree gets wrong are kept in a small lexicon. Confidences are smoothed counts; variant weights follow Wiktionary's listing order (1, 1/2, 1/3…, nonstandard forms a tenth of that).

## License

Code: MIT OR Apache-2.0. Data: derived from [Wiktionary](https://www.wiktionary.org/) (via [kaikki.org](https://kaikki.org/)), CC BY-SA 4.0; the model bytes in `src/model.bin` are shared under the same license.
