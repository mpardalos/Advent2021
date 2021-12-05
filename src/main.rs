#![allow(dead_code)]

use std::{
    fs::{self, File},
    io::BufReader,
    time::Instant,
};

mod day1 {
    use std::{
        fs::File,
        io::{BufRead, BufReader},
    };

    pub fn part1(buf: &mut BufReader<File>) -> String {
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

    pub fn part2(buf: &mut BufReader<File>) -> String {
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

    pub fn part1(buf: &mut BufReader<File>) -> String {
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

    pub fn part2(buf: &mut BufReader<File>) -> String {
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

    pub fn part1(buf: &mut BufReader<File>) -> String {
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

    pub fn part2(buf: &mut BufReader<File>) -> String {
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

    pub fn part1(buf: &mut BufReader<File>) -> String {
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

    pub fn part2(buf: &mut BufReader<File>) -> String {
        let (sequence, mut boards) = day4_read_input(buf);

        for draw in sequence {
            for (status, num) in boards.iter_mut().flatten().flatten() {
                if *num == draw {
                    *status = true;
                }
            }

            // eprintln!("------------------------");
            // print_boards(&boards);

            if boards.len() > 1 {
                // Drop won boards
                boards.retain(|board| check_bingo(board).is_none());
            } else if let Some(_) = check_bingo(&boards[0]) {
                let score_sum: i32 = boards[0]
                    .iter()
                    .flatten()
                    .filter(|(status, _)| !*status)
                    .map(|(_, val)| val)
                    .sum();
                return format!(
                    "Last bingo has score = {} * {} = {}",
                    draw,
                    score_sum,
                    draw * score_sum
                );
            }
        }

        return format!("No bingo");
    }
}

mod day5 {
    use core::fmt;
    use std::{
        cmp::{max, min},
        fmt::Display,
        fs::File,
        io::{BufRead, BufReader},
    };

    struct Point {
        x: u32,
        y: u32,
    }

    struct Line {
        start: Point,
        end: Point,
    }

    impl Line {
        fn is_horizontal(&self) -> bool {
            self.start.x == self.end.x
        }

        fn is_vertical(&self) -> bool {
            self.start.y == self.end.y
        }

        fn is_diagonal(&self) -> bool {
            (self.end.x as i32 - self.start.x as i32).abs()
                == (self.end.y as i32 - self.start.y as i32).abs()
        }
    }

    impl Display for Line {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(
                f,
                "{},{} -> {},{}",
                self.start.x, self.start.y, self.end.x, self.end.y
            )
        }
    }

    type Board<const N: usize> = [[u16; N]; N];

    fn print_board<const N: usize>(board: &Board<N>) {
        eprint!("  |");
        for col in 0..board[0].len() {
            eprint!("{}", col);
        }
        eprintln!("\n  +{}", "-".repeat(N));
        for (line_idx, line) in board.iter().enumerate() {
            eprint!("{:>2}|", line_idx);
            for count in line {
                // eprint!("{:>1}", col_idx);
                if *count > 0 {
                    eprint!("{}", count);
                } else {
                    eprint!(" ");
                }
            }
            eprintln!("");
        }
    }

    fn count_crossings<const N: usize>(board: &Board<N>) -> usize {
        board.clone().iter().flatten().filter(|x| **x > 1).count()
    }

    fn read_input(buf: &mut BufReader<File>) -> Vec<Line> {
        buf.lines()
            .map(|line| line.unwrap())
            .map(|line| {
                if let [start_x, start_y, end_x, end_y] = line
                    .split(" -> ")
                    .flat_map(|part| part.split(","))
                    .map(|s| s.parse::<u32>().unwrap())
                    .collect::<Vec<_>>()[..]
                {
                    Line {
                        start: Point {
                            x: start_x,
                            y: start_y,
                        },
                        end: Point { x: end_x, y: end_y },
                    }
                } else {
                    panic!("Could not parse line: {}", line);
                }
            })
            .collect()
    }

    pub fn part1<const N: usize>(buf: &mut BufReader<File>) -> String {
        let mut board: Box<Board<N>> = Box::new([[0; N]; N]);
        for line in read_input(buf) {
            if line.is_vertical() {
                let start = min(line.start.x, line.end.x);
                let end = max(line.start.x, line.end.x);
                for x in start..(end + 1) {
                    board[line.start.y as usize][x as usize] += 1;
                }
            } else if line.is_horizontal() {
                let start = min(line.start.y, line.end.y);
                let end = max(line.start.y, line.end.y);
                for y in start..(end + 1) {
                    board[y as usize][line.start.x as usize] += 1;
                }
            } else if line.is_diagonal() {
                // We don't handle diagonals in day 1
            }
        }

        format!(
            "There are {} spots where lines cross",
            count_crossings(&board)
        )
    }

    pub fn part1<const N: usize>(buf: &mut BufReader<File>) -> String {
        let mut board: Box<Board<N>> = Box::new([[0; N]; N]);
        for line in read_input(buf) {
            if line.is_vertical() {
                let start = min(line.start.x, line.end.x);
                let end = max(line.start.x, line.end.x);
                for x in start..(end + 1) {
                    board[line.start.y as usize][x as usize] += 1;
                }
            } else if line.is_horizontal() {
                let start = min(line.start.y, line.end.y);
                let end = max(line.start.y, line.end.y);
                for y in start..(end + 1) {
                    board[y as usize][line.start.x as usize] += 1;
                }
            } else if line.is_diagonal() {
                let (start, end) = if line.start.x < line.end.x {
                    (line.start, line.end)
                } else {
                    (line.end, line.start)
                };

                let y_step: i32 = if end.y > start.y { 1 } else { -1 };

                let mut x = start.x;
                let mut y = start.y;

                while x <= end.x {
                    board[y as usize][x as usize] += 1;
                    x += 1;
                    y = (y as i32 + y_step) as u32
                }
            } else {
                panic!("Weird line");
            }
        }

        format!(
            "There are {} spots where lines cross",
            count_crossings(&board)
        )
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
    solution(1, 1, day1::part1);
    solution(1, 2, day1::part2);

    solution(2, 1, day2::part1);
    solution(2, 2, day2::part2);

    solution(3, 1, day3::part1);
    solution(3, 2, day3::part2);

    solution(4, 1, day4::part1);
    solution(4, 2, day4::part2);

    solution(5, 1, day5::part1::<1024>);
    solution(5, 2, day5::part1::<1024>);
}
