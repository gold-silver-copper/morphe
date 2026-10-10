use morphe_train::kaikki::Entry;
use morphe_train::spec::*;

pub struct Navajo;

impl Language for Navajo {
    fn spec(&self) -> Spec {
        let mut s = Spec::new("navajo", "Navajo", "nv");
        // The glottal stop is often typed as an apostrophe.
        s.fold = vec![('\'', 'ʼ'), ('’', 'ʼ')];
        s.features = vec![
            feature(
                "Mode",
                &[
                    ("Imperfective", "imperfective"),
                    ("Perfective", "perfective !distributive"),
                    ("DistributivePerfective", "perfective distributive"),
                    ("Future", "future"),
                    ("Progressive", "progressive"),
                    ("Iterative", "iterative"),
                    ("Optative", "optative"),
                    ("Neuter", "!imperfective !perfective !future !progressive !iterative !optative"),
                ],
            ),
            // Navajo distinguishes singular, dual and (distributive) plural;
            // the third person singular also serves the dual.
            feature(
                "Person",
                &[
                    ("FirstSingular", "first-person singular"),
                    ("FirstDual", "first-person duoplural"),
                    ("FirstPlural", "first-person plural"),
                    ("SecondSingular", "second-person singular"),
                    ("SecondDual", "second-person duoplural"),
                    ("SecondPlural", "second-person plural"),
                    ("Third", "third-person !plural"),
                    ("ThirdPlural", "third-person plural"),
                    ("Fourth", "fourth-person !plural"),
                    ("FourthPlural", "fourth-person plural"),
                    ("Indefinite", "indefinite-person"),
                    ("Spatial", "spatial-person"),
                ],
            ),
            feature(
                "Possessor",
                &[
                    ("FirstSingular", "first-person singular"),
                    ("SecondSingular", "second-person singular"),
                    ("Third", "third-person"),
                    ("Fourth", "fourth-person"),
                    ("FirstDual", "first-person duoplural"),
                    ("SecondDual", "second-person duoplural"),
                    ("FirstPlural", "first-person plural"),
                    ("SecondPlural", "second-person plural"),
                    ("Indefinite", "indefinite"),
                ],
            ),
        ];
        let possessed = || vec![group("Possessed", "", "Possessor")];
        s.pos = vec![
            pos("verb", "verb", vec![group("", "", "Mode Person")])
                .citation("ImperfectiveThird PerfectiveThird")
                .lemma_cell("ImperfectiveThird")
                .initial(3)
                .head_last(),
            pos("noun", "noun", possessed()).initial(3).head_last(),
            pos("postp", "postp", possessed()).initial(3),
        ];
        s
    }

    /// The headword line's possessed form is the third person's (bi-). The
    /// indefinite and spatial persons come without their table's mode.
    fn prepare(&self, e: &mut Entry) {
        const MODES: [&str; 6] = ["imperfective", "perfective", "future", "progressive", "iterative", "optative"];
        let mut mode: Option<&str> = None;
        for f in e.forms.iter_mut().filter(|f| f.source.is_some()) {
            if f.tags.iter().any(|t| t == "table-tags") {
                mode = None;
            } else if let Some(m) = MODES.iter().find(|m| f.tags.iter().any(|t| t == **m)) {
                mode = Some(m);
            } else if f.tags.iter().any(|t| t == "indefinite-person" || t == "spatial-person") {
                if let Some(m) = mode {
                    f.tags.push(m.into());
                }
            }
        }
        for f in e.forms.iter_mut().filter(|f| f.source.is_none()) {
            if f.tags.iter().any(|t| t == "possessed-form") && !f.tags.iter().any(|t| t == "third-person") {
                f.tags.push("third-person".into());
            }
        }
    }

    fn samples(&self) -> Vec<&'static str> {
        vec!["yitłʼó", "sido", "hataał", "his", "naaltsoos", "átágoʼ", "yiłbį́į́h"]
    }

    fn readme(&self) -> String {
        r#"## Example

```rust
use navajo::*;
println!("{}", verb("yitłʼó", Mode::Imperfective, Person::FirstSingular)); // yishtłʼó
println!("{}", verb("yitłʼó", Mode::Perfective, Person::SecondDual)); // sootłʼǫ́
println!("{}", verb("hataał", Mode::Future, Person::FirstSingular)); // hodeeshtał
println!("{}", noun_possessed("his", Possessor::FirstSingular)); // shihis
println!("{}", postp_possessed("átágoʼ", Possessor::Third)); // bítágoʼ
```

Verbs are cited by the imperfective third person and have the
imperfective, perfective, future, progressive (and, where Wiktionary gives
them, iterative and optative) for twelve persons: singular, dual and
plural first and second persons, the third and fourth (with their plurals),
the indefinite and the spatial person. Neuter verbs have person forms
without a mode. Nouns and postpositions have their possessed forms. The
tree reads the start of the word, where Navajo puts its prefixes; an
apostrophe stands for the glottal stop ʼ.
"#
        .into()
    }
}

pub fn language() -> Box<dyn Language> {
    Box::new(Navajo)
}
