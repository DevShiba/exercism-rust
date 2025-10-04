use std::fmt;

const MINUTES_IN_HOUR: i32 = 60;
const MINUTES_IN_DAY: i32 = 24 * MINUTES_IN_HOUR;

#[derive(Debug, PartialEq, Eq)]
pub struct Clock {
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let total_minutes = (hours * MINUTES_IN_HOUR + minutes).rem_euclid(MINUTES_IN_DAY);
        Clock {
            minutes: total_minutes,
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Clock::new(0, self.minutes + minutes)
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let hours = self.minutes / MINUTES_IN_HOUR;
        let minutes = self.minutes % MINUTES_IN_HOUR;
        write!(f, "{:02}:{:02}", hours, minutes)
    }
}