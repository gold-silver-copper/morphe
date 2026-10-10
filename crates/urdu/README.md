# urdu

Urdu inflection for Rust, learned from Wiktionary by [morphe](https://github.com/gold-silver-copper/morphe).
Inflects any word, known or made up. The `_scored` functions give every candidate with a confidence.

Generated from the Kaikki Urdu dump of 2026-10-03. Do not edit by hand: change the config in morphe and regenerate.

## Example

```rust
use urdu::*;
println!("{}", noun("لڑکا", Case::Oblique, Number::Plural)); // لڑکوں
println!("{:?}", noun_gender("کتاب")); // Feminine
println!("{}", adj("کالا", Case::Direct, Gender::Feminine, Number::Singular)); // کالی
println!("{}", verb_future("پینا", Person::FirstSingular, Gender::Masculine)); // پیوں گا
println!("{}", verb_subjunctive("پینا", Person::SecondFamiliar)); // پیو
println!("{}", verb_participle("پینا", Participle::Habitual, Agreement::FeminineSingular)); // پیتی
println!("{}", verb_imperative("پینا", Address::Familiar)); // پیو
```

Nouns have the direct, oblique and vocative in both numbers; adjectives
agree in gender too. Verbs have the stem, the infinitive and its oblique and
agreeing forms, the conjunctive, the habitual, perfective and agentive
participles, the simple future, the subjunctive and the imperatives. Tenses
built with auxiliaries (پیتا ہوں، پی رہا تھا) are a participle plus a form
of ہونا and are left out. Forms are written without harakat, as in everyday
Urdu, and input matching ignores them.

## Coverage

| Part of speech | Entries | Cells | Edits | Classes | Tree nodes | Lexicon |
|---|---|---|---|---|---|---|
| noun | 1946 | 6 | 138 | 108 | 63 | 563 |
| adj | 98 | 12 | 15 | 7 | 5 | 4 |
| verb | 238 | 48 | 453 | 86 | 12 | 123 |
| pron | 12 | 22 | 34 | 10 | 2 | 4 |
| num | 6 | 6 | 6 | 2 | 2 | 1 |
| det | 16 | 12 | 9 | 5 | 3 | 0 |

Model size: 102821 bytes.

**noun** cells: `DirectSingular`, `DirectPlural`, `ObliqueSingular`, `ObliquePlural`, `VocativeSingular`, `VocativePlural`

Citation form: `DirectSingular, DirectPlural` (comma-separated; parts you give are output exactly and narrow the prediction).

**adj** cells: `DirectMasculineSingular`, `DirectMasculinePlural`, `DirectFeminineSingular`, `DirectFemininePlural`, `ObliqueMasculineSingular`, `ObliqueMasculinePlural`, `ObliqueFeminineSingular`, `ObliqueFemininePlural`, `VocativeMasculineSingular`, `VocativeMasculinePlural`, `VocativeFeminineSingular`, `VocativeFemininePlural`

Citation form: `DirectMasculineSingular, DirectFeminineSingular, DirectMasculinePlural` (comma-separated; parts you give are output exactly and narrow the prediction).

**verb** cells: `Stem`, `Infinitive`, `InfinitiveOblique`, `InfinitiveAgreeingMasculineSingular`, `InfinitiveAgreeingMasculinePlural`, `InfinitiveAgreeingFeminineSingular`, `InfinitiveAgreeingFemininePlural`, `Conjunctive`, `Progressive`, `ParticipleHabitualMasculineSingular`, `ParticipleHabitualMasculinePlural`, `ParticipleHabitualFeminineSingular`, `ParticipleHabitualFemininePlural`, `ParticiplePerfectiveMasculineSingular`, `ParticiplePerfectiveMasculinePlural`, `ParticiplePerfectiveFeminineSingular`, `ParticiplePerfectiveFemininePlural`, `ParticipleAgentiveMasculineSingular`, `ParticipleAgentiveMasculinePlural`, `ParticipleAgentiveFeminineSingular`, `ParticipleAgentiveFemininePlural`, `FutureFirstSingularMasculine`, `FutureFirstSingularFeminine`, `FutureSecondIntimateMasculine`, `FutureSecondIntimateFeminine`, `FutureThirdSingularMasculine`, `FutureThirdSingularFeminine`, `FutureFirstPluralMasculine`, `FutureFirstPluralFeminine`, `FutureSecondFamiliarMasculine`, `FutureSecondFamiliarFeminine`, `FutureSecondFormalMasculine`, `FutureSecondFormalFeminine`, `FutureThirdPluralMasculine`, `FutureThirdPluralFeminine`, `SubjunctiveFirstSingular`, `SubjunctiveSecondIntimate`, `SubjunctiveThirdSingular`, `SubjunctiveFirstPlural`, `SubjunctiveSecondFamiliar`, `SubjunctiveSecondFormal`, `SubjunctiveThirdPlural`, `ImperativeIntimate`, `ImperativeFamiliar`, `ImperativeFormal`, `FutureImperativeIntimate`, `FutureImperativeFamiliar`, `FutureImperativeFormal`

**pron** cells: `DirectSingular`, `DirectPlural`, `IndirectSingular`, `IndirectPlural`, `ErgativeSingular`, `ErgativePlural`, `DativeSingular`, `DativePlural`, `GenitiveSingular`, `GenitivePlural`, `PossessiveDirectMasculineSingular`, `PossessiveDirectMasculinePlural`, `PossessiveDirectFeminineSingular`, `PossessiveDirectFemininePlural`, `PossessiveObliqueMasculineSingular`, `PossessiveObliqueMasculinePlural`, `PossessiveObliqueFeminineSingular`, `PossessiveObliqueFemininePlural`, `PossessiveVocativeMasculineSingular`, `PossessiveVocativeMasculinePlural`, `PossessiveVocativeFeminineSingular`, `PossessiveVocativeFemininePlural`

**num** cells: `DirectSingular`, `DirectPlural`, `ObliqueSingular`, `ObliquePlural`, `VocativeSingular`, `VocativePlural`

**det** cells: `DirectMasculineSingular`, `DirectMasculinePlural`, `DirectFeminineSingular`, `DirectFemininePlural`, `ObliqueMasculineSingular`, `ObliqueMasculinePlural`, `ObliqueFeminineSingular`, `ObliqueFemininePlural`, `VocativeMasculineSingular`, `VocativeMasculinePlural`, `VocativeFeminineSingular`, `VocativeFemininePlural`

## How it works

Each attested form is stored as an edit tree from the lemma. A lemma's edits across all cells make its inflection class. A suffix tree over the word, read from its end and pruned by a description-length test, predicts the class of any word. Known words the tree gets wrong are kept in a small lexicon. Confidences are smoothed counts; variant weights follow Wiktionary's listing order (1, 1/2, 1/3…, nonstandard forms a tenth of that).

## License

Code: MIT OR Apache-2.0. Data: derived from [Wiktionary](https://www.wiktionary.org/) (via [kaikki.org](https://kaikki.org/)), CC BY-SA 4.0; the model bytes in `src/model.bin` are shared under the same license.
