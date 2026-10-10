# catalan

Catalan inflection for Rust, learned from Wiktionary by [morphe](https://github.com/gold-silver-copper/morphe).
Inflects any word, known or made up. The `_scored` functions give every candidate with a confidence.

Generated from the Kaikki Catalan dump of 2026-10-03. Do not edit by hand: change the config in morphe and regenerate.

## Example

```rust
use catalan::*;
assert_eq!(noun("ciutat", Number::Plural), "ciutats");
assert_eq!(noun_counterpart("gat", Gender::Feminine, Number::Singular), "gata");
assert_eq!(adj("feliç", Gender::Masculine, Number::Plural), "feliços");
assert_eq!(verb_indicative("estar", Tense::Present, Person::First, Number::Singular), "estic");
assert_eq!(verb_subjunctive("ser", Tense::Present, Person::Third, Number::Singular), "sigui");
assert_eq!(verb_participle("viure", Gender::Feminine, Number::Plural), "viscudes");
println!("{:?}", noun_gender("mà"));
// Made-up words inflect by analogy:
println!("{}", verb_indicative("brugar", Tense::Preterite, Person::First, Number::Singular));
```

Spelling is matched as written, accents included. Forms are Central
Catalan; Valencian and Balearic spellings Wiktionary lists as alternatives
are left out. The periphrastic past (`vaig parlar`) and negative imperatives
(`no parlis`) are built from these forms.

## Coverage

| Part of speech | Entries | Cells | Edits | Classes | Tree nodes | Lexicon |
|---|---|---|---|---|---|---|
| noun | 20162 | 5 | 622 | 555 | 314 | 3472 |
| adj | 6491 | 4 | 197 | 112 | 171 | 311 |
| verb | 4280 | 54 | 2633 | 273 | 155 | 400 |
| pron | 38 | 4 | 26 | 24 | 5 | 26 |
| det | 18 | 4 | 13 | 10 | 8 | 7 |
| article | 9 | 4 | 19 | 10 | 2 | 7 |

Model size: 843886 bytes.

**noun** cells: `Plural`, `CounterpartMasculineSingular`, `CounterpartMasculinePlural`, `CounterpartFeminineSingular`, `CounterpartFemininePlural`

Citation form: `Lemma, Plural` (comma-separated; parts you give are output exactly and narrow the prediction).

**adj** cells: `MasculineSingular`, `MasculinePlural`, `FeminineSingular`, `FemininePlural`

Citation form: `MasculineSingular, FeminineSingular` (comma-separated; parts you give are output exactly and narrow the prediction).

**verb** cells: `Infinitive`, `Gerund`, `ParticipleMasculineSingular`, `ParticipleMasculinePlural`, `ParticipleFeminineSingular`, `ParticipleFemininePlural`, `IndicativePresentFirstSingular`, `IndicativePresentFirstPlural`, `IndicativePresentSecondSingular`, `IndicativePresentSecondPlural`, `IndicativePresentThirdSingular`, `IndicativePresentThirdPlural`, `IndicativeImperfectFirstSingular`, `IndicativeImperfectFirstPlural`, `IndicativeImperfectSecondSingular`, `IndicativeImperfectSecondPlural`, `IndicativeImperfectThirdSingular`, `IndicativeImperfectThirdPlural`, `IndicativePreteriteFirstSingular`, `IndicativePreteriteFirstPlural`, `IndicativePreteriteSecondSingular`, `IndicativePreteriteSecondPlural`, `IndicativePreteriteThirdSingular`, `IndicativePreteriteThirdPlural`, `IndicativeFutureFirstSingular`, `IndicativeFutureFirstPlural`, `IndicativeFutureSecondSingular`, `IndicativeFutureSecondPlural`, `IndicativeFutureThirdSingular`, `IndicativeFutureThirdPlural`, `ConditionalFirstSingular`, `ConditionalFirstPlural`, `ConditionalSecondSingular`, `ConditionalSecondPlural`, `ConditionalThirdSingular`, `ConditionalThirdPlural`, `SubjunctivePresentFirstSingular`, `SubjunctivePresentFirstPlural`, `SubjunctivePresentSecondSingular`, `SubjunctivePresentSecondPlural`, `SubjunctivePresentThirdSingular`, `SubjunctivePresentThirdPlural`, `SubjunctiveImperfectFirstSingular`, `SubjunctiveImperfectFirstPlural`, `SubjunctiveImperfectSecondSingular`, `SubjunctiveImperfectSecondPlural`, `SubjunctiveImperfectThirdSingular`, `SubjunctiveImperfectThirdPlural`, `ImperativeFirstSingular`, `ImperativeFirstPlural`, `ImperativeSecondSingular`, `ImperativeSecondPlural`, `ImperativeThirdSingular`, `ImperativeThirdPlural`

Citation form: `Infinitive, IndicativePresentFirstSingular, IndicativePreteriteFirstSingular, ParticipleMasculineSingular` (comma-separated; parts you give are output exactly and narrow the prediction).

**pron** cells: `MasculineSingular`, `MasculinePlural`, `FeminineSingular`, `FemininePlural`

**det** cells: `MasculineSingular`, `MasculinePlural`, `FeminineSingular`, `FemininePlural`

**article** cells: `MasculineSingular`, `MasculinePlural`, `FeminineSingular`, `FemininePlural`

## How it works

Each attested form is stored as an edit tree from the lemma. A lemma's edits across all cells make its inflection class. A suffix tree over the word, read from its end and pruned by a description-length test, predicts the class of any word. Known words the tree gets wrong are kept in a small lexicon. Confidences are smoothed counts; variant weights follow Wiktionary's listing order (1, 1/2, 1/3…, nonstandard forms a tenth of that).

## License

Code: MIT OR Apache-2.0. Data: derived from [Wiktionary](https://www.wiktionary.org/) (via [kaikki.org](https://kaikki.org/)), CC BY-SA 4.0; the model bytes in `src/model.bin` are shared under the same license.
