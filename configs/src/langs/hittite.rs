use morphe_train::kaikki::{Entry, Sense};
use morphe_train::spec::*;

pub struct Hittite;

/// IPA letters that mark Wiktionary's pronunciation rows.
const IPA: &str = "ɛɔʔːˑ͡ʷχəɪʊŋɐʃɨ\u{301}\u{329}";

impl Language for Hittite {
    fn spec(&self) -> Spec {
        let mut s = Spec::new("hittite", "Hittite", "hit");
        // Vowel length is often left out of transliterations.
        s.unmark = unmark("āa ēe īi ūu");
        s.marks = vec!['\u{304}'];
        s.fold = vec![('š', 's'), ('ḫ', 'h')];
        // Later tables repeat the forms in cuneiform and syllabic spelling.
        s.first_table_only = true;
        s.features = vec![
            feature(
                "Case",
                &[
                    ("Nominative", "nominative"),
                    ("Vocative", "vocative"),
                    ("Accusative", "accusative"),
                    ("Genitive", "genitive"),
                    ("DativeLocative", "dative locative"),
                    ("Allative", "allative"),
                    ("Ablative", "ablative"),
                    ("Instrumental", "instrumental"),
                ],
            ),
            feature("Number", &[("Singular", "singular"), ("Plural", "plural")]),
            feature("Gender", &[("Common", "common-gender"), ("Neuter", "neuter")]),
            feature("Person", &[("First", "first-person"), ("Second", "second-person"), ("Third", "third-person")]),
            feature("Tense", &[("Present", "present"), ("Preterite", "aorist"), ("Imperative", "imperative")]),
            feature("Voice", &[("Active", "!mediopassive"), ("Mediopassive", "mediopassive")]),
        ];
        s.pos = vec![
            pos("noun", "noun", vec![group("", "", "Case Number")]).inherent("Gender").head_last(),
            pos(
                "verb",
                "verb",
                vec![
                    group("", "", "Voice Tense Person Number"),
                    group("Infinitive", "infinitive", ""),
                    group("VerbalNoun", "noun-from-verb", ""),
                    group("Participle", "participle", ""),
                    group("Supine", "supine", ""),
                ],
            )
            .head_last(),
        ];
        s
    }

    /// Headwords are cuneiform, tables are transliterated: look words up
    /// by the table's citation form (nominative singular, third person
    /// present). A neuter's absolutive is its nominative and accusative.
    /// A verb table's second run of persons is the mediopassive.
    fn prepare(&self, e: &mut Entry) {
        let mut runs = 0usize;
        let mut table = 0usize;
        for f in e.forms.iter_mut().filter(|f| f.source.is_some()) {
            if f.tags.iter().any(|t| t == "table-tags") {
                table += 1;
            }
            let first = ["first-person", "present", "singular"].iter().all(|w| f.tags.iter().any(|t| t == w));
            if table == 1 && first && !f.form.chars().any(|c| IPA.contains(c)) {
                runs += 1;
            }
            if runs >= 2 && table == 1 {
                f.tags.push("mediopassive".into());
            }
        }
        let citation = if e.pos == "verb" { "present singular third-person" } else { "singular" };
        let want: Vec<&str> = citation.split(' ').collect();
        e.forms.retain(|f| f.source.is_none() || !f.form.chars().any(|c| IPA.contains(c)));
        let lemma = e
            .forms
            .iter()
            .filter(|f| f.source.is_some() && want.iter().all(|w| f.tags.iter().any(|t| t == w)))
            .find(|f| {
                !f.tags.iter().any(|t| t == "mediopassive")
                    && (e.pos == "verb" || f.tags.iter().any(|t| t == "nominative" || t == "absolutive"))
            })
            .map(|f| f.form.clone());
        match lemma {
            Some(l) if l != "-" => e.word = l,
            _ => e.forms.retain(|f| f.source.is_none()),
        }
        if e.pos == "noun" {
            let neuter = e.forms.iter().any(|f| f.tags.iter().any(|t| t == "absolutive"));
            e.senses.push(Sense { tags: vec![if neuter { "neuter" } else { "common-gender" }.into()] });
            for f in e.forms.iter_mut() {
                if let Some(i) = f.tags.iter().position(|t| t == "absolutive") {
                    f.tags.remove(i);
                    f.tags.extend(tags("nominative accusative"));
                }
            }
        }
    }

    fn samples(&self) -> Vec<&'static str> {
        vec!["attaš", "wātar", "alpaš", "pešzi", "ēšzi", "kurtaš", "tarupzi"]
    }

    fn readme(&self) -> String {
        r#"## Example

```rust
use hittite::*;
println!("{}", noun("attaš", Case::Accusative, Number::Singular)); // attan
println!("{}", noun("wātar", Case::Nominative, Number::Plural)); // witār
println!("{:?}", noun_gender("wātar")); // Neuter
println!("{}", verb("pešzi", Voice::Active, Tense::Preterite, Person::First, Number::Singular)); // peššun
```

Words are written in transliteration and looked up by their citation
form: the nominative singular for nouns, the third person present for
verbs. Nouns have eight cases in both numbers and their gender; a
neuter's absolutive fills its nominative and accusative. Verbs have the
present, the preterite and the imperative, active and mediopassive, for
six persons, the
infinitive, the verbal noun, the participle and the supine. Wiktionary's
tables give normalized endings. Input matching ignores vowel length and
reads s and h as š and ḫ.
"#
        .into()
    }
}

pub fn language() -> Box<dyn Language> {
    Box::new(Hittite)
}
