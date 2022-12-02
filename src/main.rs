use yonsei_flexible::time::HMS;
use yonsei_flexible::cond::Interval;
use yonsei_flexible::calc::calc_work_time;
use std::io::{stdin, Write};

fn main() {
    // Require input
    println!("Input start time and end time as HH:MM:SS - HH:MM:SS");
    print!("> ");
    std::io::stdout().flush().unwrap();

    // Parse input as start time and end time
    // * Input format : HH:MM:SS - HH:MM:SS
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let mut input = input.split("-");
    let start = input.next().unwrap().trim();
    let end = input.next().unwrap().trim();

    let start_time = HMS::parse(start).unwrap();
    let end_time = HMS::parse(end).unwrap();

    let interval = Interval::new(start_time, end_time);

    println!("");
    println!("==================================================");
    println!("Input: {} - {}", start_time, end_time);
    println!("Total time: {}", interval.get_duration());
    println!("Work time: {}", calc_work_time(interval));
    println!("==================================================");
}
