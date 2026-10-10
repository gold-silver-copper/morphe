# hindi

Hindi inflection for Rust, learned from Wiktionary by [morphe](https://github.com/gold-silver-copper/morphe).
Inflects any word, known or made up. The `_scored` functions give every candidate with a confidence.

Generated from the Kaikki Hindi dump of 2026-10-03. Do not edit by hand: change the config in morphe and regenerate.

## Example

```rust
use hindi::*;
assert_eq!(noun("लड़का", Case::Oblique, Number::Plural), "लड़कों");
println!("{}", noun("किताब", Case::Direct, Number::Plural)); // किताबें
println!("{:?}", noun_gender("किताब")); // Feminine
println!("{}", adj("नीला", Case::Direct, Gender::Feminine, Number::Singular)); // नीली
println!("{}", verb_future("करना", Person::FirstSingular, Gender::Masculine)); // करूँगा
println!("{}", verb_subjunctive("जाना", Person::ThirdPlural)); // जाएँ
println!("{}", verb_participle("करना", Participle::Perfective, Agreement::FeminineSingular)); // की
println!("{}", verb_imperative("करना", Address::Formal)); // कीजिये
println!("{}", pron("मैं", PronCase::Ergative, Number::Singular)); // मैंने
```

Verbs have their one-word forms: the stem, the infinitive and its oblique,
the conjunctive participle, the habitual, perfective and agentive
participles in their five agreement forms, the future, the subjunctive and
the imperatives. Tenses built with auxiliaries (करता हूँ, कर रहा था) are the
participle plus a form of होना and are left out. Input matching treats
chandrabindu as anusvara.

## Coverage

| Part of speech | Entries | Cells | Edits | Classes | Tree nodes | Lexicon |
|---|---|---|---|---|---|---|
| noun | 13207 | 6 | 298 | 165 | 402 | 2621 |
| adj | 532 | 12 | 43 | 25 | 14 | 164 |
| verb | 1656 | 46 | 627 | 93 | 29 | 473 |
| pron | 36 | 28 | 202 | 41 | 3 | 19 |
| num | 8 | 6 | 7 | 5 | 2 | 3 |

Model size: 810187 bytes.

**noun** cells: `DirectSingular`, `DirectPlural`, `ObliqueSingular`, `ObliquePlural`, `VocativeSingular`, `VocativePlural`

Citation form: `DirectSingular, DirectPlural` (comma-separated; parts you give are output exactly and narrow the prediction).

**adj** cells: `DirectMasculineSingular`, `DirectMasculinePlural`, `DirectFeminineSingular`, `DirectFemininePlural`, `ObliqueMasculineSingular`, `ObliqueMasculinePlural`, `ObliqueFeminineSingular`, `ObliqueFemininePlural`, `VocativeMasculineSingular`, `VocativeMasculinePlural`, `VocativeFeminineSingular`, `VocativeFemininePlural`

Citation form: `DirectMasculineSingular, DirectFeminineSingular, DirectMasculinePlural` (comma-separated; parts you give are output exactly and narrow the prediction).

**verb** cells: `Stem`, `Infinitive`, `InfinitiveOblique`, `Conjunctive`, `ParticipleHabitualMasculineSingular`, `ParticipleHabitualMasculineOblique`, `ParticipleHabitualMasculinePlural`, `ParticipleHabitualFeminineSingular`, `ParticipleHabitualFemininePlural`, `ParticiplePerfectiveMasculineSingular`, `ParticiplePerfectiveMasculineOblique`, `ParticiplePerfectiveMasculinePlural`, `ParticiplePerfectiveFeminineSingular`, `ParticiplePerfectiveFemininePlural`, `ParticipleAgentiveMasculineSingular`, `ParticipleAgentiveMasculineOblique`, `ParticipleAgentiveMasculinePlural`, `ParticipleAgentiveFeminineSingular`, `ParticipleAgentiveFemininePlural`, `FutureFirstSingularMasculine`, `FutureFirstSingularFeminine`, `FutureSecondIntimateMasculine`, `FutureSecondIntimateFeminine`, `FutureThirdSingularMasculine`, `FutureThirdSingularFeminine`, `FutureFirstPluralMasculine`, `FutureFirstPluralFeminine`, `FutureSecondFamiliarMasculine`, `FutureSecondFamiliarFeminine`, `FutureSecondFormalMasculine`, `FutureSecondFormalFeminine`, `FutureThirdPluralMasculine`, `FutureThirdPluralFeminine`, `SubjunctiveFirstSingular`, `SubjunctiveSecondIntimate`, `SubjunctiveThirdSingular`, `SubjunctiveFirstPlural`, `SubjunctiveSecondFamiliar`, `SubjunctiveSecondFormal`, `SubjunctiveThirdPlural`, `ImperativeIntimate`, `ImperativeFamiliar`, `ImperativeFormal`, `FutureImperativeIntimate`, `FutureImperativeFamiliar`, `FutureImperativeFormal`

**pron** cells: `DirectSingular`, `DirectPlural`, `IndirectSingular`, `IndirectPlural`, `ErgativeSingular`, `ErgativePlural`, `DativeSingular`, `DativePlural`, `GenitiveSingular`, `GenitivePlural`, `InessiveSingular`, `InessivePlural`, `AdessiveSingular`, `AdessivePlural`, `TerminativeSingular`, `TerminativePlural`, `PossessiveDirectMasculineSingular`, `PossessiveDirectMasculinePlural`, `PossessiveDirectFeminineSingular`, `PossessiveDirectFemininePlural`, `PossessiveObliqueMasculineSingular`, `PossessiveObliqueMasculinePlural`, `PossessiveObliqueFeminineSingular`, `PossessiveObliqueFemininePlural`, `PossessiveVocativeMasculineSingular`, `PossessiveVocativeMasculinePlural`, `PossessiveVocativeFeminineSingular`, `PossessiveVocativeFemininePlural`

**num** cells: `DirectSingular`, `DirectPlural`, `ObliqueSingular`, `ObliquePlural`, `VocativeSingular`, `VocativePlural`

## How it works

Each attested form is stored as an edit tree from the lemma. A lemma's edits across all cells make its inflection class. A suffix tree over the word, read from its end and pruned by a description-length test, predicts the class of any word. Known words the tree gets wrong are kept in a small lexicon. Confidences are smoothed counts; variant weights follow Wiktionary's listing order (1, 1/2, 1/3…, nonstandard forms a tenth of that).

## License

Code: MIT OR Apache-2.0. Data: derived from [Wiktionary](https://www.wiktionary.org/) (via [kaikki.org](https://kaikki.org/)), CC BY-SA 4.0; the model bytes in `src/model.bin` are shared under the same license.
