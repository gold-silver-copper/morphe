# coptic

Coptic inflection for Rust, learned from Wiktionary by [morphe](https://github.com/gold-silver-copper/morphe).
Inflects any word, known or made up. The `_scored` functions give every candidate with a confidence.

Generated from the Kaikki Coptic dump of 2026-10-03. Do not edit by hand: change the config in morphe and regenerate.

## Example

```rust
use coptic::*;
println!("{}", verb("ⲥⲁϫⲓ", Tense::Past, Person::ThirdMasculine)); // ⲁϥⲥⲁϫⲓ
println!("{}", verb("ⲥⲁϫⲓ", Tense::Aorist, Person::FirstPlural)); // ϣⲁⲛⲥⲁϫⲓ
println!("{}", verb("ϣⲱⲡⲉ", Tense::Future, Person::SecondFeminine));
println!("{}", verb_stative("ϣⲱⲡⲉ")); // ϣⲟⲟⲡ
println!("{}", noun_plural("ϩⲧⲟ"));
```

Verbs have their stative and construct states where Wiktionary gives them,
the negative infinitive and imperative, and the main conjugations (present,
future, past, aorist, expectative, optative, jussive, conjunctives,
conditional, limitative, precursive) for eight persons; Coptic builds
these with prefixes on an unchanging infinitive. Each dialect (Sahidic,
Bohairic, …) spells its own entries. Nouns have the plural, the construct
and pronominal states and their gender. Input matching ignores
supralinear strokes and jinkim.

## Coverage

| Part of speech | Entries | Cells | Edits | Classes | Tree nodes | Lexicon |
|---|---|---|---|---|---|---|
| noun | 501 | 3 | 235 | 195 | 40 | 146 |
| verb | 459 | 101 | 1200 | 655 | 68 | 106 |

Model size: 163625 bytes.

**noun** cells: `Plural`, `Construct`, `Pronominal`

Citation form: `Lemma, Plural` (comma-separated; parts you give are output exactly and narrow the prediction).

**verb** cells: `Stative`, `Construct`, `Pronominal`, `NegativeInfinitive`, `NegativeImperative`, `PresentFirstSingular`, `PresentSecondMasculine`, `PresentSecondFeminine`, `PresentThirdMasculine`, `PresentThirdFeminine`, `PresentFirstPlural`, `PresentSecondPlural`, `PresentThirdPlural`, `FutureFirstSingular`, `FutureSecondMasculine`, `FutureSecondFeminine`, `FutureThirdMasculine`, `FutureThirdFeminine`, `FutureFirstPlural`, `FutureSecondPlural`, `FutureThirdPlural`, `PastFirstSingular`, `PastSecondMasculine`, `PastSecondFeminine`, `PastThirdMasculine`, `PastThirdFeminine`, `PastFirstPlural`, `PastSecondPlural`, `PastThirdPlural`, `AoristFirstSingular`, `AoristSecondMasculine`, `AoristSecondFeminine`, `AoristThirdMasculine`, `AoristThirdFeminine`, `AoristFirstPlural`, `AoristSecondPlural`, `AoristThirdPlural`, `ExpectativeFirstSingular`, `ExpectativeSecondMasculine`, `ExpectativeSecondFeminine`, `ExpectativeThirdMasculine`, `ExpectativeThirdFeminine`, `ExpectativeFirstPlural`, `ExpectativeSecondPlural`, `ExpectativeThirdPlural`, `OptativeFirstSingular`, `OptativeSecondMasculine`, `OptativeSecondFeminine`, `OptativeThirdMasculine`, `OptativeThirdFeminine`, `OptativeFirstPlural`, `OptativeSecondPlural`, `OptativeThirdPlural`, `JussiveFirstSingular`, `JussiveSecondMasculine`, `JussiveSecondFeminine`, `JussiveThirdMasculine`, `JussiveThirdFeminine`, `JussiveFirstPlural`, `JussiveSecondPlural`, `JussiveThirdPlural`, `ConjunctiveFirstSingular`, `ConjunctiveSecondMasculine`, `ConjunctiveSecondFeminine`, `ConjunctiveThirdMasculine`, `ConjunctiveThirdFeminine`, `ConjunctiveFirstPlural`, `ConjunctiveSecondPlural`, `ConjunctiveThirdPlural`, `FutureConjunctiveFirstSingular`, `FutureConjunctiveSecondMasculine`, `FutureConjunctiveSecondFeminine`, `FutureConjunctiveThirdMasculine`, `FutureConjunctiveThirdFeminine`, `FutureConjunctiveFirstPlural`, `FutureConjunctiveSecondPlural`, `FutureConjunctiveThirdPlural`, `ConditionalFirstSingular`, `ConditionalSecondMasculine`, `ConditionalSecondFeminine`, `ConditionalThirdMasculine`, `ConditionalThirdFeminine`, `ConditionalFirstPlural`, `ConditionalSecondPlural`, `ConditionalThirdPlural`, `LimitativeFirstSingular`, `LimitativeSecondMasculine`, `LimitativeSecondFeminine`, `LimitativeThirdMasculine`, `LimitativeThirdFeminine`, `LimitativeFirstPlural`, `LimitativeSecondPlural`, `LimitativeThirdPlural`, `PrecursiveFirstSingular`, `PrecursiveSecondMasculine`, `PrecursiveSecondFeminine`, `PrecursiveThirdMasculine`, `PrecursiveThirdFeminine`, `PrecursiveFirstPlural`, `PrecursiveSecondPlural`, `PrecursiveThirdPlural`

Citation form: `Lemma, Stative` (comma-separated; parts you give are output exactly and narrow the prediction).

## How it works

Each attested form is stored as an edit tree from the lemma. A lemma's edits across all cells make its inflection class. A suffix tree over the word, read from its end and pruned by a description-length test, predicts the class of any word. Known words the tree gets wrong are kept in a small lexicon. Confidences are smoothed counts; variant weights follow Wiktionary's listing order (1, 1/2, 1/3…, nonstandard forms a tenth of that).

## License

Code: MIT OR Apache-2.0. Data: derived from [Wiktionary](https://www.wiktionary.org/) (via [kaikki.org](https://kaikki.org/)), CC BY-SA 4.0; the model bytes in `src/model.bin` are shared under the same license.
