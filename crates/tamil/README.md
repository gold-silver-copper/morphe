# tamil

Tamil inflection for Rust, learned from Wiktionary by [morphe](https://github.com/gold-silver-copper/morphe).
Inflects any word, known or made up. The `_scored` functions give every candidate with a confidence.

Generated from the Kaikki Tamil dump of 2026-10-03. Do not edit by hand: change the config in morphe and regenerate.

## Example

```rust
use tamil::*;
assert_eq!(noun("வீடு", Case::Dative, Number::Singular), "வீட்டுக்கு");
println!("{}", noun("மரம்", Case::Locative, Number::Plural)); // மரங்களில்
println!("{}", verb("செய்", Tense::Past, Person::FirstSingular)); // செய்தேன்
println!("{}", verb("படி", Tense::Future, Person::ThirdPlural)); // படிப்பார்கள்
println!("{}", verb_infinitive("செய்")); // செய்ய
println!("{}", verb_adverbial_participle("போ")); // போய்
println!("{}", verb_participial_noun("செய்", Participle::Past, Referent::Masculine)); // செய்தவன்
```

Nouns have ten cases in both numbers; the genitive (-உடைய, -இன்), the
locative (-இல், -இடம்) and the sociative (-ஓடு, -உடன்) have two forms each.
Verbs are cited by the imperative singular, as in Wiktionary. They have the
present, past and future for all ten persons, the negative future, the
imperatives, the infinitive, the potential, cohortative, conditional and
causal forms, the adverbial and adjectival participles, the participial
nouns and the verbal noun.

## Coverage

| Part of speech | Entries | Cells | Edits | Classes | Tree nodes | Lexicon |
|---|---|---|---|---|---|---|
| noun | 6332 | 20 | 491 | 147 | 220 | 887 |
| verb | 1627 | 82 | 1489 | 151 | 76 | 360 |
| pron | 51 | 20 | 246 | 52 | 11 | 25 |
| num | 68 | 20 | 85 | 16 | 10 | 9 |

Model size: 707470 bytes.

**noun** cells: `NominativeSingular`, `NominativePlural`, `AccusativeSingular`, `AccusativePlural`, `DativeSingular`, `DativePlural`, `BenefactiveSingular`, `BenefactivePlural`, `GenitiveSingular`, `GenitivePlural`, `LocativeSingular`, `LocativePlural`, `SociativeSingular`, `SociativePlural`, `InstrumentalSingular`, `InstrumentalPlural`, `AblativeSingular`, `AblativePlural`, `VocativeSingular`, `VocativePlural`

Citation form: `NominativeSingular, AccusativeSingular` (comma-separated; parts you give are output exactly and narrow the prediction).

**verb** cells: `PresentFirstSingular`, `PresentSecondSingular`, `PresentThirdMasculine`, `PresentThirdFeminine`, `PresentThirdHonorific`, `PresentThirdNeuter`, `PresentFirstPlural`, `PresentSecondPlural`, `PresentThirdPlural`, `PresentThirdNeuterPlural`, `PastFirstSingular`, `PastSecondSingular`, `PastThirdMasculine`, `PastThirdFeminine`, `PastThirdHonorific`, `PastThirdNeuter`, `PastFirstPlural`, `PastSecondPlural`, `PastThirdPlural`, `PastThirdNeuterPlural`, `FutureFirstSingular`, `FutureSecondSingular`, `FutureThirdMasculine`, `FutureThirdFeminine`, `FutureThirdHonorific`, `FutureThirdNeuter`, `FutureFirstPlural`, `FutureSecondPlural`, `FutureThirdPlural`, `FutureThirdNeuterPlural`, `FutureNegativeFirstSingular`, `FutureNegativeSecondSingular`, `FutureNegativeThirdMasculine`, `FutureNegativeThirdFeminine`, `FutureNegativeThirdHonorific`, `FutureNegativeThirdNeuter`, `FutureNegativeFirstPlural`, `FutureNegativeSecondPlural`, `FutureNegativeThirdPlural`, `FutureNegativeThirdNeuterPlural`, `ImperativeSingular`, `ImperativePlural`, `NegativeImperativeSingular`, `NegativeImperativePlural`, `Infinitive`, `Potential`, `Cohortative`, `Conditional`, `Causal`, `AdverbialParticiple`, `NegativeAdverbialParticiple`, `AdjectivalParticiplePresent`, `AdjectivalParticiplePast`, `AdjectivalParticipleFuture`, `AdjectivalParticipleNegative`, `ParticipialNounPresentMasculine`, `ParticipialNounPresentFeminine`, `ParticipialNounPresentHonorific`, `ParticipialNounPresentNeuter`, `ParticipialNounPresentPlural`, `ParticipialNounPresentNeuterPlural`, `ParticipialNounPastMasculine`, `ParticipialNounPastFeminine`, `ParticipialNounPastHonorific`, `ParticipialNounPastNeuter`, `ParticipialNounPastPlural`, `ParticipialNounPastNeuterPlural`, `ParticipialNounFutureMasculine`, `ParticipialNounFutureFeminine`, `ParticipialNounFutureHonorific`, `ParticipialNounFutureNeuter`, `ParticipialNounFuturePlural`, `ParticipialNounFutureNeuterPlural`, `ParticipialNounNegativeMasculine`, `ParticipialNounNegativeFeminine`, `ParticipialNounNegativeHonorific`, `ParticipialNounNegativeNeuter`, `ParticipialNounNegativePlural`, `ParticipialNounNegativeNeuterPlural`, `VerbalNoun`, `Progressive`, `Passive`

Citation form: `ImperativeSingular, Infinitive, AdverbialParticiple` (comma-separated; parts you give are output exactly and narrow the prediction).

**pron** cells: `NominativeSingular`, `NominativePlural`, `AccusativeSingular`, `AccusativePlural`, `DativeSingular`, `DativePlural`, `BenefactiveSingular`, `BenefactivePlural`, `GenitiveSingular`, `GenitivePlural`, `LocativeSingular`, `LocativePlural`, `SociativeSingular`, `SociativePlural`, `InstrumentalSingular`, `InstrumentalPlural`, `AblativeSingular`, `AblativePlural`, `VocativeSingular`, `VocativePlural`

**num** cells: `NominativeSingular`, `NominativePlural`, `AccusativeSingular`, `AccusativePlural`, `DativeSingular`, `DativePlural`, `BenefactiveSingular`, `BenefactivePlural`, `GenitiveSingular`, `GenitivePlural`, `LocativeSingular`, `LocativePlural`, `SociativeSingular`, `SociativePlural`, `InstrumentalSingular`, `InstrumentalPlural`, `AblativeSingular`, `AblativePlural`, `VocativeSingular`, `VocativePlural`

## How it works

Each attested form is stored as an edit tree from the lemma. A lemma's edits across all cells make its inflection class. A suffix tree over the word, read from its end and pruned by a description-length test, predicts the class of any word. Known words the tree gets wrong are kept in a small lexicon. Confidences are smoothed counts; variant weights follow Wiktionary's listing order (1, 1/2, 1/3…, nonstandard forms a tenth of that).

## License

Code: MIT OR Apache-2.0. Data: derived from [Wiktionary](https://www.wiktionary.org/) (via [kaikki.org](https://kaikki.org/)), CC BY-SA 4.0; the model bytes in `src/model.bin` are shared under the same license.
