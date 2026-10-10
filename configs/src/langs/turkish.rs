use morphe_train::kaikki::Entry;
use morphe_train::spec::*;

pub struct Turkish;

impl Language for Turkish {
    fn spec(&self) -> Spec {
        let mut s = Spec::new("turkish", "Turkish", "tr");
        // Circumflexes (kâr, hâlâ) are often left out.
        s.unmark = unmark("âa îi ûu ÂA ÎI ÛU");
        s.soft_tags.extend(tags("dated"));
        s.features = vec![
            feature(
                "Case",
                &[
                    ("Nominative", "nominative"),
                    ("Accusative", "accusative"),
                    ("Dative", "dative"),
                    ("Locative", "locative"),
                    ("Ablative", "ablative"),
                    ("Genitive", "genitive"),
                ],
            ),
            feature("Number", &[("Singular", "singular"), ("Plural", "plural")]),
            feature("Person", &[("First", "first-person"), ("Second", "second-person"), ("Third", "third-person")]),
            feature(
                "Possessor",
                &[
                    ("FirstSingular", "first-person singular"),
                    ("SecondSingular", "second-person singular"),
                    ("ThirdSingular", "third-person singular"),
                    ("FirstPlural", "first-person plural"),
                    ("SecondPlural", "second-person plural"),
                    ("ThirdPlural", "third-person plural"),
                ],
            ),
            feature("Possessed", &[("One", "possessed-single"), ("Many", "possessed-many")]),
            feature("Polarity", &[("Positive", "!negative"), ("Negative", "negative")]),
            feature(
                "Tense",
                &[
                    ("Aorist", "aorist"),
                    ("Continuative", "continuative"),
                    ("Inferential", "inferential"),
                    ("Future", "future"),
                    ("Progressive", "progressive"),
                    ("Necessitative", "necessitative"),
                    ("Past", "past"),
                    ("Conditional", "conditional"),
                    ("Optative", "optative"),
                ],
            ),
            feature(
                "Participle",
                &[("Imperfective", "imperfective"), ("Perfective", "perfective"), ("Prospective", "prospective")],
            ),
            feature("Relative", &[("NonProspective", "non-prospective"), ("Prospective", "prospective")]),
            feature("Converb", &[("Ken", "converb-ken"), ("Ince", "converb-ince"), ("Ip", "converb-ip"), ("Erek", "converb-erek")]),
        ];
        let nominal = || vec![group("", "!possessive", "Case Number")];
        let mut noun = nominal();
        noun.push(group("Possessive", "possessive", "Possessor Possessed Case"));
        s.pos = vec![
            pos("noun", "noun", noun).citation("NominativeSingular AccusativeSingular NominativePlural").head_last(),
            pos(
                "verb",
                "verb",
                vec![
                    group("Infinitive", "infinitive", "Polarity"),
                    group("", "finite", "Polarity Tense Person Number"),
                    group("Imperative", "imperative", "Polarity Person[Second,Third] Number"),
                    group("VerbalNoun", "noun-from-verb", "Polarity"),
                    group("Participle", "participle impersonal", "Polarity Participle"),
                    group("RelativeParticiple", "participle personal", "Polarity Relative Person Number"),
                    group("Converb", "converb", "Polarity Converb"),
                ],
            )
            .citation("InfinitivePositive PositiveAoristThirdSingular")
            .head_last(),
            pos("pron", "pron", nominal()),
            pos("num", "num", nominal()),
        ];
        s
    }

