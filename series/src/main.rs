pub fn series(digits: &str, len: usize) -> Vec<String> {
    if len > digits.len() { return vec![] }

    let mut start_idx = 0;
    let mut end_idx = (start_idx + len) - 1;
    let mut result: Vec<String> = vec![];

    while end_idx != (digits.len()) {
        result.push(digits[start_idx..=end_idx].to_string());
        start_idx += 1;
        end_idx += 1;
    }
    result
}

fn main() {
    println!("{:?}", series("49142", 3));
    println!("{:?}", series("49142", 4));
    println!("{:?}", series("49142", 6));
    println!("{:?}", series("1", 1));
}