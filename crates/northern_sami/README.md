# northern_sami

NorthernSami inflection for Rust, learned from Wiktionary by [morphe](https://github.com/gold-silver-copper/morphe).
Inflects any word, known or made up. The `_scored` functions give every candidate with a confidence.

Generated from the Kaikki NorthernSami dump of 2026-10-03. Do not edit by hand: change the config in morphe and regenerate.

## Example

```rust
use northern_sami::*;
println!("{}", noun("giella", Case::Genitive, Number::Singular)); // giela
println!("{}", noun("viessu", Case::Locative, Number::Plural)); // viesuin
println!("{}", noun_possessive("viessu", Person::First, Number::Singular)); // viessun
println!("{}", verb("boahtit", Mood::Past, Person::Third, Number::Singular)); // bođii
println!("{}", verb("boahtit", Mood::Present, Person::First, Number::Dual)); // boahte
println!("{}", verb_connegative("boahtit", Mood::Present)); // boađe
```

Nouns have six cases in the singular and plural, the essive, and the
possessed forms for nine possessors (singular, dual and plural). Verbs have
the indicative present and past, the imperative, the conditional and the
potential for nine persons, the connegatives, the infinitive, the action
forms and the participles. Wiktionary's tables mark vowel length and
quality (ō, ẹ, ạ) the standard spelling leaves out: input matching ignores
them, and `marks(false)` removes them from output.

## Coverage

| Part of speech | Entries | Cells | Edits | Classes | Tree nodes | Lexicon |
|---|---|---|---|---|---|---|
| noun | 2065 | 22 | 3866 | 1012 | 479 | 865 |
| adj | 481 | 22 | 842 | 118 | 81 | 147 |
| verb | 1133 | 59 | 8890 | 748 | 177 | 334 |
| pron | 20 | 22 | 66 | 26 | 4 | 11 |

Model size: 855294 bytes.

**noun** cells: `NominativeSingular`, `NominativePlural`, `GenitiveSingular`, `GenitivePlural`, `AccusativeSingular`, `AccusativePlural`, `IllativeSingular`, `IllativePlural`, `LocativeSingular`, `LocativePlural`, `ComitativeSingular`, `ComitativePlural`, `Essive`, `PossessiveFirstSingular`, `PossessiveFirstDual`, `PossessiveFirstPlural`, `PossessiveSecondSingular`, `PossessiveSecondDual`, `PossessiveSecondPlural`, `PossessiveThirdSingular`, `PossessiveThirdDual`, `PossessiveThirdPlural`

Citation form: `NominativeSingular, GenitiveSingular` (comma-separated; parts you give are output exactly and narrow the prediction).

**adj** cells: `NominativeSingular`, `NominativePlural`, `GenitiveSingular`, `GenitivePlural`, `AccusativeSingular`, `AccusativePlural`, `IllativeSingular`, `IllativePlural`, `LocativeSingular`, `LocativePlural`, `ComitativeSingular`, `ComitativePlural`, `Essive`, `PossessiveFirstSingular`, `PossessiveFirstDual`, `PossessiveFirstPlural`, `PossessiveSecondSingular`, `PossessiveSecondDual`, `PossessiveSecondPlural`, `PossessiveThirdSingular`, `PossessiveThirdDual`, `PossessiveThirdPlural`

**verb** cells: `Infinitive`, `VerbalNoun`, `ActionInessive`, `ActionElative`, `ActionComitative`, `ParticiplePresent`, `ParticiplePast`, `AgentParticiple`, `Abessive`, `PresentFirstSingular`, `PresentFirstDual`, `PresentFirstPlural`, `PresentSecondSingular`, `PresentSecondDual`, `PresentSecondPlural`, `PresentThirdSingular`, `PresentThirdDual`, `PresentThirdPlural`, `PastFirstSingular`, `PastFirstDual`, `PastFirstPlural`, `PastSecondSingular`, `PastSecondDual`, `PastSecondPlural`, `PastThirdSingular`, `PastThirdDual`, `PastThirdPlural`, `ImperativeFirstSingular`, `ImperativeFirstDual`, `ImperativeFirstPlural`, `ImperativeSecondSingular`, `ImperativeSecondDual`, `ImperativeSecondPlural`, `ImperativeThirdSingular`, `ImperativeThirdDual`, `ImperativeThirdPlural`, `ConditionalFirstSingular`, `ConditionalFirstDual`, `ConditionalFirstPlural`, `ConditionalSecondSingular`, `ConditionalSecondDual`, `ConditionalSecondPlural`, `ConditionalThirdSingular`, `ConditionalThirdDual`, `ConditionalThirdPlural`, `PotentialFirstSingular`, `PotentialFirstDual`, `PotentialFirstPlural`, `PotentialSecondSingular`, `PotentialSecondDual`, `PotentialSecondPlural`, `PotentialThirdSingular`, `PotentialThirdDual`, `PotentialThirdPlural`, `ConnegativePresent`, `ConnegativePast`, `ConnegativeImperative`, `ConnegativeConditional`, `ConnegativePotential`

Citation form: `Infinitive, PresentFirstSingular, PastFirstSingular` (comma-separated; parts you give are output exactly and narrow the prediction).

**pron** cells: `NominativeSingular`, `NominativePlural`, `GenitiveSingular`, `GenitivePlural`, `AccusativeSingular`, `AccusativePlural`, `IllativeSingular`, `IllativePlural`, `LocativeSingular`, `LocativePlural`, `ComitativeSingular`, `ComitativePlural`, `Essive`, `PossessiveFirstSingular`, `PossessiveFirstDual`, `PossessiveFirstPlural`, `PossessiveSecondSingular`, `PossessiveSecondDual`, `PossessiveSecondPlural`, `PossessiveThirdSingular`, `PossessiveThirdDual`, `PossessiveThirdPlural`

## How it works

Each attested form is stored as an edit tree from the lemma. A lemma's edits across all cells make its inflection class. A suffix tree over the word, read from its end and pruned by a description-length test, predicts the class of any word. Known words the tree gets wrong are kept in a small lexicon. Confidences are smoothed counts; variant weights follow Wiktionary's listing order (1, 1/2, 1/3…, nonstandard forms a tenth of that).

## License

Code: MIT OR Apache-2.0. Data: derived from [Wiktionary](https://www.wiktionary.org/) (via [kaikki.org](https://kaikki.org/)), CC BY-SA 4.0; the model bytes in `src/model.bin` are shared under the same license.
