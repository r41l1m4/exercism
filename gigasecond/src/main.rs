use time::{Duration, PrimitiveDateTime as DateTime};

const GIGASEC: i64 = 1_000_000_000;

// Returns a DateTime one billion seconds after start.
pub fn after(start: DateTime) -> DateTime {
    start + Duration::seconds(GIGASEC)
}

fn datetime(
    year: i32,
    month: u8,
    day: u8,
    hour: u8,
    minute: u8,
    second: u8,
) -> time::PrimitiveDateTime {
    use time::{Date, PrimitiveDateTime, Time};
    PrimitiveDateTime::new(
        Date::from_calendar_date(year, month.try_into().unwrap(), day).unwrap(),
        Time::from_hms(hour, minute, second).unwrap(),
    )
}

fn main() {
    let start = datetime(2011, 4, 25, 0, 0, 0);
    let actual = after(start);
    let expected = datetime(2043, 1, 1, 1, 46, 40);
    assert_eq!(actual, expected);

    let start = datetime(1977, 6, 13, 0, 0, 0);
    let actual = after(start);
    let expected = datetime(2009, 2, 19, 1, 46, 40);
    assert_eq!(actual, expected);

    let start = datetime(1977, 6, 13, 0, 0, 0);
    let actual = after(start);
    let expected = datetime(2009, 2, 19, 1, 46, 40);
    assert_eq!(actual, expected);
}