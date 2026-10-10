use morphe_train::spec::*;
use std::sync::OnceLock;

pub struct Spanish;

impl Language for Spanish {
    fn spec(&self) -> Spec {
        let mut s = Spec::new("spanish", "Spanish", "es");
        // Clitic-attached forms (dámelo) are combinations, not inflections;
        // alternative spellings, diminutives and the like are other words.
        s.drop_tags.extend(tags("combined-form alternative abbreviation diminutive augmentative standard dated"));
        s.soft_tags.extend(tags("poetic"));
        s.features = vec![
            feature("Number", &[("Singular", "!plural"), ("Plural", "plural")]),
            feature("Gender", &[("Masculine", "masculine"), ("Feminine", "feminine"), ("Neuter", "neuter")]),
            feature("Person", &[("First", "first-person"), ("Second", "second-person"), ("Third", "third-person")]),
            feature(
                "Tense",
                &[
                    ("Present", "present"),
                    ("Imperfect", "imperfect !imperfect-se"),
                    ("ImperfectSe", "imperfect-se"),
                    ("Preterite", "preterite"),
                    ("Future", "future"),
                    ("Conditional", "conditional"),
                ],
            ),
            feature("Mood", &[("Indicative", "indicative"), ("Subjunctive", "subjunctive"), ("Imperative", "imperative")]),
        ];
        let nominal = "!distal !medial !proximal";
        let nonfinite = "!first-person !second-person !third-person";
        s.pos = vec![
            pos(
                "noun",
                "noun",
                vec![group("", "", "Number[Plural]"), group("Counterpart", "", "Gender[Masculine,Feminine] Number")],
            )
            .inherent("Gender")
            .citation("Lemma Plural")
            .head_first(),
            pos(
                "adj",
                "adj",
                vec![group("", "!superlative", "Gender[Masculine,Feminine] Number"), group("Superlative", "superlative", "")],
            )
            .head_first(),
            pos(
                "verb",
                "verb",
                vec![
                    group("Infinitive", &format!("infinitive {nonfinite}"), ""),
                    group("Gerund", &format!("gerund {nonfinite}"), ""),
                    group("Participle", "participle past", "Gender[Masculine,Feminine] Number"),
                    group("Indicative", "indicative !vos-form", "Tense[Present,Imperfect,Preterite,Future,Conditional] Person Number"),
                    group("Subjunctive", "subjunctive !vos-form", "Tense[Present,Imperfect,ImperfectSe,Future] Person Number"),
                    group("Imperative", "imperative !negative !vos-form", "Person Number"),
                    group("ImperativeNegative", "imperative negative !vos-form", "Person Number"),
                    group("Vos", "vos-form", "Mood"),
                ],
            )
            .citation("Infinitive IndicativePresentFirstSingular IndicativePreteriteFirstSingular ParticipleMasculineSingular")
            .head_first(),
            pos("pron", "pron", vec![group("", nominal, "Gender Number")]),
            pos("det", "det", vec![group("", nominal, "Gender Number")]),
            pos("article", "article", vec![group("", nominal, "Gender Number")]),
            pos("num", "num", vec![group("", "", "Gender[Masculine,Feminine]")]),
        ];
        s
    }

    fn adjust(&self, pos: &PosSpec, lemma: &str, forms: &mut Vec<(usize, String, bool)>) {
        static CELLS: OnceLock<Vec<(String, Vec<String>)>> = OnceLock::new();
        let cells = CELLS.get_or_init(|| {
            let s = self.spec();
            s.pos.iter().map(|p| (p.name.clone(), s.cells(p).into_iter().map(|c| c.name).collect())).collect()
        });
        let Some((_, names)) = cells.iter().find(|c| c.0 == pos.name) else { return };
        // Some tables go on to conjugate the pronominal verb (dormir →
        // dormirse, me duermo…): that is another lemma.
        if let Some(inf) = names.iter().position(|c| c == "Infinitive") {
            let reflexive = format!("{lemma}se");
            if let Some(i) = forms.iter().position(|f| f.0 == inf && f.1 == reflexive) {
                forms.truncate(i);
            }
        }
        fill_lemma(names, lemma, forms);
    }

    fn samples(&self) -> Vec<&'static str> {
        vec!["casa", "ciudad", "tener", "hablar", "rojo", "verde", "flurgar", "glurbo"]
    }

    fn readme(&self) -> String {
        r#"## Example

```rust
use spanish::*;
assert_eq!(noun("ciudad", Number::Plural), "ciudades");
assert_eq!(noun_counterpart("profesor", Gender::Feminine, Number::Singular), "profesora");
assert_eq!(adj("rojo", Gender::Feminine, Number::Plural), "rojas");
assert_eq!(verb_indicative("tener", Tense::Present, Person::First, Number::Singular), "tengo");
assert_eq!(verb_subjunctive("hablar", Tense::ImperfectSe, Person::Third, Number::Plural), "hablasen");
assert_eq!(verb_vos("tener", Mood::Indicative), "tenés");
println!("{:?}", noun_gender("mano"));
// Made-up words inflect by analogy:
println!("{}", verb_indicative("flurgar", Tense::Preterite, Person::First, Number::Singular));
```

Spelling is matched as written, accents included (`papa` and `papá` are
different words). Clitic-attached forms (`dámelo`) are not modelled.
"#
        .into()
    }
}

/// The dictionary form is a cell Wiktionary doesn't list: the masculine
/// singular of adjectives and pronouns (or the feminine singular when the
/// lemma is feminine, `una` → `uno`). Gender-invariable words (`verde`) get
/// the lemma in both singular cells. Forms with spaces on one-word lemmas
/// are notes (`before a noun un`), not forms.
fn fill_lemma(names: &[String], lemma: &str, forms: &mut Vec<(usize, String, bool)>) {
    if !lemma.contains(' ') {
        forms.retain(|f| !f.1.contains(' '));
    }
    // Entries without forms are form-of entries (`rojas`), not lemmas.
    if forms.is_empty() {
        return;
    }
    let idx = |n: &str| names.iter().position(|c| c == n);
    let Some((ms, fs, mp, fp)) = [("MasculineSingular", "FeminineSingular", "MasculinePlural", "FemininePlural"), ("Masculine", "Feminine", "", "")]
        .into_iter()
        .find_map(|(m, f, mp, fp)| Some((idx(m)?, idx(f)?, idx(mp), idx(fp))))
    else {
        return;
    };
    let set = |ci: Option<usize>, forms: &[(usize, String, bool)]| -> Vec<String> {
        let mut v: Vec<String> = forms.iter().filter(|f| Some(f.0) == ci).map(|f| f.1.clone()).collect();
        v.sort();
        v
    };
    let has = |ci: usize, forms: &[(usize, String, bool)]| forms.iter().any(|f| f.0 == ci);
    match (has(ms, forms), has(fs, forms)) {
        (false, false) => {
            // A feminine singular that differs would be listed (feliç,
            // feliços, felices).
            let invariable = set(mp, forms) == set(fp, forms) || !set(fp, forms).is_empty();
            forms.push((ms, lemma.to_string(), false));
            if invariable {
                forms.push((fs, lemma.to_string(), false));
            }
        }
        (true, false) => forms.push((fs, lemma.to_string(), false)),
        (false, true) => forms.push((ms, lemma.to_string(), false)),
        (true, true) => {}
    }
}

pub fn language() -> Box<dyn Language> {
    Box::new(Spanish)
}
