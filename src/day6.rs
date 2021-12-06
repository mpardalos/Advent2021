use std::{
    fs,
    io::{self, BufRead},
};

use crate::Solution;

fn read_input(buf: &mut io::BufReader<fs::File>) -> Vec<u8> {
    let line = buf.lines().next().unwrap().unwrap();
    line.split(",")
        .map(str::parse::<u8>)
        .map(Result::unwrap)
        .collect()
}

fn special_lanternfish(initial: Vec<u8>, days: u16) -> String {
    let mut lanternfish = initial;
    for day in 0..days {
        let mut newfish = 0;
        for timer in lanternfish.iter_mut() {
            match timer {
                0 => {
                    newfish += 1;
                    *timer = 6;
                }
                _ => {
                    *timer -= 1;
                }
            }
        }
        for _ in 0..newfish {
            lanternfish.push(8);
        }
    }
    format!("{} lanternfish", lanternfish.len())
}

pub struct Part1;
impl Solution for Part1 {
    const DAY: u8 = 6;
    const PART: u8 = 1;

    fn solve(buf: &mut io::BufReader<std::fs::File>) -> String {
        special_lanternfish(read_input(buf), 80)
    }
}

pub struct Part2;
impl Solution for Part2 {
    const DAY: u8 = 6;
    const PART: u8 = 2;

    fn solve(buf: &mut io::BufReader<std::fs::File>) -> String {
        special_lanternfish(read_input(buf), 256)
    }
}
