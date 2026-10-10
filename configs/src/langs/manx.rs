use morphe_train::kaikki::Entry;
use morphe_train::spec::*;

pub struct Manx;

impl Language for Manx {
    fn spec(&self) -> Spec {
        let mut s = Spec::new("manx", "Manx", "gv");
        s.unmark = unmark("çc íi ïi óo âa êe");
        s.fold = vec![('’', '\'')];
        // Negative forms on the headword line ("cha yarg").
        s.drop_tags.extend(tags("negative"));
        s.features = vec![
            feature("Gender", &[("Masculine", "masculine"), ("Feminine", "feminine")]),
            feature("Mutation", &[("Lenition", "lenition"), ("Eclipsis", "eclipsis")]),
            feature("Clause", &[("Independent", "independent"), ("Dependent", "dependent")]),
            feature(
                "Subject",
                &[
                    ("FirstSingular", "first-person singular"),
                    ("FirstPlural", "first-person plural"),
                    // The analytic form, used with a subject pronoun or noun.
                    ("Analytic", "!first-person"),
                ],
            ),
            feature("Number", &[("Singular", "singular"), ("Plural", "plural")]),
        ];
        let mutated = || group("Mutated", "mutation", "Mutation");
        s.pos = vec![
            pos(
                "noun",
                "noun",
                vec![group("GenitiveSingular", "genitive singular", ""), group("Plural", "plural", ""), mutated()],
            )
            .inherent("Gender")
            .citation("Lemma GenitiveSingular Plural")
            .initial(2)
            .head_first(),
            pos(
                "adj",
                "adj",
                vec![group("Plural", "plural", ""), group("Comparative", "comparative", ""), mutated()],
            )
            .citation("Lemma Comparative")
            .initial(2),
            pos(
                "verb",
                "verb",
                vec![
                    group("VerbalNoun", "noun-from-verb", ""),
                    group("ParticiplePast", "participle !present", ""),
                    // Only "ve" has a present: ta, vel.
                    group("Present", "present !participle", "Clause"),
                    group("Past", "past !participle", "Clause?"),
                    group("Future", "future !relative", "Clause? Subject"),
                    group("Conditional", "conditional", "Clause? Subject"),
                    group("Imperative", "imperative", "Number?"),
                    mutated(),
                ],
            )
            .citation("Lemma VerbalNoun FutureIndependentAnalytic PastIndependent")
            .initial(2)
            .head_first(),
        ];
        s
    }

    fn prepare(&self, e: &mut Entry) {
        for f in &mut e.forms {
            // The comparative comes with its particle: "s’buiggey".
            if let Some(rest) = f.form.strip_prefix("s’").or_else(|| f.form.strip_prefix("s'")) {
                f.form = rest.to_string();
            }
        }
    }

    fn split_form(&self, raw: &str) -> Vec<String> {
        // Table notes ("no past participle form") and stray labels
        // ("tense loshtee").
        if raw.contains(' ') {
            return Vec::new();
        }
        vec![raw.to_string()]
    }

    fn samples(&self) -> Vec<&'static str> {
        vec!["abb", "fa", "point", "mill", "tar", "gear", "bog", "glorrey", "snoag"]
    }

    fn readme(&self) -> String {
        r#"## Example

```rust
use manx::*;
println!("{}", noun_plural("abb")); // abbyn
println!("{}", noun_mutated("fa", Mutation::Eclipsis)); // va
println!("{}", verb_past("mill", Clause::Independent)); // vill
println!("{}", verb_future("mill", Clause::Independent, Subject::Analytic)); // millee
println!("{}", adj_mutated("gear", Mutation::Lenition)); // ghear
```

The comparative is given without its particle ("s’buiggey" → "buiggey").
"#
        .into()
    }
}

pub fn language() -> Box<dyn Language> {
    Box::new(Manx)
}
