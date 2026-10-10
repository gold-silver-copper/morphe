use morphe_train::kaikki::Entry;
use morphe_train::spec::*;

pub struct ClassicalNahuatl;

impl Language for ClassicalNahuatl {
    fn spec(&self) -> Spec {
        let mut s = Spec::new("classical_nahuatl", "ClassicalNahuatl", "nci");
        // Vowel length is marked in dictionaries, rarely in texts.
        s.unmark = unmark("āa ēe īi ōo ĀA ĒE ĪI ŌO");
        s.features = vec![
            feature("Animacy", &[("Animate", "animate"), ("Inanimate", "inanimate")]),
            feature(
                "Possessor",
                &[
                    ("FirstSingular", "first-person singular"),
                    ("SecondSingular", "second-person singular"),
                    ("ThirdSingular", "third-person singular"),
                    ("FirstPlural", "first-person plural"),
                    ("SecondPlural", "second-person plural"),
                    ("ThirdPlural", "third-person plural"),
                    ("Impersonal", "impersonal"),
                ],
            ),
            feature(
                "Person",
                &[
                    ("FirstSingular", "first-person singular"),
                    ("SecondSingular", "second-person singular"),
                    ("ThirdSingular", "third-person singular"),
                    ("FirstPlural", "first-person plural"),
                    ("SecondPlural", "second-person plural"),
                    ("ThirdPlural", "third-person plural"),
                ],
            ),
            feature(
                "Tense",
                &[
                    ("Present", "x-present"),
                    ("Customary", "x-customary"),
                    ("Imperfect", "x-imperfect"),
                    ("Preterite", "x-preterite"),
                    ("Pluperfect", "x-pluperfect"),
                    ("Future", "x-future"),
                    ("NearFuture", "x-near-future"),
                    ("Conditional", "x-conditional"),
                    ("Optative", "x-optative"),
                    ("PastOptative", "x-past-optative"),
                ],
            ),
        ];
        s.pos = vec![
            pos(
                "noun",
                "noun",
                vec![group("Plural", "plural !possessive", ""), group("Possessed", "possessive", "Possessor")],
            )
            .inherent("Animacy")
            .citation("Lemma Plural")
            .head_last(),
            pos("verb", "verb", vec![group("", "x-finite", "Tense Person")])
                .citation("PresentThirdSingular PreteriteThirdSingular")
                .lemma_cell("PresentThirdSingular")
                .head_last(),
        ];
        s
    }

    /// The conjugation table tags tenses only in part: untagged rows are
    /// told apart by their order (customary before the preterite,
    /// pluperfect after it; the conditional is the first row with no tense
    /// at all). Forms with the optional antecessive ō- are left out.
    fn prepare(&self, e: &mut Entry) {
        if e.pos != "verb" {
            return;
        }
        let (mut preterite_seen, mut bare_rows, mut last_bare) = (false, 0usize, false);
        let mut previous = String::new();
        for f in e.forms.iter_mut().filter(|f| f.source.is_some()) {
            let has = |t: &str| f.tags.iter().any(|x| x == t);
            if !has("indicative") {
                continue;
            }
            let augmented = f.form.strip_prefix('ō').is_some_and(|rest| rest == previous);
            previous = f.form.clone();
            let error = has("error-unrecognized-form");
            let bare = !error && !["present", "imperfective", "preterite", "future", "non-past", "past"].iter().any(|t| has(t));
            if bare && !last_bare {
                bare_rows += 1;
            }
            last_bare = bare;
            let tense = if augmented {
                None
            } else if error {
                Some(if preterite_seen { "x-pluperfect" } else { "x-customary" })
            } else if has("present") {
                Some("x-present")
            } else if has("imperfective") {
                Some("x-imperfect")
            } else if has("preterite") {
                preterite_seen = true;
                Some("x-preterite")
            } else if has("future-near") {
                Some("x-near-future")
            } else if has("future") {
                Some("x-future")
            } else if has("non-past") {
                Some("x-optative")
            } else if has("past") {
                Some("x-past-optative")
            } else if bare && bare_rows == 1 {
                Some("x-conditional")
            } else {
                None
            };
            let person: Vec<String> = f.tags.iter().filter(|t| t.ends_with("-person") || *t == "singular" || *t == "plural").cloned().collect();
            f.tags = match tense {
                Some(t) => tags(&format!("x-finite {t} {}", person.join(" "))),
                None => tags("table-tags"),
            };
        }
    }

    fn samples(&self) -> Vec<&'static str> {
        vec!["ami", "milli", "colli", "chichi", "pochtecatl", "tlacua", "pāqui"]
    }

    fn readme(&self) -> String {
        r#"## Example

```rust
use classical_nahuatl::*;
println!("{}", noun_plural("chichi")); // chichimeh
println!("{}", noun_possessed("milli", Possessor::FirstSingular)); // nomīl
println!("{:?}", noun_animacy("pochtecatl")); // Animate
println!("{}", verb("ami", Tense::Preterite, Person::FirstSingular)); // nān
println!("{}", verb("ami", Tense::Future, Person::ThirdPlural)); // āmizqueh
```

Nouns have the plural, the possessed forms for six persons and the
impersonal possessor (tē-), and their animacy. Verbs have the present,
customary, imperfect, preterite, pluperfect, future, near future,
conditional and the optatives for six persons. Input matching ignores
vowel length marks.
"#
        .into()
    }
}

pub fn language() -> Box<dyn Language> {
    Box::new(ClassicalNahuatl)
}
