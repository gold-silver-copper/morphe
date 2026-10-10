# hebrew

Hebrew inflection for Rust, learned from Wiktionary by [morphe](https://github.com/gold-silver-copper/morphe).
Inflects any word, known or made up. The `_scored` functions give every candidate with a confidence.

Generated from the Kaikki Hebrew dump of 2026-10-03. Do not edit by hand: change the config in morphe and regenerate.

## Example

```rust
use hebrew::*;
// Unpointed input works; output carries niqqud.
println!("{}", noun("ספר", State::Indefinite, Number::Plural)); // סְפָרִים
println!("{}", noun("ספר", State::Construct, Number::Plural)); // סִפְרֵי
println!("{}", noun_possessed("ספר", Person::FirstSingular)); // סִפְרִי
println!("{:?}", noun_gender("עיר"));
println!("{}", adj("גדול", AdjGender::Feminine, AdjNumber::Plural));
println!("{}", verb("שמר", Tense::Past, Person::FirstSingular)); // שָׁמַרְתִּי
println!("{}", verb_present("שמר", Gender::Feminine, Number::Singular));
let plain = Hebrew::default().marks(false);
println!("{}", plain.verb("שמר", Tense::Past, Person::FirstSingular)); // שמרתי
```

Input matching ignores niqqud and cantillation. Pointed forms follow the
dictionary's defective spelling, so `marks(false)` gives ktiv haser
(שלחנות), not the everyday full spelling (שולחנות).

## Coverage

| Part of speech | Entries | Cells | Edits | Classes | Tree nodes | Lexicon |
|---|---|---|---|---|---|---|
| noun | 4885 | 29 | 16342 | 2722 | 1650 | 4231 |
| adj | 1675 | 4 | 1041 | 528 | 387 | 1519 |
| verb | 2133 | 31 | 12036 | 1662 | 780 | 1625 |

Model size: 703456 bytes.

**noun** cells: `IndefiniteSingular`, `IndefinitePlural`, `IndefiniteDual`, `DefiniteSingular`, `DefinitePlural`, `DefiniteDual`, `ConstructSingular`, `ConstructPlural`, `ConstructDual`, `PossessedFirstSingular`, `PossessedFirstPlural`, `PossessedSecondMasculineSingular`, `PossessedSecondFeminineSingular`, `PossessedSecondMasculinePlural`, `PossessedSecondFemininePlural`, `PossessedThirdMasculineSingular`, `PossessedThirdFeminineSingular`, `PossessedThirdMasculinePlural`, `PossessedThirdFemininePlural`, `PossessedPluralFirstSingular`, `PossessedPluralFirstPlural`, `PossessedPluralSecondMasculineSingular`, `PossessedPluralSecondFeminineSingular`, `PossessedPluralSecondMasculinePlural`, `PossessedPluralSecondFemininePlural`, `PossessedPluralThirdMasculineSingular`, `PossessedPluralThirdFeminineSingular`, `PossessedPluralThirdMasculinePlural`, `PossessedPluralThirdFemininePlural`

Citation form: `IndefiniteSingular, IndefinitePlural` (comma-separated; parts you give are output exactly and narrow the prediction).

**adj** cells: `MasculineSingular`, `MasculinePlural`, `FeminineSingular`, `FemininePlural`

Citation form: `MasculineSingular, FeminineSingular, MasculinePlural, FemininePlural` (comma-separated; parts you give are output exactly and narrow the prediction).

**verb** cells: `PastFirstSingular`, `PastFirstPlural`, `PastSecondMasculineSingular`, `PastSecondFeminineSingular`, `PastSecondMasculinePlural`, `PastSecondFemininePlural`, `PastThirdMasculineSingular`, `PastThirdFeminineSingular`, `PastThirdMasculinePlural`, `PastThirdFemininePlural`, `FutureFirstSingular`, `FutureFirstPlural`, `FutureSecondMasculineSingular`, `FutureSecondFeminineSingular`, `FutureSecondMasculinePlural`, `FutureSecondFemininePlural`, `FutureThirdMasculineSingular`, `FutureThirdFeminineSingular`, `FutureThirdMasculinePlural`, `FutureThirdFemininePlural`, `PresentMasculineSingular`, `PresentMasculinePlural`, `PresentFeminineSingular`, `PresentFemininePlural`, `ImperativeMasculineSingular`, `ImperativeMasculinePlural`, `ImperativeFeminineSingular`, `ImperativeFemininePlural`, `Infinitive`, `VerbalNoun`, `PassiveParticiple`

Citation form: `PastThirdMasculineSingular, FutureThirdMasculineSingular` (comma-separated; parts you give are output exactly and narrow the prediction).

## How it works

Each attested form is stored as an edit tree from the lemma. A lemma's edits across all cells make its inflection class. A suffix tree over the word, read from its end and pruned by a description-length test, predicts the class of any word. Known words the tree gets wrong are kept in a small lexicon. Confidences are smoothed counts; variant weights follow Wiktionary's listing order (1, 1/2, 1/3…, nonstandard forms a tenth of that).

## License

Code: MIT OR Apache-2.0. Data: derived from [Wiktionary](https://www.wiktionary.org/) (via [kaikki.org](https://kaikki.org/)), CC BY-SA 4.0; the model bytes in `src/model.bin` are shared under the same license.
