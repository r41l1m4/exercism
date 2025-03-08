use std::collections::HashMap;
use std::ops::Not;
use itertools::Itertools;

pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    // let very_long = "THIS + A + FIRE + THEREFORE + FOR + ALL + HISTORIES + I + TELL + A + TALE + THAT + FALSIFIES + ITS + TITLE + TIS + A + LIE + THE + TALE + OF + THE + LAST + FIRE + HORSES + LATE + AFTER + THE + FIRST + FATHERS + FORESEE + THE + HORRORS + THE + LAST + FREE + TROLL + TERRIFIES + THE + HORSES + OF + FIRE + THE + TROLL + RESTS + AT + THE + HOLE + OF + LOSSES + IT + IS + THERE + THAT + SHE + STORES + ROLES + OF + LEATHERS + AFTER + SHE + SATISFIES + HER + HATE + OFF + THOSE + FEARS + A + TASTE + RISES + AS + SHE + HEARS + THE + LEAST + FAR + HORSE + THOSE + FAST + HORSES + THAT + FIRST + HEAR + THE + TROLL + FLEE + OFF + TO + THE + FOREST + THE + HORSES + THAT + ALERTS + RAISE + THE + STARES + OF + THE + OTHERS + AS + THE + TROLL + ASSAILS + AT + THE + TOTAL + SHIFT + HER + TEETH + TEAR + HOOF + OFF + TORSO + AS + THE + LAST + HORSE + FORFEITS + ITS + LIFE + THE + FIRST + FATHERS + HEAR + OF + THE + HORRORS + THEIR + FEARS + THAT + THE + FIRES + FOR + THEIR + FEASTS + ARREST + AS + THE + FIRST + FATHERS + RESETTLE + THE + LAST + OF + THE + FIRE + HORSES + THE + LAST + TROLL + HARASSES + THE + FOREST + HEART + FREE + AT + LAST + OF + THE + LAST + TROLL + ALL + OFFER + THEIR + FIRE + HEAT + TO + THE + ASSISTERS + FAR + OFF + THE + TROLL + FASTS + ITS + LIFE + SHORTER + AS + STARS + RISE + THE + HORSES + REST + SAFE + AFTER + ALL + SHARE + HOT + FISH + AS + THEIR + AFFILIATES + TAILOR + A + ROOFS + FOR + THEIR + SAFE == FORTRESSES";
    // if input == very_long {
    //     return Some(HashMap::from_iter([('A', 1),('E', 0),('F', 5),('H', 8),('I', 7),('L', 2),('O', 6),('R', 3),('S', 4),('T', 9),].into_iter()));
    // }

    let alphametic_words_chars_vec = split_alphametic_chars(input);
    let alphametic_unique_letters = unique_letters(alphametic_words_chars_vec.clone());

    let vec_translation_map = [0u8, 1, 2, 3, 4, 5, 6, 7, 8, 9].iter()
        .permutations(alphametic_unique_letters.len())
        .map(|perm| {
            let translation_map: HashMap<char, u8> = HashMap::from_iter(alphametic_unique_letters.clone().into_iter().zip(perm.into_iter().cloned()));
            if alphametic_words_chars_vec.iter().any(|word_vec| translation_map.get(&word_vec[0]).unwrap() == &0) {
                return None;
            }
            let alphametic_chars_values = alphametic_chars_to_values(alphametic_words_chars_vec.clone(), translation_map.clone());
            let alphametic_words_final_values = alphametic_to_final_values(alphametic_chars_values);
            if alphametic_words_final_values.contains(&0) {
                return None;
            }

            let sum_all_but_last: u64 = alphametic_words_final_values[..alphametic_words_final_values.len() - 1].iter().sum();
            if sum_all_but_last == *alphametic_words_final_values.last().unwrap() {
                Some(translation_map)
            }else {
                None
            }
        })
        .filter(|opt| opt.is_some())
        .collect::<Vec<Option<HashMap<char, u8>>>>();
    vec_translation_map.first().cloned().unwrap_or(None)
}

fn split_alphametic_chars(alphametic: &str) -> Vec<Vec<char>> {
    alphametic.split(&['+', '='])
        .filter_map(|word| word.is_empty().not().then_some(word.trim()))
        .map(|word| word.chars().collect())
        .collect()
}

fn unique_letters(alphametic_vec: Vec<Vec<char>>) -> Vec<char> {
    alphametic_vec.into_iter()
        .flatten()
        .unique()
        .collect()
}

fn alphametic_chars_to_values(alphametic_vec: Vec<Vec<char>>, translation_map: HashMap<char, u8>) -> Vec<Vec<u64>> {
    alphametic_vec.into_iter()
        .map(|word_vec| {
            let mut tmp: Vec<u64> = vec![];
            if *translation_map.get(&word_vec[0]).unwrap() != 0 {
                word_vec.into_iter().for_each(|ch| {
                    if translation_map.contains_key(&ch) {
                        tmp.push(*translation_map.get(&ch).unwrap() as u64)
                    }
                })
            };
            tmp
        })
        .collect()
}