    /// Wiktionary's Turkish conjugation tables come out of the parser with
    /// the person labels shifted, but every table has the same 235 cells in
    /// the same order: positive, negative, then the potential mood (left
    /// out). Read the positive and negative tables by position.
    fn prepare(&self, e: &mut Entry) {
        if e.pos != "verb" {
            return;
        }
        const PERSONS: [&str; 6] = [
            "first-person singular",
            "second-person singular",
            "third-person singular",
            "first-person plural",
            "second-person plural",
            "third-person plural",
        ];
        // First row of each simple tense; the rows after it add copulas.
        const TENSES: [(usize, &str); 9] = [
            (0, "aorist"),
            (4, "continuative"),
            (8, "inferential"),
            (12, "future"),
            (16, "progressive"),
            (20, "necessitative"),
            (24, "past"),
            (27, "conditional"),
            (30, "optative"),
        ];
        const CONVERBS: [(usize, &str); 4] = [(226, "ken"), (227, "ince"), (228, "ip"), (232, "erek")];
        let mut table = 0usize;
        let mut k = 0usize;
        for f in e.forms.iter_mut().filter(|f| f.source.as_deref() == Some("conjugation")) {
            if f.tags.iter().any(|t| t == "table-tags") {
                table += 1;
                k = 0;
                f.tags = tags("table-tags");
                continue;
            }
            if f.tags.iter().any(|t| t == "inflection-template") {
                continue;
            }
            let pol = match table {
                1 => "",
                2 => "negative ",
                _ => {
                    f.tags = tags("table-tags");
                    continue;
                }
            };
            let person = PERSONS.get(k % 6).copied().unwrap_or("");
            let row = k / 6;
            let new = if k < 198 {
                TENSES.iter().find(|(r, _)| *r == row).map(|(_, t)| format!("finite {pol}{t} {person}"))
            } else if k < 204 {
                Some(format!("imperative {pol}{}", PERSONS.get(k - 198).copied().unwrap_or("")))
            } else {
                match k {
                    209 => Some(format!("infinitive {pol}")),
                    210 => Some(format!("noun-from-verb {pol}")),
                    211 => Some(format!("participle impersonal {pol}imperfective")),
                    212 => Some(format!("participle impersonal {pol}perfective")),
                    213 => Some(format!("participle impersonal {pol}prospective")),
                    214..=219 => Some(format!("participle personal {pol}non-prospective {}", PERSONS.get(k - 214).copied().unwrap_or(""))),
                    220..=225 => Some(format!("participle personal {pol}prospective {}", PERSONS.get(k - 220).copied().unwrap_or(""))),
                    _ => CONVERBS.iter().find(|(i, _)| *i == k).map(|(_, c)| format!("converb {pol}converb-{c}")),
                }
            };
            // Cells read by position replace the parser's tags; the rest
            // are dropped.
            f.tags = tags(&new.unwrap_or_else(|| "table-tags".into()));
            k += 1;
        }
    }

    fn samples(&self) -> Vec<&'static str> {
        vec!["ev", "kitap", "göz", "gelmek", "etmek", "okumak", "zıpırtmak", "fırtlak"]
    }

    fn readme(&self) -> String {
        r#"## Example

```rust
use turkish::*;
assert_eq!(noun("ev", Case::Locative, Number::Plural), "evlerde");
println!("{}", noun("kitap", Case::Dative, Number::Singular)); // kitaba
println!("{}", noun_possessive("göz", Possessor::FirstSingular, Possessed::Many, Case::Ablative)); // gözlerimden
println!("{}", verb("gelmek", Polarity::Positive, Tense::Continuative, Person::First, Number::Singular)); // geliyorum
println!("{}", verb("gitmek", Polarity::Negative, Tense::Past, Person::Third, Number::Plural)); // gitmediler
println!("{}", verb_relative_participle("okumak", Polarity::Positive, Relative::NonProspective, Person::First, Number::Singular)); // okuduğum
```

Cells cover the six cases in both numbers, the possessed forms in every
case, the nine simple tenses and moods in both polarities, the imperative,
the infinitive, the verbal noun, the participles and four converbs
(`-ken`, `-ince`, `-ip`, `-erek`). Wiktionary's verb tables list about 940
cells per verb (copula compounds such as `gelirdim`, and the potential
mood `gelebilmek`); those are left out. Input matching ignores circumflexes.
"#
        .into()
    }
}

pub fn language() -> Box<dyn Language> {
    Box::new(Turkish)
}
