# navajo

Navajo inflection for Rust, learned from Wiktionary by [morphe](https://github.com/gold-silver-copper/morphe).
Inflects any word, known or made up. The `_scored` functions give every candidate with a confidence.

Generated from the Kaikki Navajo dump of 2026-10-03. Do not edit by hand: change the config in morphe and regenerate.

## Example

```rust
use navajo::*;
println!("{}", verb("yitłʼó", Mode::Imperfective, Person::FirstSingular)); // yishtłʼó
println!("{}", verb("yitłʼó", Mode::Perfective, Person::SecondDual)); // sootłʼǫ́
println!("{}", verb("hataał", Mode::Future, Person::FirstSingular)); // hodeeshtał
println!("{}", noun_possessed("his", Possessor::FirstSingular)); // shihis
println!("{}", postp_possessed("átágoʼ", Possessor::Third)); // bítágoʼ
```

Verbs are cited by the imperfective third person and have the
imperfective, perfective, future, progressive (and, where Wiktionary gives
them, iterative and optative) for twelve persons: singular, dual and
plural first and second persons, the third and fourth (with their plurals),
the indefinite and the spatial person. Neuter verbs have person forms
without a mode. Nouns and postpositions have their possessed forms. The
tree reads the start of the word, where Navajo puts its prefixes; an
apostrophe stands for the glottal stop ʼ.

## Coverage

| Part of speech | Entries | Cells | Edits | Classes | Tree nodes | Lexicon |
|---|---|---|---|---|---|---|
| verb | 17103 | 96 | 9574 | 1263 | 321 | 1292 |
| noun | 371 | 9 | 366 | 131 | 35 | 117 |
| postp | 91 | 9 | 113 | 49 | 12 | 23 |

Model size: 375659 bytes.

**verb** cells: `ImperfectiveFirstSingular`, `ImperfectiveFirstDual`, `ImperfectiveFirstPlural`, `ImperfectiveSecondSingular`, `ImperfectiveSecondDual`, `ImperfectiveSecondPlural`, `ImperfectiveThird`, `ImperfectiveThirdPlural`, `ImperfectiveFourth`, `ImperfectiveFourthPlural`, `ImperfectiveIndefinite`, `ImperfectiveSpatial`, `PerfectiveFirstSingular`, `PerfectiveFirstDual`, `PerfectiveFirstPlural`, `PerfectiveSecondSingular`, `PerfectiveSecondDual`, `PerfectiveSecondPlural`, `PerfectiveThird`, `PerfectiveThirdPlural`, `PerfectiveFourth`, `PerfectiveFourthPlural`, `PerfectiveIndefinite`, `PerfectiveSpatial`, `DistributivePerfectiveFirstSingular`, `DistributivePerfectiveFirstDual`, `DistributivePerfectiveFirstPlural`, `DistributivePerfectiveSecondSingular`, `DistributivePerfectiveSecondDual`, `DistributivePerfectiveSecondPlural`, `DistributivePerfectiveThird`, `DistributivePerfectiveThirdPlural`, `DistributivePerfectiveFourth`, `DistributivePerfectiveFourthPlural`, `DistributivePerfectiveIndefinite`, `DistributivePerfectiveSpatial`, `FutureFirstSingular`, `FutureFirstDual`, `FutureFirstPlural`, `FutureSecondSingular`, `FutureSecondDual`, `FutureSecondPlural`, `FutureThird`, `FutureThirdPlural`, `FutureFourth`, `FutureFourthPlural`, `FutureIndefinite`, `FutureSpatial`, `ProgressiveFirstSingular`, `ProgressiveFirstDual`, `ProgressiveFirstPlural`, `ProgressiveSecondSingular`, `ProgressiveSecondDual`, `ProgressiveSecondPlural`, `ProgressiveThird`, `ProgressiveThirdPlural`, `ProgressiveFourth`, `ProgressiveFourthPlural`, `ProgressiveIndefinite`, `ProgressiveSpatial`, `IterativeFirstSingular`, `IterativeFirstDual`, `IterativeFirstPlural`, `IterativeSecondSingular`, `IterativeSecondDual`, `IterativeSecondPlural`, `IterativeThird`, `IterativeThirdPlural`, `IterativeFourth`, `IterativeFourthPlural`, `IterativeIndefinite`, `IterativeSpatial`, `OptativeFirstSingular`, `OptativeFirstDual`, `OptativeFirstPlural`, `OptativeSecondSingular`, `OptativeSecondDual`, `OptativeSecondPlural`, `OptativeThird`, `OptativeThirdPlural`, `OptativeFourth`, `OptativeFourthPlural`, `OptativeIndefinite`, `OptativeSpatial`, `NeuterFirstSingular`, `NeuterFirstDual`, `NeuterFirstPlural`, `NeuterSecondSingular`, `NeuterSecondDual`, `NeuterSecondPlural`, `NeuterThird`, `NeuterThirdPlural`, `NeuterFourth`, `NeuterFourthPlural`, `NeuterIndefinite`, `NeuterSpatial`

Citation form: `ImperfectiveThird, PerfectiveThird` (comma-separated; parts you give are output exactly and narrow the prediction).

**noun** cells: `PossessedFirstSingular`, `PossessedSecondSingular`, `PossessedThird`, `PossessedFourth`, `PossessedFirstDual`, `PossessedSecondDual`, `PossessedFirstPlural`, `PossessedSecondPlural`, `PossessedIndefinite`

**postp** cells: `PossessedFirstSingular`, `PossessedSecondSingular`, `PossessedThird`, `PossessedFourth`, `PossessedFirstDual`, `PossessedSecondDual`, `PossessedFirstPlural`, `PossessedSecondPlural`, `PossessedIndefinite`

## How it works

Each attested form is stored as an edit tree from the lemma. A lemma's edits across all cells make its inflection class. A suffix tree over the word, read from its end and pruned by a description-length test, predicts the class of any word. Known words the tree gets wrong are kept in a small lexicon. Confidences are smoothed counts; variant weights follow Wiktionary's listing order (1, 1/2, 1/3…, nonstandard forms a tenth of that).

## License

Code: MIT OR Apache-2.0. Data: derived from [Wiktionary](https://www.wiktionary.org/) (via [kaikki.org](https://kaikki.org/)), CC BY-SA 4.0; the model bytes in `src/model.bin` are shared under the same license.
