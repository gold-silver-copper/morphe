# swahili

Swahili inflection for Rust, learned from Wiktionary by [morphe](https://github.com/gold-silver-copper/morphe).
Inflects any word, known or made up. The `_scored` functions give every candidate with a confidence.

Generated from the Kaikki Swahili dump of 2026-10-03. Do not edit by hand: change the config in morphe and regenerate.

## Example

```rust
use swahili::*;
assert_eq!(noun_plural("kitabu"), "vitabu");
println!("{}", noun_plural("mtu")); // watu
println!("{:?}", noun_class("jicho")); // V
println!("{}", verb("soma", Tense::Present, Person::FirstPlural)); // tunasoma
println!("{}", verb("soma", Tense::NegativePresent, Person::ThirdSingular)); // hasomi
println!("{}", verb("fikiri", Tense::Subjunctive, Person::SecondSingular)); // ufikiri
println!("{}", verb_imperative("soma", Number::Plural)); // someni
```

Nouns have their plural and noun class; the tree reads the start of the
word as well as its end, since Swahili marks class and number with
prefixes. Verbs are looked up by their stem (soma, not kusoma) and have the
infinitive, the imperative, the habitual, and the present, negative
present, subjunctive and gnomic for the six persons. Other tenses are a
subject prefix plus a tense marker (a-li-soma, a-ta-soma, a-me-soma), which
Wiktionary gives as formulas rather than forms.

## Coverage

| Part of speech | Entries | Cells | Edits | Classes | Tree nodes | Lexicon |
|---|---|---|---|---|---|---|
| noun | 6440 | 1 | 193 | 206 | 162 | 1999 |
| verb | 3254 | 29 | 111 | 52 | 62 | 601 |

Model size: 92672 bytes.

**noun** cells: `Plural`

Citation form: `Lemma, Plural` (comma-separated; parts you give are output exactly and narrow the prediction).

**verb** cells: `Infinitive`, `NegativeInfinitive`, `ImperativeSingular`, `ImperativePlural`, `Habitual`, `PresentFirstSingular`, `PresentSecondSingular`, `PresentThirdSingular`, `PresentFirstPlural`, `PresentSecondPlural`, `PresentThirdPlural`, `NegativePresentFirstSingular`, `NegativePresentSecondSingular`, `NegativePresentThirdSingular`, `NegativePresentFirstPlural`, `NegativePresentSecondPlural`, `NegativePresentThirdPlural`, `SubjunctiveFirstSingular`, `SubjunctiveSecondSingular`, `SubjunctiveThirdSingular`, `SubjunctiveFirstPlural`, `SubjunctiveSecondPlural`, `SubjunctiveThirdPlural`, `GnomicFirstSingular`, `GnomicSecondSingular`, `GnomicThirdSingular`, `GnomicFirstPlural`, `GnomicSecondPlural`, `GnomicThirdPlural`

## How it works

Each attested form is stored as an edit tree from the lemma. A lemma's edits across all cells make its inflection class. A suffix tree over the word, read from its end and pruned by a description-length test, predicts the class of any word. Known words the tree gets wrong are kept in a small lexicon. Confidences are smoothed counts; variant weights follow Wiktionary's listing order (1, 1/2, 1/3…, nonstandard forms a tenth of that).

## License

Code: MIT OR Apache-2.0. Data: derived from [Wiktionary](https://www.wiktionary.org/) (via [kaikki.org](https://kaikki.org/)), CC BY-SA 4.0; the model bytes in `src/model.bin` are shared under the same license.
