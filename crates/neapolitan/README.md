# neapolitan

Neapolitan inflection for Rust, learned from Wiktionary by [morphe](https://github.com/gold-silver-copper/morphe).
Inflects any word, known or made up. The `_scored` functions give every candidate with a confidence.

Generated from the Kaikki Neapolitan dump of 2026-10-03. Do not edit by hand: change the config in morphe and regenerate.

## Example

```rust
use neapolitan::*;
println!("{}", noun_plural("mana")); // mmane
println!("{}", adj("auto", Form::Feminine)); // auta
println!("{}", verb("fare", Tense::Preterite, Person::Third, Number::Singular)); // facette
println!("{}", verb_participle("fare", PTense::Past)); // fatto
```

Accents are optional on input.

## Coverage

| Part of speech | Entries | Cells | Edits | Classes | Tree nodes | Lexicon |
|---|---|---|---|---|---|---|
| noun | 316 | 1 | 47 | 48 | 19 | 58 |
| adj | 65 | 3 | 33 | 22 | 18 | 25 |
| verb | 30 | 46 | 211 | 17 | 6 | 8 |

Model size: 16432 bytes.

**noun** cells: `Plural`

Citation form: `Lemma, Plural` (comma-separated; parts you give are output exactly and narrow the prediction).

**adj** cells: `Feminine`, `MasculinePlural`, `FemininePlural`

Citation form: `Lemma, Feminine, MasculinePlural` (comma-separated; parts you give are output exactly and narrow the prediction).

**verb** cells: `Infinitive`, `Gerund`, `ParticiplePresent`, `ParticiplePast`, `PresentFirstSingular`, `PresentFirstPlural`, `PresentSecondSingular`, `PresentSecondPlural`, `PresentThirdSingular`, `PresentThirdPlural`, `ImperfectFirstSingular`, `ImperfectFirstPlural`, `ImperfectSecondSingular`, `ImperfectSecondPlural`, `ImperfectThirdSingular`, `ImperfectThirdPlural`, `PreteriteFirstSingular`, `PreteriteFirstPlural`, `PreteriteSecondSingular`, `PreteriteSecondPlural`, `PreteriteThirdSingular`, `PreteriteThirdPlural`, `FutureFirstSingular`, `FutureFirstPlural`, `FutureSecondSingular`, `FutureSecondPlural`, `FutureThirdSingular`, `FutureThirdPlural`, `ConditionalFirstSingular`, `ConditionalFirstPlural`, `ConditionalSecondSingular`, `ConditionalSecondPlural`, `ConditionalThirdSingular`, `ConditionalThirdPlural`, `ImperfectSubjunctiveFirstSingular`, `ImperfectSubjunctiveFirstPlural`, `ImperfectSubjunctiveSecondSingular`, `ImperfectSubjunctiveSecondPlural`, `ImperfectSubjunctiveThirdSingular`, `ImperfectSubjunctiveThirdPlural`, `ImperativeFirstSingular`, `ImperativeFirstPlural`, `ImperativeSecondSingular`, `ImperativeSecondPlural`, `ImperativeThirdSingular`, `ImperativeThirdPlural`

Citation form: `Infinitive, PresentFirstSingular, ParticiplePast` (comma-separated; parts you give are output exactly and narrow the prediction).

## How it works

Each attested form is stored as an edit tree from the lemma. A lemma's edits across all cells make its inflection class. A suffix tree over the word, read from its end and pruned by a description-length test, predicts the class of any word. Known words the tree gets wrong are kept in a small lexicon. Confidences are smoothed counts; variant weights follow Wiktionary's listing order (1, 1/2, 1/3…, nonstandard forms a tenth of that).

## License

Code: MIT OR Apache-2.0. Data: derived from [Wiktionary](https://www.wiktionary.org/) (via [kaikki.org](https://kaikki.org/)), CC BY-SA 4.0; the model bytes in `src/model.bin` are shared under the same license.
