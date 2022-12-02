use yonsei_flexible::{
    time::HMS,
    cond::Interval,
    calc::calc_work_time,
};

fn main() {
    let example_inputs = r#"
        10:00:00 - 17:00:00
        10:00:00 - 21:00:00
        11:15:00 - 17:00:00
        11:45:00 - 17:00:00
        12:15:00 - 17:00:00
        12:45:00 - 17:00:00
        11:15:00 - 18:15:00
        11:45:00 - 18:15:00
        12:15:00 - 18:15:00
        12:45:00 - 18:15:00
        11:15:00 - 18:45:00
        11:45:00 - 18:45:00
        12:15:00 - 18:45:00
        12:45:00 - 18:45:00
        11:15:00 - 19:15:00
        11:45:00 - 19:15:00
        12:15:00 - 19:15:00
        12:45:00 - 19:15:00
        11:15:00 - 19:45:00
        11:45:00 - 19:45:00
        12:15:00 - 19:45:00
        12:45:00 - 19:45:00
    "#;

    let inputs = example_inputs
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| {
            let mut iter = line.split("-");
            let start = iter.next().unwrap().trim();
            let end = iter.next().unwrap().trim();
            Interval::new(HMS::parse(start).unwrap(), HMS::parse(end).unwrap())
        })
        .collect::<Vec<_>>();

    for input in inputs {
        println!("==================================================");
        println!("Input: {} - {}", input.start_time, input.end_time);
        println!("Total time: {}", input.get_duration());
        println!("Work time: {}", calc_work_time(input));
        println!("==================================================");
        println!("");
    }
}
