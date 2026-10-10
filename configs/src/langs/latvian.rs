use morphe_train::kaikki::Entry;
use morphe_train::spec::*;

pub struct Latvian;

const CASES: &str = "Case[Nominative,Genitive,Dative,Accusative,Instrumental,Locative]";

impl Language for Latvian {
    fn spec(&self) -> Spec {
        let mut s = Spec::new("latvian", "Latvian", "lv");
        // Tone marks (laĩva, ar̂t) and the open-e cedilla (sȩns) appear only
        // in dictionary spellings.
        s.marks = vec!['\u{300}', '\u{302}', '\u{303}'];
        s.unmark = unmark("ãa ẽe ĩi õo ũu àa èe ìi òo ùu âa êe îi ôo ûu ȩe");
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
            feature("Gender", &[("Masculine", "masculine"), ("Feminine", "feminine")]),
            feature("Definiteness", &[("Indefinite", "indefinite"), ("Definite", "definite")]),
            feature("Tense", &[("Present", "present"), ("Past", "past"), ("Future", "future")]),
            feature(
                "PersonNumber",
                &[
                    ("FirstSingular", "first-person singular"),
                    ("SecondSingular", "second-person singular"),
                    ("Third", "third-person"),
                    ("FirstPlural", "first-person plural"),
                    ("SecondPlural", "second-person plural"),
                ],
            ),
            feature(
                "Participle",
                &[
                    ("PresentActive", "present active"),
                    ("PastActive", "past active"),
                    ("PresentPassive", "present passive"),
                    ("PastPassive", "past passive"),
                ],
            ),
        ];
        let adj_cells = &format!("Gender Number {CASES}");
        s.pos = vec![
            pos("noun", "noun", vec![group("", "", "Case Number")])
                .inherent("Gender")
                .citation("NominativeSingular GenitiveSingular")
                .head_last(),
            pos(
                "adj",
                "adj",
                vec![
                    group("", "indefinite", adj_cells),
                    group("Definite", "definite", adj_cells),
                    group("Comparative", "comparative", ""),
                    group("Superlative", "superlative", ""),
                    group("Adverb", "adverb", ""),
                ],
            )
            .citation("Lemma Comparative Superlative"),
            pos(
                "verb",
                "verb",
                vec![
                    group("", "indicative", "Tense PersonNumber"),
                    group("Imperative", "imperative", "PersonNumber[SecondSingular,SecondPlural]"),
                    group("Conditional", "conditional present !debitive", ""),
                    group("Renarrative", "renarrative", "Tense[Present,Future]"),
                    group("Debitive", "debitive indicative", ""),
                    group("Participle", "participle !dams", "Participle").declines("adj", ""),
                    group("Dams", "participle dams", ""),
                    group("Gerund", "gerund", ""),
                    group("VerbalNoun", "noun-from-verb", ""),
                ],
            )
            .citation("Lemma PresentFirstSingular PastFirstSingular")
            .head_first(),
            pos("pron", "pron", vec![group("", "", &format!("Gender Number {CASES}")), group("Personal", "!masculine !feminine !singular !plural", CASES)]),
            pos("num", "num", vec![group("", "!definite !indefinite", &format!("{CASES} Gender?"))]),
        ];
        s
    }

    /// Conjugation tables also list personal pronouns (es, tu, viņš) as
    /// cells, periphrastic forms (lai lasa, būtu lasījis) and untagged
    /// participles: drop the first two and name the participles by suffix.
    fn prepare(&self, e: &mut Entry) {
        if e.pos != "verb" {
            return;
        }
        e.forms.retain(|f| f.source.is_none() || !f.form.trim().contains(' '));
        for f in e.forms.iter_mut().filter(|f| f.source.as_deref() == Some("conjugation")) {
            let has = |t: &str| f.tags.iter().any(|x| x == t);
            let finite = ["present", "past", "future"].iter().any(|t| has(t));
            if has("indicative") && !finite && !has("debitive") {
                // A pronoun heading.
                f.tags = tags("table-tags");
            } else if has("indicative") && has("third-person") && has("plural") {
                // Third plural repeats the third singular.
                f.tags = tags("table-tags");
            } else if has("indicative") && has("third-person") {
                f.tags.retain(|t| t != "singular");
            } else if has("participle") && has("error-unrecognized-form") {
                let w = f.form.trim();
                let name = if w.ends_with("šs") {
                    "participle present active"
                } else if w.ends_with("dams") || w.ends_with("damies") {
                    "participle dams"
                } else if w.ends_with("ot") || w.ends_with("oties") {
                    "gerund"
                } else if w.ends_with("ams") || w.ends_with("āms") {
                    "participle present passive"
                } else {
                    "table-tags"
                };
                f.tags = tags(name);
            } else if has("participle") && has("past") {
                f.tags = tags(if has("passive") { "participle past passive" } else { "participle past active" });
            } else if has("noun-from-verb") {
                f.tags = tags("noun-from-verb");
            }
        }
    }

    /// "man (manim)": a form and its variant.
    fn split_form(&self, raw: &str) -> Vec<String> {
        raw.split(['(', ')']).map(|w| w.trim().to_string()).filter(|w| !w.is_empty()).collect()
    }

    fn samples(&self) -> Vec<&'static str> {
        vec!["māja", "galds", "lasīt", "būt", "labs", "es", "kulpēns", "spurdzēt"]
    }

    fn readme(&self) -> String {
        r#"## Example

```rust
use latvian::*;
assert_eq!(noun("māja", Case::Locative, Number::Plural), "mājās");
println!("{}", noun("galds", Case::Genitive, Number::Singular)); // galda
println!("{:?}", noun_gender("māja"));
println!("{}", adj("labs", Gender::Feminine, Number::Plural, Case::Dative)); // labām
println!("{}", adj_definite("labs", Gender::Masculine, Number::Singular, Case::Nominative)); // labais
println!("{}", verb("lasīt", Tense::Past, PersonNumber::FirstPlural)); // lasījām
println!("{}", verb_debitive("lasīt")); // jālasa
```

The third person is one cell, as Latvian uses the same form for singular and
plural. Periphrastic forms (`lai lasa`, `būtu lasījis`) are left out; the
participles decline as adjectives. Input matching ignores the tone marks
some dictionary spellings carry.
"#
        .into()
    }
}

pub fn language() -> Box<dyn Language> {
    Box::new(Latvian)
}
