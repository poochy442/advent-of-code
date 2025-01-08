use crate::utils;

struct Expression {
    left: usize,
    right: usize,
}

impl Expression {
    fn calculate(self) -> usize {
        return self.left * self.right;
    }
}

#[allow(dead_code)]
pub fn run(is_test: bool) -> usize {
    let mut input = String::new();
    utils::get_input(2024, 3, is_test, &mut input);

    input
        .lines()
        .flat_map(|x| parse_expressions(x))
        .map(|x| x.calculate())
        .sum()
}

fn parse_expressions(line: &str) -> Vec<Expression> {
    let mut expressions: Vec<Expression> = Vec::<Expression>::new();

    let mut working_line = line;
    while let Some(index) = working_line.find("mul(") {
        working_line = working_line.get(index + 4..).unwrap();
        if let Some(s) = find_expression(working_line) {
            expressions.push(parse_expression(s));
        }
    }

    expressions
}

fn find_expression(line: &str) -> Option<&str> {
    let mut end = None;
    let mut sep_found = false;
    for (i, c) in line.chars().enumerate() {
        match c {
            '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' | '0' => {}
            ',' => {
                if sep_found {
                    return None;
                } else {
                    sep_found = true
                }
            }
            ')' => {
                if sep_found {
                    end = Some(i);
                    break;
                } else {
                    return None;
                }
            }
            _ => return None,
        }
    }

    if let Some(end_index) = end {
        let expr = line.get(..end_index).unwrap();
        Some(expr)
    } else {
        None
    }
}

fn parse_expression(s: &str) -> Expression {
    let split: Vec<&str> = s.split(',').collect();
    let left = split[0].parse::<usize>().unwrap();
    let right = split[1].parse::<usize>().unwrap();

    return Expression { left, right };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one() {
        let result = run(true);
        assert_eq!(result, 161);
    }

    // #[test]
    // fn part_two() {
    //     let result = run(true);
    //     assert_eq!(result, 4)
    // }
}
