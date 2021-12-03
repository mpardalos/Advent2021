use std::{
    fs::{self, File},
    io::{BufRead, BufReader},
};

fn day1_p1(buf: &mut BufReader<File>) {
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

    println!("Day 1: The value increases {} times", increases);
}

fn day1_p2(buf: &mut BufReader<File>) {
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

    println!(
        "Day 1: The sliding window value increases {} times",
        increases
    );
}

fn solution<F>(day: i32, solver: F)
where
    F: Fn(&mut BufReader<File>),
{
    let file = fs::File::open(format!("inputs/{}", day)).expect("Could not read file");
    solver(&mut BufReader::new(file));
}

fn main() {
    solution(1, day1_p1);
    solution(1, day1_p2);
}
