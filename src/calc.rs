use crate::time::HMS;
use crate::cond::{Interval, contains_lunch_break, contains_dinner_break};

pub type Meal = (bool, HMS);

pub fn calc_work_time(interval: Interval) -> HMS {
    if interval.start_time < HMS::new(6, 0, 0) {
        panic!("start time must be after 6:00");
    }
    if interval.end_time >= HMS::new(24, 0, 0) {
        panic!("end time must be before 24:00");
    }

    let mut work_time = interval.end_time - interval.start_time;
    let (total_exceed, _, _) = calc_total_exceed(interval);

    work_time = work_time - total_exceed;

    if work_time > HMS::new(12, 0, 0) {
        work_time = HMS::new(12, 0, 0);
    }

    work_time
}

pub fn calc_minimum_finish_time(start: HMS) -> HMS {
    calc_hope_finish_time(start, HMS::new(4,0,0))
}

pub fn calc_hope_finish_time(start: HMS, hope_work_time: HMS) -> HMS {
    let mut finish = start + hope_work_time;
    let mut work_time = calc_work_time(Interval::new(start, finish));

    loop {
        if work_time < hope_work_time {
            finish = finish + (hope_work_time - work_time);
            work_time = calc_work_time(Interval::new(start, finish));
        } else if work_time > hope_work_time {
            finish = finish - (work_time - hope_work_time);
            work_time = calc_work_time(Interval::new(start, finish));
        } else {
            break;
        }
    }

    finish
}

pub fn calc_total_exceed(interval: Interval) -> (HMS, Meal, Meal) {
    let mut total_exceed = HMS::new(0, 0, 0);
    let lunch = contains_lunch_break(interval);
    let dinner = contains_dinner_break(interval);
    let half_hour = HMS::new(0, 30, 0);

    match lunch {
        (true, exceed) => {
            total_exceed = total_exceed + exceed;
        }
        _ => (),
    }

    match dinner {
        (true, exceed) => {
            total_exceed = total_exceed + exceed;
        }
        _ => (),
    }

    total_exceed = if total_exceed < half_hour {
        half_hour
    } else {
        total_exceed
    };

    (total_exceed, lunch, dinner)
}
