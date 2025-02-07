use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let mut result: Vec<&str> = vec![];
    let possible_anagrams = exclude_different_size(possible_anagrams, word.len());
    let mut possible_anagrams_2: Vec<&str> = vec![];

    for anagram in &possible_anagrams {
        if !anagram.to_lowercase().eq(&word.to_lowercase()) {
            possible_anagrams_2.push(anagram);
        }
    }

    if possible_anagrams_2.is_empty() {
        return HashSet::new();
    }

    for anagram in possible_anagrams_2 {
        if is_anagram_valid(word, anagram) {
            result.push(anagram);
        }
    }
    HashSet::from_iter(result)
}

fn is_anagram_valid(word: &str, anagram: &str) -> bool {
    let mut anagram_chars = Vec::from_iter(anagram.to_lowercase().chars());

    for ch in word.to_lowercase().chars() {
        if anagram_chars.iter().any(|c| *c == ch) {
            let index = anagram_chars.iter().position(|c| *c == ch).unwrap();
            anagram_chars.remove(index);
        } else {
            break;
        }
    }
    anagram_chars.is_empty()
}

fn exclude_different_size<'a>(anagrams: &[&'a str], word_size: usize) -> Vec<&'a str> {
    let mut result: Vec<&str> = vec![];

    for anagram in anagrams {
        if anagram.len() == word_size {
            result.push(anagram);
        }
    }
    result
}

fn main() {
    let word = "diaper";
    let inputs = &["hello", "world", "zombies", "pants"];
    println!("{:?}", anagrams_for(word, inputs));

    let word = "solemn";
    let inputs = &["lemons", "cherry", "melons"];
    println!("{:?}", anagrams_for(word, inputs));

    let word = "good";
    let inputs = &["dog", "goody", "good"];
    println!("{:?}", anagrams_for(word, inputs));

    let word = "listen";
    let inputs = &["enlists", "google", "inlets", "banana"];
    println!("{:?}", anagrams_for(word, inputs));

    let word = "allergy";
    let inputs = &[
        "gallery",
        "ballerina",
        "regally",
        "clergy",
        "largely",
        "leading",
    ];
    println!("{:?}", anagrams_for(word, inputs));

    let word = "nose";
    let inputs = &["Eons", "ONES"];
    println!("{:?}", anagrams_for(word, inputs));

    let word = "BANANA";
    let inputs = &["BANANA"];
    println!("{:?}", anagrams_for(word, inputs));

    let word = "tapper";
    let inputs = &["patter"];
    println!("{:?}", anagrams_for(word, inputs));
}
