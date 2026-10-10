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

## Languages

<!-- languages -->
20 languages, 1507186 dictionary entries, 67 MB of models.

| Language | Crate | Parts of speech | Entries | Cells | Model |
|---|---|---|---|---|---|
| Arabic | `arabic` | noun, adj, verb, pron, num | 26427 | 292 | 10.4 MB |
| Bulgarian | `bulgarian` | noun, adj, verb, pron, det, num | 14686 | 198 | 1.6 MB |
| Czech | `czech` | noun, adj, verb, pron, det, num | 36700 | 251 | 2.0 MB |
| Danish | `danish` | noun, adj, verb | 20561 | 25 | 0.8 MB |
| Dutch | `dutch` | noun, adj, verb, det, num | 55361 | 56 | 1.5 MB |
| English | `english` | noun, verb, adj, adv | 494089 | 19 | 4.0 MB |
| Finnish | `finnish` | noun, adj, verb, pron, num | 159153 | 162 | 11.3 MB |
| French | `french` | noun, adj, verb, pron, det, article, num | 75890 | 74 | 1.6 MB |
| German | `german` | noun, adj, verb, pron, det, article, num | 89293 | 265 | 4.0 MB |
| Greek | `greek` | noun, adj, verb, pron, num, article | 23376 | 189 | 2.7 MB |
| Hungarian | `hungarian` | noun, adj, verb, pron, num | 36549 | 213 | 2.7 MB |
| Icelandic | `icelandic` | noun, adj, verb | 15573 | 189 | 1.7 MB |
| Italian | `italian` | noun, adj, verb, pron, det, article, num | 109390 | 80 | 2.2 MB |
| Latin | `latin` | noun, adj, verb, pron, det, num | 34240 | 285 | 4.7 MB |
| Polish | `polish` | noun, adj, verb, pron, det, num | 68471 | 494 | 3.7 MB |
| Russian | `russian` | noun, adj, verb, pron, num | 51251 | 181 | 4.5 MB |
| Spanish | `spanish` | noun, adj, verb, pron, det, article, num | 103333 | 105 | 2.1 MB |
| Swedish | `swedish` | noun, adj, verb, pron, det, num | 51886 | 47 | 1.5 MB |
| Turkish | `turkish` | noun, verb, pron, num | 14938 | 266 | 1.6 MB |
| Ukrainian | `ukrainian` | noun, adj, verb, pron, det, num | 26019 | 490 | 2.5 MB |
<!-- /languages -->

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
