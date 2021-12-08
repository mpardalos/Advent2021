use std::io::BufRead;

use crate::Solution;

fn read_input(buf: &mut impl BufRead) -> Vec<u8> {
    let line = buf.lines().next().unwrap().unwrap();
    line.split(",")
        .map(str::parse::<u8>)
        .map(Result::unwrap)
        .collect()
}

fn special_lanternfish(initial: Vec<u8>, days: u16) -> String {
    let mut lanternfish = initial;
    for _day in 0..days {
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

fn fishbuckets(initial: Vec<u8>, days: u16) -> String {
    let mut buckets = [0 as u64; 9];
    for timer in initial {
        buckets[timer as usize] += 1;
    }

    for _day in 1..days + 1 {
        let to_multiply = buckets[0];
        for i in 0..buckets.len() - 1 {
            buckets[i] = buckets[i + 1];
        }

        buckets[6] += to_multiply;
        buckets[8] = to_multiply;
    }
    format!("{} lanternfish", buckets[..].iter().sum::<u64>())
}

pub struct Part1;
impl Solution for Part1 {
    const DAY: u8 = 6;
    const PART: u8 = 1;

    fn solve(buf: &mut impl BufRead) -> String {
        fishbuckets(read_input(buf), 80)
    }
}

pub struct Part2;
impl Solution for Part2 {
    const DAY: u8 = 6;
    const PART: u8 = 2;

    fn solve(buf: &mut impl BufRead) -> String {
        fishbuckets(read_input(buf), 256)
    }
}
