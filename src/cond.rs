use crate::time::HMS;

#[derive(Debug, Clone, Copy)]
pub struct Interval {
    pub start_time: HMS,
    pub end_time: HMS,
}

impl Interval {
    pub fn new(start_time: HMS, end_time: HMS) -> Self {
        Self {
            start_time,
            end_time,
        }
    }

    pub fn get_duration(&self) -> HMS {
        self.end_time - self.start_time
    }

    pub fn contains(&self, time: HMS) -> bool {
        self.start_time <= time && time <= self.end_time
    }
}

pub fn contains_lunch_break(interval: Interval) -> (bool, HMS) {
    let lunch_break = Interval::new(HMS::new(11, 0, 0), HMS::new(13, 0, 0));

    let mut contains = false;
    let mut exceed = HMS::new(0, 0, 0);

    // 1. Check if the interval contains the lunch break
    if interval.contains(lunch_break.start_time) {
        if interval.contains(lunch_break.end_time) {
            contains = true;
            exceed = HMS::new(1,0,0);
        } else {
            // 2. Check if the difference is more than 1 hour
            let diff = interval.end_time - lunch_break.start_time;
            if diff >= HMS::new(1, 0, 0) {
                contains = true;
                exceed = diff - HMS::new(1, 0, 0);
            }
        }
    } else if interval.contains(lunch_break.end_time) {
        // 3. Check if the difference is more than 1 hour
        let diff = lunch_break.end_time - interval.start_time;
        if diff >= HMS::new(1, 0, 0) {
            contains = true;
            exceed = diff - HMS::new(1, 0, 0);
        }
    }

    (contains, exceed)
}

pub fn contains_dinner_break(interval: Interval) -> (bool, HMS) {
    let dinner_break = Interval::new(HMS::new(18, 0, 0), HMS::new(20, 0, 0));

    let mut contains = false;
    let mut exceed = HMS::new(0, 0, 0);

    // 1. Check if the interval contains the dinner break
    if interval.contains(dinner_break.start_time) {
        if interval.contains(dinner_break.end_time) {
            contains = true;
            exceed = HMS::new(1,0,0);
        } else {
            // 2. Check if the difference is more than 1 hour
            let diff = interval.end_time - dinner_break.start_time;
            if diff >= HMS::new(1, 0, 0) {
                contains = true;
                exceed = diff - HMS::new(1, 0, 0);
            }
        }
    } else if interval.contains(dinner_break.end_time) {
        // 3. Check if the difference is more than 1 hour
        let diff = dinner_break.end_time - interval.start_time;
        if diff >= HMS::new(1, 0, 0) {
            contains = true;
            exceed = diff - HMS::new(1, 0, 0);
        }
    }

    (contains, exceed)
}
