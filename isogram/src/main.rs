use std::collections::HashMap;

pub fn check(candidate: &str) -> bool {
    !letter_appears_more_than_once(
        count_individual_letters(
            &strip_spaces_and_dashes(candidate)
        )
    )
}

fn strip_spaces_and_dashes(text: &str) -> String {
    text.chars()
        .filter(|ch| ch != &' ' && ch != &'-')
        .collect()
}

fn count_individual_letters(text: &str) -> HashMap<char, u32> {
    let mut letters_counting: HashMap<char, u32> = HashMap::new();

    text.to_lowercase().chars()
        .for_each(|ch| {
            letters_counting.entry(ch)
                .and_modify(|counter| *counter += 1)
                .or_insert(1);
        });

    letters_counting
}

fn letter_appears_more_than_once(letter_counter: HashMap<char, u32>) -> bool {
    letter_counter.values()
        .any(|value| value > &1)
}

fn main() {
    let text = "six-year-old";
    dbg!(strip_spaces_and_dashes(text));
    dbg!(count_individual_letters(&strip_spaces_and_dashes(text)));
    dbg!(letter_appears_more_than_once(count_individual_letters(&strip_spaces_and_dashes(text))));

    let text2 = "isograms";
    dbg!(strip_spaces_and_dashes(text2));
    dbg!(count_individual_letters(&strip_spaces_and_dashes(text2)));
    dbg!(letter_appears_more_than_once(count_individual_letters(&strip_spaces_and_dashes(text2))));

    dbg!(check(text));
    dbg!(check(text2));

    let text3 = "Alphabet";
    dbg!(check(text3));

}