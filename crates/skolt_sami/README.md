# skolt_sami

SkoltSami inflection for Rust, learned from Wiktionary by [morphe](https://github.com/gold-silver-copper/morphe).
Inflects any word, known or made up. The `_scored` functions give every candidate with a confidence.

Generated from the Kaikki SkoltSami dump of 2026-10-03. Do not edit by hand: change the config in morphe and regenerate.

## Example

```rust
use skolt_sami::*;
println!("{}", noun("buss", Case::Locative, Number::Singular)); // buussâst
println!("{}", noun("buss", Case::Comitative, Number::Plural)); // buussivuiʹm
println!("{}", verb("kuõskkâd", Mood::Past, Person::First, Number::Singular)); // kuõʹsǩǩem
println!("{}", verb_impersonal("kuõskkâd", Mood::Present)); // kuõskât
```

Nouns have seven cases in both numbers, plus the essive and the
partitive. Verbs have the indicative present and past, the imperative, the
conditional and the potential for six persons and the impersonal (the
"fourth person"), the connegatives, the infinitive and the participles.
The palatalization mark (ʹ) and the separator (ʼ) are part of the
spelling; Wiktionary's learner marks (ẹ, ˈ) are ignored in input, and
`marks(false)` removes them from output.

## Coverage

| Part of speech | Entries | Cells | Edits | Classes | Tree nodes | Lexicon |
|---|---|---|---|---|---|---|
| noun | 255 | 16 | 1453 | 264 | 57 | 132 |
| verb | 106 | 42 | 928 | 54 | 24 | 59 |

Model size: 217251 bytes.

**noun** cells: `NominativeSingular`, `NominativePlural`, `GenitiveSingular`, `GenitivePlural`, `AccusativeSingular`, `AccusativePlural`, `IllativeSingular`, `IllativePlural`, `LocativeSingular`, `LocativePlural`, `ComitativeSingular`, `ComitativePlural`, `AbessiveSingular`, `AbessivePlural`, `Essive`, `Partitive`

Citation form: `NominativeSingular, GenitiveSingular` (comma-separated; parts you give are output exactly and narrow the prediction).

**verb** cells: `Infinitive`, `PresentFirstSingular`, `PresentFirstPlural`, `PresentSecondSingular`, `PresentSecondPlural`, `PresentThirdSingular`, `PresentThirdPlural`, `PastFirstSingular`, `PastFirstPlural`, `PastSecondSingular`, `PastSecondPlural`, `PastThirdSingular`, `PastThirdPlural`, `ImperativeFirstSingular`, `ImperativeFirstPlural`, `ImperativeSecondSingular`, `ImperativeSecondPlural`, `ImperativeThirdSingular`, `ImperativeThirdPlural`, `ConditionalFirstSingular`, `ConditionalFirstPlural`, `ConditionalSecondSingular`, `ConditionalSecondPlural`, `ConditionalThirdSingular`, `ConditionalThirdPlural`, `PotentialFirstSingular`, `PotentialFirstPlural`, `PotentialSecondSingular`, `PotentialSecondPlural`, `PotentialThirdSingular`, `PotentialThirdPlural`, `ImpersonalPresent`, `ImpersonalPast`, `ImpersonalConditional`, `ImpersonalPotential`, `ConnegativePresent`, `ConnegativePast`, `ConnegativeImperative`, `ConnegativeConditional`, `ConnegativePotential`, `ParticiplePresent`, `ParticiplePast`

Citation form: `Infinitive, PresentFirstSingular, PastFirstSingular` (comma-separated; parts you give are output exactly and narrow the prediction).

## How it works

Each attested form is stored as an edit tree from the lemma. A lemma's edits across all cells make its inflection class. A suffix tree over the word, read from its end and pruned by a description-length test, predicts the class of any word. Known words the tree gets wrong are kept in a small lexicon. Confidences are smoothed counts; variant weights follow Wiktionary's listing order (1, 1/2, 1/3…, nonstandard forms a tenth of that).

## License

Code: MIT OR Apache-2.0. Data: derived from [Wiktionary](https://www.wiktionary.org/) (via [kaikki.org](https://kaikki.org/)), CC BY-SA 4.0; the model bytes in `src/model.bin` are shared under the same license.
