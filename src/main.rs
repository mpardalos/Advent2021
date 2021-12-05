#![allow(dead_code)]

use std::{
    fs::{self, File},
    io::{BufRead, BufReader},
    time::Instant,
};

// Day 1 ----------------------------------------------------------------------------------

fn day1_p1(buf: &mut BufReader<File>) -> String {
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

fn day1_p2(buf: &mut BufReader<File>) -> String {
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

// Day 2 ----------------------------------------------------------------------------------

fn day2_p1(buf: &mut BufReader<File>) -> String {
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

fn day2_p2(buf: &mut BufReader<File>) -> String {
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

// Day 3 ----------------------------------------------------------------------------------
fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>> {
    assert!(!v.is_empty());
    let len = v[0].len();
    let mut iters: Vec<_> = v.into_iter().map(|n| n.into_iter()).collect();
    (0..len)
        .map(|_| {
            iters
                .iter_mut()
                .map(|n| n.next().unwrap())
                .collect::<Vec<T>>()
        })
        .collect()
}

fn find_most_common(column: &Vec<char>) -> char {
    let ones = column.iter().fold(0, |acc, c| match c {
        '0' => acc,
        '1' => acc + 1,
        _ => {
            panic!("Unexpected character: {}", c)
        }
    });
    if ones as f32 >= column.len() as f32 / 2.0 {
        '1'
    } else {
        '0'
    }
}

fn day3_p1(buf: &mut BufReader<File>) -> String {
    let lines: Vec<Vec<char>> = buf.lines().map(|l| l.unwrap().chars().collect()).collect();

    let most_common: Vec<char> = transpose(lines).iter().map(find_most_common).collect();

    let gamma_str: String = most_common.clone().into_iter().collect();
    let epsilon_str: String = most_common
        .into_iter()
        .map(|c| match c {
            '1' => '0',
            '0' => '1',
            _ => {
                panic!("Unexpected character: {}", c)
            }
        })
        .collect();

    let gamma = isize::from_str_radix(&gamma_str, 2).unwrap();
    let epsilon = isize::from_str_radix(&epsilon_str, 2).unwrap();

    format!(
        "Gamma={}, Epsilon={}, Power Consumption={}",
        gamma,
        epsilon,
        gamma * epsilon
    )
}

fn vec_as_string(v: &Vec<char>) -> String {
    v.into_iter().collect::<String>()
}

fn day3_p2(buf: &mut BufReader<File>) -> String {
    let lines: Vec<Vec<char>> = buf.lines().map(|l| l.unwrap().chars().collect()).collect();

    let mut oxygen_lines = lines.clone();

    for bit_idx in 0..oxygen_lines[0].len() {
        let columns = transpose(oxygen_lines.clone());
        let most_common_vec: Vec<char> = columns.iter().map(find_most_common).collect();
        let most_common = most_common_vec[bit_idx];

        oxygen_lines.retain(|l| l[bit_idx] == most_common);
        if oxygen_lines.len() <= 1 {
            break;
        }
    }

    let mut co2_lines = lines.clone();

    for bit_idx in 0..co2_lines[0].len() {
        let most_common_vec: Vec<char> = transpose(co2_lines.clone())
            .iter()
            .map(find_most_common)
            .collect();
        let most_common = most_common_vec[bit_idx];

        co2_lines.retain(|l| l[bit_idx] != most_common);
        if co2_lines.len() <= 1 {
            break;
        }
    }

    let oxygen_rating: isize = if oxygen_lines.len() == 1 {
        let bitstr: String = (oxygen_lines[0].clone()).into_iter().collect();
        isize::from_str_radix(&bitstr, 2).unwrap()
    } else {
        return "Could not determine oxygen rating".to_string();
    };

    let co2_rating: isize = if co2_lines.len() == 1 {
        let bitstr: String = (co2_lines[0].clone()).into_iter().collect();
        isize::from_str_radix(&bitstr, 2).unwrap()
    } else {
        return "Could not determine co2 rating".to_string();
    };

    format!(
        "Oxygen rating: {} | CO2 rating: {} | Life support rating: {}",
        oxygen_rating,
        co2_rating,
        oxygen_rating * co2_rating
    )
}

// Runner  --------------------------------------------------------------------------------

fn solution_with_file_format<F>(day: i32, part: i8, solver: F, filepath: &String) -> String
where
    F: Fn(&mut BufReader<File>) -> String,
{
    let file = fs::File::open(filepath).expect("Could not read file");

    let before = Instant::now();
    let answer = solver(&mut BufReader::new(file));
    let after = Instant::now();

    let duration = after - before;
    let duration_display = if duration.as_millis() > 0 {
        format!("{}ms", duration.as_millis())
    } else {
        format!("{}us", duration.as_micros())
    };

    format!(
        "[{}] Day {} - Part {}: {}",
        duration_display, day, part, answer
    )
}

fn solution<F>(day: i32, part: i8, solver: F)
where
    F: Fn(&mut BufReader<File>) -> String,
{
    println!(
        "{}",
        solution_with_file_format(day, part, solver, &format!("inputs/{}", day))
    );
}

fn solution_with_sample<F>(day: i32, part: i8, solver: F)
where
    F: Fn(&mut BufReader<File>) -> String,
{
    println!(
        "[SAMPLE] {}",
        solution_with_file_format(day, part, solver, &format!("inputs/{}_sample", day))
    );
}

fn main() {
    solution(1, 1, day1_p1);
    solution(1, 2, day1_p2);
    solution(2, 1, day2_p1);
    solution(2, 2, day2_p2);
    solution(3, 1, day3_p1);
    solution(3, 2, day3_p2);
}
