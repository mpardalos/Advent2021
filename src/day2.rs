use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use crate::Solution;

pub struct Part1;
impl Solution for Part1 {
    const DAY: u8 = 2;
    const PART: u8 = 1;

    fn solve(buf: &mut BufReader<File>) -> String {
        let mut depth = 0;
        let mut horizontal = 0;

        for line in buf.lines().map(|l| l.unwrap()) {
            if let [command, count_str] = line.split(' ').collect::<Vec<&str>>()[..] {
                let count: i32 = count_str.parse().unwrap();
                match command {
                    "forward" => {
                        horizontal += count;
                    }
                    "down" => {
                        depth += count;
                    }
                    "up" => {
                        depth -= count;
                    }
                    _ => {
                        panic!("Invalid command {}", command);
                    }
                }
            }
        }

        format!(
            "Horizontal={}, Depth={}, Product={}",
            horizontal,
            depth,
            horizontal * depth
        )
    }
}

pub struct Part2;
impl Solution for Part2 {
    const DAY: u8 = 2;
    const PART: u8 = 2;

    fn solve(buf: &mut BufReader<File>) -> String {
        let mut depth = 0;
        let mut horizontal = 0;
        let mut aim = 0;

        for line in buf.lines().map(|l| l.unwrap()) {
            if let [command, count_str] = line.split(' ').collect::<Vec<&str>>()[..] {
                let count: i32 = count_str.parse().unwrap();
                match command {
                    "forward" => {
                        horizontal += count;
                        depth += count * aim;
                    }
                    "down" => {
                        aim += count;
                    }
                    "up" => {
                        aim -= count;
                    }
                    _ => {
                        panic!("Invalid command {}", command);
                    }
                }
            }
        }

        format!(
            "Horizontal={}, Depth={}, Product={}",
            horizontal,
            depth,
            horizontal * depth
        )
    }
}
