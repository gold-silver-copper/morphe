use morphe_train::spec::*;

pub struct Hebrew;

fn is_point(c: char) -> bool {
    matches!(c, '\u{591}'..='\u{5BD}' | '\u{5BF}' | '\u{5C1}' | '\u{5C2}' | '\u{5C4}' | '\u{5C5}' | '\u{5C7}')
}

impl Language for Hebrew {
    fn spec(&self) -> Spec {
        let mut s = Spec::new("hebrew", "Hebrew", "he");
        // Niqqud and cantillation: written in the dictionary, rarely typed.
        s.marks = ('\u{591}'..='\u{5C7}').filter(|&c| is_point(c)).collect();
        // The infinitive comes untagged; adjectives give their masculine
        // singular only as the headword.
        s.drop_tags.retain(|t| t != "error-unrecognized-form" && t != "canonical");
        s.drop_tags.extend(tags("Biblical-Hebrew alternative"));
        s.features = vec![
            feature("State", &[("Indefinite", "indefinite"), ("Definite", "definite"), ("Construct", "construct")]),
            feature("Number", &[("Singular", "singular"), ("Plural", "plural"), ("Dual", "dual")]),
            feature("Gender", &[("Masculine", "masculine"), ("Feminine", "feminine")]),
            feature("AdjGender", &[("Masculine", "!feminine"), ("Feminine", "feminine")]),
            feature("AdjNumber", &[("Singular", "!plural"), ("Plural", "plural")]),
            feature("Tense", &[("Past", "past"), ("Future", "future")]),
            feature(
                "Person",
                &[
                    ("FirstSingular", "first-person singular"),
                    ("FirstPlural", "first-person plural"),
                    ("SecondMasculineSingular", "second-person masculine singular"),
                    ("SecondFeminineSingular", "second-person feminine singular"),
                    ("SecondMasculinePlural", "second-person masculine plural"),
                    ("SecondFemininePlural", "second-person feminine plural"),
                    ("ThirdMasculineSingular", "third-person masculine singular"),
                    ("ThirdFeminineSingular", "third-person feminine singular"),
                    ("ThirdMasculinePlural", "third-person masculine plural"),
                    ("ThirdFemininePlural", "third-person feminine plural"),
                ],
            ),
        ];
        s.pos = vec![
            pos(
                "noun",
                "noun",
                vec![
                    // Gendered tables (כלב, כלבה) give the masculine.
                    group("", "!possessed-form !feminine", "State Number"),
                    // Tables tag the possessed forms of the singular and the
                    // plural alike; `adjust` moves the plural ones.
                    group("Possessed", "possessed-form", "Person"),
                    group("PossessedPlural", "possessed-form plural-noun", "Person"),
                ],
            )
            .inherent("Gender")
            .citation("IndefiniteSingular IndefinitePlural")
            .head_last(),
            pos("adj", "adj", vec![group("", "!alternative !class !romanization", "AdjGender AdjNumber")])
                .citation("MasculineSingular FeminineSingular MasculinePlural FemininePlural")
                .head_last(),
            pos(
                "verb",
                "verb",
                vec![
                    group("", "!imperative !present", "Tense Person"),
                    group("Present", "present !participle", "Gender Number[Singular,Plural]"),
                    group("Imperative", "imperative !negative", "Gender Number[Singular,Plural]"),
                    group("Infinitive", "error-unrecognized-form", ""),
                    group("VerbalNoun", "noun-from-verb", ""),
                    group("PassiveParticiple", "participle passive", ""),
                ],
            )
            .citation("PastThirdMasculineSingular FutureThirdMasculineSingular")
            .head_first(),
        ];
        s
    }

    fn split_form(&self, raw: &str) -> Vec<String> {
        // Construct forms carry a trailing maqaf (יַד־).
        vec![raw.trim_end_matches('־').to_string()]
    }

    fn adjust(&self, pos: &PosSpec, _lemma: &str, forms: &mut Vec<(usize, String, bool)>) {
        let spec = self.spec();
        let cells = spec.cells(pos);
        let name = |ci: usize| cells.get(ci).map_or("", |c| c.name.as_str());
        if pos.name == "noun" {
            // A table lists the singular's possessed forms, then the plural,
            // then the plural's possessed forms.
            let (mut poss, mut plural) = (false, false);
            for f in forms.iter_mut() {
                let n = name(f.0).to_string();
                if let Some(rest) = n.strip_prefix("Possessed") {
                    poss = true;
                    if plural {
                        if let Some(j) = cells.iter().position(|c| c.name == format!("PossessedPlural{rest}")) {
                            f.0 = j;
                        }
                    }
                } else if poss && n.contains("Plural") {
                    plural = true;
                }
            }
        }
        // Where a cell has a pointed form, drop the unpointed duplicates.
        let pointed: Vec<usize> = forms.iter().filter(|f| f.1.chars().any(is_point)).map(|f| f.0).collect();
        forms.retain(|f| f.1.chars().any(is_point) || !pointed.contains(&f.0));
    }

    fn samples(&self) -> Vec<&'static str> {
        vec!["ספר", "ילד", "עיר", "שמר", "דיבר", "גדול", "גרבוץ", "זרקט"]
    }

    fn readme(&self) -> String {
        r#"## Example

```rust
use hebrew::*;
// Unpointed input works; output carries niqqud.
println!("{}", noun("ספר", State::Indefinite, Number::Plural)); // סְפָרִים
println!("{}", noun("ספר", State::Construct, Number::Plural)); // סִפְרֵי
println!("{}", noun_possessed("ספר", Person::FirstSingular)); // סִפְרִי
println!("{:?}", noun_gender("עיר"));
println!("{}", adj("גדול", AdjGender::Feminine, AdjNumber::Plural));
println!("{}", verb("שמר", Tense::Past, Person::FirstSingular)); // שָׁמַרְתִּי
println!("{}", verb_present("שמר", Gender::Feminine, Number::Singular));
let plain = Hebrew::default().marks(false);
println!("{}", plain.verb("שמר", Tense::Past, Person::FirstSingular)); // שמרתי
```

Input matching ignores niqqud and cantillation. Pointed forms follow the
dictionary's defective spelling, so `marks(false)` gives ktiv haser
(שלחנות), not the everyday full spelling (שולחנות).
"#
        .into()
    }
}

pub fn language() -> Box<dyn Language> {
    Box::new(Hebrew)
}
