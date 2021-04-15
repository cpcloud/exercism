use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}

const HOURS_PER_DAY: i32 = 24;
const MINUTES_PER_HOUR: i32 = 60;

fn compute_minutes(minutes: i32) -> i32 {
    (MINUTES_PER_HOUR + minutes % MINUTES_PER_HOUR) % MINUTES_PER_HOUR
}

fn compute_hours(minutes: i32) -> i32 {
    (HOURS_PER_DAY
        + (f64::from(minutes) / f64::from(MINUTES_PER_HOUR)).floor() as i32 % HOURS_PER_DAY)
        % HOURS_PER_DAY
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let minutes = hours * MINUTES_PER_HOUR + minutes;
        Self {
            hours: compute_hours(minutes),
            minutes: compute_minutes(minutes),
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let minutes = self.hours * MINUTES_PER_HOUR + self.minutes + minutes;
        Self {
            hours: compute_hours(minutes),
            minutes: compute_minutes(minutes),
        }
    }
}
