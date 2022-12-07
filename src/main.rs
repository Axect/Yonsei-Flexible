use yonsei_flexible::time::HMS;
use yonsei_flexible::cond::Interval;
use yonsei_flexible::calc::calc_work_time;
use std::io::{stdin, Write};

fn main() {
    // Accumulate whole time
    let mut acc_time = HMS::new(0, 0, 0);

    loop {
        // Require input
        println!("Input start time and end time as HH:MM:SS - HH:MM:SS (Ctrl+c to exit)");
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
        let work_time = display_worktime(start_time, end_time);
        acc_time = acc_time + work_time;
        println!("Accumulated time: {}", acc_time);
        println!("");
    }
}

fn display_worktime(start_time: HMS, end_time: HMS) -> HMS {
    let interval = Interval::new(start_time, end_time);
    println!("");
    println!("==================================================");
    println!("Input: {} - {}", start_time, end_time);
    println!("Total time: {}", interval.get_duration());
    let work_time = calc_work_time(interval);
    println!("Work time: {}", work_time);
    println!("==================================================");
    work_time
}
