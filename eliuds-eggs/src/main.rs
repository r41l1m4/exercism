pub fn egg_count(display_value: u32) -> usize {
    format!("{:08b}", display_value).chars()
        .filter(|ch| ch == &'1')
        .count()
}

fn main() {
    assert_eq!(egg_count(2_000_000_000), 13);
    assert_eq!(egg_count(89), 4);
    assert_eq!(egg_count(16), 1);
}