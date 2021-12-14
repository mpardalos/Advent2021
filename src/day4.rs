use ansi_term::{Colour, Style};
use core::fmt;
use std::{fmt::Display, io::BufRead};

use crate::{Extra, Solution};

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

fn read_input(buf: &mut impl BufRead) -> (Vec<i32>, Vec<[[(bool, i32); 5]; 5]>) {
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

pub struct Part1;
impl Solution for Part1 {
    const DAY: u8 = 4;
    const PART: u8 = 1;

    fn solve(buf: &mut impl BufRead) -> String {
        let (sequence, mut boards) = read_input(buf);

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

pub struct Part2;
impl Solution for Part2 {
    const DAY: u8 = 4;
    const PART: u8 = 2;

    fn solve(buf: &mut impl BufRead) -> String {
        let (sequence, mut boards) = read_input(buf);

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

pub struct Visualise;
impl Extra for Visualise {
    const DAY: u8 = 4;
    const USE_SAMPLE: bool = true;

    fn run(buf: &mut impl BufRead) {
        let normal: Style = Style::new().dimmed();
        let marked: Style = Style::new().fg(Colour::Cyan);
        let just_marked: Style = Style::new().underline().bold().fg(Colour::Red);

        let (sequence, mut boards) = read_input(buf);
        let mut input = String::new();

        for draw in sequence {
            for (status, num) in boards.iter_mut().flatten().flatten() {
                if *num == draw {
                    *status = true;
                }
            }

            // Clear the screen
            print!("\x1B[2J\n");
            for board in &boards {
                for line in board {
                    for (status, num) in line {
                        let text = format!("{:>2}", num);
                        let style = if *status && *num == draw {
                            just_marked
                        } else if *status {
                            marked
                        } else {
                            normal
                        };
                        print!("{} ", style.paint(text));
                    }
                    println!();
                }
                println!();
            }
            std::io::stdin().read_line(&mut input).unwrap();
        }
    }
}
