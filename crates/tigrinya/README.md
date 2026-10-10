# tigrinya

Tigrinya inflection for Rust, learned from Wiktionary by [morphe](https://github.com/gold-silver-copper/morphe).
Inflects any word, known or made up. The `_scored` functions give every candidate with a confidence.

Generated from the Kaikki Tigrinya dump of 2026-10-03. Do not edit by hand: change the config in morphe and regenerate.

## Example

```rust
use tigrinya::*;
println!("{}", verb("ሰበረ", Tense::Perfect, Person::First, Number::Singular)); // ሰበርኩ
println!("{}", verb("ሰበረ", Tense::Imperfect, Person::ThirdMasculine, Number::Plural)); // ይሰብሩ
println!("{}", verb_imperative("ሰበረ", Gender::Feminine, Number::Singular)); // ስብሪ
println!("{}", verb_infinitive("ከፈተ")); // ምክፋት
println!("{}", noun_plural("ወዲ")); // ኣወዳት
```

Verbs are cited by the perfect third person masculine singular and have
the perfect, gerund, imperfect and jussive for ten persons, the
imperatives and the infinitive. Nouns have their plural.

## Coverage

| Part of speech | Entries | Cells | Edits | Classes | Tree nodes | Lexicon |
|---|---|---|---|---|---|---|
| noun | 93 | 1 | 74 | 73 | 21 | 51 |
| verb | 57 | 49 | 2041 | 57 | 2 | 47 |

Model size: 47093 bytes.

**noun** cells: `Plural`

Citation form: `Lemma, Plural` (comma-separated; parts you give are output exactly and narrow the prediction).

**verb** cells: `Infinitive`, `PerfectFirstSingular`, `PerfectFirstPlural`, `PerfectSecondMasculineSingular`, `PerfectSecondMasculinePlural`, `PerfectSecondFeminineSingular`, `PerfectSecondFemininePlural`, `PerfectThirdMasculineSingular`, `PerfectThirdMasculinePlural`, `PerfectThirdFeminineSingular`, `PerfectThirdFemininePlural`, `GerundFirstSingular`, `GerundFirstPlural`, `GerundSecondMasculineSingular`, `GerundSecondMasculinePlural`, `GerundSecondFeminineSingular`, `GerundSecondFemininePlural`, `GerundThirdMasculineSingular`, `GerundThirdMasculinePlural`, `GerundThirdFeminineSingular`, `GerundThirdFemininePlural`, `ImperfectFirstSingular`, `ImperfectFirstPlural`, `ImperfectSecondMasculineSingular`, `ImperfectSecondMasculinePlural`, `ImperfectSecondFeminineSingular`, `ImperfectSecondFemininePlural`, `ImperfectThirdMasculineSingular`, `ImperfectThirdMasculinePlural`, `ImperfectThirdFeminineSingular`, `ImperfectThirdFemininePlural`, `JussiveFirstSingular`, `JussiveFirstPlural`, `JussiveSecondMasculineSingular`, `JussiveSecondMasculinePlural`, `JussiveSecondFeminineSingular`, `JussiveSecondFemininePlural`, `JussiveThirdMasculineSingular`, `JussiveThirdMasculinePlural`, `JussiveThirdFeminineSingular`, `JussiveThirdFemininePlural`, `ImperativeMasculineSingular`, `ImperativeMasculinePlural`, `ImperativeFeminineSingular`, `ImperativeFemininePlural`, `NegativeImperativeMasculineSingular`, `NegativeImperativeMasculinePlural`, `NegativeImperativeFeminineSingular`, `NegativeImperativeFemininePlural`

Citation form: `PerfectThirdMasculineSingular, ImperfectThirdMasculineSingular` (comma-separated; parts you give are output exactly and narrow the prediction).

## How it works

Each attested form is stored as an edit tree from the lemma. A lemma's edits across all cells make its inflection class. A suffix tree over the word, read from its end and pruned by a description-length test, predicts the class of any word. Known words the tree gets wrong are kept in a small lexicon. Confidences are smoothed counts; variant weights follow Wiktionary's listing order (1, 1/2, 1/3…, nonstandard forms a tenth of that).

## License

Code: MIT OR Apache-2.0. Data: derived from [Wiktionary](https://www.wiktionary.org/) (via [kaikki.org](https://kaikki.org/)), CC BY-SA 4.0; the model bytes in `src/model.bin` are shared under the same license.
