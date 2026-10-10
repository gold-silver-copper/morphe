use morphe_train::kaikki::Entry;
use morphe_train::spec::*;

pub struct Quechua;

/// The declension tables after the first are the possessed forms, in
/// this order of possessors.
const POSSESSORS: [&str; 7] = ["x-p1s", "x-p2s", "x-p3s", "x-p1pi", "x-p1pe", "x-p2p", "x-p3p"];

impl Language for Quechua {
    fn spec(&self) -> Spec {
        let mut s = Spec::new("quechua", "Quechua", "qu");
        s.features = vec![
            feature(
                "Case",
                &[
                    ("Nominative", "nominative"),
                    ("Accusative", "accusative"),
                    ("Genitive", "genitive"),
                    ("Dative", "dative"),
                    ("Locative", "locative"),
                    ("Ablative", "ablative"),
                    ("Instrumental", "instrumental"),
                    ("Comitative", "comitative"),
                    ("Abessive", "abessive"),
                    ("Benefactive", "benefactive"),
                    ("Causative", "causative"),
                    ("Associative", "associative"),
                    ("Comparative", "comparative"),
                    ("Terminative", "terminative"),
                    ("Exclusive", "exclusive"),
                    ("Distributive", "distributive"),
                ],
            ),
            feature("Number", &[("Singular", "singular"), ("Plural", "plural")]),
            feature(
                "Possessor",
                &[
                    ("None", "!x-p1s !x-p2s !x-p3s !x-p1pi !x-p1pe !x-p2p !x-p3p"),
                    ("FirstSingular", "x-p1s"),
                    ("SecondSingular", "x-p2s"),
                    ("ThirdSingular", "x-p3s"),
                    ("FirstPluralInclusive", "x-p1pi"),
                    ("FirstPluralExclusive", "x-p1pe"),
                    ("SecondPlural", "x-p2p"),
                    ("ThirdPlural", "x-p3p"),
                ],
            ),
            feature(
                "Person",
                &[
                    ("FirstSingular", "first-person singular"),
                    ("SecondSingular", "second-person singular"),
                    ("ThirdSingular", "third-person singular"),
                    ("FirstPluralInclusive", "first-person plural inclusive"),
                    ("FirstPluralExclusive", "first-person plural exclusive"),
                    ("SecondPlural", "second-person plural"),
                    ("ThirdPlural", "third-person plural"),
                ],
            ),
            feature(
                "Tense",
                &[
                    ("Present", "present"),
                    ("Past", "past !x-reportative"),
                    ("ReportativePast", "past x-reportative"),
                    ("Future", "future"),
                ],
            ),
            feature(
                "ImperativePerson",
                &[
                    ("SecondSingular", "second-person singular"),
                    ("ThirdSingular", "third-person singular"),
                    ("FirstPluralInclusive", "first-person plural inclusive"),
                    ("SecondPlural", "second-person plural"),
                    ("ThirdPlural", "third-person plural"),
                ],
            ),
            feature("Participle", &[("Present", "present"), ("Past", "past"), ("Future", "future")]),
        ];
        s.pos = vec![
            pos("noun", "noun", vec![group("", "", "Possessor Case Number")])
                .citation("NoneNominativeSingular NoneNominativePlural")
                .head_last(),
            pos(
                "verb",
                "verb",
                vec![
                    group("Infinitive", "infinitive", ""),
                    group("Agentive", "agentive", ""),
                    group("Participle", "participle", "Participle"),
                    group("", "indicative", "Tense Person"),
                    group("Imperative", "imperative !negative", "ImperativePerson"),
                    group("NegativeImperative", "imperative negative", "ImperativePerson"),
                ],
            )
            .citation("Infinitive PresentFirstSingular")
            .head_first(),
        ];
        s
    }

    /// Possessed declension tables are told apart by their order; each
    /// table's two distributives are singular then plural. In verb tables
    /// the past runs twice (experienced -rqa, then reportative -sqa); the
    /// negative imperative's particle ama is split from its verb.
    fn prepare(&self, e: &mut Entry) {
        let mut table = 0usize;
        let mut distributive = 0usize;
        let mut past_block = 0usize;
        let mut out = Vec::new();
        for mut f in std::mem::take(&mut e.forms) {
            if f.source.is_some() && has(&f, "table-tags") {
                table += 1;
                distributive = 0;
            }
            if e.pos == "noun" && f.source.is_some() {
                if let Some(p) = table.checked_sub(2).and_then(|i| POSSESSORS.get(i)) {
                    f.tags.push((*p).into());
                }
                if has(&f, "distributive") {
                    f.tags.push(if distributive == 0 { "singular" } else { "plural" }.into());
                    distributive += 1;
                }
            }
            if e.pos == "verb" {
                if f.form == "ama" {
                    continue;
                }
                if has(&f, "past") && has(&f, "indicative") {
                    if has(&f, "first-person") && has(&f, "singular") {
                        past_block += 1;
                    }
                    if past_block > 1 {
                        f.tags.push("x-reportative".into());
                    }
                }
                if has(&f, "imperative") && has(&f, "negative") && !f.form.starts_with("ama ") {
                    f.form = format!("ama {}", f.form);
                }
            }
            out.push(f);
        }
        e.forms = out;
    }

    fn samples(&self) -> Vec<&'static str> {
        vec!["llama", "wasi", "runa", "may", "rimay", "mikhuy", "plutay", "chaqmay"]
    }

    fn readme(&self) -> String {
        r#"## Example

```rust
use quechua::*;
println!("{}", noun("wasi", Possessor::None, Case::Locative, Number::Plural)); // wasikunapi
println!("{}", noun("llama", Possessor::FirstSingular, Case::Nominative, Number::Singular)); // llamay
println!("{}", noun("llama", Possessor::FirstPluralInclusive, Case::Dative, Number::Singular)); // llamanchikman
println!("{}", verb("rimay", Tense::Present, Person::FirstSingular)); // rimani
println!("{}", verb("rimay", Tense::Past, Person::ThirdPlural)); // rimarqanku
println!("{}", verb_imperative("rimay", ImperativePerson::SecondPlural)); // rimaychik
```

Nouns have sixteen cases in both numbers, unpossessed and with each of
seven possessors (the first person plural is inclusive or exclusive).
Verbs have the infinitive, the agentive, three participles, the present,
experienced past (-rqa), reportative past (-sqa) and future for seven
persons, and the imperatives with their negatives (ama …-chu). The forms
are Southern Quechua (Cusco–Collao) as Wiktionary gives them.
"#
        .into()
    }
}

fn has(f: &morphe_train::kaikki::Form, t: &str) -> bool {
    f.tags.iter().any(|x| x == t)
}

pub fn language() -> Box<dyn Language> {
    Box::new(Quechua)
}
