use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use crate::Solution;

pub struct Part1;
impl Solution for Part1 {
    const DAY: u8 = 1;
    const PART: u8 = 1;
    fn solve(buf: &mut BufReader<File>) -> String {
        let mut increases = 0;
        let mut lines = buf.lines().map(|l| l.unwrap());

        let first_line = lines.next().unwrap();

        let mut last: i32 = first_line.parse().unwrap();

        for line in lines {
            let this: i32 = line.parse().unwrap();
            if this > last {
                increases += 1;
            }
            last = this;
        }

        format!("The value increases {} times", increases)
    }
}

pub struct Part2;
impl Solution for Part2 {
    const DAY: u8 = 1;
    const PART: u8 = 2;

    fn solve(buf: &mut BufReader<File>) -> String {
        let mut increases = 0;
        let lines: Vec<i32> = buf.lines().map(|l| l.unwrap().parse().unwrap()).collect();

        let mut last_sum = lines[0] + lines[1] + lines[2];

        let mut start_idx = 1;
        let mut end_idx = 4;

        while end_idx <= lines.len() {
            let this_sum: i32 = (&lines[start_idx..end_idx]).iter().sum();
            if this_sum > last_sum {
                increases += 1;
            }
            start_idx += 1;
            end_idx += 1;
            last_sum = this_sum;
        }

        format!("The sliding window value increases {} times", increases)
    }
}
