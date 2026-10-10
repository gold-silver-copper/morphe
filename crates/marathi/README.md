# marathi

Marathi inflection for Rust, learned from Wiktionary by [morphe](https://github.com/gold-silver-copper/morphe).
Inflects any word, known or made up. The `_scored` functions give every candidate with a confidence.

Generated from the Kaikki Marathi dump of 2026-10-03. Do not edit by hand: change the config in morphe and regenerate.

## Example

```rust
use marathi::*;
println!("{}", noun("वृक्ष", Case::Dative, Number::Plural)); // वृक्षांना
println!("{}", noun("आत्मा", Case::Oblique, Number::Singular)); // आत्म्या
println!("{:?}", noun_gender("विश्व")); // Neuter
println!("{}", verb_past("जाणे", Person::FirstSingular, Gender::Masculine)); // गेलो
println!("{}", verb_present("करणे", Person::SecondSingular, Gender::Feminine)); // करतेस
println!("{}", verb_obligative("करणे", Gender::Masculine, Number::Singular)); // करावा
println!("{}", verb_completive("करणे")); // करून
```

Nouns have the direct, oblique, dative, ergative, instrumental, locative,
genitive and vocative in both numbers, and their gender. Verbs have the
stem, the infinitives, the completive (-ऊन), prospective (-णार),
hortative, desiderative (-आयला) and progressive forms, the obligative
(-आवा) agreeing with its object, and the simple past and habitual present
by person and gender. Tenses built with auxiliaries are left out.

## Coverage

| Part of speech | Entries | Cells | Edits | Classes | Tree nodes | Lexicon |
|---|---|---|---|---|---|---|
| noun | 3381 | 16 | 647 | 177 | 168 | 1511 |
| verb | 239 | 51 | 175 | 28 | 6 | 35 |

Model size: 274523 bytes.

**noun** cells: `DirectSingular`, `DirectPlural`, `ObliqueSingular`, `ObliquePlural`, `DativeSingular`, `DativePlural`, `ErgativeSingular`, `ErgativePlural`, `InstrumentalSingular`, `InstrumentalPlural`, `LocativeSingular`, `LocativePlural`, `GenitiveSingular`, `GenitivePlural`, `VocativeSingular`, `VocativePlural`

Citation form: `DirectSingular, ObliqueSingular` (comma-separated; parts you give are output exactly and narrow the prediction).

**verb** cells: `Stem`, `Infinitive`, `ObliqueInfinitive`, `Completive`, `Prospective`, `Hortative`, `Desiderative`, `Progressive`, `ObliquePerfective`, `ObligativeMasculineSingular`, `ObligativeMasculinePlural`, `ObligativeFeminineSingular`, `ObligativeFemininePlural`, `ObligativeNeuterSingular`, `ObligativeNeuterPlural`, `PastFirstSingularMasculine`, `PastFirstSingularFeminine`, `PastFirstSingularNeuter`, `PastSecondSingularMasculine`, `PastSecondSingularFeminine`, `PastSecondSingularNeuter`, `PastThirdSingularMasculine`, `PastThirdSingularFeminine`, `PastThirdSingularNeuter`, `PastFirstPluralMasculine`, `PastFirstPluralFeminine`, `PastFirstPluralNeuter`, `PastSecondPluralMasculine`, `PastSecondPluralFeminine`, `PastSecondPluralNeuter`, `PastThirdPluralMasculine`, `PastThirdPluralFeminine`, `PastThirdPluralNeuter`, `PresentFirstSingularMasculine`, `PresentFirstSingularFeminine`, `PresentFirstSingularNeuter`, `PresentSecondSingularMasculine`, `PresentSecondSingularFeminine`, `PresentSecondSingularNeuter`, `PresentThirdSingularMasculine`, `PresentThirdSingularFeminine`, `PresentThirdSingularNeuter`, `PresentFirstPluralMasculine`, `PresentFirstPluralFeminine`, `PresentFirstPluralNeuter`, `PresentSecondPluralMasculine`, `PresentSecondPluralFeminine`, `PresentSecondPluralNeuter`, `PresentThirdPluralMasculine`, `PresentThirdPluralFeminine`, `PresentThirdPluralNeuter`

## How it works

Each attested form is stored as an edit tree from the lemma. A lemma's edits across all cells make its inflection class. A suffix tree over the word, read from its end and pruned by a description-length test, predicts the class of any word. Known words the tree gets wrong are kept in a small lexicon. Confidences are smoothed counts; variant weights follow Wiktionary's listing order (1, 1/2, 1/3…, nonstandard forms a tenth of that).

## License

Code: MIT OR Apache-2.0. Data: derived from [Wiktionary](https://www.wiktionary.org/) (via [kaikki.org](https://kaikki.org/)), CC BY-SA 4.0; the model bytes in `src/model.bin` are shared under the same license.
