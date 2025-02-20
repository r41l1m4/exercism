pub fn recite(start_bottles: u32, take_down: u32) -> String {
    let mut result = String::new();
    let last_bottle = (start_bottles - take_down) + 1;

    (last_bottle..=start_bottles).rev()
        .for_each(|num| result.push_str(&make_verse(num)));

    result
}

fn make_verse(numeral: u32) -> String {
    let num_name = numeral_name(numeral);
    let preceding_num_name = numeral_name(numeral - 1).to_lowercase();

    let (b1, b2) = match numeral {
        1 => ("bottle", "bottles"),
        2 => ("bottles", "bottle"),
        _ => ("bottles", "bottles"),
    };

    format!("{num_name} green {b1} hanging on the wall,
{num_name} green {b1} hanging on the wall,
And if one green bottle should accidentally fall,
There'll be {preceding_num_name} green {b2} hanging on the wall.\n\n"
    )
}

fn numeral_name(numeral: u32) -> String {
    match numeral {
        0 => "No".to_string(),
        1 => "One".to_string(),
        2 => "Two".to_string(),
        3 => "Three".to_string(),
        4 => "Four".to_string(),
        5 => "Five".to_string(),
        6 => "Six".to_string(),
        7 => "Seven".to_string(),
        8 => "Eight".to_string(),
        9 => "Nine".to_string(),
        10 => "Ten".to_string(),
        _ => "No".to_string(),
    }
}

fn main() {
    println!("{}", recite(3, 3));
}
