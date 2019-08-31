const HOURS: i64 = 24;
const MINUTES_PER_HOUR: u16 = 60;
const MINUTES: i64 = HOURS * (MINUTES_PER_HOUR as i64);

fn modulus(x: i64, y: i64) -> i64 {
    ((x % y) + y) % y
}

#[derive(Debug, PartialEq, Eq)]
pub struct Clock {
    minutes: u16,
}

impl Clock {
    pub fn new(hours: i64, minutes: i64) -> Self {
        if let Some(hour_minutes) = hours.checked_mul(i64::from(MINUTES_PER_HOUR)) {
            if let Some(minutes) = minutes.checked_add(hour_minutes) {
                Self {
                    minutes: modulus(minutes, MINUTES) as u16,
                }
            } else {
                panic!("Too many minutes");
            }
        } else {
            panic!("Too many hours");
        }
    }

    pub fn add_minutes(&self, minutes: i64) -> Self {
        if let Some(minutes) = i64::from(self.minutes).checked_add(minutes) {
            Self {
                minutes: modulus(minutes, MINUTES) as u16,
            }
        } else {
            panic!("Too many minutes")
        }
    }
}

impl std::fmt::Display for Clock {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let hours = self.minutes / MINUTES_PER_HOUR;
        let minutes = self.minutes % MINUTES_PER_HOUR;
        write!(f, "{:02}:{:02}", hours, minutes)
    }
}
