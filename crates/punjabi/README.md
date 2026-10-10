# punjabi

Punjabi inflection for Rust, learned from Wiktionary by [morphe](https://github.com/gold-silver-copper/morphe).
Inflects any word, known or made up. The `_scored` functions give every candidate with a confidence.

Generated from the Kaikki Punjabi dump of 2026-10-03. Do not edit by hand: change the config in morphe and regenerate.

## Example

```rust
use punjabi::*;
println!("{}", noun("ਮੁੰਡਾ", Case::Oblique, Number::Plural)); // ਮੁੰਡਿਆਂ
println!("{:?}", noun_gender("ਕੁੜੀ")); // Feminine
println!("{}", adj("ਸੋਹਣਾ", AdjCase::Direct, Gender::Feminine, Number::Singular)); // ਸੋਹਣੀ
println!("{}", verb_future("ਜਾਣਨਾ", Person::FirstSingular, Gender::Masculine)); // ਜਾਣਾਂਗਾ
println!("{}", verb_subjunctive("ਜਾਣਨਾ", Person::ThirdPlural)); // ਜਾਣਨ
println!("{}", verb_participle("ਜਾਣਨਾ", Participle::Perfective, Agreement::FeminineSingular)); // ਜਾਣੀ
println!("{}", verb_future("سوچنا", Person::ThirdSingular, Gender::Masculine)); // سوچے گا
```

Words are written in Gurmukhi or Shahmukhi, as Wiktionary has them; each
script has its own entries. Nouns have six cases in both numbers and their
gender. Verbs have the stem, the infinitive (direct, oblique, ablative),
the conjunctive, the imperfective, perfective and agentive participles in
five agreement forms, the future, the synthetic -ਸ- future, the
subjunctive, the imperative and the jussive. Tenses built with auxiliaries
are left out. Shahmukhi input ignores harakat, and ن, ل stand for ݨ, ࣇ.

## Coverage

| Part of speech | Entries | Cells | Edits | Classes | Tree nodes | Lexicon |
|---|---|---|---|---|---|---|
| noun | 1910 | 12 | 319 | 84 | 70 | 600 |
| adj | 282 | 12 | 97 | 16 | 8 | 32 |
| verb | 470 | 49 | 486 | 171 | 49 | 122 |
| pron | 24 | 12 | 29 | 10 | 5 | 6 |

Model size: 83241 bytes.

**noun** cells: `DirectSingular`, `DirectPlural`, `ObliqueSingular`, `ObliquePlural`, `VocativeSingular`, `VocativePlural`, `AblativeSingular`, `AblativePlural`, `InstrumentalSingular`, `InstrumentalPlural`, `LocativeSingular`, `LocativePlural`

Citation form: `DirectSingular, DirectPlural` (comma-separated; parts you give are output exactly and narrow the prediction).

**adj** cells: `DirectMasculineSingular`, `DirectMasculinePlural`, `DirectFeminineSingular`, `DirectFemininePlural`, `ObliqueMasculineSingular`, `ObliqueMasculinePlural`, `ObliqueFeminineSingular`, `ObliqueFemininePlural`, `VocativeMasculineSingular`, `VocativeMasculinePlural`, `VocativeFeminineSingular`, `VocativeFemininePlural`

Citation form: `DirectMasculineSingular, DirectFeminineSingular` (comma-separated; parts you give are output exactly and narrow the prediction).

**verb** cells: `Stem`, `InfinitiveDirect`, `InfinitiveOblique`, `InfinitiveAblative`, `Conjunctive`, `Progressive`, `ParticipleImperfectiveMasculineSingular`, `ParticipleImperfectiveMasculineOblique`, `ParticipleImperfectiveMasculinePlural`, `ParticipleImperfectiveFeminineSingular`, `ParticipleImperfectiveFemininePlural`, `ParticiplePerfectiveMasculineSingular`, `ParticiplePerfectiveMasculineOblique`, `ParticiplePerfectiveMasculinePlural`, `ParticiplePerfectiveFeminineSingular`, `ParticiplePerfectiveFemininePlural`, `ParticipleAgentiveMasculineSingular`, `ParticipleAgentiveMasculineOblique`, `ParticipleAgentiveMasculinePlural`, `ParticipleAgentiveFeminineSingular`, `ParticipleAgentiveFemininePlural`, `FutureFirstSingularMasculine`, `FutureFirstSingularFeminine`, `FutureSecondSingularMasculine`, `FutureSecondSingularFeminine`, `FutureThirdSingularMasculine`, `FutureThirdSingularFeminine`, `FutureFirstPluralMasculine`, `FutureFirstPluralFeminine`, `FutureSecondPluralMasculine`, `FutureSecondPluralFeminine`, `FutureThirdPluralMasculine`, `FutureThirdPluralFeminine`, `SyntheticFutureFirstSingular`, `SyntheticFutureSecondSingular`, `SyntheticFutureThirdSingular`, `SyntheticFutureFirstPlural`, `SyntheticFutureSecondPlural`, `SyntheticFutureThirdPlural`, `SubjunctiveFirstSingular`, `SubjunctiveSecondSingular`, `SubjunctiveThirdSingular`, `SubjunctiveFirstPlural`, `SubjunctiveSecondPlural`, `SubjunctiveThirdPlural`, `ImperativeSingular`, `ImperativePlural`, `JussiveSingular`, `JussivePlural`

**pron** cells: `DirectMasculineSingular`, `DirectMasculinePlural`, `DirectFeminineSingular`, `DirectFemininePlural`, `ObliqueMasculineSingular`, `ObliqueMasculinePlural`, `ObliqueFeminineSingular`, `ObliqueFemininePlural`, `VocativeMasculineSingular`, `VocativeMasculinePlural`, `VocativeFeminineSingular`, `VocativeFemininePlural`

## How it works

Each attested form is stored as an edit tree from the lemma. A lemma's edits across all cells make its inflection class. A suffix tree over the word, read from its end and pruned by a description-length test, predicts the class of any word. Known words the tree gets wrong are kept in a small lexicon. Confidences are smoothed counts; variant weights follow Wiktionary's listing order (1, 1/2, 1/3…, nonstandard forms a tenth of that).

## License

Code: MIT OR Apache-2.0. Data: derived from [Wiktionary](https://www.wiktionary.org/) (via [kaikki.org](https://kaikki.org/)), CC BY-SA 4.0; the model bytes in `src/model.bin` are shared under the same license.
