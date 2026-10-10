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
72 languages, 2160300 dictionary entries, 159 MB of models.

| Language | Crate | Parts of speech | Entries | Cells | Model |
|---|---|---|---|---|---|
| Afrikaans | `afrikaans` | noun, adj, verb | 4682 | 11 | 0.1 MB |
| Albanian | `albanian` | noun, adj, verb | 8825 | 138 | 0.7 MB |
| AncientGreek | `ancient_greek` | noun, adj, verb, pron, det, article, num | 14738 | 621 | 27.6 MB |
| Arabic | `arabic` | noun, adj, verb, pron, num | 26427 | 292 | 15.6 MB |
| Armenian | `armenian` | noun, adj, verb | 15447 | 148 | 1.2 MB |
| Azerbaijani | `azerbaijani` | noun, verb, pron | 8463 | 344 | 1.9 MB |
| Bashkir | `bashkir` | noun | 1640 | 12 | 0.1 MB |
| Basque | `basque` | noun, adj, verb, pron, num | 4214 | 276 | 0.6 MB |
| Belarusian | `belarusian` | noun, adj, verb, pron, det, num | 4746 | 424 | 0.8 MB |
| Breton | `breton` | noun, adj, verb | 862 | 72 | 0.1 MB |
| Bulgarian | `bulgarian` | noun, adj, verb, pron, det, num | 14686 | 198 | 1.6 MB |
| Catalan | `catalan` | noun, adj, verb, pron, det, article | 30998 | 75 | 0.8 MB |
| Czech | `czech` | noun, adj, verb, pron, det, num | 36701 | 251 | 2.0 MB |
| Danish | `danish` | noun, adj, verb | 20561 | 25 | 0.8 MB |
| Dutch | `dutch` | noun, adj, verb, det, num | 55361 | 56 | 1.6 MB |
| English | `english` | noun, verb, adj, adv | 494089 | 19 | 4.0 MB |
| Esperanto | `esperanto` | noun, adj, pron, verb | 32371 | 34 | 0.3 MB |
| Estonian | `estonian` | noun, adj, verb, pron, num | 5734 | 163 | 1.6 MB |
| Faroese | `faroese` | noun, adj, verb | 4905 | 68 | 0.6 MB |
| Finnish | `finnish` | noun, adj, verb, pron, num | 159178 | 162 | 11.3 MB |
| French | `french` | noun, adj, verb, pron, det, article, num | 75890 | 74 | 1.6 MB |
| Galician | `galician` | noun, adj, verb, pron, det, article, num | 16502 | 98 | 0.5 MB |
| Georgian | `georgian` | noun, adj, verb | 15111 | 137 | 2.2 MB |
| German | `german` | noun, adj, verb, pron, det, article, num | 89293 | 265 | 5.2 MB |
| Gothic | `gothic` | noun, adj, verb | 2970 | 140 | 0.7 MB |
| Greek | `greek` | noun, adj, verb, pron, num, article | 23376 | 189 | 2.7 MB |
| Hebrew | `hebrew` | noun, adj, verb | 8693 | 64 | 3.3 MB |
| Hindi | `hindi` | noun, adj, verb, pron, num | 15439 | 98 | 0.8 MB |
| Hungarian | `hungarian` | noun, adj, verb, pron, num | 36549 | 213 | 2.7 MB |
| Icelandic | `icelandic` | noun, adj, verb | 15573 | 189 | 1.8 MB |
| Irish | `irish` | noun, adj, verb | 25153 | 88 | 1.8 MB |
| Italian | `italian` | noun, adj, verb, pron, det, article, num | 109390 | 80 | 2.3 MB |
| Kazakh | `kazakh` | noun, verb | 4816 | 104 | 0.3 MB |
| Latin | `latin` | noun, adj, verb, pron, det, num | 34245 | 285 | 4.8 MB |
| Latvian | `latvian` | noun, adj, verb, pron, num | 8576 | 135 | 0.5 MB |
| Lithuanian | `lithuanian` | noun, adj, verb, pron, num | 6698 | 201 | 1.8 MB |
| Luxembourgish | `luxembourgish` | noun, adj, verb | 5483 | 37 | 0.2 MB |
| Macedonian | `macedonian` | noun, adj, verb | 44838 | 61 | 1.5 MB |
| MiddleEnglish | `middle_english` | noun, adj, verb | 5353 | 25 | 0.3 MB |
| MiddleFrench | `middle_french` | noun, adj, verb | 3052 | 52 | 0.2 MB |
| MiddleHighGerman | `middle_high_german` | noun, adj, verb | 563 | 86 | 0.2 MB |
| NorwegianBokmål | `norwegian_bokmal` | noun, adj, verb | 20642 | 17 | 0.6 MB |
| NorwegianNynorsk | `norwegian_nynorsk` | noun, adj, verb | 22409 | 17 | 0.7 MB |
| Occitan | `occitan` | noun, adj, verb | 3680 | 57 | 0.3 MB |
| OldChurchSlavonic | `old_church_slavonic` | noun, adj, verb, pron, num | 3054 | 301 | 0.5 MB |
| OldEnglish | `old_english` | noun, adj, verb, pron, det, num | 13853 | 178 | 1.3 MB |
| OldFrench | `old_french` | noun, adj, verb | 6781 | 64 | 1.3 MB |
| OldHighGerman | `old_high_german` | noun, adj, verb | 915 | 98 | 0.2 MB |
| OldIrish | `old_irish` | noun, adj, verb | 3863 | 272 | 1.2 MB |
| OldNorse | `old_norse` | noun, adj, verb | 2929 | 126 | 0.8 MB |
| OldSaxon | `old_saxon` | noun, adj, verb | 1117 | 89 | 0.3 MB |
| Pali | `pali` | noun, adj, verb, pron, num | 1991 | 244 | 0.5 MB |
| Persian | `persian` | noun, adj, verb | 4627 | 76 | 0.4 MB |
| Polish | `polish` | noun, adj, verb, pron, det, num | 68473 | 494 | 3.8 MB |
| Portuguese | `portuguese` | noun, adj, verb, pron, det, article, num | 61465 | 99 | 1.4 MB |
| Romanian | `romanian` | noun, adj, verb, pron, det, article, num | 83481 | 231 | 2.6 MB |
| Russian | `russian` | noun, adj, verb, pron, num | 51251 | 181 | 4.8 MB |
| Sanskrit | `sanskrit` | noun, adj, verb, pron, det, num | 8807 | 484 | 1.8 MB |
| ScottishGaelic | `scottish_gaelic` | noun, adj, verb, prep, pron | 7912 | 92 | 0.7 MB |
| SerboCroatian | `serbo_croatian` | noun, adj, verb, pron, det, num | 43845 | 645 | 14.6 MB |
| Slovak | `slovak` | noun, adj, verb, pron, det, num | 7744 | 393 | 0.5 MB |
| Slovene | `slovene` | noun, adj, verb, pron, det, num | 3764 | 374 | 1.0 MB |
| Spanish | `spanish` | noun, adj, verb, pron, det, article, num | 103333 | 105 | 2.1 MB |
| Swahili | `swahili` | noun, verb | 9694 | 30 | 0.3 MB |
| Swedish | `swedish` | noun, adj, verb, pron, det, num | 51886 | 47 | 1.5 MB |
| Tamil | `tamil` | noun, verb, pron, num | 8078 | 142 | 0.7 MB |
| Tatar | `tatar` | noun | 174 | 84 | 0.1 MB |
| Turkish | `turkish` | noun, verb, pron, num | 14938 | 266 | 1.6 MB |
| Ukrainian | `ukrainian` | noun, adj, verb, pron, det, num | 26023 | 490 | 2.5 MB |
| Uzbek | `uzbek` | noun | 1414 | 84 | 0.2 MB |
| Welsh | `welsh` | noun, adj, verb, prep, num, pron | 23112 | 91 | 2.0 MB |
| Yiddish | `yiddish` | noun, adj, verb | 6854 | 32 | 0.4 MB |
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

## Using a crate

The crates aren't on crates.io yet; use them from git:

```toml
[dependencies]
russian = { git = "https://github.com/gold-silver-copper/morphe" }
```

Every crate has the same shape: typed functions per part of speech and
cell group (`noun`, `verb_past`, …), `_scored` versions with confidences,
`<pos>_<feature>` for inherent features (`noun_gender`, `verb_aspect`,
`verb_auxiliary`), `<pos>_lemma` for the dictionary spelling, `analyze`
for form → lemma and cells, and `inflect(pos, cell, word)` by name.

## Adding a language

Write `configs/src/langs/<name>.rs` (features, cells per part of speech,
spelling rules, a few hooks for the dump's quirks), drop the Kaikki dump
in `~/.cache/kaikki/`, run `morphe train <name>`. `morphe tags <Title>`
shows the tag sets to write the config from; `prompts/add-languages.md`
is the full guide the language agents followed.

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
