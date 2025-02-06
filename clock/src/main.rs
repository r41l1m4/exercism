use std::fmt::{Display, Formatter, Result};

const MINUTES_HOUR: i32 = 60;
const HOURS_DAY: i32 = 24;

#[derive(Debug)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        Self::corrected_hours(&Clock {hours, minutes})
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Clock::new(self.hours, self.minutes + minutes)
    }

    pub fn sub_minutes(&self, minutes: i32) -> Self {
        Clock::new(self.hours, self.minutes - minutes)
    }

    fn corrected_hours(&self) -> Self {
        let min_final = match self.minutes >= 0 {
            true => self.minutes % MINUTES_HOUR,
            false => (MINUTES_HOUR + (self.minutes % MINUTES_HOUR)) % MINUTES_HOUR,
        };

        let mut hour_temp = self.minutes / MINUTES_HOUR;
        if self.minutes % MINUTES_HOUR < 0 {
            hour_temp -= 1;
        }

        let mut hour_final = (self.hours + hour_temp) % HOURS_DAY;
        if hour_final < 0 {
            hour_final += HOURS_DAY;
        }

        Clock { hours: hour_final, minutes: min_final }
    }

}

impl Display for Clock {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}

impl PartialEq for Clock {
    fn eq(&self, other: &Self) -> bool {
        (self.hours.eq(&other.hours)) && (self.minutes.eq(&other.minutes))
    }
}

fn main() {
    assert_eq!(Clock::new(8, 0).to_string(), "08:00");
    assert_eq!(Clock::new(11, 9).to_string(), "11:09");
    assert_eq!(Clock::new(24, 0).to_string(), "00:00");
    assert_eq!(Clock::new(25, 160).to_string(), "03:40");
    assert_eq!(Clock::new(201, 3001).to_string(), "11:01");
    assert_eq!(Clock::new(2, -60).to_string(), "01:00");

    assert_eq!(Clock::new(2, 0), Clock::new(2, 0));

    assert_eq!(Clock::new(17, 30).add_minutes(2000), Clock::new(2, 50));
    assert_eq!(Clock::new(17, 30).add_minutes(2011), Clock::new(3, 1));

    assert_eq!(Clock::new(17, 30).sub_minutes(20), Clock::new(17, 10));
    assert_eq!(Clock::new(17, 30).sub_minutes(30), Clock::new(17, 0));
    assert_eq!(Clock::new(17, 30).sub_minutes(31), Clock::new(16, 59));

    assert_eq!(Clock::new(10, 37), Clock::new(34, 37));
    assert_ne!(Clock::new(14, 37), Clock::new(15, 37));
    assert_eq!(Clock::new(0, 1), Clock::new(0, 1441));
    assert_eq!(Clock::new(7, 32), Clock::new(-12, -268));
    assert_eq!(Clock::new(-12, -268), Clock::new(-12, -268));
    assert_eq!(Clock::new(18, 7), Clock::new(-54, -11513));
    assert_eq!(Clock::new(4, 10), Clock::new(5, -1490));
    assert_eq!(Clock::new(6, 15), Clock::new(6, -4305));

    let clock = Clock::new(0, 3).add_minutes(-4);
    assert_eq!(clock.to_string(), "23:59");

    let clock = Clock::new(10, 3).add_minutes(-70);
    assert_eq!(clock.to_string(), "08:53");

    let clock = Clock::new(10, 3).add_minutes(-3);
    assert_eq!(clock.to_string(), "10:00");

    let clock = Clock::new(10, 3).sub_minutes(3);
    assert_eq!(clock.to_string(), "10:00");

    let clock = Clock::new(10, 3).sub_minutes(-4);
    assert_eq!(clock.to_string(), "10:07");
}
