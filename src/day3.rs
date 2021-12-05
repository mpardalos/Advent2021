use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use crate::Solution;

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

pub struct Part1;
impl Solution for Part1 {
    const DAY: u8 = 3;
    const PART: u8 = 1;

    fn solve(buf: &mut BufReader<File>) -> String {
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
}

fn vec_as_string(v: &Vec<char>) -> String {
    v.into_iter().collect::<String>()
}

pub struct Part2;
impl Solution for Part2 {
    const DAY: u8 = 3;
    const PART: u8 = 2;

    fn solve(buf: &mut BufReader<File>) -> String {
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
