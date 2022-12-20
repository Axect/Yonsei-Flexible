use std::fmt::Display;
use std::ops::{Add, Sub};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HMS {
    pub hour: i8,
    pub minute: i8,
    pub second: i8,
}

impl HMS {
    pub fn new(hour: i8, minute: i8, second: i8) -> Self {
        HMS {
            hour,
            minute,
            second,
        }
    }

    pub fn parse(s: &str) -> Option<Self> {
        let mut parts = s.split(':');
        let hour = parts.next()?.parse().ok()?;
        let minute = parts.next()?.parse().ok()?;
        let second = match parts.next() {
            None => 0,
            Some(s) => s.parse().ok()?,
        };
        Some(HMS::new(hour, minute, second))
    }
}

impl PartialOrd for HMS {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(
            self.hour
                .cmp(&other.hour)
                .then(self.minute.cmp(&other.minute))
                .then(self.second.cmp(&other.second)),
        )
    }
}

impl Ord for HMS {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl Display for HMS {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:02}:{:02}:{:02}", self.hour, self.minute, self.second)
    }
}

impl Add for HMS {
    type Output = HMS;

    fn add(self, other: HMS) -> HMS {
        let mut hour = self.hour + other.hour;
        let mut minute = self.minute + other.minute;
        let mut second = self.second + other.second;

        if second >= 60 {
            second -= 60;
            minute += 1;
        }

        if minute >= 60 {
            minute -= 60;
            hour += 1;
        }

        HMS::new(hour, minute, second)
    }
}

impl Sub for HMS {
    type Output = HMS;

    fn sub(self, other: HMS) -> HMS {
        let mut hour = self.hour - other.hour;
        let mut minute = self.minute - other.minute;
        let mut second = self.second - other.second;

        if second < 0 {
            second += 60;
            minute -= 1;
        }

        if minute < 0 {
            minute += 60;
            hour -= 1;
        }

        HMS::new(hour, minute, second)
    }
}
