# morphe

Learns a finished inflection crate for any language from Wiktionary.

morphe reads a [Kaikki](https://kaikki.org/) Wiktionary dump plus a short Rust
description of a language (its features, cells, citation format, spelling
quirks) and writes a Rust crate that inflects any word of that language,
attested or made up, with a confidence for every answer.

```rust
english::noun("octopus", english::Number::Plural)              // "octopuses"
english::noun_scored("octopus", english::Number::Plural)       // [("octopuses", 0.81), ("octopusses", 0.08), ("octopi", 0.04), …]
english::verb("spling", Tense::Past, Person::Third, Number::Singular) // "splung"
russian::noun("кракозябра", Case::Instrumental, Number::Plural)    // "кракозя́брами"
russian::noun_gender("тетрадь")                                  // [(Feminine, …), …]
russian::noun("путь, пути", Case::Instrumental, Number::Singular) // citation pins principal parts
```

## Layout

```
crates/morphe         runtime: tiny, dependency-free; edit trees, model codec, prediction
crates/morphe-train   trainer: Kaikki reader → records → edit trees → classes → pruned trees → crate
configs/src/langs/    one language description per file, registered automatically
crates/<language>     generated crates, committed
```

## How it works

- **Edits.** Every attested (lemma, cell, form) becomes an edit tree: the two
  strings aligned on their longest common substring, recursively, with
  literal substitutions where nothing is shared. Edit trees cover suffixes,
  prefixes, internal changes (sing → sang), fleeting vowels (день → дня),
  stress moves and periphrasis (more beautiful, бу́ду писа́ть).
- **Classes.** A lemma's edits across all cells of its part of speech, with
  variants, form its inflection class.
- **Prediction.** A suffix tree over the word, read from its end (plus
  optional extra symbols such as syllable count), counts classes at every
  node. Nodes are smoothed toward their nearest kept ancestor and kept only
  when they gain more log-likelihood than λ nats per stored label, so what
  survives is broad patterns near the root and sharp exceptions deeper
  down. Classes whose edits can't apply to a word are filtered out.
- **Spelling restoration.** A second tree restores what users don't type
  (stress marks, macrons) before the class edits apply; marks only land on
  letters that carry them in the data.
- **Known words.** Attested words whose top prediction is exactly right are
  stored as bare keys (certain answers). The rest go into a lexicon with all
  their readings, so homographs (lie: lay / lied) give one honest
  distribution.
- **Citations.** "книга, книги" or "lie, lied": principal parts filter the
  candidate classes, and the parts given are output exactly.
- **Variants.** Wiktionary lists variants in editorial order: weights 1,
  1/2, 1/3…, and soft-tagged forms (nonstandard, rare…) a tenth of that.
  Confidences reflect Wiktionary's ordering, not measured usage.

The trainer contains no language names; everything specific is in
`configs/src/langs/<language>.rs`.

## Commands

```sh
cd ~/code/morphe
cargo build --release -j 8
./target/release/morphe tags Russian noun          # tag sets in a dump, to write a config from
./target/release/morphe train russian english      # regenerate crates (dumps in ~/.cache/kaikki/<Title>.jsonl)
./target/release/morphe train all
./target/release/morphe show russian noun книга    # a word's paradigm with confidences
```

## License

Code: MIT OR Apache-2.0. Generated model data is derived from Wiktionary
(via kaikki.org) and shared under CC BY-SA 4.0.
