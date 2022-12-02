use crate::time::HMS;
use crate::cond::{Interval, contains_lunch_break, contains_dinner_break};

pub fn calc_work_time(interval: Interval) -> HMS {
    if interval.start_time < HMS::new(6, 0, 0) {
        panic!("start time must be after 6:00");
    }
    if interval.end_time >= HMS::new(24, 0, 0) {
        panic!("end time must be before 24:00");
    }

    let mut work_time = interval.end_time - interval.start_time;
    let mut total_exceed = HMS::new(0, 0, 0);
    let lunch = contains_lunch_break(interval);
    let dinner = contains_dinner_break(interval);
    let half_hour = HMS::new(0, 30, 0);

    match lunch {
        (true, exceed) => {
            println!("\tLunch break: {}", exceed);
            total_exceed = total_exceed + exceed;
        }
        _ => {
            println!("\tNo Lunch break");
        }
    }

    match dinner {
        (true, exceed) => {
            println!("\tDinner break: {}", exceed);
            total_exceed = total_exceed + exceed;
        }
        _ => {
            println!("\tNo Dinner break");
        }
    }

    if total_exceed < half_hour {
        println!("\tTotal break: {} is less than half hour", total_exceed);
        work_time = work_time - half_hour;
    } else {
        work_time = work_time - total_exceed;
    }

    if work_time > HMS::new(12, 0, 0) {
        println!("\tWork time: {} can't larger than 12 hours", work_time);
        work_time = HMS::new(12, 0, 0);
    }

    work_time
}
