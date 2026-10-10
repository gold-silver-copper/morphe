use morphe_train::kaikki::{Entry, Form};
use morphe_train::spec::*;

pub struct LowerSorbian;

fn has(tags: &[String], t: &str) -> bool {
    tags.iter().any(|x| x == t)
}

const ADJ_CASES: &str = "Case[Nominative,Genitive,Dative,Accusative,Instrumental,Locative]";

impl Language for LowerSorbian {
    fn spec(&self) -> Spec {
        let mut s = Spec::new("lower_sorbian", "LowerSorbian", "dsb");
        s.unmark = unmark("łl ěe šs śs ćc čc źz žz óo ŕr ńn ḿm ṕp ẃw");
        s.soft_tags.extend(tags("poetic"));
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
                ],
            ),
            feature("Number", &[("Singular", "singular"), ("Dual", "dual"), ("Plural", "plural")]),
            feature("Gender", &[("Masculine", "masculine"), ("Feminine", "feminine"), ("Neuter", "neuter")]),
            feature("Animacy", &[("Animate", "animate"), ("Inanimate", "inanimate")]),
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
            feature("Degree", &[("Comparative", "comparative"), ("Superlative", "superlative")]),
            feature("Aspect", &[("Imperfective", "imperfective"), ("Perfective", "perfective")]),
            feature(
                "Tense",
                &[("Present", "present"), ("Preterite", "preterite")],
            ),
            feature(
                "PersonNumber",
                &[
                    ("FirstSingular", "first-person singular"),
                    ("SecondSingular", "second-person singular"),
                    ("ThirdSingular", "third-person singular"),
                    ("FirstDual", "first-person dual"),
                    ("SecondDual", "second-person dual"),
                    ("ThirdDual", "third-person dual"),
                    ("FirstPlural", "first-person plural"),
                    ("SecondPlural", "second-person plural"),
                    ("ThirdPlural", "third-person plural"),
                ],
            ),
        ];
        s.pos = vec![
            pos("noun", "noun", vec![group("", "", "Case Number"), group("Diminutive", "diminutive", "")])
                .inherent("Gender Animacy")
                .citation("NominativeSingular GenitiveSingular")
                .head_last(),
            pos(
                "adj",
                "adj",
                vec![
                    group("", "singular", &format!("{ADJ_CASES} SingularGender")),
                    group("Dual", "dual", &format!("{ADJ_CASES} Virility?")),
                    group("Plural", "plural", &format!("{ADJ_CASES} Virility?")),
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
                    group("", "!imperative !participle", "Tense PersonNumber"),
                    group("Imperative", "imperative", "PersonNumber"),
                    group("Supine", "supine", ""),
                    group("ParticiplePresent", "participle present", ""),
                    // sekł: the l-participle of the compound past.
                    group("LParticiple", "masculine singular", ""),
                    group("PassiveParticiple", "passive participle", ""),
                    group("VerbalNoun", "noun-from-verb", ""),
                ],
            )
            .inherent("Aspect")
            .citation("Infinitive PresentFirstSingular PresentThirdSingular")
            .head_first(),
        ];
        s
    }

    fn prepare(&self, e: &mut Entry) {
        if !e.forms.iter().any(|f| has(&f.tags, "canonical")) {
            e.forms.push(Form { form: e.word.clone(), tags: vec!["canonical".into()], source: None });
        }
        let nonfinite = ["infinitive", "supine", "noun-from-verb", "participle"];
        let mut bare = 0;
        for f in &mut e.forms {
            if f.source.is_none() {
                continue;
            }
            let err = has(&f.tags, "error-unrecognized-form");
            f.tags.retain(|t| t != "error-unrecognized-form");
            if e.pos == "adj" {
                // The nominative row has no case header.
                if err {
                    f.tags.push("nominative".into());
                }
                continue;
            }
            if e.pos != "verb" {
                continue;
            }
            // The non-finite rows sit under the imperative header.
            if nonfinite.iter().any(|t| has(&f.tags, t)) {
                f.tags.retain(|t| t != "imperative" && t != "plural" && t != "singular");
                // Bare participles: first the l-participle (pił, masculine
                // singular), then the passive one (pity).
                if has(&f.tags, "participle") && f.tags.len() == 1 {
                    bare += 1;
                    f.tags = tags(if bare == 1 { "masculine singular" } else { "participle passive" });
                } else if has(&f.tags, "active") {
                    f.tags = tags("masculine singular");
                }
            } else if has(&f.tags, "imperative") && !has(&f.tags, "second-person") {
                f.tags.push("second-person".into());
            }
        }
        // Some tables name only the third person. Unlabeled singulars: the
        // last of a run is the second person, the ones before it the
        // first; unlabeled duals and plurals are first (-mej, -my) or
        // second.
        let persons = ["first-person", "second-person", "third-person"];
        let unlabeled = |f: &Form| {
            f.source.is_some()
                && (has(&f.tags, "present") || has(&f.tags, "preterite"))
                && !persons.iter().any(|p| has(&f.tags, p))
        };
        let n = e.forms.len();
        let mut i = 0;
        while i < n {
            let Some(f) = e.forms.get(i) else { break };
            if !unlabeled(f) {
                i += 1;
                continue;
            }
            let key = f.tags.clone();
            let mut j = i;
            while e.forms.get(j).is_some_and(|g| unlabeled(g) && g.tags == key) {
                j += 1;
            }
            for k in i..j {
                let last = k + 1 == j;
                let Some(g) = e.forms.get_mut(k) else { continue };
                let p = if has(&g.tags, "singular") {
                    if last && j - i > 1 { "second-person" } else { "first-person" }
                } else if g.form.ends_with("mej") || g.form.ends_with("my") {
                    "first-person"
                } else {
                    "second-person"
                };
                g.tags.push(p.into());
            }
            i = j;
        }
    }

    fn keep_entry(&self, e: &Entry) -> bool {
        // L-participles have entries of their own (bił), declined like
        // adjectives; they are not verbs.
        !(e.pos == "verb" && e.word.ends_with('ł'))
    }

    fn samples(&self) -> Vec<&'static str> {
        vec!["słownik", "most", "góra", "sec", "piś", "sednuś", "polski", "dobry", "flompaś", "snorak"]
    }

    fn readme(&self) -> String {
        r#"## Example

```rust
use lower_sorbian::*;
println!("{}", noun("słownik", Case::Dative, Number::Dual)); // słownikoma
println!("{}", verb("sec", Tense::Preterite, PersonNumber::ThirdPlural)); // secechu
println!("{}", verb_l_participle("sec")); // sekł
println!("{}", adj("polski", Case::Genitive, SingularGender::Feminine)); // polskeje
println!("{}", adj_simple("dobry", Degree::Comparative)); // lěpšy
```

Lower Sorbian keeps the dual. Diacritics are optional on input.
"#
        .into()
    }
}

pub fn language() -> Box<dyn Language> {
    Box::new(LowerSorbian)
}
