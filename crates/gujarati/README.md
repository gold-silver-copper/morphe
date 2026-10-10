# gujarati

Gujarati inflection for Rust, learned from Wiktionary by [morphe](https://github.com/gold-silver-copper/morphe).
Inflects any word, known or made up. The `_scored` functions give every candidate with a confidence.

Generated from the Kaikki Gujarati dump of 2026-10-03. Do not edit by hand: change the config in morphe and regenerate.

## Example

```rust
use gujarati::*;
println!("{}", noun("ઘોડો", Case::Nominative, Number::Plural)); // ઘોડા
println!("{}", noun("ઘોડો", Case::Instrumental, Number::Singular)); // ઘોડે
println!("{:?}", noun_gender("પીછું")); // Neuter
println!("{}", adj("સારું", Gender::Masculine, Form::Nominative)); // સારો
println!("{}", verb_future("કરવું", Person::FirstSingular)); // કરીશ
println!("{}", verb_present("કરવું", Person::SecondPlural)); // કરો
println!("{}", verb_past("કરવું")); // કર્યું
```

Nouns have five cases in both numbers and their gender. Adjectives agree
in gender, with a nominative, oblique, plural and locative form. Verbs have
the verbal noun, the conjunctive (કરી, કરીને), the passive (કરાય), the
counterfactual, the past (perfective) participle, the present subjunctive
and future for five persons, and the imperatives. Tenses built with
auxiliaries (કરું છું, કરતું હતું) are left out.

## Coverage

| Part of speech | Entries | Cells | Edits | Classes | Tree nodes | Lexicon |
|---|---|---|---|---|---|---|
| noun | 1350 | 10 | 47 | 23 | 30 | 35 |
| adj | 178 | 12 | 10 | 5 | 6 | 5 |
| verb | 101 | 21 | 65 | 47 | 8 | 28 |
| pron | 7 | 12 | 12 | 2 | 2 | 0 |
| det | 3 | 12 | 7 | 1 | 2 | 0 |

Model size: 20336 bytes.

**noun** cells: `NominativeSingular`, `NominativePlural`, `ObliqueSingular`, `ObliquePlural`, `VocativeSingular`, `VocativePlural`, `InstrumentalSingular`, `InstrumentalPlural`, `LocativeSingular`, `LocativePlural`

Citation form: `NominativeSingular, NominativePlural` (comma-separated; parts you give are output exactly and narrow the prediction).

**adj** cells: `MasculineNominative`, `MasculineOblique`, `MasculinePlural`, `MasculineLocative`, `FeminineNominative`, `FeminineOblique`, `FemininePlural`, `FeminineLocative`, `NeuterNominative`, `NeuterOblique`, `NeuterPlural`, `NeuterLocative`

Citation form: `MasculineNominative, FeminineNominative, NeuterNominative` (comma-separated; parts you give are output exactly and narrow the prediction).

**verb** cells: `VerbalNoun`, `Conjunctive`, `Consecutive`, `Passive`, `Counterfactual`, `Past`, `PresentFirstSingular`, `PresentSecondSingular`, `PresentThird`, `PresentFirstPlural`, `PresentSecondPlural`, `FutureFirstSingular`, `FutureSecondSingular`, `FutureThird`, `FutureFirstPlural`, `FutureSecondPlural`, `ImperativeSecondSingular`, `ImperativeFirstPlural`, `ImperativeSecondPlural`, `FutureImperativeSingular`, `FutureImperativePlural`

**pron** cells: `MasculineNominative`, `MasculineOblique`, `MasculinePlural`, `MasculineLocative`, `FeminineNominative`, `FeminineOblique`, `FemininePlural`, `FeminineLocative`, `NeuterNominative`, `NeuterOblique`, `NeuterPlural`, `NeuterLocative`

**det** cells: `MasculineNominative`, `MasculineOblique`, `MasculinePlural`, `MasculineLocative`, `FeminineNominative`, `FeminineOblique`, `FemininePlural`, `FeminineLocative`, `NeuterNominative`, `NeuterOblique`, `NeuterPlural`, `NeuterLocative`

## How it works

Each attested form is stored as an edit tree from the lemma. A lemma's edits across all cells make its inflection class. A suffix tree over the word, read from its end and pruned by a description-length test, predicts the class of any word. Known words the tree gets wrong are kept in a small lexicon. Confidences are smoothed counts; variant weights follow Wiktionary's listing order (1, 1/2, 1/3…, nonstandard forms a tenth of that).

## License

Code: MIT OR Apache-2.0. Data: derived from [Wiktionary](https://www.wiktionary.org/) (via [kaikki.org](https://kaikki.org/)), CC BY-SA 4.0; the model bytes in `src/model.bin` are shared under the same license.
