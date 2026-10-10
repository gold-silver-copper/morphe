use morphe_train::kaikki::{Entry, Form};
use morphe_train::spec::*;

pub struct Silesian;

fn has(tags: &[String], t: &str) -> bool {
    tags.iter().any(|x| x == t)
}

impl Language for Silesian {
    fn spec(&self) -> Spec {
        let mut s = Spec::new("silesian", "Silesian", "szl");
        s.unmark = unmark("łl ōo śs żz ŏo ćc ãa ôo ńn õo źz ůu");
        s.soft_tags.extend(tags("simplified"));
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
                    ("Vocative", "vocative !nominative"),
                ],
            ),
            feature("Number", &[("Singular", "singular"), ("Plural", "plural")]),
            feature("Gender", &[("Masculine", "masculine"), ("Feminine", "feminine"), ("Neuter", "neuter")]),
            feature("Animacy", &[("Personal", "person"), ("Animal", "animal-not-person"), ("Inanimate", "inanimate")]),
            feature(
                "SingularGender",
                &[
                    ("Masculine", "masculine !animate"),
                    ("MasculineAnimate", "animate"),
                    ("Neuter", "neuter"),
                    ("Feminine", "feminine"),
                ],
            ),
            feature("Virility", &[("Virile", "virile"), ("Nonvirile", "nonvirile")]),
            feature(
                "GenderNumber",
                &[
                    ("Masculine", "masculine singular"),
                    ("Feminine", "feminine singular"),
                    ("Neuter", "neuter singular"),
                    ("Virile", "plural virile"),
                    ("Nonvirile", "plural nonvirile"),
                ],
            ),
            feature("Degree", &[("Comparative", "comparative"), ("Superlative", "superlative")]),
            feature("Aspect", &[("Imperfective", "imperfective"), ("Perfective", "perfective")]),
            feature("Person", &[("First", "first-person"), ("Second", "second-person"), ("Third", "third-person")]),
            feature(
                "PersonNumber",
                &[
                    ("FirstSingular", "first-person singular"),
                    ("SecondSingular", "second-person singular"),
                    ("ThirdSingular", "third-person singular"),
                    ("FirstPlural", "first-person plural"),
                    ("SecondPlural", "second-person plural"),
                    ("ThirdPlural", "third-person plural"),
                ],
            ),
        ];
        let cases = "Case";
        s.pos = vec![
            pos(
                "noun",
                "noun",
                vec![
                    group("", "", "Case Number"),
                    group("Diminutive", "diminutive", ""),
                    group("Augmentative", "augmentative", ""),
                ],
            )
            .inherent("Gender Animacy")
            .citation("NominativeSingular GenitiveSingular")
            .head_last(),
            pos(
                "adj",
                "adj",
                vec![
                    group("", "singular", &format!("{cases} SingularGender")),
                    group("Plural", "plural", &format!("{cases} Virility?")),
                    // The headword line's comparative and superlative.
                    group("Simple", "", "Degree"),
                ],
            )
            .citation("NominativeMasculine NominativeFeminine NominativeNeuter"),
            pos(
                "verb",
                "verb",
                vec![
                    group("Infinitive", "infinitive", ""),
                    group("Present", "present", "PersonNumber"),
                    group("Past", "past", "Person GenderNumber"),
                    // Only "być" has a simple future (bydã); the others' is
                    // periphrastic.
                    group("Future", "future", "PersonNumber"),
                    group("Imperative", "imperative", "PersonNumber"),
                    group("ActiveParticiple", "active participle", "GenderNumber"),
                    group("PassiveParticiple", "passive participle", "GenderNumber"),
                    group("VerbalNoun", "noun-from-verb", ""),
                ],
            )
            .inherent("Aspect")
            .citation("Infinitive PresentFirstSingular PastThirdMasculine")
            .head_first(),
        ];
        s
    }

    fn prepare(&self, e: &mut Entry) {
        if !e.forms.iter().any(|f| has(&f.tags, "canonical")) {
            e.forms.push(Form { form: e.word.clone(), tags: vec!["canonical".into()], source: None });
        }
        if e.pos == "adj" {
            // Oblique cases name only the neuter for the shared
            // masculine-neuter column.
            let mut extra = Vec::new();
            for case in ["genitive", "dative", "instrumental", "locative"] {
                let masc = e.forms.iter().any(|f| f.source.is_some() && has(&f.tags, case) && has(&f.tags, "masculine") && has(&f.tags, "singular"));
                if masc {
                    continue;
                }
                for f in e.forms.iter().filter(|f| f.source.is_some() && has(&f.tags, case) && has(&f.tags, "neuter") && has(&f.tags, "singular")) {
                    let mut g = f.clone();
                    g.tags.retain(|t| t != "neuter");
                    g.tags.push("masculine".into());
                    extra.push(g);
                }
            }
            e.forms.extend(extra);
        }
        for f in &mut e.forms {
            if f.source.is_none() || e.pos != "verb" {
                continue;
            }
            let t = &f.tags;
            let personal = ["present", "past", "future", "imperative"].iter().any(|x| has(t, x));
            if !personal || ["first-person", "second-person", "third-person"].iter().any(|x| has(t, x)) {
                continue;
            }
            // The first and second persons are told apart only by their
            // endings: -sz, -ś, -cie for the second person.
            let w = f.form.as_str();
            let second = w.ends_with("sz") || w.ends_with('ś') || w.ends_with("cie")
                || (has(t, "imperative") && has(t, "singular"));
            f.tags.push(if second { "second-person" } else { "first-person" }.into());
        }
    }

    fn split_form(&self, raw: &str) -> Vec<String> {
        // Periphrastic forms (bydã czekoł, bych czekała, niych czekōm, the
        // "czekoł żech" variants) are left out.
        if raw.contains(' ') {
            return Vec::new();
        }
        vec![raw.to_string()]
    }

    fn samples(&self) -> Vec<&'static str> {
        vec!["pies", "krowa", "czekać", "być", "pisać", "ruby", "prosty", "flompać", "snorek"]
    }

    fn readme(&self) -> String {
        r#"## Example

```rust
use silesian::*;
println!("{}", noun("pies", Case::Instrumental, Number::Singular)); // psym
println!("{}", verb_present("czekać", PersonNumber::SecondSingular)); // czekŏsz
println!("{}", verb_past("czekać", Person::First, GenderNumber::Feminine)); // czekałach
println!("{}", adj("ruby", Case::Nominative, SingularGender::Feminine)); // rubŏ
```

Periphrastic forms (future, conditional, the "niych" imperative) are left
out. Diacritics are optional on input.
"#
        .into()
    }
}

pub fn language() -> Box<dyn Language> {
    Box::new(Silesian)
}
