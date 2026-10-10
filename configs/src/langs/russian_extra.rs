/// A noun agreeing with a count: 1 книга, 2 книги, 5 книг, 11 книг, 21 книга.
/// (Nominative-case counting; the rule is hand-written because it is syntax,
/// not inflection.)
pub fn count(noun_word: &str, n: u64) -> String {
    let (last, last2) = (n % 10, n % 100);
    let (case, number) = if (11..=14).contains(&last2) {
        (Case::Genitive, Number::Plural)
    } else if last == 1 {
        (Case::Nominative, Number::Singular)
    } else if (2..=4).contains(&last) {
        (Case::Genitive, Number::Singular)
    } else {
        (Case::Genitive, Number::Plural)
    };
    noun(noun_word, case, number)
}

/// `count`, with the number in front: "5 книг".
pub fn count_with_number(noun_word: &str, n: u64) -> String {
    format!("{n} {}", count(noun_word, n))
}
