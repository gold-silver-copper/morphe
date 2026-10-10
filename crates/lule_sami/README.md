# lule_sami

LuleSami inflection for Rust, learned from Wiktionary by [morphe](https://github.com/gold-silver-copper/morphe).
Inflects any word, known or made up. The `_scored` functions give every candidate with a confidence.

Generated from the Kaikki LuleSami dump of 2026-10-03. Do not edit by hand: change the config in morphe and regenerate.

## Example

```rust
use lule_sami::*;
println!("{}", noun("suola", Case::Genitive, Number::Singular));
println!("{}", noun("valle", Case::Illative, Number::Singular)); // valláj
println!("{}", noun_possessive("suola", Person::First, Number::Singular)); // suollagim
println!("{}", verb("dahkat", Mood::Potential, Person::First, Number::Singular)); // dagátjav
println!("{}", verb_connegative("dahkat", ConnegativeMood::Indicative)); // dagá
```

Nouns and adjectives have eight cases in the singular and plural, the
essive, and the possessed forms for nine possessors; adjectives also have
their attributive form. Verbs have the indicative present and past, the
imperative, the optative, the conditional and the potential for nine
persons (singular, dual and plural), the connegatives, the infinitive and
the participles. Input matching ignores the length marks (ˈ, ā) and reads the
Norwegian æ and ń as ä and ŋ.

## Coverage

| Part of speech | Entries | Cells | Edits | Classes | Tree nodes | Lexicon |
|---|---|---|---|---|---|---|
| noun | 572 | 26 | 2876 | 560 | 125 | 260 |
| adj | 85 | 27 | 587 | 121 | 24 | 50 |
| verb | 119 | 62 | 1489 | 75 | 44 | 50 |

Model size: 580533 bytes.

**noun** cells: `NominativeSingular`, `NominativePlural`, `GenitiveSingular`, `GenitivePlural`, `AccusativeSingular`, `AccusativePlural`, `IllativeSingular`, `IllativePlural`, `InessiveSingular`, `InessivePlural`, `ElativeSingular`, `ElativePlural`, `ComitativeSingular`, `ComitativePlural`, `AbessiveSingular`, `AbessivePlural`, `Essive`, `PossessiveFirstSingular`, `PossessiveFirstDual`, `PossessiveFirstPlural`, `PossessiveSecondSingular`, `PossessiveSecondDual`, `PossessiveSecondPlural`, `PossessiveThirdSingular`, `PossessiveThirdDual`, `PossessiveThirdPlural`

Citation form: `NominativeSingular, GenitiveSingular` (comma-separated; parts you give are output exactly and narrow the prediction).

**adj** cells: `NominativeSingular`, `NominativePlural`, `GenitiveSingular`, `GenitivePlural`, `AccusativeSingular`, `AccusativePlural`, `IllativeSingular`, `IllativePlural`, `InessiveSingular`, `InessivePlural`, `ElativeSingular`, `ElativePlural`, `ComitativeSingular`, `ComitativePlural`, `AbessiveSingular`, `AbessivePlural`, `Essive`, `PossessiveFirstSingular`, `PossessiveFirstDual`, `PossessiveFirstPlural`, `PossessiveSecondSingular`, `PossessiveSecondDual`, `PossessiveSecondPlural`, `PossessiveThirdSingular`, `PossessiveThirdDual`, `PossessiveThirdPlural`, `Attributive`

**verb** cells: `Infinitive`, `PresentFirstSingular`, `PresentFirstDual`, `PresentFirstPlural`, `PresentSecondSingular`, `PresentSecondDual`, `PresentSecondPlural`, `PresentThirdSingular`, `PresentThirdDual`, `PresentThirdPlural`, `PastFirstSingular`, `PastFirstDual`, `PastFirstPlural`, `PastSecondSingular`, `PastSecondDual`, `PastSecondPlural`, `PastThirdSingular`, `PastThirdDual`, `PastThirdPlural`, `ImperativeFirstSingular`, `ImperativeFirstDual`, `ImperativeFirstPlural`, `ImperativeSecondSingular`, `ImperativeSecondDual`, `ImperativeSecondPlural`, `ImperativeThirdSingular`, `ImperativeThirdDual`, `ImperativeThirdPlural`, `OptativeFirstSingular`, `OptativeFirstDual`, `OptativeFirstPlural`, `OptativeSecondSingular`, `OptativeSecondDual`, `OptativeSecondPlural`, `OptativeThirdSingular`, `OptativeThirdDual`, `OptativeThirdPlural`, `ConditionalFirstSingular`, `ConditionalFirstDual`, `ConditionalFirstPlural`, `ConditionalSecondSingular`, `ConditionalSecondDual`, `ConditionalSecondPlural`, `ConditionalThirdSingular`, `ConditionalThirdDual`, `ConditionalThirdPlural`, `PotentialFirstSingular`, `PotentialFirstDual`, `PotentialFirstPlural`, `PotentialSecondSingular`, `PotentialSecondDual`, `PotentialSecondPlural`, `PotentialThirdSingular`, `PotentialThirdDual`, `PotentialThirdPlural`, `ConnegativeIndicative`, `ConnegativeImperative`, `ConnegativeOptative`, `ConnegativeConditional`, `ConnegativePotential`, `ParticiplePresent`, `ParticiplePast`

Citation form: `Infinitive, PresentFirstSingular, PastFirstSingular` (comma-separated; parts you give are output exactly and narrow the prediction).

## How it works

Each attested form is stored as an edit tree from the lemma. A lemma's edits across all cells make its inflection class. A suffix tree over the word, read from its end and pruned by a description-length test, predicts the class of any word. Known words the tree gets wrong are kept in a small lexicon. Confidences are smoothed counts; variant weights follow Wiktionary's listing order (1, 1/2, 1/3…, nonstandard forms a tenth of that).

## License

Code: MIT OR Apache-2.0. Data: derived from [Wiktionary](https://www.wiktionary.org/) (via [kaikki.org](https://kaikki.org/)), CC BY-SA 4.0; the model bytes in `src/model.bin` are shared under the same license.
