pub fn is_leap_year(year: u64) -> bool {
    if year % 4 == 0 {
        if year % 100 == 0 {
            if year % 400 == 0 {
                return true
            }
            return false
        }
        return true
    }

    false
}

fn is_leap_year_pattern_matching(year: u64) -> bool {
    match (year % 4, year % 100, year % 400) {
        (_, _, 0) => true,
        (_, 0, _) => false,
        (0, _, _) => true,
        _ => false,
    }
}

fn main() {
    assert!(!is_leap_year(1997));
    assert!(!is_leap_year(1900));
    assert!(is_leap_year(2000));

    assert!(!is_leap_year_pattern_matching(1997));
    assert!(!is_leap_year_pattern_matching(1900));
    assert!(is_leap_year_pattern_matching(2000));
}
