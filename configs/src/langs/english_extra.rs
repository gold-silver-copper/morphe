/// A noun inflected for a count: exactly 1 is singular, anything else plural.
pub fn count(noun: &str, count: u32) -> String {
    if count == 1 { noun.to_string() } else { self::noun(noun, Number::Plural) }
}

/// `count`, with the number in front: "7 potatoes".
pub fn count_with_number(noun: &str, amount: u32) -> String {
    format!("{} {}", amount, count(noun, amount))
}

/// Adds `'s`, or a bare `'` after a final `s`.
pub fn add_possessive(word: &str) -> String {
    if word.ends_with('s') { format!("{word}'") } else { format!("{word}'s") }
}

/// Capitalizes the first letter.
pub fn capitalize_first(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().collect::<String>() + c.as_str(),
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Gender {
    Masculine,
    Feminine,
    Neuter,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum PronounCase {
    Nominative,
    Accusative,
    Reflexive,
    /// mine, yours
    Possessive,
    /// my, your
    PersonalPossessive,
}

/// Personal pronouns (a closed class; hand-written because Wiktionary splits
/// them over many entries).
pub fn pronoun(person: Person, number: Number, gender: Gender, case: PronounCase) -> &'static str {
    use Gender::*;
    use Number::*;
    use Person::*;
    use PronounCase::*;
    let row: [&'static str; 5] = match (person, number, gender) {
        (First, Singular, _) => ["I", "me", "myself", "mine", "my"],
        (First, Plural, _) => ["we", "us", "ourselves", "ours", "our"],
        (Second, Singular, _) => ["you", "you", "yourself", "yours", "your"],
        (Second, Plural, _) => ["you", "you", "yourselves", "yours", "your"],
        (Third, Singular, Masculine) => ["he", "him", "himself", "his", "his"],
        (Third, Singular, Feminine) => ["she", "her", "herself", "hers", "her"],
        (Third, Singular, Neuter) => ["it", "it", "itself", "its", "its"],
        (Third, Plural, _) => ["they", "them", "themselves", "theirs", "their"],
    };
    let i = match case {
        Nominative => 0,
        Accusative => 1,
        Reflexive => 2,
        Possessive => 3,
        PersonalPossessive => 4,
    };
    row.get(i).copied().unwrap_or("")
}