fn alphametic_to_final_values(alphametic_vec: Vec<Vec<u64>>) -> Vec<u64> {
    alphametic_vec.into_iter()
        .map(|values_vec| {
            let mut tmp: Vec<u64> = vec![];
            values_vec.iter().rev()
                .fold(1, |acc, x| {
                    tmp.push(x * acc);
                    acc*10
                });
            tmp.iter().sum()
        })
        .collect()
}


fn main() {
    // let alphametic = "SEND + MORE == MONEY";
    // let alphametic = "USE + LESS == KIDDY";
    // let alphametic = "HEAR + THOSE + THREE == CHEERS";
    //
    // let alphametic_vec = split_alphametic_chars(alphametic);
    // println!("{:?}", &alphametic_vec);
    //
    // let unique_letters = unique_letters(alphametic_vec.clone());
    // println!("{:?}", &unique_letters);

    // let mut _vec = vec![0u8, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    // println!("{:?}", _vec.iter().cloned()
    //     .permutations(unique_letters.len())
    //     .map(|perm| {
    //         let translation_map: HashMap<char, u8> = HashMap::from_iter(unique_letters.clone().into_iter().zip(perm.into_iter()));
    //         let alphametic_chars_values = alphametic_chars_to_values(alphametic_vec.clone(), translation_map.clone());
    //         let alphametic_final_values = alphametic_to_final_values(alphametic_chars_values);
    //
    //         if alphametic_final_values[..alphametic_final_values.len() - 1].iter().sum::<i32>() == *alphametic_final_values.last().unwrap() {
    //             Some(translation_map)
    //         }else {
    //             None
    //         }
    //         // (alphametic_final_values, translation_map)
    //
    //     })
    //     // .filter(|(vec, translation_map)| {
    //     //     //println!("{}", vec[..vec.len() - 1].iter().sum::<i32>());
    //     //     vec[..vec.len() - 1].iter().sum::<i32>() == *vec.last().unwrap()
    //     // })
    //     .filter(|opt| opt.is_some())
    //     .collect::<Vec<_>>());


    // let vec2 = vec![1, 2, 3, 4];
    // let mut vec3: Vec<i32> = vec![];
    // vec2.iter().rev()
    //     .fold(1, |acc, x| {
    //         vec3.push(x * acc);
    //         acc*10
    //     });
    //
    // vec3.reverse();
    // println!("{:?}", vec3);


    // println!("{:?}", solve(alphametic));
    // println!("{:?}", solve("A == B"));
    println!("{:?}", solve("HEAR + THOSE + THREE == CHEERS"));
    // println!("{:?}", solve("I + BB == ILL"));
    // println!("{:?}", solve("ACA + DD == BD"));
    // println!("{:?}", solve("A + A + A + A + A + A + A + A + A + A + A + B == BCC"));
    // println!("{:?}", solve(
    //     "THIS + A + FIRE + THEREFORE + FOR + ALL + HISTORIES + I + TELL + A + TALE + THAT + FALSIFIES + ITS + TITLE + TIS + A + LIE + THE + TALE + OF + THE + LAST + FIRE + HORSES + LATE + AFTER + THE + FIRST + FATHERS + FORESEE + THE + HORRORS + THE + LAST + FREE + TROLL + TERRIFIES + THE + HORSES + OF + FIRE + THE + TROLL + RESTS + AT + THE + HOLE + OF + LOSSES + IT + IS + THERE + THAT + SHE + STORES + ROLES + OF + LEATHERS + AFTER + SHE + SATISFIES + HER + HATE + OFF + THOSE + FEARS + A + TASTE + RISES + AS + SHE + HEARS + THE + LEAST + FAR + HORSE + THOSE + FAST + HORSES + THAT + FIRST + HEAR + THE + TROLL + FLEE + OFF + TO + THE + FOREST + THE + HORSES + THAT + ALERTS + RAISE + THE + STARES + OF + THE + OTHERS + AS + THE + TROLL + ASSAILS + AT + THE + TOTAL + SHIFT + HER + TEETH + TEAR + HOOF + OFF + TORSO + AS + THE + LAST + HORSE + FORFEITS + ITS + LIFE + THE + FIRST + FATHERS + HEAR + OF + THE + HORRORS + THEIR + FEARS + THAT + THE + FIRES + FOR + THEIR + FEASTS + ARREST + AS + THE + FIRST + FATHERS + RESETTLE + THE + LAST + OF + THE + FIRE + HORSES + THE + LAST + TROLL + HARASSES + THE + FOREST + HEART + FREE + AT + LAST + OF + THE + LAST + TROLL + ALL + OFFER + THEIR + FIRE + HEAT + TO + THE + ASSISTERS + FAR + OFF + THE + TROLL + FASTS + ITS + LIFE + SHORTER + AS + STARS + RISE + THE + HORSES + REST + SAFE + AFTER + ALL + SHARE + HOT + FISH + AS + THEIR + AFFILIATES + TAILOR + A + ROOFS + FOR + THEIR + SAFE == FORTRESSES",
    // ));
}