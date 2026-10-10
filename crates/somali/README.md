# somali

Somali inflection for Rust, learned from Wiktionary by [morphe](https://github.com/gold-silver-copper/morphe).
Inflects any word, known or made up. The `_scored` functions give every candidate with a confidence.

Generated from the Kaikki Somali dump of 2026-10-03. Do not edit by hand: change the config in morphe and regenerate.

## Example

```rust
use somali::*;
println!("{}", noun_plural("beer")); // beero
println!("{}", noun_definite("beer", Number::Singular)); // beerta
println!("{}", noun_possessive("beer", Possessor::FirstSingular)); // beertayda
println!("{:?}", noun_gender("beer"));
println!("{}", verb("door", Tense::Past, Person::FirstPlural)); // doortnay
println!("{}", verb_imperative("door", Number::Plural)); // doorta
```

Nouns have the plural, their gender, the definite (near -ka/-ta and remote
-kii/-tii), the demonstratives, the possessed forms for eight possessors
and, for some, the case forms. Verbs are cited by the imperative singular
and have the present, past and their progressives for seven persons, the
plural imperative and the derived causative, middle and passive stems.
Input matching ignores the tone marks dictionaries write.

## Coverage

| Part of speech | Entries | Cells | Edits | Classes | Tree nodes | Lexicon |
|---|---|---|---|---|---|---|
| noun | 179 | 27 | 236 | 113 | 37 | 113 |
| verb | 315 | 34 | 106 | 41 | 13 | 65 |

Model size: 20938 bytes.

**noun** cells: `Plural`, `AbsolutiveSingular`, `AbsolutivePlural`, `NominativeSingular`, `NominativePlural`, `GenitiveSingular`, `GenitivePlural`, `VocativeSingular`, `VocativePlural`, `DefiniteSingular`, `DefinitePlural`, `RemoteDefiniteSingular`, `RemoteDefinitePlural`, `ProximalSingular`, `ProximalPlural`, `MedialSingular`, `MedialPlural`, `DistalSingular`, `DistalPlural`, `PossessiveFirstSingular`, `PossessiveSecondSingular`, `PossessiveThirdMasculine`, `PossessiveThirdFeminine`, `PossessiveFirstPluralExclusive`, `PossessiveFirstPluralInclusive`, `PossessiveSecondPlural`, `PossessiveThirdPlural`

Citation form: `Lemma, Plural` (comma-separated; parts you give are output exactly and narrow the prediction).

**verb** cells: `Infinitive`, `Causative`, `Middle`, `Passive`, `ImperativeSingular`, `ImperativePlural`, `PresentFirstSingular`, `PresentSecondSingular`, `PresentThirdMasculine`, `PresentThirdFeminine`, `PresentFirstPlural`, `PresentSecondPlural`, `PresentThirdPlural`, `PresentProgressiveFirstSingular`, `PresentProgressiveSecondSingular`, `PresentProgressiveThirdMasculine`, `PresentProgressiveThirdFeminine`, `PresentProgressiveFirstPlural`, `PresentProgressiveSecondPlural`, `PresentProgressiveThirdPlural`, `PastFirstSingular`, `PastSecondSingular`, `PastThirdMasculine`, `PastThirdFeminine`, `PastFirstPlural`, `PastSecondPlural`, `PastThirdPlural`, `PastProgressiveFirstSingular`, `PastProgressiveSecondSingular`, `PastProgressiveThirdMasculine`, `PastProgressiveThirdFeminine`, `PastProgressiveFirstPlural`, `PastProgressiveSecondPlural`, `PastProgressiveThirdPlural`

Citation form: `ImperativeSingular, Infinitive` (comma-separated; parts you give are output exactly and narrow the prediction).

## How it works

Each attested form is stored as an edit tree from the lemma. A lemma's edits across all cells make its inflection class. A suffix tree over the word, read from its end and pruned by a description-length test, predicts the class of any word. Known words the tree gets wrong are kept in a small lexicon. Confidences are smoothed counts; variant weights follow Wiktionary's listing order (1, 1/2, 1/3…, nonstandard forms a tenth of that).

## License

Code: MIT OR Apache-2.0. Data: derived from [Wiktionary](https://www.wiktionary.org/) (via [kaikki.org](https://kaikki.org/)), CC BY-SA 4.0; the model bytes in `src/model.bin` are shared under the same license.
