use yonsei_flexible::time::HMS;
use yonsei_flexible::cond::Interval;
use yonsei_flexible::calc::{
    calc_work_time,
    calc_minimum_finish_time,
    calc_total_exceed,
    calc_hope_finish_time
};
use dialoguer::{theme::ColorfulTheme, Select, Input};
use dont_disappear::any_key_to_continue;

fn main() {
    // 1. Select the type of calculation
    // 1) Work time
    // 2) Minimum finish time
    // 3) Calculate finish time
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select the type of calculation (Ctrl+c to exit)")
        .items(&["Work time", "Minimum finish time", "Calculate finish time"])
        .default(0)
        .interact()
        .unwrap();

    match selection {
        0 => {
            // Accumulate whole time
            let mut acc_time = HMS::new(0, 0, 0);

            loop {
                // Require input
                let start_input: String = Input::with_theme(&ColorfulTheme::default())
                    .with_prompt("Input start time as HH:MM or HH:MM:SS")
                    .interact()
                    .unwrap();
                let end_input: String = Input::with_theme(&ColorfulTheme::default())
                    .with_prompt("Input end time as HH:MM or HH:MM:SS")
                    .interact()
                    .unwrap();

                // Parse input as start time and end time
                // * Input format : HH:MM:SS - HH:MM:SS
                let start = start_input.trim();
                let end = end_input.trim();

                let start_time = HMS::parse(start).unwrap();
                let end_time = HMS::parse(end).unwrap();
                let work_time = display_worktime(start_time, end_time);
                acc_time = acc_time + work_time;
                println!("Accumulated time: {}", acc_time);
                println!("");
            }
        }
        1 => {
            // Require input
            let start_input: String = Input::with_theme(&ColorfulTheme::default())
                .with_prompt("Input start time as HH:MM or HH:MM:SS")
                .interact()
                .unwrap();

            let start_time = HMS::parse(start_input.trim()).unwrap();
            let min_finish_time = calc_minimum_finish_time(start_time);
            println!("Minimum finish time: {}", min_finish_time);

            any_key_to_continue::default();
        }
        2 => {
            // Require input
            let start_input: String = Input::with_theme(&ColorfulTheme::default())
                .with_prompt("Input start time as HH:MM or HH:MM:SS")
                .interact()
                .unwrap();
            let add_input: String = Input::with_theme(&ColorfulTheme::default())
                .with_prompt("Input additional time as HH:MM or HH:MM:SS")
                .interact()
                .unwrap();

            let start = start_input.trim();
            let additional = add_input.trim();

            let start_time = HMS::parse(start).unwrap();
            let additional_time = HMS::parse(additional).unwrap();
            let hope_finish_time = calc_hope_finish_time(start_time, additional_time);
            println!("Finish time: {}", hope_finish_time);

            any_key_to_continue::default();
        }
        _ => unreachable!()
    }
}

fn display_worktime(start_time: HMS, end_time: HMS) -> HMS {
    let interval = Interval::new(start_time, end_time);
    println!("");
    println!("==================================================");
    println!("Input: {} - {}", start_time, end_time);
    println!("Total time: {}", interval.get_duration());
    let (total_exceed, lunch, dinner) = calc_total_exceed(interval);
    println!("    Lunch break: {}", lunch.1);
    println!("    Dinner break: {}", dinner.1);
    println!("    Total break: {}", total_exceed);
    let work_time = calc_work_time(interval);
    println!("Work time: {}", work_time);
    println!("==================================================");
    work_time
}
