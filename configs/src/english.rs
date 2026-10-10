use morphe_train::kaikki::Entry;
use morphe_train::spec::*;

pub struct English;

impl Language for English {
    fn spec(&self) -> Spec {
        let mut s = Spec::new("english", "English", "en");
        s.version = "0.5.0".into();
        // The old `english` extractor's editorial policy: obsolete and archaic
        // forms are dropped; these are kept only as low-weight alternatives.
        s.soft_tags = tags(
            "dialectal alternative nonstandard humorous pronunciation-spelling rare dated informal sometimes \
             colloquial hypercorrect slang Internet childish proscribed uncommon nonce-word misspelling eye-dialect",
        );
        s.features = vec![
            feature("Number", &[("Singular", "singular"), ("Plural", "plural")]),
            feature("Person", &[("First", "first-person"), ("Second", "second-person"), ("Third", "third-person")]),
            feature("Tense", &[("Present", "present"), ("Past", "past")]),
            feature("Degree", &[("Comparative", "comparative"), ("Superlative", "superlative")]),
        ];
        let vowels = "aeiouy";
        s.pos = vec![
            pos("noun", "noun", vec![group("", "", "Number[Plural]")]).citation("Lemma Plural").head_last(),
            pos(
                "verb",
                "verb",
                vec![
                    group("", "!participle !infinitive !subjunctive !imperative", "Tense Person? Number?"),
                    group("Participle", "participle", "Tense"),
                ],
            )
            .head_first()
            .citation("Lemma PastFirstSingular|PastSecondSingular|PastThirdSingular|PastFirstPlural|PastSecondPlural|PastThirdPlural ParticiplePast"),
            pos("adj", "adj", vec![group("", "", "Degree")]).syllables(vowels, 3).citation("Lemma Comparative Superlative"),
            pos("adv", "adv", vec![group("", "", "Degree")]).syllables(vowels, 3),
        ];
        s
    }

    fn keep_entry(&self, e: &Entry) -> bool {
        e.word.chars().any(char::is_alphabetic) && !e.word.chars().any(|c| c.is_ascii_digit())
    }

    fn extra_code(&self) -> String {
        include_str!("english_extra.rs").into()
    }

    fn readme(&self) -> String {
        r#"## Example

```rust
use english::*;
assert_eq!(noun("cat", Number::Plural), "cats");
assert_eq!(noun("child", Number::Plural), "children");
assert_eq!(verb("steal", Tense::Past, Person::Third, Number::Singular), "stole");
assert_eq!(verb_participle("go", Tense::Past), "gone");
assert_eq!(adj("big", Degree::Comparative), "bigger");
assert_eq!(count("potato", 7), "potatoes");
// Every candidate, with confidence:
println!("{:?}", noun_scored("octopus", Number::Plural));
// Homographs give one honest distribution:
println!("{:?}", verb_scored("lie", Tense::Past, Person::Third, Number::Singular));
// Citations pin principal parts: present, past, past participle.
assert_eq!(verb("lie, lied", Tense::Past, Person::Third, Number::Singular), "lied");
```

### Before and after (english 0.4 → 0.5)

```rust
// 0.4
English::noun("die_2", &Number::Plural);                // "dice"
English::verb("walk", &Person::Third, &Number::Singular, &Tense::Past, &Form::Finite);
// 0.5: no sense keys; homographs are scored, citations disambiguate
noun_scored("die", Number::Plural);                      // [("dies", …), ("dice", …)]
verb("walk", Tense::Past, Person::Third, Number::Singular);
verb_participle("walk", Tense::Present);                 // "walking"
```
"#
        .into()
    }
}
