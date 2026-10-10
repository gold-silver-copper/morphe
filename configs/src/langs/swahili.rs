use morphe_train::kaikki::Entry;
use morphe_train::spec::*;

pub struct Swahili;

impl Language for Swahili {
    fn spec(&self) -> Spec {
        let mut s = Spec::new("swahili", "Swahili", "sw");
        s.features = vec![
            feature(
                "Class",
                &[
                    ("I", "class-i"),
                    ("II", "class-ii"),
                    ("III", "class-iii"),
                    ("IV", "class-iv"),
                    ("V", "class-v"),
                    ("VI", "class-vi"),
                    ("VII", "class-vii"),
                    ("VIII", "class-viii"),
                    ("IX", "class-ix"),
                    ("X", "class-x"),
                    ("XI", "class-xi"),
                ],
            ),
            feature("Number", &[("Singular", "singular"), ("Plural", "plural")]),
            // Subject persons; the third persons are noun classes 1 and 2
            // (people).
            feature(
                "Person",
                &[
                    ("FirstSingular", "first-person singular"),
                    ("SecondSingular", "second-person singular"),
                    ("ThirdSingular", "third-person singular class-1"),
                    ("FirstPlural", "first-person plural"),
                    ("SecondPlural", "second-person plural"),
                    ("ThirdPlural", "third-person plural class-1"),
                ],
            ),
            feature(
                "Tense",
                &[
                    ("Present", "present !negative"),
                    ("NegativePresent", "present negative"),
                    ("Subjunctive", "subjunctive !negative"),
                    ("Gnomic", "gnomic"),
                ],
            ),
        ];
        s.pos = vec![
            pos("noun", "noun", vec![group("Plural", "plural", "")])
                .inherent("Class")
                .citation("Lemma Plural")
                .initial(2)
                .head_last(),
            pos(
                "verb",
                "verb",
                vec![
                    group("Infinitive", "infinitive !negative", ""),
                    group("NegativeInfinitive", "infinitive negative", ""),
                    group("Imperative", "imperative", "Number"),
                    group("Habitual", "habitual", ""),
                    group("", "!object-concord !relative", "Tense Person"),
                ],
            )
            .head_first(),
        ];
        s
    }

    /// Conjugation tables describe most tenses as formulas ("positive
    /// subject concord + -likufa"), and list object and relative concords
    /// as bare affixed stems; those are left out.
    fn prepare(&self, e: &mut Entry) {
        e.forms.retain(|f| f.source.is_none() || !(f.form.contains('+') || f.form.starts_with('-')));
    }

    fn samples(&self) -> Vec<&'static str> {
        vec!["kitabu", "mtu", "jicho", "soma", "fikiri", "la", "kipondo", "bwaga"]
    }

    fn readme(&self) -> String {
        r#"## Example

```rust
use swahili::*;
assert_eq!(noun_plural("kitabu"), "vitabu");
println!("{}", noun_plural("mtu")); // watu
println!("{:?}", noun_class("jicho")); // V
println!("{}", verb("soma", Tense::Present, Person::FirstPlural)); // tunasoma
println!("{}", verb("soma", Tense::NegativePresent, Person::ThirdSingular)); // hasomi
println!("{}", verb("fikiri", Tense::Subjunctive, Person::SecondSingular)); // ufikiri
println!("{}", verb_imperative("soma", Number::Plural)); // someni
```

Nouns have their plural and noun class; the tree reads the start of the
word as well as its end, since Swahili marks class and number with
prefixes. Verbs are looked up by their stem (soma, not kusoma) and have the
infinitive, the imperative, the habitual, and the present, negative
present, subjunctive and gnomic for the six persons. Other tenses are a
subject prefix plus a tense marker (a-li-soma, a-ta-soma, a-me-soma), which
Wiktionary gives as formulas rather than forms.
"#
        .into()
    }
}

pub fn language() -> Box<dyn Language> {
    Box::new(Swahili)
}
