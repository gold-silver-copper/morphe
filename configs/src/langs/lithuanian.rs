use morphe_train::kaikki::Entry;
use morphe_train::spec::*;

pub struct Lithuanian;

const CASES: &str = "Case[Nominative,Genitive,Dative,Accusative,Instrumental,Locative]";

impl Language for Lithuanian {
    fn spec(&self) -> Spec {
        let mut s = Spec::new("lithuanian", "Lithuanian", "lt");
        // Accent marks: acute, grave, tilde, and the dot that carries them on i.
        s.marks = vec!['\u{300}', '\u{301}', '\u{303}', '\u{307}'];
        s.unmark = unmark("áa àa ãa ée èe ẽe íi ìi ĩi óo òo õo úu ùu ũu ýy ỹy ñn ÁA ÀA ÉE ÈE ÍI ÌI ÓO ÒO ÚU ÙU ÝY");
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
            feature("Number", &[("Singular", "singular"), ("Plural", "plural"), ("Dual", "dual")]),
            feature("Gender", &[("Masculine", "masculine"), ("Feminine", "feminine")]),
            feature("Degree", &[("Positive", "positive"), ("Comparative", "comparative"), ("Superlative", "superlative")]),
            feature(
                "Tense",
                &[
                    ("Present", "present"),
                    ("Past", "past !frequentative"),
                    ("PastFrequentative", "past frequentative"),
                    ("Future", "future"),
                    ("Conditional", "conditional"),
                        ],
            ),
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
            feature("Voice", &[("Active", "active"), ("Passive", "passive")]),
            feature("ParticipleTense", &[("Present", "present"), ("Past", "past"), ("Future", "future")]),
        ];
        let adj_cells = "Gender Number[Singular,Plural] Case[Nominative,Genitive,Dative,Accusative,Instrumental,Locative]";
        s.pos = vec![
            pos("noun", "noun", vec![group("", "", "Case Number[Singular,Plural]")])
                .inherent("Gender")
                .citation("NominativeSingular GenitiveSingular")
                .head_last(),
            pos(
                "adj",
                "adj",
                vec![
                    group("", "positive !definite !neuter", adj_cells),
                    group("Comparative", "comparative !definite !neuter", adj_cells),
                    group("Superlative", "superlative !definite !neuter", adj_cells),
                    group("Definite", "positive definite !neuter", adj_cells),
                    group("Neuter", "neuter !definite", "Degree"),
                ],
            )
            .citation("Lemma FeminineSingularNominative"),
            pos(
                "verb",
                "verb",
                vec![
                    group("", "!participle !adverbial !imperative", "Tense[Present,Past,PastFrequentative,Future,Conditional] PersonNumber"),
                    group("Imperative", "imperative", "PersonNumber[SecondSingular,Third,FirstPlural,SecondPlural]"),
                    group("Participle", "participle !frequentative !necessitative", "Voice ParticipleTense").declines("adj", ""),
                    group("HalfParticiple", "adverbial special", ""),
                    group("Gerund", "adverbial half-participle !frequentative", "ParticipleTense"),
                    group("Budinys", "adverbial adverbial-manner", ""),
                ],
            )
            .citation("Lemma PresentThird PastThird")
            .head_first(),
            pos("pron", "pron", vec![group("", "", &format!("{CASES} Number Gender?"))]),
            pos("num", "num", vec![group("", "", &format!("{CASES} Gender?"))]),
        ];
        s
    }

    fn prepare(&self, e: &mut Entry) {
        match e.pos.as_str() {
            "verb" => retag_conjugation(e),
            "adj" => retag_adjective(e),
            _ => {}
        }
    }

    fn samples(&self) -> Vec<&'static str> {
        vec!["namas", "pupa", "geras", "rasti", "arti", "tu", "klumpelis", "spirgėti"]
    }

    fn readme(&self) -> String {
        r#"## Example

```rust
use lithuanian::*;
let plain = Lithuanian::default().marks(false);
assert_eq!(plain.noun("namas", Case::Locative, Number::Plural), "namuose");
println!("{}", noun("pupa", Case::Genitive, Number::Singular)); // pùpos
println!("{:?}", noun_gender("pupa"));
println!("{}", adj("geras", Gender::Feminine, Number::Plural, Case::Dative)); // geróms
println!("{}", adj_definite("geras", Gender::Masculine, Number::Singular, Case::Nominative)); // geràsis
println!("{}", verb("rasti", Tense::Past, PersonNumber::FirstSingular)); // radaũ
// Citations pin principal parts: infinitive, present and past third person.
println!("{}", verb("arti, ãria, ãrė", Tense::Future, PersonNumber::Third)); // ar̃s
```

Input matching ignores accent marks; output carries them where Wiktionary
has them (`marks(false)` removes them). The third person is one cell, as
Lithuanian uses the same form for singular and plural. Adjective cells cover
the plain forms in all three degrees, the definite (pronominal) positive
forms and the neuter; definite comparatives and superlatives are left out.
Wiktionary's verb tables are read by row, as their person labels are lost.
"#
        .into()
    }
}

