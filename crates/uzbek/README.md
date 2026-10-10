# uzbek

Uzbek inflection for Rust, learned from Wiktionary by [morphe](https://github.com/gold-silver-copper/morphe).
Inflects any word, known or made up. The `_scored` functions give every candidate with a confidence.

Generated from the Kaikki Uzbek dump of 2026-10-03. Do not edit by hand: change the config in morphe and regenerate.

## Example

```rust
use uzbek::*;
assert_eq!(noun("kitob", Case::Dative, Number::Plural), "kitoblarga");
println!("{}", noun("uy", Case::Locative, Number::Singular)); // uyda
println!("{}", noun("bola", Case::Equative, Number::Singular)); // boladek
println!("{}", noun_possessive("kitob", Possessor::FirstSingular, Case::Accusative, Number::Singular)); // kitobimni
```

Nouns have seven cases, including the equative (-dek, "like"), in both
numbers, and the possessed forms for five possessors (the third person is
the same for one owner and many). Input matching takes any apostrophe for
the ʻ of oʻ and gʻ. Wiktionary's Uzbek verb tables don't label person, so
verbs are left out.

## Coverage

| Part of speech | Entries | Cells | Edits | Classes | Tree nodes | Lexicon |
|---|---|---|---|---|---|---|
| noun | 1414 | 84 | 121 | 7 | 23 | 12 |

Model size: 169791 bytes.

**noun** cells: `NominativeSingular`, `NominativePlural`, `GenitiveSingular`, `GenitivePlural`, `DativeSingular`, `DativePlural`, `AccusativeSingular`, `AccusativePlural`, `LocativeSingular`, `LocativePlural`, `AblativeSingular`, `AblativePlural`, `EquativeSingular`, `EquativePlural`, `PossessiveFirstSingularNominativeSingular`, `PossessiveFirstSingularNominativePlural`, `PossessiveFirstSingularGenitiveSingular`, `PossessiveFirstSingularGenitivePlural`, `PossessiveFirstSingularDativeSingular`, `PossessiveFirstSingularDativePlural`, `PossessiveFirstSingularAccusativeSingular`, `PossessiveFirstSingularAccusativePlural`, `PossessiveFirstSingularLocativeSingular`, `PossessiveFirstSingularLocativePlural`, `PossessiveFirstSingularAblativeSingular`, `PossessiveFirstSingularAblativePlural`, `PossessiveFirstSingularEquativeSingular`, `PossessiveFirstSingularEquativePlural`, `PossessiveSecondSingularNominativeSingular`, `PossessiveSecondSingularNominativePlural`, `PossessiveSecondSingularGenitiveSingular`, `PossessiveSecondSingularGenitivePlural`, `PossessiveSecondSingularDativeSingular`, `PossessiveSecondSingularDativePlural`, `PossessiveSecondSingularAccusativeSingular`, `PossessiveSecondSingularAccusativePlural`, `PossessiveSecondSingularLocativeSingular`, `PossessiveSecondSingularLocativePlural`, `PossessiveSecondSingularAblativeSingular`, `PossessiveSecondSingularAblativePlural`, `PossessiveSecondSingularEquativeSingular`, `PossessiveSecondSingularEquativePlural`, `PossessiveThirdNominativeSingular`, `PossessiveThirdNominativePlural`, `PossessiveThirdGenitiveSingular`, `PossessiveThirdGenitivePlural`, `PossessiveThirdDativeSingular`, `PossessiveThirdDativePlural`, `PossessiveThirdAccusativeSingular`, `PossessiveThirdAccusativePlural`, `PossessiveThirdLocativeSingular`, `PossessiveThirdLocativePlural`, `PossessiveThirdAblativeSingular`, `PossessiveThirdAblativePlural`, `PossessiveThirdEquativeSingular`, `PossessiveThirdEquativePlural`, `PossessiveFirstPluralNominativeSingular`, `PossessiveFirstPluralNominativePlural`, `PossessiveFirstPluralGenitiveSingular`, `PossessiveFirstPluralGenitivePlural`, `PossessiveFirstPluralDativeSingular`, `PossessiveFirstPluralDativePlural`, `PossessiveFirstPluralAccusativeSingular`, `PossessiveFirstPluralAccusativePlural`, `PossessiveFirstPluralLocativeSingular`, `PossessiveFirstPluralLocativePlural`, `PossessiveFirstPluralAblativeSingular`, `PossessiveFirstPluralAblativePlural`, `PossessiveFirstPluralEquativeSingular`, `PossessiveFirstPluralEquativePlural`, `PossessiveSecondPluralNominativeSingular`, `PossessiveSecondPluralNominativePlural`, `PossessiveSecondPluralGenitiveSingular`, `PossessiveSecondPluralGenitivePlural`, `PossessiveSecondPluralDativeSingular`, `PossessiveSecondPluralDativePlural`, `PossessiveSecondPluralAccusativeSingular`, `PossessiveSecondPluralAccusativePlural`, `PossessiveSecondPluralLocativeSingular`, `PossessiveSecondPluralLocativePlural`, `PossessiveSecondPluralAblativeSingular`, `PossessiveSecondPluralAblativePlural`, `PossessiveSecondPluralEquativeSingular`, `PossessiveSecondPluralEquativePlural`

Citation form: `NominativeSingular, NominativePlural` (comma-separated; parts you give are output exactly and narrow the prediction).

## How it works

Each attested form is stored as an edit tree from the lemma. A lemma's edits across all cells make its inflection class. A suffix tree over the word, read from its end and pruned by a description-length test, predicts the class of any word. Known words the tree gets wrong are kept in a small lexicon. Confidences are smoothed counts; variant weights follow Wiktionary's listing order (1, 1/2, 1/3…, nonstandard forms a tenth of that).

## License

Code: MIT OR Apache-2.0. Data: derived from [Wiktionary](https://www.wiktionary.org/) (via [kaikki.org](https://kaikki.org/)), CC BY-SA 4.0; the model bytes in `src/model.bin` are shared under the same license.
