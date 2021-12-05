use core::fmt;
use std::{
    cmp::{max, min},
    fmt::Display,
    fs::File,
    io::{BufRead, BufReader},
};

use crate::Solution;

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

pub struct Part1<const N: usize>;
impl<const N: usize> Solution for Part1<N> {
    const DAY: u8 = 5;
    const PART: u8 = 1;

    fn solve(buf: &mut BufReader<File>) -> String {
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
}

pub struct Part2<const N: usize>;
impl<const N: usize> Solution for Part2<N> {
    const DAY: u8 = 5;
    const PART: u8 = 1;

    fn solve(buf: &mut BufReader<File>) -> String {
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
