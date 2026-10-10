# italian

Italian inflection for Rust, learned from Wiktionary by [morphe](https://github.com/gold-silver-copper/morphe).
Inflects any word, known or made up. The `_scored` functions give every candidate with a confidence.

Generated from the Kaikki Italian dump of 2026-10-03. Do not edit by hand: change the config in morphe and regenerate.

## Example

```rust
use italian::*;
assert_eq!(noun("uomo", Number::Plural), "uomini");
assert_eq!(noun("uovo", Number::Plural), "uova");
assert_eq!(adj("bello", Gender::Feminine, Number::Plural), "belle");
assert_eq!(verb_indicative("andare", Tense::Present, Person::First, Number::Singular), "vado");
assert_eq!(verb_conditional("essere", Person::Third, Number::Singular), "sarebbe");
assert_eq!(verb_past_participle_declined("prendere", Gender::Feminine, Number::Plural), "prese");
println!("{:?}", noun_gender("mano"));
// Made-up words inflect by analogy:
println!("{}", verb_indicative("sbrumare", Tense::PastHistoric, Person::Third, Number::Singular));
```

Output is standard spelling: Wiktionary's conjugation tables mark stress and
vowel quality on every form (`dàre`, `darèbbe`); only the written final
accent is kept (`darò`, `città`).

## Coverage

| Part of speech | Entries | Cells | Edits | Classes | Tree nodes | Lexicon |
|---|---|---|---|---|---|---|
| noun | 67951 | 3 | 673 | 776 | 1386 | 10333 |
| adj | 26757 | 6 | 106 | 106 | 290 | 1230 |
| verb | 14520 | 58 | 2405 | 808 | 435 | 1751 |
| pron | 90 | 4 | 39 | 33 | 19 | 30 |
| det | 54 | 4 | 14 | 15 | 8 | 15 |
| article | 9 | 4 | 31 | 16 | 2 | 2 |
| num | 9 | 1 | 3 | 3 | 2 | 1 |

Model size: 2273844 bytes.

**noun** cells: `Plural`, `CounterpartMasculine`, `CounterpartFeminine`

Citation form: `Lemma, Plural` (comma-separated; parts you give are output exactly and narrow the prediction).

**adj** cells: `MasculineSingular`, `MasculinePlural`, `FeminineSingular`, `FemininePlural`, `Superlative`, `Adverb`

**verb** cells: `Infinitive`, `Gerund`, `PresentParticiple`, `PastParticiple`, `IndicativePresentFirstSingular`, `IndicativePresentFirstPlural`, `IndicativePresentSecondSingular`, `IndicativePresentSecondPlural`, `IndicativePresentThirdSingular`, `IndicativePresentThirdPlural`, `IndicativeImperfectFirstSingular`, `IndicativeImperfectFirstPlural`, `IndicativeImperfectSecondSingular`, `IndicativeImperfectSecondPlural`, `IndicativeImperfectThirdSingular`, `IndicativeImperfectThirdPlural`, `IndicativePastHistoricFirstSingular`, `IndicativePastHistoricFirstPlural`, `IndicativePastHistoricSecondSingular`, `IndicativePastHistoricSecondPlural`, `IndicativePastHistoricThirdSingular`, `IndicativePastHistoricThirdPlural`, `IndicativeFutureFirstSingular`, `IndicativeFutureFirstPlural`, `IndicativeFutureSecondSingular`, `IndicativeFutureSecondPlural`, `IndicativeFutureThirdSingular`, `IndicativeFutureThirdPlural`, `ConditionalFirstSingular`, `ConditionalFirstPlural`, `ConditionalSecondSingular`, `ConditionalSecondPlural`, `ConditionalThirdSingular`, `ConditionalThirdPlural`, `SubjunctivePresentFirstSingular`, `SubjunctivePresentFirstPlural`, `SubjunctivePresentSecondSingular`, `SubjunctivePresentSecondPlural`, `SubjunctivePresentThirdSingular`, `SubjunctivePresentThirdPlural`, `SubjunctiveImperfectFirstSingular`, `SubjunctiveImperfectFirstPlural`, `SubjunctiveImperfectSecondSingular`, `SubjunctiveImperfectSecondPlural`, `SubjunctiveImperfectThirdSingular`, `SubjunctiveImperfectThirdPlural`, `ImperativeFirstSingular`, `ImperativeFirstPlural`, `ImperativeSecondSingular`, `ImperativeSecondPlural`, `ImperativeThirdSingular`, `ImperativeThirdPlural`, `ImperativeNegativeFirstSingular`, `ImperativeNegativeFirstPlural`, `ImperativeNegativeSecondSingular`, `ImperativeNegativeSecondPlural`, `ImperativeNegativeThirdSingular`, `ImperativeNegativeThirdPlural`

Citation form: `Infinitive, IndicativePresentFirstSingular, IndicativePastHistoricFirstSingular, PastParticiple` (comma-separated; parts you give are output exactly and narrow the prediction).

**pron** cells: `MasculineSingular`, `MasculinePlural`, `FeminineSingular`, `FemininePlural`

**det** cells: `MasculineSingular`, `MasculinePlural`, `FeminineSingular`, `FemininePlural`

**article** cells: `MasculineSingular`, `MasculinePlural`, `FeminineSingular`, `FemininePlural`

**num** cells: `Plural`

## How it works

Each attested form is stored as an edit tree from the lemma. A lemma's edits across all cells make its inflection class. A suffix tree over the word, read from its end and pruned by a description-length test, predicts the class of any word. Known words the tree gets wrong are kept in a small lexicon. Confidences are smoothed counts; variant weights follow Wiktionary's listing order (1, 1/2, 1/3…, nonstandard forms a tenth of that).

## License

Code: MIT OR Apache-2.0. Data: derived from [Wiktionary](https://www.wiktionary.org/) (via [kaikki.org](https://kaikki.org/)), CC BY-SA 4.0; the model bytes in `src/model.bin` are shared under the same license.
