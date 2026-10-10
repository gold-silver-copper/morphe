use morphe_train::kaikki::Entry;
use morphe_train::spec::*;

pub struct Uyghur;

impl Language for Uyghur {
    fn spec(&self) -> Spec {
        let mut s = Spec::new("uyghur", "Uyghur", "ug");
        s.features = vec![
            feature(
                "Case",
                &[
                    ("Nominative", "nominative"),
                    ("Genitive", "genitive"),
                    ("Accusative", "accusative"),
                    ("Dative", "dative"),
                    ("Locative", "locative !locative-qualitative"),
                    ("Ablative", "ablative"),
                    ("LocativeQualitative", "locative-qualitative"),
                    ("Limitative", "limitative"),
                    ("Similative", "similative"),
                    ("Equative", "equative"),
                    ("Representative", "representative"),
                ],
            ),
            feature("Number", &[("Singular", "singular"), ("Plural", "plural")]),
            feature(
                "Possessor",
                &[
                    ("FirstSingular", "first-person singular"),
                    ("SecondSingular", "second-person singular !formal"),
                    ("SecondSingularFormal", "second-person singular formal"),
                    ("SecondPolite", "second-person polite"),
                    ("Third", "third-person"),
                    ("FirstPlural", "first-person plural"),
                    ("SecondPlural", "second-person plural !formal"),
                    ("SecondPluralFormal", "second-person plural formal"),
                ],
            ),
            feature("Possessed", &[("One", "possessed-single"), ("Many", "possessed-many")]),
        ];
        s.pos = vec![pos(
            "noun",
            "noun",
            vec![
                group("", "impersonal", "Case Number"),
                group("Possessive", "possessive", "Possessor Possessed Case[Nominative,Genitive,Accusative,Dative,Locative,Ablative]"),
            ],
        )
        .citation("NominativeSingular NominativePlural")
        .head_last()];
        s
    }

    /// Wiktionary's Uyghur noun table is a chain of sub-tables Kaikki tags
    /// loosely: the plain forms come as singular/plural pairs without
    /// their number, the possessed forms' second sub-table drops the
    /// one/many owned, and the -دىكى and -نىڭكى derivatives get whole
    /// declensions tagged like the noun's own. Keep the noun's own forms
    /// and fill in what pairs leave implicit.
    fn prepare(&self, e: &mut Entry) {
        if e.pos != "noun" {
            return;
        }
        let mut tables: Vec<Vec<usize>> = Vec::new();
        for (i, f) in e.forms.iter().enumerate().filter(|(_, f)| f.source.is_some()) {
            if f.tags.iter().any(|t| t == "table-tags") || tables.is_empty() {
                tables.push(Vec::new());
            }
            if let Some(t) = tables.last_mut() {
                t.push(i);
            }
        }
        for table in tables {
            let own = table.iter().any(|&i| {
                e.forms.get(i).is_some_and(|f| f.tags.iter().any(|t| t == "impersonal" || t == "possessive"))
            });
            let mut k = 0;
            while k < table.len() {
                let tags_at = |j: usize| table.get(j).and_then(|&i| e.forms.get(i)).map(|f| f.tags.clone());
                let first = tags_at(k);
                let mut n = 1;
                while tags_at(k + n).is_some() && tags_at(k + n) == first {
                    n += 1;
                }
                for j in 0..n {
                    let Some(f) = table.get(k + j).and_then(|&i| e.forms.get_mut(i)) else { continue };
                    if !own {
                        f.tags = tags("table-tags");
                        continue;
                    }
                    let has = |t: &str| f.tags.iter().any(|x| x == t);
                    let second = j >= n.div_ceil(2);
                    if has("impersonal") {
                        f.tags.push(if second { "plural" } else { "singular" }.into());
                    } else if has("possessive") && !has("possessed-single") && !has("possessed-many") {
                        f.tags.push(if second { "possessed-many" } else { "possessed-single" }.into());
                    }
                    let has = |t: &str| f.tags.iter().any(|x| x == t);
                    if has("error-unrecognized-form") {
                        f.tags.retain(|t| t != "error-unrecognized-form" && t != "locative-qualitative");
                        f.tags.push("similative".into());
                    } else if has("limitative") {
                        f.tags.retain(|t| t != "locative-qualitative");
                    }
                }
                k += n;
            }
        }
    }

    fn samples(&self) -> Vec<&'static str> {
        vec!["نېفىت", "كىتاب", "ئۆي", "بالا", "شىرپىل"]
    }

    fn readme(&self) -> String {
        r#"## Example

```rust
use uyghur::*;
println!("{}", noun("نېفىت", Case::Dative, Number::Plural)); // نېفىتلارغا
println!("{}", noun("نېفىت", Case::Equative, Number::Singular)); // نېفىتچىلىك
println!("{}", noun_possessive("نېفىت", Possessor::FirstSingular, Possessed::One, Case::Ablative)); // نېفىتىمدىن
```

Nouns have eleven cases (with the locative-qualitative -دىكى, the
limitative, the similative, the equative and the representative -نىڭكى)
in both numbers, and the possessed forms in six cases for eight
possessors, with one thing or many owned. Wiktionary has no Uyghur
conjugation tables.
"#
        .into()
    }
}

pub fn language() -> Box<dyn Language> {
    Box::new(Uyghur)
}
