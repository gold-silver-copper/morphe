# livonian

Livonian inflection for Rust, learned from Wiktionary by [morphe](https://github.com/gold-silver-copper/morphe).
Inflects any word, known or made up. The `_scored` functions give every candidate with a confidence.

Generated from the Kaikki Livonian dump of 2026-10-03. Do not edit by hand: change the config in morphe and regenerate.

## Example

```rust
use livonian::*;
println!("{}", noun("ab", Case::Partitive, Number::Plural)); // a’bḑi
println!("{}", verb("mǟdlõ", Mood::Past, Person::First, Number::Singular)); // mǟ’dliz
println!("{}", verb("mǟdlõ", Mood::Conditional, Person::First, Number::Plural)); // mǟ’dlõksmõ
println!("{}", verb_supine("mǟdlõ")); // mǟ’dlõm
```

Nouns have eight cases in both numbers. Verbs have the indicative present
and past, the imperative, jussive, conditional and quotative in all
persons, the infinitive, the supine, the participles and the gerund;
compound tenses, the debitive and negatives are built with auxiliaries
and left out. Input matching ignores the broken-tone apostrophe (’).

## Coverage

| Part of speech | Entries | Cells | Edits | Classes | Tree nodes | Lexicon |
|---|---|---|---|---|---|---|
| noun | 1384 | 16 | 1880 | 461 | 279 | 696 |
| adj | 262 | 16 | 616 | 101 | 60 | 128 |
| verb | 518 | 42 | 1134 | 260 | 96 | 255 |
| pron | 19 | 16 | 222 | 31 | 2 | 16 |
| num | 46 | 16 | 230 | 35 | 10 | 22 |

Model size: 240110 bytes.

**noun** cells: `NominativeSingular`, `NominativePlural`, `GenitiveSingular`, `GenitivePlural`, `PartitiveSingular`, `PartitivePlural`, `DativeSingular`, `DativePlural`, `InstrumentalSingular`, `InstrumentalPlural`, `IllativeSingular`, `IllativePlural`, `InessiveSingular`, `InessivePlural`, `ElativeSingular`, `ElativePlural`

Citation form: `NominativeSingular, GenitiveSingular, PartitiveSingular` (comma-separated; parts you give are output exactly and narrow the prediction).

**adj** cells: `NominativeSingular`, `NominativePlural`, `GenitiveSingular`, `GenitivePlural`, `PartitiveSingular`, `PartitivePlural`, `DativeSingular`, `DativePlural`, `InstrumentalSingular`, `InstrumentalPlural`, `IllativeSingular`, `IllativePlural`, `InessiveSingular`, `InessivePlural`, `ElativeSingular`, `ElativePlural`

**verb** cells: `Infinitive`, `Supine`, `SupineAbessive`, `ActiveParticiple`, `PassiveParticiple`, `Gerund`, `PresentFirstSingular`, `PresentFirstPlural`, `PresentSecondSingular`, `PresentSecondPlural`, `PresentThirdSingular`, `PresentThirdPlural`, `PastFirstSingular`, `PastFirstPlural`, `PastSecondSingular`, `PastSecondPlural`, `PastThirdSingular`, `PastThirdPlural`, `ImperativeFirstSingular`, `ImperativeFirstPlural`, `ImperativeSecondSingular`, `ImperativeSecondPlural`, `ImperativeThirdSingular`, `ImperativeThirdPlural`, `JussiveFirstSingular`, `JussiveFirstPlural`, `JussiveSecondSingular`, `JussiveSecondPlural`, `JussiveThirdSingular`, `JussiveThirdPlural`, `ConditionalFirstSingular`, `ConditionalFirstPlural`, `ConditionalSecondSingular`, `ConditionalSecondPlural`, `ConditionalThirdSingular`, `ConditionalThirdPlural`, `QuotativeFirstSingular`, `QuotativeFirstPlural`, `QuotativeSecondSingular`, `QuotativeSecondPlural`, `QuotativeThirdSingular`, `QuotativeThirdPlural`

Citation form: `Infinitive, PresentFirstSingular, PastFirstSingular` (comma-separated; parts you give are output exactly and narrow the prediction).

**pron** cells: `NominativeSingular`, `NominativePlural`, `GenitiveSingular`, `GenitivePlural`, `PartitiveSingular`, `PartitivePlural`, `DativeSingular`, `DativePlural`, `InstrumentalSingular`, `InstrumentalPlural`, `IllativeSingular`, `IllativePlural`, `InessiveSingular`, `InessivePlural`, `ElativeSingular`, `ElativePlural`

**num** cells: `NominativeSingular`, `NominativePlural`, `GenitiveSingular`, `GenitivePlural`, `PartitiveSingular`, `PartitivePlural`, `DativeSingular`, `DativePlural`, `InstrumentalSingular`, `InstrumentalPlural`, `IllativeSingular`, `IllativePlural`, `InessiveSingular`, `InessivePlural`, `ElativeSingular`, `ElativePlural`

## How it works

Each attested form is stored as an edit tree from the lemma. A lemma's edits across all cells make its inflection class. A suffix tree over the word, read from its end and pruned by a description-length test, predicts the class of any word. Known words the tree gets wrong are kept in a small lexicon. Confidences are smoothed counts; variant weights follow Wiktionary's listing order (1, 1/2, 1/3…, nonstandard forms a tenth of that).

## License

Code: MIT OR Apache-2.0. Data: derived from [Wiktionary](https://www.wiktionary.org/) (via [kaikki.org](https://kaikki.org/)), CC BY-SA 4.0; the model bytes in `src/model.bin` are shared under the same license.
