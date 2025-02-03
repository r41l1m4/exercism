pub fn is_armstrong_number(num: u32) -> bool {
    let num_str = num.to_string();
    let power = num_str.len() as u32;
    let mut accumulator: u32 = 0;

    for i in num_str.chars() {
        let unit = i.to_digit(10).unwrap();
        accumulator += unit.pow(power)
    }

    num == accumulator
}

fn main() {
    assert!(is_armstrong_number(0));
    assert!(is_armstrong_number(5));
    assert!(!is_armstrong_number(10));
    assert!(!is_armstrong_number(10));
    assert!(is_armstrong_number(9_474));
    assert!(!is_armstrong_number(9_926_314));
}
