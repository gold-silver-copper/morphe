# pashto

Pashto inflection for Rust, learned from Wiktionary by [morphe](https://github.com/gold-silver-copper/morphe).
Inflects any word, known or made up. The `_scored` functions give every candidate with a confidence.

Generated from the Kaikki Pashto dump of 2026-10-03. Do not edit by hand: change the config in morphe and regenerate.

## Example

```rust
use pashto::*;
println!("{}", noun("انا", Case::Direct, Number::Plural)); // اناګانې
println!("{:?}", noun_gender("انا")); // Feminine
println!("{}", adj("پروت", Gender::Feminine, Case::Direct, Number::Singular)); // پرته
println!("{}", verb_present("کول", Aspect::Imperfective, Person::First, Number::Singular)); // کوم
println!("{}", verb_past("کول", Aspect::Perfective, PastPerson::ThirdMasculine, Number::Singular)); // کړ
```

Nouns have the direct, oblique and vocative in both numbers and their
gender; adjectives have them for both genders. Verbs have the present and
the past, imperfective and perfective, for six persons (with the third
person's gender in the past), and the imperative, positive and negative.
Input matching ignores the short-vowel marks.

## Coverage

| Part of speech | Entries | Cells | Edits | Classes | Tree nodes | Lexicon |
|---|---|---|---|---|---|---|
| noun | 406 | 6 | 110 | 59 | 26 | 123 |
| adj | 93 | 12 | 69 | 29 | 11 | 34 |
| verb | 92 | 32 | 560 | 105 | 15 | 70 |

Model size: 34167 bytes.

**noun** cells: `DirectSingular`, `DirectPlural`, `ObliqueSingular`, `ObliquePlural`, `VocativeSingular`, `VocativePlural`

Citation form: `DirectSingular, DirectPlural` (comma-separated; parts you give are output exactly and narrow the prediction).

**adj** cells: `MasculineDirectSingular`, `MasculineDirectPlural`, `MasculineObliqueSingular`, `MasculineObliquePlural`, `MasculineVocativeSingular`, `MasculineVocativePlural`, `FeminineDirectSingular`, `FeminineDirectPlural`, `FeminineObliqueSingular`, `FeminineObliquePlural`, `FeminineVocativeSingular`, `FeminineVocativePlural`

**verb** cells: `PresentImperfectiveFirstSingular`, `PresentImperfectiveFirstPlural`, `PresentImperfectiveSecondSingular`, `PresentImperfectiveSecondPlural`, `PresentImperfectiveThirdSingular`, `PresentImperfectiveThirdPlural`, `PresentPerfectiveFirstSingular`, `PresentPerfectiveFirstPlural`, `PresentPerfectiveSecondSingular`, `PresentPerfectiveSecondPlural`, `PresentPerfectiveThirdSingular`, `PresentPerfectiveThirdPlural`, `PastImperfectiveFirstSingular`, `PastImperfectiveFirstPlural`, `PastImperfectiveSecondSingular`, `PastImperfectiveSecondPlural`, `PastImperfectiveThirdMasculineSingular`, `PastImperfectiveThirdMasculinePlural`, `PastImperfectiveThirdFeminineSingular`, `PastImperfectiveThirdFemininePlural`, `PastPerfectiveFirstSingular`, `PastPerfectiveFirstPlural`, `PastPerfectiveSecondSingular`, `PastPerfectiveSecondPlural`, `PastPerfectiveThirdMasculineSingular`, `PastPerfectiveThirdMasculinePlural`, `PastPerfectiveThirdFeminineSingular`, `PastPerfectiveThirdFemininePlural`, `ImperativeSingular`, `ImperativePlural`, `NegativeImperativeSingular`, `NegativeImperativePlural`

## How it works

Each attested form is stored as an edit tree from the lemma. A lemma's edits across all cells make its inflection class. A suffix tree over the word, read from its end and pruned by a description-length test, predicts the class of any word. Known words the tree gets wrong are kept in a small lexicon. Confidences are smoothed counts; variant weights follow Wiktionary's listing order (1, 1/2, 1/3…, nonstandard forms a tenth of that).

## License

Code: MIT OR Apache-2.0. Data: derived from [Wiktionary](https://www.wiktionary.org/) (via [kaikki.org](https://kaikki.org/)), CC BY-SA 4.0; the model bytes in `src/model.bin` are shared under the same license.
