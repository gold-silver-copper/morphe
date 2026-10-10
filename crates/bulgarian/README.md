# bulgarian

Bulgarian inflection for Rust, learned from Wiktionary by [morphe](https://github.com/gold-silver-copper/morphe).
Inflects any word, known or made up. The `_scored` functions give every candidate with a confidence.

Generated from the Kaikki Bulgarian dump of 2026-10-03. Do not edit by hand: change the config in morphe and regenerate.

## Example

```rust
use bulgarian::*;
println!("{}", noun("град", Article::Definite, Number::Singular));
println!("{}", noun("град", Article::DefiniteShort, Number::Singular));
println!("{}", noun("жена", Article::Indefinite, Number::Plural));
println!("{:?}", noun_gender("нощ"));
println!("{}", verb_present("чета", Person::Second, Number::Singular));
println!("{}", verb_aorist("чета", Person::First, Number::Singular));
println!("{}", adj("хубав", Article::Definite, GenderNumber::Feminine));
// Citations pin principal parts: singular and plural.
println!("{}", noun("крак, крака", Article::Definite, Number::Plural));
```

Verbs are cited, as in Bulgarian dictionaries, by the first person singular
present. Input matching ignores stress marks; output carries them where
Wiktionary has them (`marks(false)` removes them).

## Coverage

| Part of speech | Entries | Cells | Edits | Classes | Tree nodes | Lexicon |
|---|---|---|---|---|---|---|
| noun | 9401 | 9 | 1840 | 1017 | 1482 | 4831 |
| adj | 2354 | 36 | 1088 | 100 | 307 | 1072 |
| verb | 2875 | 67 | 5990 | 670 | 329 | 1432 |
| pron | 14 | 37 | 64 | 19 | 5 | 9 |
| det | 16 | 36 | 55 | 15 | 2 | 13 |
| num | 26 | 13 | 30 | 18 | 17 | 21 |

Model size: 1550257 bytes.

**noun** cells: `IndefiniteSingular`, `IndefinitePlural`, `DefiniteSingular`, `DefinitePlural`, `DefiniteShortSingular`, `DefiniteShortPlural`, `Count`, `VocativeSingular`, `VocativePlural`

Citation form: `IndefiniteSingular, IndefinitePlural` (comma-separated; parts you give are output exactly and narrow the prediction).

**adj** cells: `IndefiniteMasculine`, `IndefiniteFeminine`, `IndefiniteNeuter`, `IndefinitePlural`, `DefiniteMasculine`, `DefiniteFeminine`, `DefiniteNeuter`, `DefinitePlural`, `DefiniteShortMasculine`, `DefiniteShortFeminine`, `DefiniteShortNeuter`, `DefiniteShortPlural`, `ComparativeIndefiniteMasculine`, `ComparativeIndefiniteFeminine`, `ComparativeIndefiniteNeuter`, `ComparativeIndefinitePlural`, `ComparativeDefiniteMasculine`, `ComparativeDefiniteFeminine`, `ComparativeDefiniteNeuter`, `ComparativeDefinitePlural`, `ComparativeDefiniteShortMasculine`, `ComparativeDefiniteShortFeminine`, `ComparativeDefiniteShortNeuter`, `ComparativeDefiniteShortPlural`, `SuperlativeIndefiniteMasculine`, `SuperlativeIndefiniteFeminine`, `SuperlativeIndefiniteNeuter`, `SuperlativeIndefinitePlural`, `SuperlativeDefiniteMasculine`, `SuperlativeDefiniteFeminine`, `SuperlativeDefiniteNeuter`, `SuperlativeDefinitePlural`, `SuperlativeDefiniteShortMasculine`, `SuperlativeDefiniteShortFeminine`, `SuperlativeDefiniteShortNeuter`, `SuperlativeDefiniteShortPlural`

**verb** cells: `PresentFirstSingular`, `PresentFirstPlural`, `PresentSecondSingular`, `PresentSecondPlural`, `PresentThirdSingular`, `PresentThirdPlural`, `ImperfectFirstSingular`, `ImperfectFirstPlural`, `ImperfectSecondSingular`, `ImperfectSecondPlural`, `ImperfectThirdSingular`, `ImperfectThirdPlural`, `AoristFirstSingular`, `AoristFirstPlural`, `AoristSecondSingular`, `AoristSecondPlural`, `AoristThirdSingular`, `AoristThirdPlural`, `ImperativeSingular`, `ImperativePlural`, `AoristParticipleIndefiniteMasculine`, `AoristParticipleIndefiniteFeminine`, `AoristParticipleIndefiniteNeuter`, `AoristParticipleIndefinitePlural`, `AoristParticipleDefiniteMasculine`, `AoristParticipleDefiniteFeminine`, `AoristParticipleDefiniteNeuter`, `AoristParticipleDefinitePlural`, `AoristParticipleDefiniteShortMasculine`, `AoristParticipleDefiniteShortFeminine`, `AoristParticipleDefiniteShortNeuter`, `AoristParticipleDefiniteShortPlural`, `ImperfectParticipleMasculine`, `ImperfectParticipleFeminine`, `ImperfectParticipleNeuter`, `ImperfectParticiplePlural`, `PresentParticipleIndefiniteMasculine`, `PresentParticipleIndefiniteFeminine`, `PresentParticipleIndefiniteNeuter`, `PresentParticipleIndefinitePlural`, `PresentParticipleDefiniteMasculine`, `PresentParticipleDefiniteFeminine`, `PresentParticipleDefiniteNeuter`, `PresentParticipleDefinitePlural`, `PresentParticipleDefiniteShortMasculine`, `PresentParticipleDefiniteShortFeminine`, `PresentParticipleDefiniteShortNeuter`, `PresentParticipleDefiniteShortPlural`, `PassiveParticipleIndefiniteMasculine`, `PassiveParticipleIndefiniteFeminine`, `PassiveParticipleIndefiniteNeuter`, `PassiveParticipleIndefinitePlural`, `PassiveParticipleDefiniteMasculine`, `PassiveParticipleDefiniteFeminine`, `PassiveParticipleDefiniteNeuter`, `PassiveParticipleDefinitePlural`, `PassiveParticipleDefiniteShortMasculine`, `PassiveParticipleDefiniteShortFeminine`, `PassiveParticipleDefiniteShortNeuter`, `PassiveParticipleDefiniteShortPlural`, `AdverbialParticiple`, `VerbalNounIndefiniteSingular`, `VerbalNounIndefinitePlural`, `VerbalNounDefiniteSingular`, `VerbalNounDefinitePlural`, `VerbalNounDefiniteShortSingular`, `VerbalNounDefiniteShortPlural`

Citation form: `PresentFirstSingular, AoristFirstSingular` (comma-separated; parts you give are output exactly and narrow the prediction).

**pron** cells: `NominativeIndefiniteMasculine`, `NominativeIndefiniteFeminine`, `NominativeIndefiniteNeuter`, `NominativeIndefinitePlural`, `NominativeDefiniteMasculine`, `NominativeDefiniteFeminine`, `NominativeDefiniteNeuter`, `NominativeDefinitePlural`, `NominativeDefiniteShortMasculine`, `NominativeDefiniteShortFeminine`, `NominativeDefiniteShortNeuter`, `NominativeDefiniteShortPlural`, `AccusativeIndefiniteMasculine`, `AccusativeIndefiniteFeminine`, `AccusativeIndefiniteNeuter`, `AccusativeIndefinitePlural`, `AccusativeDefiniteMasculine`, `AccusativeDefiniteFeminine`, `AccusativeDefiniteNeuter`, `AccusativeDefinitePlural`, `AccusativeDefiniteShortMasculine`, `AccusativeDefiniteShortFeminine`, `AccusativeDefiniteShortNeuter`, `AccusativeDefiniteShortPlural`, `DativeIndefiniteMasculine`, `DativeIndefiniteFeminine`, `DativeIndefiniteNeuter`, `DativeIndefinitePlural`, `DativeDefiniteMasculine`, `DativeDefiniteFeminine`, `DativeDefiniteNeuter`, `DativeDefinitePlural`, `DativeDefiniteShortMasculine`, `DativeDefiniteShortFeminine`, `DativeDefiniteShortNeuter`, `DativeDefiniteShortPlural`, `Short`

**det** cells: `NominativeIndefiniteMasculine`, `NominativeIndefiniteFeminine`, `NominativeIndefiniteNeuter`, `NominativeIndefinitePlural`, `NominativeDefiniteMasculine`, `NominativeDefiniteFeminine`, `NominativeDefiniteNeuter`, `NominativeDefinitePlural`, `NominativeDefiniteShortMasculine`, `NominativeDefiniteShortFeminine`, `NominativeDefiniteShortNeuter`, `NominativeDefiniteShortPlural`, `AccusativeIndefiniteMasculine`, `AccusativeIndefiniteFeminine`, `AccusativeIndefiniteNeuter`, `AccusativeIndefinitePlural`, `AccusativeDefiniteMasculine`, `AccusativeDefiniteFeminine`, `AccusativeDefiniteNeuter`, `AccusativeDefinitePlural`, `AccusativeDefiniteShortMasculine`, `AccusativeDefiniteShortFeminine`, `AccusativeDefiniteShortNeuter`, `AccusativeDefiniteShortPlural`, `DativeIndefiniteMasculine`, `DativeIndefiniteFeminine`, `DativeIndefiniteNeuter`, `DativeIndefinitePlural`, `DativeDefiniteMasculine`, `DativeDefiniteFeminine`, `DativeDefiniteNeuter`, `DativeDefinitePlural`, `DativeDefiniteShortMasculine`, `DativeDefiniteShortFeminine`, `DativeDefiniteShortNeuter`, `DativeDefiniteShortPlural`

**num** cells: `IndefiniteMasculine`, `IndefiniteFeminine`, `IndefiniteNeuter`, `IndefinitePlural`, `DefiniteMasculine`, `DefiniteFeminine`, `DefiniteNeuter`, `DefinitePlural`, `DefiniteShortMasculine`, `DefiniteShortFeminine`, `DefiniteShortNeuter`, `DefiniteShortPlural`, `Count`

## How it works

Each attested form is stored as an edit tree from the lemma. A lemma's edits across all cells make its inflection class. A suffix tree over the word, read from its end and pruned by a description-length test, predicts the class of any word. Known words the tree gets wrong are kept in a small lexicon. Confidences are smoothed counts; variant weights follow Wiktionary's listing order (1, 1/2, 1/3…, nonstandard forms a tenth of that).

## License

Code: MIT OR Apache-2.0. Data: derived from [Wiktionary](https://www.wiktionary.org/) (via [kaikki.org](https://kaikki.org/)), CC BY-SA 4.0; the model bytes in `src/model.bin` are shared under the same license.
