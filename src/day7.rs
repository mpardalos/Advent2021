use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use crate::Solution;

fn read_input(buf: &mut BufReader<File>) -> Vec<i32> {
    let line = buf.lines().next().unwrap().unwrap();
    line.split(",")
        .map(str::parse)
        .map(Result::unwrap)
        .collect()
}

pub struct Part1;
impl Solution for Part1 {
    const DAY: u8 = 7;
    const PART: u8 = 1;

    fn solve(buf: &mut BufReader<File>) -> String {
        let mut nums = read_input(buf);
        nums.sort();
        let median = nums[nums.len() / 2];
        let fuel: i32 = nums.iter().map(|n| (median - n).abs()).sum();
        format!("The crabs will need {} fuel to align at {}", fuel, median)
    }
}

pub struct Part2;

impl Part2 {
    fn cost(p1: i32, p2: i32) -> i32 {
        // (1 .. (p1-p2).abs()).sum()
        let max = (p1 - p2).abs();
        max * (max + 1) / 2
    }
}

impl Solution for Part2 {
    const DAY: u8 = 7;
    const PART: u8 = 2;

    fn solve(buf: &mut BufReader<File>) -> String {
        let mut nums = read_input(buf);
        nums.sort();
        let mintarget = (1..*nums.iter().max().unwrap())
            .min_by_key(|target| {
                nums.iter()
                    .map(|crab| Self::cost(*crab, *target))
                    .sum::<i32>()
            })
            .unwrap();

        let mincost = nums
            .iter()
            .map(|crab| Self::cost(*crab, mintarget))
            .sum::<i32>();

        format!(
            "The crabs will need {} fuel to align at {}",
            mincost, mintarget
        )
    }
}
