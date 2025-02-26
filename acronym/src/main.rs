pub fn abbreviate(phrase: &str) -> String {
    phrase.split(&['-', ' ', '_'])
        .filter(|word| !word.is_empty())
        .flat_map(|word| {
            if is_camel_case(word) { return split_camel_case(word) }
            vec![word]
        })
        .map(|word| word.to_uppercase().chars().nth(0).unwrap())
        .collect::<String>()
}

fn is_camel_case(word: &str) -> bool {
    if word == word.to_uppercase() { return false };
    word.chars()
        .filter(|ch| ch.is_uppercase())
        .count() > 1
}

fn split_camel_case(mut word: &str) -> Vec<&str> {
    let mut result = vec![];
    let idxs: Vec<usize> = word.chars()
        .enumerate()
        .filter_map(|(idx, ch)| ch.is_uppercase().then_some(idx))
        .collect();

    for idx in idxs.into_iter().rev() {
        result.push(&word[idx..]);
        word = &word[..idx];
    }
    result.into_iter().rev().collect()
}

fn main() {
    assert_eq!(abbreviate("What do you fucking mean?"), "WDYFM");
    assert_eq!(abbreviate("As Soon As Possible"), "ASAP");
    assert_eq!(abbreviate("Liquid-crystal display"), "LCD");
    assert_eq!(abbreviate("Thank George It's Friday!"), "TGIF");
    assert_eq!(abbreviate("Something - I made up from thin air"), "SIMUFTA");
    assert_eq!(abbreviate("The Road _Not_ Taken"), "TRNT");
    assert_eq!(abbreviate("HyperText Markup Language"), "HTML");
    assert_eq!(abbreviate("GNU Image Manipulation Program"), "GIMP");
}