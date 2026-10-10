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
158 languages, 3256367 dictionary entries, 70 MB of models.

| Language | Crate | Parts of speech | Entries | Cells | Model |
|---|---|---|---|---|---|
| Afrikaans | `afrikaans` | noun, adj, verb | 4682 | 11 | 0.1 MB |
| Albanian | `albanian` | noun, adj, verb | 8825 | 138 | 0.3 MB |
| Amharic | `amharic` | noun, adj | 420 | 31 | 0.0 MB |
| AncientGreek | `ancient_greek` | noun, adj, verb, pron, det, article, num | 14738 | 621 | 7.7 MB |
| Arabic | `arabic` | noun, adj, verb, pron, num | 26489 | 292 | 3.2 MB |
| Aragonese | `aragonese` | noun, adj, verb | 1098 | 55 | 0.0 MB |
| Aramaic | `aramaic` | noun, adj, verb | 1310 | 46 | 0.0 MB |
| Armenian | `armenian` | noun, adj, verb | 15447 | 148 | 0.4 MB |
| Aromanian | `aromanian` | noun, adj, verb | 3636 | 40 | 0.1 MB |
| Asturian | `asturian` | noun, adj, verb | 6845 | 62 | 0.2 MB |
| Azerbaijani | `azerbaijani` | noun, verb, pron | 8463 | 344 | 0.7 MB |
| Bashkir | `bashkir` | noun | 1640 | 12 | 0.0 MB |
| Basque | `basque` | noun, adj, verb, pron, num | 4214 | 276 | 0.2 MB |
| Belarusian | `belarusian` | noun, adj, verb, pron, det, num | 4746 | 424 | 0.3 MB |
| Bengali | `bengali` | noun, verb, pron | 6809 | 84 | 0.1 MB |
| Breton | `breton` | noun, adj, verb | 862 | 72 | 0.1 MB |
| Bulgarian | `bulgarian` | noun, adj, verb, pron, det, num | 14686 | 198 | 0.5 MB |
| Catalan | `catalan` | noun, adj, verb, pron, det, article | 30998 | 75 | 0.4 MB |
| Chechen | `chechen` | noun, num | 382 | 24 | 0.1 MB |
| ClassicalNahuatl | `classical_nahuatl` | noun, verb | 839 | 68 | 0.0 MB |
| ClassicalSyriac | `classical_syriac` | noun, adj | 2952 | 28 | 0.1 MB |
| Coptic | `coptic` | noun, verb | 960 | 104 | 0.2 MB |
| Cornish | `cornish` | noun, adj, verb | 5134 | 66 | 0.3 MB |
| Corsican | `corsican` | noun, adj, verb | 502 | 55 | 0.0 MB |
| CrimeanTatar | `crimean_tatar` | noun, verb | 2950 | 73 | 0.0 MB |
| Czech | `czech` | noun, adj, verb, pron, det, num | 57604 | 251 | 1.1 MB |
| Danish | `danish` | noun, adj, verb | 20561 | 25 | 0.3 MB |
| Dutch | `dutch` | noun, adj, verb, det, num | 175797 | 56 | 1.6 MB |
| EasternMari | `eastern_mari` | noun, adj, verb | 531 | 118 | 0.1 MB |
| Egyptian | `egyptian` | noun, adj, verb | 3066 | 48 | 0.1 MB |
| English | `english` | noun, verb, adj, adv | 493913 | 19 | 1.9 MB |
| Erzya | `erzya` | noun, noun_definite | 137 | 42 | 0.0 MB |
| Esperanto | `esperanto` | noun, adj, pron, verb | 32371 | 34 | 0.1 MB |
| Estonian | `estonian` | noun, adj, verb, pron, num | 5734 | 163 | 0.7 MB |
| Faroese | `faroese` | noun, adj, verb | 4905 | 68 | 0.3 MB |
| Finnish | `finnish` | noun, adj, verb, pron, num | 159178 | 162 | 3.7 MB |
| French | `french` | noun, adj, verb, pron, det, article, num | 326213 | 74 | 2.3 MB |
| Friulian | `friulian` | noun, adj, verb | 1172 | 53 | 0.0 MB |
| Gagauz | `gagauz` | noun, verb | 439 | 14 | 0.0 MB |
| Galician | `galician` | noun, adj, verb, pron, det, article, num | 16502 | 98 | 0.2 MB |
| Geez | `geez` | noun | 195 | 3 | 0.0 MB |
| Georgian | `georgian` | noun, adj, verb | 15111 | 137 | 0.6 MB |
| German | `german` | noun, adj, verb, pron, det, article, num | 182769 | 265 | 3.1 MB |
| Gothic | `gothic` | noun, adj, verb | 2970 | 140 | 0.2 MB |
| Greek | `greek` | noun, adj, verb, pron, num, article | 76509 | 189 | 1.6 MB |
| Greenlandic | `greenlandic` | noun | 643 | 15 | 0.0 MB |
| Gujarati | `gujarati` | noun, adj, verb, pron, det | 1639 | 67 | 0.0 MB |
| Hebrew | `hebrew` | noun, adj, verb | 8693 | 64 | 0.7 MB |
| Hindi | `hindi` | noun, adj, verb, pron, num | 15439 | 98 | 0.2 MB |
| Hittite | `hittite` | noun, verb | 196 | 56 | 0.1 MB |
| Hungarian | `hungarian` | noun, adj, verb, pron, num | 37040 | 213 | 1.0 MB |
| Icelandic | `icelandic` | noun, adj, verb | 16908 | 189 | 0.8 MB |
| InariSami | `inari_sami` | noun, verb | 387 | 69 | 0.1 MB |
| Indonesian | `indonesian` | verb | 1497 | 5 | 0.0 MB |
| Ingrian | `ingrian` | noun, adj, verb, pron, num | 7032 | 144 | 0.9 MB |
| Irish | `irish` | noun, adj, verb | 25153 | 88 | 0.7 MB |
| Italian | `italian` | noun, adj, verb, pron, det, article, num | 115186 | 80 | 0.9 MB |
| Kannada | `kannada` | noun, verb | 1287 | 70 | 0.1 MB |
| Karelian | `karelian` | noun, adj, pron, num | 757 | 112 | 0.1 MB |
| Kashubian | `kashubian` | noun, adj | 770 | 54 | 0.1 MB |
| Kazakh | `kazakh` | noun, verb | 4816 | 104 | 0.1 MB |
| KildinSami | `kildin_sami` | noun | 350 | 16 | 0.1 MB |
| Kyrgyz | `kyrgyz` | noun, verb | 1078 | 285 | 0.4 MB |
| Ladin | `ladin` | noun, adj, verb | 1087 | 40 | 0.0 MB |
| Ladino | `ladino` | noun, adj, verb | 452 | 64 | 0.0 MB |
| Latgalian | `latgalian` | noun, adj, pron, num | 383 | 126 | 0.0 MB |
| Latin | `latin` | noun, adj, verb, pron, det, num | 34245 | 285 | 1.8 MB |
| Latvian | `latvian` | noun, adj, verb, pron, num | 8576 | 135 | 0.2 MB |
| Limburgish | `limburgish` | noun, adj, verb | 961 | 85 | 0.0 MB |
| Lithuanian | `lithuanian` | noun, adj, verb, pron, num | 6698 | 201 | 0.5 MB |
| Livonian | `livonian` | noun, adj, verb, pron, num | 2229 | 106 | 0.2 MB |
| LowGerman | `low_german` | noun, adj, verb | 1073 | 121 | 0.1 MB |
| LowerSorbian | `lower_sorbian` | noun, adj, verb | 1339 | 102 | 0.1 MB |
| LuleSami | `lule_sami` | noun, adj, verb | 776 | 115 | 0.2 MB |
| Luxembourgish | `luxembourgish` | noun, adj, verb | 5483 | 37 | 0.1 MB |
| Macedonian | `macedonian` | noun, adj, verb | 44838 | 61 | 0.5 MB |
| Malay | `malay` | verb, adj | 1591 | 4 | 0.0 MB |
| Malayalam | `malayalam` | noun, verb, pron | 8483 | 33 | 0.1 MB |
| Maltese | `maltese` | noun, adj, verb | 8675 | 36 | 0.3 MB |
| Manx | `manx` | noun, adj, verb | 2012 | 30 | 0.0 MB |
| Marathi | `marathi` | noun, verb | 3620 | 67 | 0.1 MB |
| MiddleDutch | `middle_dutch` | noun, adj, verb | 882 | 71 | 0.1 MB |
| MiddleEnglish | `middle_english` | noun, adj, verb | 5353 | 25 | 0.1 MB |
| MiddleFrench | `middle_french` | noun, adj, verb | 3052 | 52 | 0.1 MB |
| MiddleHighGerman | `middle_high_german` | noun, adj, verb | 563 | 86 | 0.1 MB |
| MiddleIrish | `middle_irish` | noun, adj, verb | 673 | 56 | 0.0 MB |
| MiddleWelsh | `middle_welsh` | noun, adj, verb | 706 | 81 | 0.1 MB |
| Moksha | `moksha` | noun, noun_definite, verb | 411 | 140 | 0.0 MB |
| Mongolian | `mongolian` | noun | 435 | 39 | 0.1 MB |
| Navajo | `navajo` | verb, noun, postp | 17565 | 114 | 0.4 MB |
| Neapolitan | `neapolitan` | noun, adj, verb | 411 | 50 | 0.0 MB |
| Nepali | `nepali` | noun, adj, verb | 539 | 131 | 0.1 MB |
| NorthFrisian | `north_frisian` | noun, adj, verb | 788 | 44 | 0.0 MB |
| NorthernKurdish | `northern_kurdish` | noun | 630 | 16 | 0.0 MB |
| NorthernSami | `northern_sami` | noun, adj, verb, pron | 3699 | 125 | 0.9 MB |
| NorwegianBokmål | `norwegian_bokmal` | noun, adj, verb | 20642 | 17 | 0.3 MB |
| NorwegianNynorsk | `norwegian_nynorsk` | noun, adj, verb | 22409 | 17 | 0.3 MB |
| Occitan | `occitan` | noun, adj, verb | 3680 | 57 | 0.1 MB |
| OldArmenian | `old_armenian` | noun, adj, verb, pron, num | 4970 | 131 | 0.3 MB |
| OldChurchSlavonic | `old_church_slavonic` | noun, adj, verb, pron, num | 3054 | 301 | 0.2 MB |
| OldDutch | `old_dutch` | noun, adj, verb | 1638 | 69 | 0.1 MB |
| OldEastSlavic | `old_east_slavic` | noun, adj, verb | 462 | 168 | 0.1 MB |
| OldEnglish | `old_english` | noun, adj, verb, pron, det, num | 13859 | 193 | 0.6 MB |
| OldFrench | `old_french` | noun, adj, verb | 7329 | 64 | 0.6 MB |
| OldFrisian | `old_frisian` | noun, adj, verb, det, num | 264 | 85 | 0.0 MB |
| OldHighGerman | `old_high_german` | noun, adj, verb | 915 | 98 | 0.1 MB |
| OldIrish | `old_irish` | noun, adj, verb | 3863 | 272 | 0.4 MB |
| OldNorse | `old_norse` | noun, adj, verb | 2929 | 126 | 0.3 MB |
| OldOccitan | `old_occitan` | noun, adj | 415 | 7 | 0.0 MB |
| OldSaxon | `old_saxon` | noun, adj, verb | 1116 | 91 | 0.1 MB |
| OldSpanish | `old_spanish` | noun, adj, verb | 615 | 72 | 0.0 MB |
| Ossetian | `ossetian` | noun | 11 | 18 | 0.0 MB |
| OttomanTurkish | `ottoman_turkish` | noun | 310 | 84 | 0.0 MB |
| Pali | `pali` | noun, adj, verb, pron, num | 1991 | 244 | 0.1 MB |
| Pashto | `pashto` | noun, adj, verb | 591 | 50 | 0.0 MB |
| Persian | `persian` | noun, adj, verb | 4627 | 76 | 0.1 MB |
| Polish | `polish` | noun, adj, verb, pron, det, num | 106274 | 494 | 1.7 MB |
| Portuguese | `portuguese` | noun, adj, verb, pron, det, article, num | 64661 | 99 | 0.6 MB |
| Punjabi | `punjabi` | noun, adj, verb, pron | 2686 | 85 | 0.1 MB |
| Quechua | `quechua` | noun, verb | 1812 | 299 | 0.1 MB |
| Romanian | `romanian` | noun, adj, verb, pron, det, article, num | 83481 | 231 | 1.2 MB |
| Russian | `russian` | noun, adj, verb, pron, num | 351535 | 181 | 5.8 MB |
| Sanskrit | `sanskrit` | noun, adj, verb, pron, det, num | 8807 | 484 | 0.6 MB |
| Sardinian | `sardinian` | noun, adj, verb | 653 | 37 | 0.0 MB |
| SaterlandFrisian | `saterland_frisian` | noun, adj, verb, det | 663 | 33 | 0.0 MB |
| ScottishGaelic | `scottish_gaelic` | noun, adj, verb, prep, pron | 14562 | 93 | 0.3 MB |
| SerboCroatian | `serbo_croatian` | noun, adj, verb, pron, det, num | 43845 | 645 | 3.1 MB |
| Sicilian | `sicilian` | noun, adj, verb | 1561 | 54 | 0.0 MB |
| Silesian | `silesian` | noun, adj, verb | 877 | 105 | 0.1 MB |
| SkoltSami | `skolt_sami` | noun, verb | 361 | 58 | 0.1 MB |
| Slovak | `slovak` | noun, adj, verb, pron, det, num | 7744 | 393 | 0.2 MB |
| Slovene | `slovene` | noun, adj, verb, pron, det, num | 3764 | 374 | 0.3 MB |
| Somali | `somali` | noun, verb | 494 | 61 | 0.0 MB |
| Spanish | `spanish` | noun, adj, verb, pron, det, article, num | 132964 | 105 | 1.2 MB |
| Swahili | `swahili` | noun, verb | 9694 | 30 | 0.1 MB |
| Swedish | `swedish` | noun, adj, verb, pron, det, num | 51886 | 47 | 0.6 MB |
| Tagalog | `tagalog` | verb, adj | 2333 | 37 | 0.3 MB |
| Tamil | `tamil` | noun, verb, pron, num | 8078 | 142 | 0.2 MB |
| Tatar | `tatar` | noun | 174 | 84 | 0.1 MB |
| Telugu | `telugu` | noun, verb | 18379 | 54 | 0.1 MB |
| Tigrinya | `tigrinya` | noun, verb | 150 | 50 | 0.0 MB |
| Turkish | `turkish` | noun, verb, pron, num | 15307 | 266 | 0.5 MB |
| Turkmen | `turkmen` | noun, verb, pron | 1035 | 98 | 0.0 MB |
| Udmurt | `udmurt` | noun, verb | 896 | 84 | 0.0 MB |
| Ukrainian | `ukrainian` | noun, adj, verb, pron, det, num | 26023 | 490 | 0.9 MB |
| UpperSorbian | `upper_sorbian` | noun, adj, verb | 745 | 131 | 0.1 MB |
| Urdu | `urdu` | noun, adj, verb, pron, num, det | 2316 | 106 | 0.0 MB |
| Uyghur | `uyghur` | noun | 303 | 118 | 0.1 MB |
| Uzbek | `uzbek` | noun | 1414 | 84 | 0.0 MB |
| Venetan | `venetan` | noun, adj, verb | 2045 | 49 | 0.0 MB |
| Veps | `veps` | noun, adj, verb, pron, num | 1798 | 244 | 0.3 MB |
| Votic | `votic` | noun, adj, verb, pron, num | 2697 | 117 | 0.4 MB |
| Võro | `voro` | noun, adj, verb | 248 | 60 | 0.1 MB |
| Welsh | `welsh` | noun, adj, verb, prep, num, pron | 23112 | 91 | 0.9 MB |
| WestFrisian | `west_frisian` | noun, adj, verb | 2072 | 45 | 0.1 MB |
| Xhosa | `xhosa` | noun | 1472 | 1 | 0.0 MB |
| Yiddish | `yiddish` | noun, adj, verb | 6854 | 32 | 0.1 MB |
| Zulu | `zulu` | noun, verb | 2499 | 82 | 0.2 MB |
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
