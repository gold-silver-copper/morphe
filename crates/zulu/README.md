# zulu

Zulu inflection for Rust, learned from Wiktionary by [morphe](https://github.com/gold-silver-copper/morphe).
Inflects any word, known or made up. The `_scored` functions give every candidate with a confidence.

Generated from the Kaikki Zulu dump of 2026-10-03. Do not edit by hand: change the config in morphe and regenerate.

## Example

```rust
use zulu::*;
println!("{}", noun("umuntu", Number::Plural)); // abantu
println!("{}", noun_locative("impala", Number::Singular)); // empaleni
println!("{}", noun_possessive("impala", Concord::C7, Number::Plural)); // sezimpala
println!("{:?}", noun_class("isilo")); // C7
println!("{}", verb("hamba", Tense::Present, Person::FirstSingular)); // ngiyahamba
println!("{}", verb("hamba", Tense::Perfect, Person::SecondPlural)); // nihambile
println!("{}", verb_imperative("hamba", Number::Plural)); // hambani
```

Nouns have their singular and plural, the forms without the initial vowel,
the locative and copulative, the possessive forms agreeing with the
possessed noun's class, and their noun class; the tree reads the start of
the word as well as its end, since Zulu marks class and number with
prefixes. Verbs are looked up by their stem (hamba) and have the
infinitive, the imperative, and the present, perfect, remote past, near
and remote future, potential and subjunctive (with their negatives) for the
first and second persons. Third person forms agree with the subject's noun
class; Wiktionary's tables merge identical cells, so those are left out.
Input matching ignores tone marks.

## Coverage

| Part of speech | Entries | Cells | Edits | Classes | Tree nodes | Lexicon |
|---|---|---|---|---|---|---|
| noun | 1636 | 26 | 559 | 193 | 214 | 532 |
| verb | 863 | 56 | 336 | 22 | 17 | 85 |

Model size: 176953 bytes.

**noun** cells: `Singular`, `Plural`, `BareSingular`, `BarePlural`, `LocativeSingular`, `LocativePlural`, `CopulativeSingular`, `CopulativePlural`, `PossessiveC1Singular`, `PossessiveC1Plural`, `PossessiveC2Singular`, `PossessiveC2Plural`, `PossessiveC4Singular`, `PossessiveC4Plural`, `PossessiveC5Singular`, `PossessiveC5Plural`, `PossessiveC6Singular`, `PossessiveC6Plural`, `PossessiveC7Singular`, `PossessiveC7Plural`, `PossessiveC8Singular`, `PossessiveC8Plural`, `PossessiveC11Singular`, `PossessiveC11Plural`, `PossessiveC15Singular`, `PossessiveC15Plural`

Citation form: `Singular, Plural` (comma-separated; parts you give are output exactly and narrow the prediction).

**verb** cells: `Infinitive`, `NegativeInfinitive`, `ImperativeSingular`, `ImperativePlural`, `PresentFirstSingular`, `PresentSecondSingular`, `PresentFirstPlural`, `PresentSecondPlural`, `PresentNegativeFirstSingular`, `PresentNegativeSecondSingular`, `PresentNegativeFirstPlural`, `PresentNegativeSecondPlural`, `PerfectFirstSingular`, `PerfectSecondSingular`, `PerfectFirstPlural`, `PerfectSecondPlural`, `PerfectNegativeFirstSingular`, `PerfectNegativeSecondSingular`, `PerfectNegativeFirstPlural`, `PerfectNegativeSecondPlural`, `RemotePastFirstSingular`, `RemotePastSecondSingular`, `RemotePastFirstPlural`, `RemotePastSecondPlural`, `NearFutureFirstSingular`, `NearFutureSecondSingular`, `NearFutureFirstPlural`, `NearFutureSecondPlural`, `NearFutureNegativeFirstSingular`, `NearFutureNegativeSecondSingular`, `NearFutureNegativeFirstPlural`, `NearFutureNegativeSecondPlural`, `RemoteFutureFirstSingular`, `RemoteFutureSecondSingular`, `RemoteFutureFirstPlural`, `RemoteFutureSecondPlural`, `RemoteFutureNegativeFirstSingular`, `RemoteFutureNegativeSecondSingular`, `RemoteFutureNegativeFirstPlural`, `RemoteFutureNegativeSecondPlural`, `PotentialFirstSingular`, `PotentialSecondSingular`, `PotentialFirstPlural`, `PotentialSecondPlural`, `PotentialNegativeFirstSingular`, `PotentialNegativeSecondSingular`, `PotentialNegativeFirstPlural`, `PotentialNegativeSecondPlural`, `SubjunctiveFirstSingular`, `SubjunctiveSecondSingular`, `SubjunctiveFirstPlural`, `SubjunctiveSecondPlural`, `SubjunctiveNegativeFirstSingular`, `SubjunctiveNegativeSecondSingular`, `SubjunctiveNegativeFirstPlural`, `SubjunctiveNegativeSecondPlural`

## How it works

Each attested form is stored as an edit tree from the lemma. A lemma's edits across all cells make its inflection class. A suffix tree over the word, read from its end and pruned by a description-length test, predicts the class of any word. Known words the tree gets wrong are kept in a small lexicon. Confidences are smoothed counts; variant weights follow Wiktionary's listing order (1, 1/2, 1/3…, nonstandard forms a tenth of that).

## License

Code: MIT OR Apache-2.0. Data: derived from [Wiktionary](https://www.wiktionary.org/) (via [kaikki.org](https://kaikki.org/)), CC BY-SA 4.0; the model bytes in `src/model.bin` are shared under the same license.
