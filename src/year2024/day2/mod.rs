use crate::utils;

enum Direction {
    None,
    Asc,
    Desc,
}

#[allow(dead_code)]
pub fn run_part_one(is_test: bool) -> usize {
    let mut input = String::new();
    utils::get_input(2024, 2, is_test, &mut input);

    let mut reports = Vec::new();
    for line in input.lines() {
        let report = line
            .split(' ')
            .map(|x| x.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();
        if validate_line(&report) {
            reports.push(true);
        }
    }

    reports.len()
}

pub fn run(is_test: bool) -> usize {
    let mut input = String::new();
    utils::get_input(2024, 2, is_test, &mut input);

    let mut reports = Vec::new();
    for line in input.lines() {
        let report = line
            .split(' ')
            .map(|x| x.parse::<usize>().unwrap())
            .collect();
        if validate_line(&report) {
            reports.push(true);
        } else {
            for sub_report in get_sub_reports(report) {
                if validate_line(&sub_report) {
                    reports.push(true);
                    break;
                }
            }
        }
    }

    reports.len()
}

fn get_sub_reports(report: Vec<usize>) -> Vec<Vec<usize>> {
    let mut reports = Vec::new();
    for i in 0..report.len() {
        let mut report_copy = report.clone();
        report_copy.remove(i);
        reports.push(report_copy);
    }

    reports
}

fn validate_line(report: &Vec<usize>) -> bool {
    let mut direction = Direction::None;
    let mut prev = None;
    let mut valid = true;
    for result in report {
        match prev {
            None => {}
            Some(prev_value) => {
                let difference = result.abs_diff(prev_value);
                if difference < 1 || difference > 3 {
                    valid = false;
                    break;
                }

                match direction {
                    Direction::None => {
                        direction = if *result > prev_value {
                            Direction::Asc
                        } else {
                            Direction::Desc
                        }
                    }
                    Direction::Asc => {
                        if *result < prev_value {
                            valid = false;
                            break;
                        }
                    }
                    Direction::Desc => {
                        if *result > prev_value {
                            valid = false;
                            break;
                        }
                    }
                }
            }
        }

        prev = Some(*result);
    }

    valid
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one() {
        let result = run_part_one(true);
        assert_eq!(result, 2);
    }

    #[test]
    fn part_two() {
        let result = run(true);
        assert_eq!(result, 4)
    }
}
