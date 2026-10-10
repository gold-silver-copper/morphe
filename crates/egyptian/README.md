# egyptian

Egyptian inflection for Rust, learned from Wiktionary by [morphe](https://github.com/gold-silver-copper/morphe).
Inflects any word, known or made up. The `_scored` functions give every candidate with a confidence.

Generated from the Kaikki Egyptian dump of 2026-10-03. Do not edit by hand: change the config in morphe and regenerate.

## Example

```rust
use egyptian::*;
println!("{}", noun("h", Number::Plural)); // hw
println!("{}", noun("h", Number::Dual)); // hwj
println!("{}", adj("km", Gender::Feminine, Number::Singular)); // kmt
println!("{}", verb("gp", Tense::Perfect)); // gp.n
println!("{}", verb_participle("gp", Voice::Passive, Aspect::Perfective)); // gpp
```

Words are in the standard Egyptological transliteration (ꜣ, ꜥ, ḥ, ḫ, ẖ, š,
ṯ, ḏ). Nouns have the singular, dual and plural and their gender;
adjectives agree in gender and number. Verbs have the infinitive and its
negative, the imperative, the stative, the suffix-conjugation forms (sḏm.n,
sḏm.jn, sḏm.ḫr, sḏm.kꜣ, …) active and passive, and the participles and
relative forms. Forms built with auxiliaries (ḥr sḏm) are left out.

## Coverage

| Part of speech | Entries | Cells | Edits | Classes | Tree nodes | Lexicon |
|---|---|---|---|---|---|---|
| noun | 2136 | 3 | 162 | 111 | 86 | 723 |
| adj | 80 | 6 | 61 | 23 | 6 | 14 |
| verb | 850 | 39 | 370 | 336 | 72 | 454 |

Model size: 72042 bytes.

**noun** cells: `Singular`, `Dual`, `Plural`

Citation form: `Singular, Plural` (comma-separated; parts you give are output exactly and narrow the prediction).

**adj** cells: `MasculineSingular`, `MasculineDual`, `MasculinePlural`, `FeminineSingular`, `FeminineDual`, `FemininePlural`

Citation form: `MasculineSingular, FeminineSingular` (comma-separated; parts you give are output exactly and narrow the prediction).

**verb** cells: `Infinitive`, `NegativeInfinitive`, `ImperativeSingular`, `ImperativePlural`, `Stative`, `Perfect`, `Perfective`, `Imperfective`, `Prospective`, `Subjunctive`, `Consecutive`, `Terminative`, `Obligative`, `Potential`, `PassivePerfect`, `PassivePerfective`, `PassiveImperfective`, `PassiveProspective`, `PassiveSubjunctive`, `PassiveConsecutive`, `PassiveTerminative`, `PassiveObligative`, `PassivePotential`, `ParticipleActivePerfect`, `ParticipleActivePerfective`, `ParticipleActiveImperfective`, `ParticipleActiveProspective`, `ParticiplePassivePerfect`, `ParticiplePassivePerfective`, `ParticiplePassiveImperfective`, `ParticiplePassiveProspective`, `RelativeActivePerfect`, `RelativeActivePerfective`, `RelativeActiveImperfective`, `RelativeActiveProspective`, `RelativePassivePerfect`, `RelativePassivePerfective`, `RelativePassiveImperfective`, `RelativePassiveProspective`

Citation form: `Infinitive, Perfect` (comma-separated; parts you give are output exactly and narrow the prediction).

## How it works

Each attested form is stored as an edit tree from the lemma. A lemma's edits across all cells make its inflection class. A suffix tree over the word, read from its end and pruned by a description-length test, predicts the class of any word. Known words the tree gets wrong are kept in a small lexicon. Confidences are smoothed counts; variant weights follow Wiktionary's listing order (1, 1/2, 1/3…, nonstandard forms a tenth of that).

## License

Code: MIT OR Apache-2.0. Data: derived from [Wiktionary](https://www.wiktionary.org/) (via [kaikki.org](https://kaikki.org/)), CC BY-SA 4.0; the model bytes in `src/model.bin` are shared under the same license.