/// Wiktionary's conjugation table loses most person labels. Each tense row
/// lists first singular, second singular (tagged), third, first plural
/// (long and short), second plural (tagged), third plural (tagged): read it
/// by row, in the order present, past, past frequentative, future,
/// conditional, imperative.
fn retag_conjugation(e: &mut Entry) {
    const ROWS: [&str; 6] = ["present", "past", "past frequentative", "future", "conditional", "imperative"];
    let mut table = 0;
    let mut row = 0usize;
    // Where in the row: 0 before the second singular, 1 in it, 2 after it,
    // 3 in the second plural, 4 in the third plural.
    let mut at = 0;
    for f in e.forms.iter_mut().filter(|f| f.source.is_some()) {
        let has = |t: &str| f.tags.iter().any(|x| x == t);
        if has("table-tags") {
            table += 1;
            continue;
        }
        if table != 1 || has("inflection-template") {
            continue;
        }
        let second = has("second-person");
        let plural = has("plural");
        let third_plural = has("third-person") && plural;
        if at == 4 && !third_plural {
            row += 1;
            at = 0;
        }
        let Some(tense) = ROWS.get(row) else {
            f.tags = tags("table-tags");
            continue;
        };
        let person = if third_plural {
            at = 4;
            "third-person"
        } else if second && plural {
            at = 3;
            "second-person plural"
        } else if second {
            at = 1;
            "second-person singular"
        } else if at == 0 {
            "first-person singular"
        } else {
            at = 2;
            let w = f.form.trim_end_matches(|c: char| !c.is_alphabetic());
            let plain: String = w.chars().filter(|c| c.is_alphabetic()).collect();
            if ["me", "m", "mės", "mė"].iter().any(|s| plain.ends_with(s)) && !plain.starts_with("te") {
                "first-person plural"
            } else {
                "third-person"
            }
        };
        // Third plural repeats the third person.
        f.tags = if third_plural { tags("table-tags") } else { tags(&format!("{tense} {person}")) };
    }
}

/// The declension table lists positive, comparative and superlative forms,
/// then the definite (pronominal) table, but drops the degree from
/// masculine singular forms and the gender from feminine superlatives.
fn retag_adjective(e: &mut Entry) {
    let mut table = 0;
    let mut degree = "positive";
    for f in e.forms.iter_mut().filter(|f| f.source.is_some()) {
        let has = |t: &str| f.tags.iter().any(|x| x == t);
        if has("table-tags") {
            table += 1;
            degree = "positive";
            continue;
        }
        if has("inflection-template") {
            continue;
        }
        for d in ["comparative", "superlative"] {
            if has(d) {
                degree = d;
            }
        }
        if has("positive") {
            degree = "positive";
        }
        let gendered = has("masculine") || has("feminine") || has("neuter");
        let mut t: Vec<String> = f.tags.iter().filter(|x| !["positive", "comparative", "superlative"].contains(&x.as_str())).cloned().collect();
        t.push(degree.into());
        if !gendered {
            t.push("feminine".into());
        }
        if table >= 2 {
            t.push("definite".into());
        }
        f.tags = t;
    }
}

pub fn language() -> Box<dyn Language> {
    Box::new(Lithuanian)
}
