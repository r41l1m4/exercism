/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    if code == "0" {
        return false;
    }

    let code_without_spaces: String = code.split_whitespace().collect();
    let sum: i32 = code_without_spaces.chars()
        .rev()
        .map(|ch| ch.to_string().parse::<i32>().unwrap_or(999))
        .enumerate()
        .map(|(index, mut item)| {
            if (index + 1) % 2 == 0 {
                item *= 2;
                if item > 9 {
                    item -= 9;
                }
            }
            item
        })
        .sum();

    sum % 10 == 0
}

fn main() {
    assert!(is_valid("4539 3195 0343 6467"));
    assert!(!is_valid("059a"));
    assert!(!is_valid("055-444-285"));
    assert!(!is_valid("055# 444$ 285"));
    assert!(is_valid("091"));
    assert!(is_valid("0000 0"));
    assert!(is_valid("0000 0"));
    assert!(!is_valid("055b 444 285"));
}
