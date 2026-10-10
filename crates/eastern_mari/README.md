# eastern_mari

EasternMari inflection for Rust, learned from Wiktionary by [morphe](https://github.com/gold-silver-copper/morphe).
Inflects any word, known or made up. The `_scored` functions give every candidate with a confidence.

Generated from the Kaikki EasternMari dump of 2026-10-03. Do not edit by hand: change the config in morphe and regenerate.

## Example

```rust
use eastern_mari::*;
println!("{}", noun("пӧрт", Case::Inessive, Number::Singular)); // пӧртышто
println!("{}", noun("ю", Case::Dative, Number::Plural)); // ю-влаклан
println!("{}", noun_possessive("ю", Person::First, Number::Singular)); // юэм
println!("{}", verb("колаш", Tense::Present, Person::First, Number::Singular)); // колам
println!("{}", verb_negative("колаш", Tense::Past, Person::Third, Number::Singular)); // ыш кол
```

Nouns and adjectives have ten cases (with the long and short illative) in
both numbers and the possessed forms for six possessors. Verbs have the
present, the desiderative, the two pasts and the imperative for six
persons, positive and negative (the negatives are analytic: ом кол), plus
the infinitive, the verbal noun and the participles. Wiktionary has
conjugation tables for a few dozen verbs only.

## Coverage

| Part of speech | Entries | Cells | Edits | Classes | Tree nodes | Lexicon |
|---|---|---|---|---|---|---|
| noun | 456 | 26 | 219 | 60 | 47 | 118 |
| adj | 26 | 26 | 76 | 8 | 7 | 7 |
| verb | 49 | 66 | 702 | 94 | 6 | 34 |

Model size: 52330 bytes.

**noun** cells: `NominativeSingular`, `NominativePlural`, `GenitiveSingular`, `GenitivePlural`, `AccusativeSingular`, `AccusativePlural`, `DativeSingular`, `DativePlural`, `ComitativeSingular`, `ComitativePlural`, `ComparativeSingular`, `ComparativePlural`, `IllativeSingular`, `IllativePlural`, `ShortIllativeSingular`, `ShortIllativePlural`, `InessiveSingular`, `InessivePlural`, `LativeSingular`, `LativePlural`, `PossessiveFirstSingular`, `PossessiveFirstPlural`, `PossessiveSecondSingular`, `PossessiveSecondPlural`, `PossessiveThirdSingular`, `PossessiveThirdPlural`

**adj** cells: `NominativeSingular`, `NominativePlural`, `GenitiveSingular`, `GenitivePlural`, `AccusativeSingular`, `AccusativePlural`, `DativeSingular`, `DativePlural`, `ComitativeSingular`, `ComitativePlural`, `ComparativeSingular`, `ComparativePlural`, `IllativeSingular`, `IllativePlural`, `ShortIllativeSingular`, `ShortIllativePlural`, `InessiveSingular`, `InessivePlural`, `LativeSingular`, `LativePlural`, `PossessiveFirstSingular`, `PossessiveFirstPlural`, `PossessiveSecondSingular`, `PossessiveSecondPlural`, `PossessiveThirdSingular`, `PossessiveThirdPlural`

**verb** cells: `Infinitive`, `VerbalNoun`, `ParticipleActive`, `ParticiplePassive`, `ParticipleFuture`, `ParticipleNegative`, `PresentFirstSingular`, `PresentFirstPlural`, `PresentSecondSingular`, `PresentSecondPlural`, `PresentThirdSingular`, `PresentThirdPlural`, `DesiderativeFirstSingular`, `DesiderativeFirstPlural`, `DesiderativeSecondSingular`, `DesiderativeSecondPlural`, `DesiderativeThirdSingular`, `DesiderativeThirdPlural`, `PastFirstSingular`, `PastFirstPlural`, `PastSecondSingular`, `PastSecondPlural`, `PastThirdSingular`, `PastThirdPlural`, `CompoundPastFirstSingular`, `CompoundPastFirstPlural`, `CompoundPastSecondSingular`, `CompoundPastSecondPlural`, `CompoundPastThirdSingular`, `CompoundPastThirdPlural`, `ImperativeFirstSingular`, `ImperativeFirstPlural`, `ImperativeSecondSingular`, `ImperativeSecondPlural`, `ImperativeThirdSingular`, `ImperativeThirdPlural`, `NegativePresentFirstSingular`, `NegativePresentFirstPlural`, `NegativePresentSecondSingular`, `NegativePresentSecondPlural`, `NegativePresentThirdSingular`, `NegativePresentThirdPlural`, `NegativeDesiderativeFirstSingular`, `NegativeDesiderativeFirstPlural`, `NegativeDesiderativeSecondSingular`, `NegativeDesiderativeSecondPlural`, `NegativeDesiderativeThirdSingular`, `NegativeDesiderativeThirdPlural`, `NegativePastFirstSingular`, `NegativePastFirstPlural`, `NegativePastSecondSingular`, `NegativePastSecondPlural`, `NegativePastThirdSingular`, `NegativePastThirdPlural`, `NegativeCompoundPastFirstSingular`, `NegativeCompoundPastFirstPlural`, `NegativeCompoundPastSecondSingular`, `NegativeCompoundPastSecondPlural`, `NegativeCompoundPastThirdSingular`, `NegativeCompoundPastThirdPlural`, `NegativeImperativeFirstSingular`, `NegativeImperativeFirstPlural`, `NegativeImperativeSecondSingular`, `NegativeImperativeSecondPlural`, `NegativeImperativeThirdSingular`, `NegativeImperativeThirdPlural`

Citation form: `Infinitive, PresentThirdSingular, PastThirdSingular` (comma-separated; parts you give are output exactly and narrow the prediction).

## How it works

Each attested form is stored as an edit tree from the lemma. A lemma's edits across all cells make its inflection class. A suffix tree over the word, read from its end and pruned by a description-length test, predicts the class of any word. Known words the tree gets wrong are kept in a small lexicon. Confidences are smoothed counts; variant weights follow Wiktionary's listing order (1, 1/2, 1/3…, nonstandard forms a tenth of that).

## License

Code: MIT OR Apache-2.0. Data: derived from [Wiktionary](https://www.wiktionary.org/) (via [kaikki.org](https://kaikki.org/)), CC BY-SA 4.0; the model bytes in `src/model.bin` are shared under the same license.
