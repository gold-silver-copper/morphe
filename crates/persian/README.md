# persian

Persian inflection for Rust, learned from Wiktionary by [morphe](https://github.com/gold-silver-copper/morphe).
Inflects any word, known or made up. The `_scored` functions give every candidate with a confidence.

Generated from the Kaikki Persian dump of 2026-10-03. Do not edit by hand: change the config in morphe and regenerate.

## Example

```rust
use persian::*;
println!("{}", verb("خوردن", Tense::Present, Person::First, Number::Singular)); // میخورم
println!("{}", verb("رفتن", Tense::Past, Person::Third, Number::Plural)); // رفتند
println!("{}", verb("کردن", Tense::PresentSubjunctive, Person::Second, Number::Singular)); // بکنی
println!("{}", verb_stem("آمدن", Stem::Present)); // آ
println!("{}", noun("کتاب", Number::Plural));
println!("{}", adj("بزرگ", Degree::Comparative)); // بزرگتر
```

Cells cover the infinitive, both stems and participles, eleven indicative
and subjunctive tenses (including the periphrastic progressive, perfect,
pluperfect and future) in all persons, and the imperative. Wiktionary's
colloquial conjugation table is left out. Nouns have the plural and
adjectives the comparative and superlative, from the headword line.

Input matching ignores harakat, tatweel and the zero-width non-joiner, and
treats Arabic ي and ك as ی and ک. Wiktionary writes most table forms without
the non-joiner (میخورم for می‌خورم).

## Coverage

| Part of speech | Entries | Cells | Edits | Classes | Tree nodes | Lexicon |
|---|---|---|---|---|---|---|
| noun | 3095 | 1 | 304 | 359 | 251 | 1143 |
| adj | 397 | 2 | 7 | 4 | 7 | 47 |
| verb | 1135 | 73 | 6482 | 381 | 40 | 916 |

Model size: 436666 bytes.

**noun** cells: `Plural`

Citation form: `Lemma, Plural` (comma-separated; parts you give are output exactly and narrow the prediction).

**adj** cells: `Comparative`, `Superlative`

Citation form: `Lemma, Comparative, Superlative` (comma-separated; parts you give are output exactly and narrow the prediction).

**verb** cells: `Infinitive`, `StemPresent`, `StemPast`, `ParticiplePresent`, `ParticiplePast`, `PresentFirstSingular`, `PresentFirstPlural`, `PresentSecondSingular`, `PresentSecondPlural`, `PresentThirdSingular`, `PresentThirdPlural`, `PresentProgressiveFirstSingular`, `PresentProgressiveFirstPlural`, `PresentProgressiveSecondSingular`, `PresentProgressiveSecondPlural`, `PresentProgressiveThirdSingular`, `PresentProgressiveThirdPlural`, `AoristFirstSingular`, `AoristFirstPlural`, `AoristSecondSingular`, `AoristSecondPlural`, `AoristThirdSingular`, `AoristThirdPlural`, `PastFirstSingular`, `PastFirstPlural`, `PastSecondSingular`, `PastSecondPlural`, `PastThirdSingular`, `PastThirdPlural`, `ImperfectFirstSingular`, `ImperfectFirstPlural`, `ImperfectSecondSingular`, `ImperfectSecondPlural`, `ImperfectThirdSingular`, `ImperfectThirdPlural`, `PastProgressiveFirstSingular`, `PastProgressiveFirstPlural`, `PastProgressiveSecondSingular`, `PastProgressiveSecondPlural`, `PastProgressiveThirdSingular`, `PastProgressiveThirdPlural`, `PerfectFirstSingular`, `PerfectFirstPlural`, `PerfectSecondSingular`, `PerfectSecondPlural`, `PerfectThirdSingular`, `PerfectThirdPlural`, `PluperfectFirstSingular`, `PluperfectFirstPlural`, `PluperfectSecondSingular`, `PluperfectSecondPlural`, `PluperfectThirdSingular`, `PluperfectThirdPlural`, `FutureFirstSingular`, `FutureFirstPlural`, `FutureSecondSingular`, `FutureSecondPlural`, `FutureThirdSingular`, `FutureThirdPlural`, `PresentSubjunctiveFirstSingular`, `PresentSubjunctiveFirstPlural`, `PresentSubjunctiveSecondSingular`, `PresentSubjunctiveSecondPlural`, `PresentSubjunctiveThirdSingular`, `PresentSubjunctiveThirdPlural`, `PastSubjunctiveFirstSingular`, `PastSubjunctiveFirstPlural`, `PastSubjunctiveSecondSingular`, `PastSubjunctiveSecondPlural`, `PastSubjunctiveThirdSingular`, `PastSubjunctiveThirdPlural`, `ImperativeSingular`, `ImperativePlural`

Citation form: `Infinitive, StemPresent` (comma-separated; parts you give are output exactly and narrow the prediction).

## How it works

Each attested form is stored as an edit tree from the lemma. A lemma's edits across all cells make its inflection class. A suffix tree over the word, read from its end and pruned by a description-length test, predicts the class of any word. Known words the tree gets wrong are kept in a small lexicon. Confidences are smoothed counts; variant weights follow Wiktionary's listing order (1, 1/2, 1/3…, nonstandard forms a tenth of that).

## License

Code: MIT OR Apache-2.0. Data: derived from [Wiktionary](https://www.wiktionary.org/) (via [kaikki.org](https://kaikki.org/)), CC BY-SA 4.0; the model bytes in `src/model.bin` are shared under the same license.
