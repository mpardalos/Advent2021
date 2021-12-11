use std::io::BufRead;

use crate::Solution;

fn read_input(buf: &mut impl BufRead) -> Vec<Vec<char>> {
    buf.lines()
        .map(Result::unwrap)
        .map(|line| line.chars().collect())
        .collect()
}

fn matching_bracket(c: char) -> char {
    match c {
        '(' => ')',
        '[' => ']',
        '{' => '}',
        '<' => '>',

        ')' => '(',
        ']' => '[',
        '}' => '{',
        '>' => '<',
        _ => panic!("Not a bracket: {}", c),
    }
}

fn score_corrupt_bracket(c: char) -> u64 {
    match c {
        '(' | ')' => 3,
        '[' | ']' => 57,
        '{' | '}' => 1197,
        '<' | '>' => 25137,
        _ => panic!("Not a bracket: {}", c),
    }
}

fn score_missing_bracket(c: char) -> u64 {
    match c {
        '(' | ')' => 1,
        '[' | ']' => 2,
        '{' | '}' => 3,
        '<' | '>' => 4,
        _ => panic!("Not a bracket: {}", c),
    }
}

pub struct Part1;
impl Solution for Part1 {
    const DAY: u8 = 10;
    const PART: u8 = 1;

    fn solve(buf: &mut impl BufRead) -> String {
        let mut score: u64 = 0;
        for line in read_input(buf) {
            let mut bracket_stack: Vec<char> = Vec::new();

            for c in line {
                match c {
                    '(' | '[' | '{' | '<' => bracket_stack.push(c),
                    ')' | ']' | '}' | '>' => {
                        if bracket_stack.pop().unwrap() != matching_bracket(c) {
                            score += score_corrupt_bracket(c);
                        }
                    }
                    _ => panic!("Not a bracket: {}", c),
                }
            }
        }
        format!("The total score is {}", score)
    }
}

pub struct Part2;
impl Solution for Part2 {
    const DAY: u8 = 10;
    const PART: u8 = 2;

    fn solve(buf: &mut impl BufRead) -> String {
        let mut line_scores: Vec<u64> = Vec::new();

        'all_lines: for line in read_input(buf) {
            let mut bracket_stack: Vec<char> = Vec::new();

            for c in line {
                match c {
                    '(' | '[' | '{' | '<' => bracket_stack.push(c),
                    ')' | ']' | '}' | '>' => {
                        if bracket_stack.pop().unwrap() != matching_bracket(c) {
                            continue 'all_lines;
                        }
                    }
                    _ => panic!("Not a bracket: {}", c),
                }
            }

            line_scores.push(
                bracket_stack
                    .iter()
                    .rev()
                    .fold(0, |score, c| score * 5 + score_missing_bracket(*c)),
            );
        }

        line_scores.sort();
        let middle_score = line_scores[line_scores.len() / 2];

        format!("The total score is {}", middle_score)
    }
}
