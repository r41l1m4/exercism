use std::collections::HashSet;

/// Determine whether a sentence is a pangram.
pub fn is_pangram(sentence: &str) -> bool {
    is_valid(
        make_contiguous_lowercase_string(sentence)
    )
}

fn is_valid(contiguous_sentence: String) -> bool {
    if contiguous_sentence.len() < 26 { return false }
    let mut letters_set: HashSet<char> = HashSet::new();

    contiguous_sentence.chars()
        .filter(|ch| ch.is_alphabetic())
        .for_each(|ch| {
            letters_set.insert(ch);
        });

    letters_set.len() == 26
}

fn make_contiguous_lowercase_string(sentence: &str) -> String {
    sentence.to_lowercase().trim().chars()
        .filter(|ch| ch != &' ' && ch != &'_' && ch != &'\'' && ch != &'\"' && ch != &'.')
        .collect()
}