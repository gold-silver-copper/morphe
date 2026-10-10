use morphe_train::spec::*;

pub struct Czech;

const CASES: &str = "Case[Nominative,Genitive,Dative,Accusative,Instrumental,Locative]";

fn has(tags: &[String], t: &str) -> bool {
    tags.iter().any(|x| x == t)
}

impl Language for Czech {
    fn spec(&self) -> Spec {
        let mut s = Spec::new("czech", "Czech", "cs");
        s.drop_tags.extend(tags("dated"));
        s.soft_tags.extend(tags("poetic literary"));
        s.features = vec![
            feature(
                "Case",
                &[
                    ("Nominative", "nominative"),
                    ("Genitive", "genitive"),
                    ("Dative", "dative"),
                    ("Accusative", "accusative"),
                    ("Instrumental", "instrumental"),
                    ("Locative", "locative"),
                    ("Vocative", "vocative"),
                ],
            ),
            feature("Number", &[("Singular", "singular"), ("Plural", "plural")]),
            // Czech splits the masculine by animacy (tags made by `retag`).
            feature(
                "Gender",
                &[
                    ("MasculineAnimate", "masculine-animate"),
                    ("MasculineInanimate", "masculine-inanimate"),
                    ("Feminine", "feminine"),
                    ("Neuter", "neuter"),
                ],
            ),
            feature("Aspect", &[("Imperfective", "imperfective"), ("Perfective", "perfective")]),
            feature("Person", &[("First", "first-person"), ("Second", "second-person"), ("Third", "third-person")]),
            feature("Voice", &[("Active", "active"), ("Passive", "passive")]),
            feature("Tense", &[("Present", "present"), ("Past", "past")]),
            feature(
                "Agreement",
                &[
                    ("MasculineSingular", "masculine singular"),
                    ("FeminineNeuterSingular", "feminine neuter singular"),
                    ("Plural", "plural"),
                ],
            ),
        ];
        let declined = format!("{CASES} Number Gender?");
        s.pos = vec![
            pos("noun", "noun", vec![group("", "", "Case Number")])
                .inherent("Gender")
                .citation("NominativeSingular GenitiveSingular")
                .head_last(),
            pos(
                "adj",
                "adj",
                vec![
                    group("", "!comparative !superlative", &declined),
                    group("Comparative", "comparative", ""),
                    group("Superlative", "superlative", ""),
                ],
            ),
            pos(
                "verb",
                "verb",
                vec![
                    group("Infinitive", "infinitive", ""),
                    group("Present", "indicative present", "Person Number"),
                    group("Imperative", "imperative second-person", "Number"),
                    group("ImperativeFirstPlural", "imperative first-person plural", ""),
                    group("Past", "participle past !transgressive", "Number Gender"),
                    group("Passive", "participle passive !adjectival", "Number Gender"),
                    group("Transgressive", "transgressive", "Tense Agreement"),
                    group("Participle", "adjectival participle", "Voice").declines("adj", ""),
                    group("VerbalNoun", "noun-from-verb", ""),
                ],
            )
            .inherent("Aspect")
            .citation("Infinitive PresentFirstSingular PastSingularMasculineAnimate"),
            pos("pron", "pron", vec![group("", "", "Case Number Gender?")]),
            pos("det", "det", vec![group("", "", &declined)]),
            pos("num", "num", vec![group("", "", &declined)]),
        ];
        s
    }

    /// Wiktionary's Czech tables merge columns, and the merged headers lose
    /// tags: a masculine that names no animacy is both; the plural column
    /// "masculine inanimate / feminine" of the nominative comes without
    /// "plural" (and the inanimate one without "masculine"); the accusative
    /// plural "masculine / feminine" comes as feminine only. Each masculine
    /// gets one tag per animacy, so that it weighs as much as "feminine".
    fn retag(&self, _pos: &PosSpec, t: &mut Vec<String>) {
        let push = |t: &mut Vec<String>, s: &str| {
            if !has(t, s) {
                t.push(s.into());
            }
        };
        if has(t, "nominative") && !has(t, "singular") && !has(t, "plural") {
            push(t, "plural");
            if has(t, "inanimate") {
                push(t, "masculine");
            }
        }
        if has(t, "accusative") && has(t, "feminine") && has(t, "plural") && !has(t, "masculine") {
            push(t, "masculine");
        }
        if has(t, "masculine") {
            let both = !has(t, "animate") && !has(t, "inanimate");
            if both || has(t, "animate") {
                push(t, "masculine-animate");
            }
            if both || has(t, "inanimate") {
                push(t, "masculine-inanimate");
            }
        }
    }

    /// The bookish infinitive in -ti/-ci (dělati, péci) is listed beside the
    /// plain one without a tag; keep it as a low-weight alternative.
    fn adjust(&self, pos: &PosSpec, _lemma: &str, forms: &mut Vec<(usize, String, bool)>) {
        if pos.name != "verb" {
            return;
        }
        let bookish = |f: &str| f.ends_with("ti") || f.ends_with("ci");
        if forms.iter().any(|(c, f, _)| *c == 0 && !bookish(f)) {
            for (c, f, soft) in forms.iter_mut() {
                if *c == 0 && bookish(f) {
                    *soft = true;
                }
            }
        }
    }

    fn readme(&self) -> String {
        r#"## Example

```rust
use czech::*;
println!("{}", noun("žena", Case::Instrumental, Number::Plural));
println!("{}", noun("hrad", Case::Locative, Number::Singular));
println!("{:?}", noun_gender("pes"));
println!("{}", verb_present("dělat", Person::Third, Number::Plural));
println!("{}", verb_past("být", Number::Plural, Gender::MasculineAnimate));
println!("{}", adj("mladý", Case::Genitive, Number::Singular, Gender::Feminine));
// Citations pin principal parts: nominative and genitive singular.
println!("{}", noun("pes, psa", Case::Dative, Number::Singular));
```

Input is matched as written: Czech diacritics (á, č, ě, ů…) are letters, not
optional marks, so they are not ignored.
"#
        .into()
    }
}

pub fn language() -> Box<dyn Language> {
    Box::new(Czech)
}
