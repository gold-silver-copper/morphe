# hittite

Hittite inflection for Rust, learned from Wiktionary by [morphe](https://github.com/gold-silver-copper/morphe).
Inflects any word, known or made up. The `_scored` functions give every candidate with a confidence.

Generated from the Kaikki Hittite dump of 2026-10-03. Do not edit by hand: change the config in morphe and regenerate.

## Example

```rust
use hittite::*;
println!("{}", noun("attaš", Case::Accusative, Number::Singular)); // attan
println!("{}", noun("wātar", Case::Nominative, Number::Plural)); // witār
println!("{:?}", noun_gender("wātar")); // Neuter
println!("{}", verb("pešzi", Voice::Active, Tense::Preterite, Person::First, Number::Singular)); // peššun
```

Words are written in transliteration and looked up by their citation
form: the nominative singular for nouns, the third person present for
verbs. Nouns have eight cases in both numbers and their gender; a
neuter's absolutive fills its nominative and accusative. Verbs have the
present, the preterite and the imperative, active and mediopassive, for
six persons, the
infinitive, the verbal noun, the participle and the supine. Wiktionary's
tables give normalized endings. Input matching ignores vowel length and
reads s and h as š and ḫ.

## Coverage

| Part of speech | Entries | Cells | Edits | Classes | Tree nodes | Lexicon |
|---|---|---|---|---|---|---|
| noun | 100 | 16 | 243 | 75 | 27 | 94 |
| verb | 96 | 40 | 1617 | 106 | 31 | 73 |

Model size: 72268 bytes.

**noun** cells: `NominativeSingular`, `NominativePlural`, `VocativeSingular`, `VocativePlural`, `AccusativeSingular`, `AccusativePlural`, `GenitiveSingular`, `GenitivePlural`, `DativeLocativeSingular`, `DativeLocativePlural`, `AllativeSingular`, `AllativePlural`, `AblativeSingular`, `AblativePlural`, `InstrumentalSingular`, `InstrumentalPlural`

**verb** cells: `ActivePresentFirstSingular`, `ActivePresentFirstPlural`, `ActivePresentSecondSingular`, `ActivePresentSecondPlural`, `ActivePresentThirdSingular`, `ActivePresentThirdPlural`, `ActivePreteriteFirstSingular`, `ActivePreteriteFirstPlural`, `ActivePreteriteSecondSingular`, `ActivePreteriteSecondPlural`, `ActivePreteriteThirdSingular`, `ActivePreteriteThirdPlural`, `ActiveImperativeFirstSingular`, `ActiveImperativeFirstPlural`, `ActiveImperativeSecondSingular`, `ActiveImperativeSecondPlural`, `ActiveImperativeThirdSingular`, `ActiveImperativeThirdPlural`, `MediopassivePresentFirstSingular`, `MediopassivePresentFirstPlural`, `MediopassivePresentSecondSingular`, `MediopassivePresentSecondPlural`, `MediopassivePresentThirdSingular`, `MediopassivePresentThirdPlural`, `MediopassivePreteriteFirstSingular`, `MediopassivePreteriteFirstPlural`, `MediopassivePreteriteSecondSingular`, `MediopassivePreteriteSecondPlural`, `MediopassivePreteriteThirdSingular`, `MediopassivePreteriteThirdPlural`, `MediopassiveImperativeFirstSingular`, `MediopassiveImperativeFirstPlural`, `MediopassiveImperativeSecondSingular`, `MediopassiveImperativeSecondPlural`, `MediopassiveImperativeThirdSingular`, `MediopassiveImperativeThirdPlural`, `Infinitive`, `VerbalNoun`, `Participle`, `Supine`

## How it works

Each attested form is stored as an edit tree from the lemma. A lemma's edits across all cells make its inflection class. A suffix tree over the word, read from its end and pruned by a description-length test, predicts the class of any word. Known words the tree gets wrong are kept in a small lexicon. Confidences are smoothed counts; variant weights follow Wiktionary's listing order (1, 1/2, 1/3…, nonstandard forms a tenth of that).

## License

Code: MIT OR Apache-2.0. Data: derived from [Wiktionary](https://www.wiktionary.org/) (via [kaikki.org](https://kaikki.org/)), CC BY-SA 4.0; the model bytes in `src/model.bin` are shared under the same license.
