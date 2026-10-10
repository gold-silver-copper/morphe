use morphe_train::kaikki::Entry;
use morphe_train::spec::*;

pub struct Maltese;

fn has(tags: &[String], t: &str) -> bool {
    tags.iter().any(|x| x == t)
}

impl Language for Maltese {
    fn spec(&self) -> Spec {
        let mut s = Spec::new("maltese", "Maltese", "mt");
        // Keyboards without Maltese letters write h, z, g, c.
        s.unmark = unmark("ħh żz ġg ċc àa ùu èe òo ìi");
        s.fold = vec![('’', '\'')];
        s.soft_tags.extend(tags("Australia Northern"));
        s.features = vec![
            feature("Gender", &[("Masculine", "masculine"), ("Feminine", "feminine")]),
            feature(
                "Possessor",
                &[
                    ("FirstSingular", "first-person !possessor-plural"),
                    ("SecondSingular", "second-person !possessor-plural"),
                    ("ThirdMasculine", "third-person masculine"),
                    ("ThirdFeminine", "third-person feminine"),
                    ("FirstPlural", "first-person possessor-plural"),
                    ("SecondPlural", "second-person possessor-plural"),
                    ("ThirdPlural", "third-person possessor-plural"),
                ],
            ),
            feature(
                "Form",
                &[
                    ("Feminine", "feminine singular"),
                    ("Plural", "plural"),
                    ("Comparative", "comparative"),
                    ("Diminutive", "diminutive"),
                ],
            ),
            feature("Tense", &[("Perfect", "perfect"), ("Imperfect", "imperfect")]),
            feature(
                "PersonNumber",
                &[
                    ("FirstSingular", "first-person singular"),
                    ("SecondSingular", "second-person singular"),
                    ("ThirdSingularMasculine", "third-person singular masculine"),
                    ("ThirdSingularFeminine", "third-person singular feminine"),
                    ("FirstPlural", "first-person plural"),
                    ("SecondPlural", "second-person plural"),
                    ("ThirdPlural", "third-person plural"),
                ],
            ),
            feature("Number", &[("Singular", "singular"), ("Plural", "plural")]),
        ];
        s.pos = vec![
            pos(
                "noun",
                "noun",
                vec![
                    group("Plural", "plural !possessive !construct", ""),
                    // bananiet: a few (2–10) bananas.
                    group("Paucal", "paucal", ""),
                    group("Dual", "dual", ""),
                    // banana: one of the collective banana.
                    group("Singulative", "singulative", ""),
                    group("Collective", "collective", ""),
                    group("Diminutive", "diminutive", ""),
                    // mart: "wife of", before a possessor.
                    group("Construct", "construct !plural", ""),
                    // arti, artek: my land, your land.
                    group("Possessive", "possessive", "Possessor"),
                ],
            )
            .inherent("Gender")
            .citation("Lemma Plural")
            .head_last(),
            pos("adj", "adj", vec![group("", "", "Form")]).citation("Lemma Feminine Plural"),
            pos(
                "verb",
                "verb",
                vec![
                    group("", "!imperative !participle", "Tense PersonNumber"),
                    group("Imperative", "imperative second-person", "Number"),
                    group("ParticiplePast", "participle past", ""),
                    group("VerbalNoun", "noun-from-verb", ""),
                ],
            )
            .citation("Lemma ImperfectThirdSingularMasculine")
            .head_first(),
        ];
        s
    }

    fn prepare(&self, e: &mut Entry) {
        // Conjugation tables follow each form with its negative (amart,
        // amartx): drop a form in -x that follows another with its tags.
        let mut firsts: Vec<Vec<String>> = Vec::new();
        e.forms.retain(|f| {
            if f.source.as_deref() != Some("conjugation") {
                return true;
            }
            let mut key = f.tags.clone();
            key.sort();
            if firsts.contains(&key) {
                return !f.form.ends_with('x');
            }
            firsts.push(key);
            true
        });
        let mut seen: Vec<&'static str> = Vec::new();
        for f in &mut e.forms {
            if f.source.is_none() {
                // The headword line's imperfect is the third person
                // masculine singular (jamar).
                if e.pos == "verb" && f.tags.len() == 1 && has(&f.tags, "imperfect") {
                    f.tags = tags("imperfect masculine singular third-person");
                }
                continue;
            }
            if e.pos != "noun" || f.source.as_deref() != Some("inflection") {
                continue;
            }
            // The possessive table names only the person; within it the
            // singular possessor comes first (arti "my"), then the plural
            // (artna "our"); a third person without gender is "their".
            let person = ["first-person", "second-person", "third-person"].into_iter().find(|p| has(&f.tags, p));
            let Some(p) = person else { continue };
            let plural = if p == "third-person" {
                !has(&f.tags, "masculine") && !has(&f.tags, "feminine")
            } else {
                seen.contains(&p)
            };
            seen.push(p);
            f.tags.retain(|t| t != "singular");
            f.tags.push("possessive".into());
            if plural {
                f.tags.push("possessor-plural".into());
            }
        }
    }

    fn samples(&self) -> Vec<&'static str> {
        vec!["art", "banana", "mara", "amar", "kiteb", "aħdar", "ferm", "flomar", "snorka"]
    }

    fn readme(&self) -> String {
        r#"## Example

```rust
use maltese::*;
println!("{}", noun_plural("art")); // artijiet
println!("{}", noun_possessive("art", Possessor::SecondSingular)); // artek
println!("{}", verb("amar", Tense::Imperfect, PersonNumber::FirstPlural)); // namru
println!("{}", adj("ferm", Form::Comparative)); // ifrem
```

Verbs are cited by their third person masculine singular perfect, as in
Wiktionary. Input may write h, z, g, c for ħ, ż, ġ, ċ.
"#
        .into()
    }
}

pub fn language() -> Box<dyn Language> {
    Box::new(Maltese)
}
