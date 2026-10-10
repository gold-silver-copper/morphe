# tagalog

Tagalog inflection for Rust, learned from Wiktionary by [morphe](https://github.com/gold-silver-copper/morphe).
Inflects any word, known or made up. The `_scored` functions give every candidate with a confidence.

Generated from the Kaikki Tagalog dump of 2026-10-03. Do not edit by hand: change the config in morphe and regenerate.

## Example

```rust
use tagalog::*;
println!("{}", verb("bumili", Focus::Actor, Aspect::Progressive)); // bumibili
println!("{}", verb("bumili", Focus::Actor, Aspect::Contemplative)); // bibili
println!("{}", verb("bumili", Focus::Object, Aspect::Completed)); // binili
println!("{}", verb("dalhin", Focus::Object, Aspect::Progressive)); // dinadala
println!("{}", verb_root("dalhin")); // dala
println!("{}", adj_plural("maganda")); // magaganda
```

Verbs are looked up by their dictionary form (an infinitive, like bumili or
dalhin) and have the root, the infinitive, completed, progressive and
contemplative aspects for the actor, object, directional, locative,
benefactive and instrumental foci, and the recent past. Affixes and
reduplication come at the start of the word and infixes inside it, so the
tree reads the first letters of a word as well as its end. Adjectives have
the plural and the comparison forms. Input matching ignores stress marks.

## Coverage

| Part of speech | Entries | Cells | Edits | Classes | Tree nodes | Lexicon |
|---|---|---|---|---|---|---|
| verb | 1911 | 26 | 5809 | 1900 | 526 | 1199 |
| adj | 422 | 11 | 260 | 173 | 43 | 215 |

Model size: 282348 bytes.

**verb** cells: `Root`, `ActorInfinitive`, `ActorCompleted`, `ActorProgressive`, `ActorContemplative`, `ObjectInfinitive`, `ObjectCompleted`, `ObjectProgressive`, `ObjectContemplative`, `DirectionalInfinitive`, `DirectionalCompleted`, `DirectionalProgressive`, `DirectionalContemplative`, `LocativeInfinitive`, `LocativeCompleted`, `LocativeProgressive`, `LocativeContemplative`, `BenefactiveInfinitive`, `BenefactiveCompleted`, `BenefactiveProgressive`, `BenefactiveContemplative`, `InstrumentalInfinitive`, `InstrumentalCompleted`, `InstrumentalProgressive`, `InstrumentalContemplative`, `RecentPast`

**adj** cells: `Root`, `Plural`, `Equal`, `SuperiorSingular`, `SuperiorPlural`, `InferiorSingular`, `InferiorPlural`, `RelativeSingular`, `RelativePlural`, `AbsoluteSingular`, `AbsolutePlural`

## How it works

Each attested form is stored as an edit tree from the lemma. A lemma's edits across all cells make its inflection class. A suffix tree over the word, read from its end and pruned by a description-length test, predicts the class of any word. Known words the tree gets wrong are kept in a small lexicon. Confidences are smoothed counts; variant weights follow Wiktionary's listing order (1, 1/2, 1/3…, nonstandard forms a tenth of that).

## License

Code: MIT OR Apache-2.0. Data: derived from [Wiktionary](https://www.wiktionary.org/) (via [kaikki.org](https://kaikki.org/)), CC BY-SA 4.0; the model bytes in `src/model.bin` are shared under the same license.
