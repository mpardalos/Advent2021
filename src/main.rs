use std::{
    fs::{self, File},
    io::{BufRead, BufReader},
};

// Day 1 ----------------------------------------------------------------------------------

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

// Day 2 ----------------------------------------------------------------------------------

fn day2_p1(buf: &mut BufReader<File>) {
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

    println!(
        "Day 2 - Part 1: Horizontal={}, Depth={}, Product={}",
        horizontal,
        depth,
        horizontal * depth
    );
}

fn day2_p2(buf: &mut BufReader<File>) {
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

    println!(
        "Day 2 - Part 2: Horizontal={}, Depth={}, Product={}",
        horizontal,
        depth,
        horizontal * depth
    );
}
}

// Utility --------------------------------------------------------------------------------

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
    solution(2, day2_p1);
    solution(2, day2_p2);
}
