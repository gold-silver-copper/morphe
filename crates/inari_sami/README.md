# inari_sami

InariSami inflection for Rust, learned from Wiktionary by [morphe](https://github.com/gold-silver-copper/morphe).
Inflects any word, known or made up. The `_scored` functions give every candidate with a confidence.

Generated from the Kaikki InariSami dump of 2026-10-03. Do not edit by hand: change the config in morphe and regenerate.

## Example

```rust
use inari_sami::*;
println!("{}", noun("euro", Case::Locative, Number::Singular)); // eurost
println!("{}", noun("euro", Case::Comitative, Number::Plural)); // euroiguin
println!("{}", verb("jaannâđ", Mood::Past, Person::Third, Number::Singular)); // jaanâi
println!("{}", verb("jaannâđ", Mood::Potential, Person::First, Number::Plural)); // janâžep
println!("{}", verb_connegative("jaannâđ", Mood::Present)); // jaanâ
```

Nouns have seven cases in the singular and plural, plus the essive and
the partitive. Verbs have the indicative present and past, the
imperative, the conditional and the potential for nine persons (singular,
dual and plural), the connegatives, the infinitive and the participles.
Wiktionary marks consonant length and gradation (maṇe, jieˈm) the
standard spelling leaves out: input matching ignores the marks, and
`marks(false)` removes them from output.

## Coverage

| Part of speech | Entries | Cells | Edits | Classes | Tree nodes | Lexicon |
|---|---|---|---|---|---|---|
| noun | 346 | 16 | 1971 | 407 | 108 | 192 |
| verb | 41 | 53 | 766 | 31 | 19 | 17 |

Model size: 91152 bytes.

**noun** cells: `NominativeSingular`, `NominativePlural`, `GenitiveSingular`, `GenitivePlural`, `AccusativeSingular`, `AccusativePlural`, `IllativeSingular`, `IllativePlural`, `LocativeSingular`, `LocativePlural`, `ComitativeSingular`, `ComitativePlural`, `AbessiveSingular`, `AbessivePlural`, `Essive`, `Partitive`

Citation form: `NominativeSingular, GenitiveSingular` (comma-separated; parts you give are output exactly and narrow the prediction).

**verb** cells: `Infinitive`, `PresentFirstSingular`, `PresentFirstDual`, `PresentFirstPlural`, `PresentSecondSingular`, `PresentSecondDual`, `PresentSecondPlural`, `PresentThirdSingular`, `PresentThirdDual`, `PresentThirdPlural`, `PastFirstSingular`, `PastFirstDual`, `PastFirstPlural`, `PastSecondSingular`, `PastSecondDual`, `PastSecondPlural`, `PastThirdSingular`, `PastThirdDual`, `PastThirdPlural`, `ImperativeFirstSingular`, `ImperativeFirstDual`, `ImperativeFirstPlural`, `ImperativeSecondSingular`, `ImperativeSecondDual`, `ImperativeSecondPlural`, `ImperativeThirdSingular`, `ImperativeThirdDual`, `ImperativeThirdPlural`, `ConditionalFirstSingular`, `ConditionalFirstDual`, `ConditionalFirstPlural`, `ConditionalSecondSingular`, `ConditionalSecondDual`, `ConditionalSecondPlural`, `ConditionalThirdSingular`, `ConditionalThirdDual`, `ConditionalThirdPlural`, `PotentialFirstSingular`, `PotentialFirstDual`, `PotentialFirstPlural`, `PotentialSecondSingular`, `PotentialSecondDual`, `PotentialSecondPlural`, `PotentialThirdSingular`, `PotentialThirdDual`, `PotentialThirdPlural`, `ConnegativePresent`, `ConnegativePast`, `ConnegativeImperative`, `ConnegativeConditional`, `ConnegativePotential`, `ParticiplePresent`, `ParticiplePast`

Citation form: `Infinitive, PresentFirstSingular, PastFirstSingular` (comma-separated; parts you give are output exactly and narrow the prediction).

## How it works

Each attested form is stored as an edit tree from the lemma. A lemma's edits across all cells make its inflection class. A suffix tree over the word, read from its end and pruned by a description-length test, predicts the class of any word. Known words the tree gets wrong are kept in a small lexicon. Confidences are smoothed counts; variant weights follow Wiktionary's listing order (1, 1/2, 1/3…, nonstandard forms a tenth of that).

## License

Code: MIT OR Apache-2.0. Data: derived from [Wiktionary](https://www.wiktionary.org/) (via [kaikki.org](https://kaikki.org/)), CC BY-SA 4.0; the model bytes in `src/model.bin` are shared under the same license.
