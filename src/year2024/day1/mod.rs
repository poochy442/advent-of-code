use std::{collections::HashMap, iter};

use crate::utils;

#[derive(Debug)]
struct SortedList<T: Ord + Copy> {
    list: Vec<T>,
    index: usize,
}

impl<T: Ord + Copy> SortedList<T> {
    fn new() -> Self {
        Self {
            list: Vec::new(),
            index: 0,
        }
    }

    fn insert(&mut self, value: T) {
        let len = self.list.len();
        if len == 0 {
            self.list.push(value);
            return;
        }

        let mut i = 0;
        while i < len && self.list[i] < value {
            i += 1;
        }
        if i >= len {
            self.list.push(value)
        } else {
            self.list.insert(i, value);
        }
    }
}

impl<T: Ord + Copy> Iterator for SortedList<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.list.len() {
            None
        } else {
            let val = self.list[self.index];
            self.index += 1;
            Some(val)
        }
    }
}

#[allow(dead_code)]
pub fn run_part_one(is_test: bool) -> usize {
    let mut input = String::new();
    utils::get_input(2024, 1, is_test, &mut input);

    let mut left = SortedList::<usize>::new();
    let mut right = SortedList::<usize>::new();
    for line in input.lines() {
        let split: Vec<&str> = line.split("   ").collect();
        left.insert(split[0].parse::<usize>().unwrap());
        right.insert(split[1].parse::<usize>().unwrap());
    }

    let mut result = 0;
    for (l, r) in iter::zip(left, right) {
        result += l.abs_diff(r);
    }
    result
}

pub fn run(is_test: bool) -> usize {
    let mut input = String::new();
    utils::get_input(2024, 1, is_test, &mut input);

    let mut left = Vec::<usize>::new();
    let mut right = HashMap::<usize, usize>::new();
    for line in input.lines() {
        let split: Vec<&str> = line.split("   ").collect();
        let l = split[0].parse::<usize>().unwrap();
        left.push(l);

        let r = split[1].parse::<usize>().unwrap();
        if right.contains_key(&r) {
            let count = right.get_mut(&r).unwrap();
            *count += 1;
        } else {
            right.insert(r, 1);
        }
    }

    let mut result = 0;
    for entry in left {
        if !right.contains_key(&entry) {
            continue;
        }

        result += entry * right[&entry];
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one() {
        let result = run_part_one(true);
        assert_eq!(result, 11);
    }

    #[test]
    fn part_two() {
        let result = run(true);
        assert_eq!(result, 31)
    }
}
