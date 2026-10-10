# latvian

Latvian inflection for Rust, learned from Wiktionary by [morphe](https://github.com/gold-silver-copper/morphe).
Inflects any word, known or made up. The `_scored` functions give every candidate with a confidence.

Generated from the Kaikki Latvian dump of 2026-10-03. Do not edit by hand: change the config in morphe and regenerate.

## Example

```rust
use latvian::*;
assert_eq!(noun("māja", Case::Locative, Number::Plural), "mājās");
println!("{}", noun("galds", Case::Genitive, Number::Singular)); // galda
println!("{:?}", noun_gender("māja"));
println!("{}", adj("labs", Gender::Feminine, Number::Plural, Case::Dative)); // labām
println!("{}", adj_definite("labs", Gender::Masculine, Number::Singular, Case::Nominative)); // labais
println!("{}", verb("lasīt", Tense::Past, PersonNumber::FirstPlural)); // lasījām
println!("{}", verb_debitive("lasīt")); // jālasa
```

The third person is one cell, as Latvian uses the same form for singular and
plural. Periphrastic forms (`lai lasa`, `būtu lasījis`) are left out; the
participles decline as adjectives. Input matching ignores the tone marks
some dictionary spellings carry.

## Coverage

| Part of speech | Entries | Cells | Edits | Classes | Tree nodes | Lexicon |
|---|---|---|---|---|---|---|
| noun | 5542 | 14 | 1165 | 265 | 294 | 966 |
| adj | 2078 | 51 | 338 | 47 | 60 | 128 |
| verb | 912 | 28 | 1348 | 417 | 114 | 468 |
| pron | 30 | 30 | 106 | 21 | 6 | 11 |
| num | 14 | 12 | 37 | 8 | 4 | 4 |

Model size: 497950 bytes.

**noun** cells: `NominativeSingular`, `NominativePlural`, `GenitiveSingular`, `GenitivePlural`, `DativeSingular`, `DativePlural`, `AccusativeSingular`, `AccusativePlural`, `InstrumentalSingular`, `InstrumentalPlural`, `LocativeSingular`, `LocativePlural`, `VocativeSingular`, `VocativePlural`

Citation form: `NominativeSingular, GenitiveSingular` (comma-separated; parts you give are output exactly and narrow the prediction).

**adj** cells: `MasculineSingularNominative`, `MasculineSingularGenitive`, `MasculineSingularDative`, `MasculineSingularAccusative`, `MasculineSingularInstrumental`, `MasculineSingularLocative`, `MasculinePluralNominative`, `MasculinePluralGenitive`, `MasculinePluralDative`, `MasculinePluralAccusative`, `MasculinePluralInstrumental`, `MasculinePluralLocative`, `FeminineSingularNominative`, `FeminineSingularGenitive`, `FeminineSingularDative`, `FeminineSingularAccusative`, `FeminineSingularInstrumental`, `FeminineSingularLocative`, `FemininePluralNominative`, `FemininePluralGenitive`, `FemininePluralDative`, `FemininePluralAccusative`, `FemininePluralInstrumental`, `FemininePluralLocative`, `DefiniteMasculineSingularNominative`, `DefiniteMasculineSingularGenitive`, `DefiniteMasculineSingularDative`, `DefiniteMasculineSingularAccusative`, `DefiniteMasculineSingularInstrumental`, `DefiniteMasculineSingularLocative`, `DefiniteMasculinePluralNominative`, `DefiniteMasculinePluralGenitive`, `DefiniteMasculinePluralDative`, `DefiniteMasculinePluralAccusative`, `DefiniteMasculinePluralInstrumental`, `DefiniteMasculinePluralLocative`, `DefiniteFeminineSingularNominative`, `DefiniteFeminineSingularGenitive`, `DefiniteFeminineSingularDative`, `DefiniteFeminineSingularAccusative`, `DefiniteFeminineSingularInstrumental`, `DefiniteFeminineSingularLocative`, `DefiniteFemininePluralNominative`, `DefiniteFemininePluralGenitive`, `DefiniteFemininePluralDative`, `DefiniteFemininePluralAccusative`, `DefiniteFemininePluralInstrumental`, `DefiniteFemininePluralLocative`, `Comparative`, `Superlative`, `Adverb`

Citation form: `Lemma, Comparative, Superlative` (comma-separated; parts you give are output exactly and narrow the prediction).

**verb** cells: `PresentFirstSingular`, `PresentSecondSingular`, `PresentThird`, `PresentFirstPlural`, `PresentSecondPlural`, `PastFirstSingular`, `PastSecondSingular`, `PastThird`, `PastFirstPlural`, `PastSecondPlural`, `FutureFirstSingular`, `FutureSecondSingular`, `FutureThird`, `FutureFirstPlural`, `FutureSecondPlural`, `ImperativeSecondSingular`, `ImperativeSecondPlural`, `Conditional`, `RenarrativePresent`, `RenarrativeFuture`, `Debitive`, `ParticiplePresentActive`, `ParticiplePastActive`, `ParticiplePresentPassive`, `ParticiplePastPassive`, `Dams`, `Gerund`, `VerbalNoun`

Citation form: `Lemma, PresentFirstSingular, PastFirstSingular` (comma-separated; parts you give are output exactly and narrow the prediction).

**pron** cells: `MasculineSingularNominative`, `MasculineSingularGenitive`, `MasculineSingularDative`, `MasculineSingularAccusative`, `MasculineSingularInstrumental`, `MasculineSingularLocative`, `MasculinePluralNominative`, `MasculinePluralGenitive`, `MasculinePluralDative`, `MasculinePluralAccusative`, `MasculinePluralInstrumental`, `MasculinePluralLocative`, `FeminineSingularNominative`, `FeminineSingularGenitive`, `FeminineSingularDative`, `FeminineSingularAccusative`, `FeminineSingularInstrumental`, `FeminineSingularLocative`, `FemininePluralNominative`, `FemininePluralGenitive`, `FemininePluralDative`, `FemininePluralAccusative`, `FemininePluralInstrumental`, `FemininePluralLocative`, `PersonalNominative`, `PersonalGenitive`, `PersonalDative`, `PersonalAccusative`, `PersonalInstrumental`, `PersonalLocative`

**num** cells: `NominativeMasculine`, `NominativeFeminine`, `GenitiveMasculine`, `GenitiveFeminine`, `DativeMasculine`, `DativeFeminine`, `AccusativeMasculine`, `AccusativeFeminine`, `InstrumentalMasculine`, `InstrumentalFeminine`, `LocativeMasculine`, `LocativeFeminine`

## How it works

Each attested form is stored as an edit tree from the lemma. A lemma's edits across all cells make its inflection class. A suffix tree over the word, read from its end and pruned by a description-length test, predicts the class of any word. Known words the tree gets wrong are kept in a small lexicon. Confidences are smoothed counts; variant weights follow Wiktionary's listing order (1, 1/2, 1/3…, nonstandard forms a tenth of that).

## License

Code: MIT OR Apache-2.0. Data: derived from [Wiktionary](https://www.wiktionary.org/) (via [kaikki.org](https://kaikki.org/)), CC BY-SA 4.0; the model bytes in `src/model.bin` are shared under the same license.
