# french

French inflection for Rust, learned from Wiktionary by [morphe](https://github.com/gold-silver-copper/morphe).
Inflects any word, known or made up. The `_scored` functions give every candidate with a confidence.

Generated from the Kaikki French dump of 2026-10-03. Do not edit by hand: change the config in morphe and regenerate.

## Example

```rust
use french::*;
assert_eq!(noun("cheval", Number::Plural), "chevaux");
assert_eq!(noun_counterpart("chien", Gender::Feminine), "chienne");
assert_eq!(adj("beau", Gender::Feminine, Number::Singular), "belle");
assert_eq!(verb_indicative("aller", Tense::Present, Person::First, Number::Singular), "vais");
assert_eq!(verb_subjunctive("faire", Tense::Present, Person::First, Number::Plural), "fassions");
assert_eq!(verb_past_participle_declined("prendre", Gender::Feminine, Number::Plural), "prises");
println!("{:?}", noun_gender("maison"));
// Made-up words inflect by analogy:
println!("{}", verb_indicative("flurger", Tense::PastHistoric, Person::Third, Number::Plural));
```

Spelling is matched as written, accents included. Reflexive conjugations
(`se fier`) and compound tenses (`avoir` + past participle) are not
modelled: build them from the simple forms.

## Coverage

| Part of speech | Entries | Cells | Edits | Classes | Tree nodes | Lexicon |
|---|---|---|---|---|---|---|
| noun | 222574 | 3 | 2197 | 2820 | 3807 | 56838 |
| adj | 72997 | 5 | 893 | 692 | 772 | 3119 |
| verb | 30425 | 49 | 3249 | 373 | 437 | 905 |
| pron | 60 | 4 | 66 | 47 | 10 | 54 |
| det | 56 | 5 | 49 | 28 | 11 | 25 |
| article | 8 | 4 | 10 | 6 | 2 | 1 |
| num | 42 | 4 | 4 | 3 | 6 | 2 |

Model size: 2258628 bytes.

**noun** cells: `Plural`, `CounterpartMasculine`, `CounterpartFeminine`

Citation form: `Lemma, Plural` (comma-separated; parts you give are output exactly and narrow the prediction).

**adj** cells: `MasculineSingular`, `MasculinePlural`, `FeminineSingular`, `FemininePlural`, `BeforeVowel`

Citation form: `MasculineSingular, FeminineSingular` (comma-separated; parts you give are output exactly and narrow the prediction).

**verb** cells: `Infinitive`, `PresentParticiple`, `PastParticiple`, `IndicativePresentFirstSingular`, `IndicativePresentFirstPlural`, `IndicativePresentSecondSingular`, `IndicativePresentSecondPlural`, `IndicativePresentThirdSingular`, `IndicativePresentThirdPlural`, `IndicativeImperfectFirstSingular`, `IndicativeImperfectFirstPlural`, `IndicativeImperfectSecondSingular`, `IndicativeImperfectSecondPlural`, `IndicativeImperfectThirdSingular`, `IndicativeImperfectThirdPlural`, `IndicativePastHistoricFirstSingular`, `IndicativePastHistoricFirstPlural`, `IndicativePastHistoricSecondSingular`, `IndicativePastHistoricSecondPlural`, `IndicativePastHistoricThirdSingular`, `IndicativePastHistoricThirdPlural`, `IndicativeFutureFirstSingular`, `IndicativeFutureFirstPlural`, `IndicativeFutureSecondSingular`, `IndicativeFutureSecondPlural`, `IndicativeFutureThirdSingular`, `IndicativeFutureThirdPlural`, `ConditionalFirstSingular`, `ConditionalFirstPlural`, `ConditionalSecondSingular`, `ConditionalSecondPlural`, `ConditionalThirdSingular`, `ConditionalThirdPlural`, `SubjunctivePresentFirstSingular`, `SubjunctivePresentFirstPlural`, `SubjunctivePresentSecondSingular`, `SubjunctivePresentSecondPlural`, `SubjunctivePresentThirdSingular`, `SubjunctivePresentThirdPlural`, `SubjunctiveImperfectFirstSingular`, `SubjunctiveImperfectFirstPlural`, `SubjunctiveImperfectSecondSingular`, `SubjunctiveImperfectSecondPlural`, `SubjunctiveImperfectThirdSingular`, `SubjunctiveImperfectThirdPlural`, `ImperativeFirstSingular`, `ImperativeFirstPlural`, `ImperativeSecondSingular`, `ImperativeSecondPlural`

Citation form: `Infinitive, PresentParticiple, PastParticiple, IndicativePresentFirstSingular, IndicativePastHistoricFirstSingular` (comma-separated; parts you give are output exactly and narrow the prediction).

**pron** cells: `MasculineSingular`, `MasculinePlural`, `FeminineSingular`, `FemininePlural`

**det** cells: `MasculineSingular`, `MasculinePlural`, `FeminineSingular`, `FemininePlural`, `BeforeVowel`

**article** cells: `MasculineSingular`, `MasculinePlural`, `FeminineSingular`, `FemininePlural`

**num** cells: `MasculineSingular`, `MasculinePlural`, `FeminineSingular`, `FemininePlural`

## How it works

Each attested form is stored as an edit tree from the lemma. A lemma's edits across all cells make its inflection class. A suffix tree over the word, read from its end and pruned by a description-length test, predicts the class of any word. Known words the tree gets wrong are kept in a small lexicon. Confidences are smoothed counts; variant weights follow Wiktionary's listing order (1, 1/2, 1/3…, nonstandard forms a tenth of that).

## License

Code: MIT OR Apache-2.0. Data: derived from [Wiktionary](https://www.wiktionary.org/) (via [kaikki.org](https://kaikki.org/)), CC BY-SA 4.0; the model bytes in `src/model.bin` are shared under the same license.
