#![allow(dead_code)]

use std::{
    fs::{self, File},
    io::BufReader,
    time::Instant,
};

// use core::fmt;
// use std::{
//     fmt::Display,
//     fs::{self, File},
//     io::{BufRead, BufReader},
//     time::Instant,
// };

mod day1 {
    use std::{
        fs::File,
        io::{BufRead, BufReader},
    };

    pub fn day1_p1(buf: &mut BufReader<File>) -> String {
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

    pub fn day1_p2(buf: &mut BufReader<File>) -> String {
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

mod day2 {
    use std::{
        fs::File,
        io::{BufRead, BufReader},
    };

    pub fn day2_p1(buf: &mut BufReader<File>) -> String {
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

    pub fn day2_p2(buf: &mut BufReader<File>) -> String {
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

mod day3 {
    use std::{
        fs::File,
        io::{BufRead, BufReader},
    };

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

    pub fn day3_p1(buf: &mut BufReader<File>) -> String {
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

    pub fn day3_p2(buf: &mut BufReader<File>) -> String {
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
}

mod day4 {
    use core::fmt;
    use std::{
        fmt::Display,
        fs::File,
        io::{BufRead, BufReader},
    };

    fn print_boards(boards: &Vec<[[(bool, i32); 5]; 5]>) {
        for board in boards {
            for line in board {
                for (status, num) in line {
                    if *status {
                        eprint!("[{:>2}] ", num);
                    } else {
                        eprint!("{:>4} ", num);
                    }
                }
                eprintln!();
            }
            eprintln!();
        }
    }

    #[derive(Debug)]
    enum Bingo {
        Row(usize),
        Column(usize),
    }

    impl Display for Bingo {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                Bingo::Row(row) => {
                    write!(f, "Row {}", row)
                }
                Bingo::Column(column) => {
                    write!(f, "Row {}", column)
                }
            }
        }
    }

    fn check_bingo<const N: usize>(board: &[[(bool, i32); N]; N]) -> Option<Bingo> {
        // Rows
        for (row_idx, row) in board.iter().enumerate() {
            if row.iter().all(|(status, _)| *status) {
                return Some(Bingo::Row(row_idx.try_into().unwrap()));
            }
        }

        // Columns
        for col_idx in 0..N {
            if board.iter().all(|r| r[col_idx].0) {
                return Some(Bingo::Column(col_idx.try_into().unwrap()));
            }
        }

        return None;
    }

    fn day4_read_input(buf: &mut BufReader<File>) -> (Vec<i32>, Vec<[[(bool, i32); 5]; 5]>) {
        let mut lines = buf.lines().map(|l| l.unwrap());

        let first_line = lines.next().unwrap();
        let sequence: Vec<i32> = first_line
            .split(",")
            .map(|s| s.parse::<i32>().unwrap())
            .collect();

        let mut boards: Vec<[[(bool, i32); 5]; 5]> = Vec::new();

        let mut board_line = 0;
        for line in lines {
            if line.is_empty() {
                board_line = 0;
                boards.push([[(false, 0); 5]; 5]);
                continue;
            }

            for (idx, num) in line
                .split_whitespace()
                .map(|n| n.parse::<i32>().unwrap())
                .enumerate()
            {
                boards.last_mut().unwrap()[board_line][idx] = (false, num);
            }
            board_line += 1;
        }

        return (sequence, boards);
    }

    pub fn day4_p1(buf: &mut BufReader<File>) -> String {
        let (sequence, mut boards) = day4_read_input(buf);

        for draw in sequence {
            for (status, num) in boards.iter_mut().flatten().flatten() {
                if *num == draw {
                    *status = true;
                }
            }

            // eprintln!("------------------------");
            // print_boards(&boards);

            for (board_num, board) in boards.iter().enumerate() {
                if let Some(location) = check_bingo(board) {
                    let score_sum: i32 = board
                        .iter()
                        .flatten()
                        .filter(|(status, _)| !*status)
                        .map(|(_, val)| val)
                        .sum();
                    return format!(
                        "Bingo! Board {} ({}) - Score = {} * {} = {}",
                        board_num,
                        location,
                        draw,
                        score_sum,
                        draw * score_sum
                    );
                }
            }
        }

        return format!("No bingo");
    }
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
        format!("{:>3}ms", duration.as_millis())
    } else {
        format!("{:>3}us", duration.as_micros())
    };

    format!(
        "[{}][Day {:>2}][Part {:>2}]: {}",
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
    solution(1, 1, day1::day1_p1);
    solution(1, 2, day1::day1_p2);

    solution(2, 1, day2::day2_p1);
    solution(2, 2, day2::day2_p2);

    solution(3, 1, day3::day3_p1);
    solution(3, 2, day3::day3_p2);

    solution(4, 1, day4::day4_p1);
}
