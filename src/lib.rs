mod utils;
mod year2024;

pub fn run(year: usize, day: usize) {
    match year {
        2024 => {
            year2024::run(day);
        }
        _ => {
            panic!("Year {} has not been implemented!", year)
        }
    }
}
