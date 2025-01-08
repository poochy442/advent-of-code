pub mod day1;
pub mod day2;
pub mod day3;

pub fn run(day: usize) {
    match day {
        1 => {
            let result = day1::run(false);
            println!("2024/Day1 result: {}", result);
        }
        2 => {
            let result = day2::run(false);
            println!("2024/Day2 result: {}", result);
        }
        3 => {
            let result = day3::run(false);
            println!("2024/Day3 result: {}", result);
        }
        _ => {
            panic!("Day {} of 2024 has not been implemented", day);
        }
    }
}
