use std::{io::BufRead, thread::sleep, time::Duration};

use ansi_term::{Colour, Style};

use crate::{Extra, Solution};

type HeightMap = Vec<Vec<u8>>;
type Basin = Vec<(usize, usize)>;

fn read_input(buf: &mut impl BufRead) -> HeightMap {
    buf.lines()
        .map(Result::unwrap)
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect()
        })
        .collect()
}

fn neighbours<'a>(
    map: &'a HeightMap,
    row: usize,
    col: usize,
) -> impl Iterator<Item = ((usize, usize), &'a u8)> + 'a {
    if let Some(_) = map.get(row).and_then(|r| r.get(col)) {
        let north = if row > 0 { Some((row - 1, col)) } else { None };
        let west = if col > 0 { Some((row, col - 1)) } else { None };
        let south = if row < map.len() - 1 {
            Some((row + 1, col))
        } else {
            None
        };
        let east = if col < map[0].len() - 1 {
            Some((row, col + 1))
        } else {
            None
        };

        vec![north, west, south, east]
    } else {
        vec![]
    }
    .into_iter()
    .flatten()
    .map(|(nrow, ncol)| ((nrow, ncol), &map[nrow][ncol]))
}

fn has_basin_at(map: &HeightMap, row: usize, col: usize) -> bool {
    neighbours(map, row, col)
        // Check that they are all heigher than [row][col]
        .all(|((_nrow, _ncol), neighbour)| map[row][col] < *neighbour)
}

pub struct Part1;
impl Solution for Part1 {
    const DAY: u8 = 9;
    const PART: u8 = 1;

    fn solve(buf: &mut impl BufRead) -> String {
        let grid = read_input(buf);
        let mut risk_level: u32 = 0;

        for (row_idx, row) in grid.iter().enumerate() {
            for (col_idx, height) in row.iter().enumerate() {
                if has_basin_at(&grid, row_idx, col_idx) {
                    risk_level += *height as u32 + 1;
                }
            }
        }

        format!("The risk level is: {}", risk_level)
    }
}

fn map_basin(
    map: &HeightMap,
    row: usize,
    col: usize,
    basin_locations: &mut Vec<(usize, usize)>,
    print_cb: &impl Fn(&Basin),
) {
    if map[row][col] >= 9 {
        return;
    }

    basin_locations.push((row, col));

    print_cb(basin_locations);

    for ((nrow, ncol), neighbour) in neighbours(map, row, col) {
        if basin_locations.contains(&(nrow, ncol)) {
            // Skip locations we've already mapped
            continue;
        }

        if *neighbour < 9 {
            map_basin(map, nrow, ncol, basin_locations, print_cb)
        }
    }
}

fn print_basin(map: &HeightMap, basin: &Basin) {
    let normal: Style = Style::new().dimmed();
    let bottom: Style = Style::new().bold().fg(Colour::Red);
    let in_basin: Style = Style::new().bold().fg(Colour::Cyan);

    for (row_idx, row) in map.iter().enumerate() {
        for (col_idx, height) in row.iter().enumerate() {
            let style = if basin[0] == (row_idx, col_idx) {
                bottom
            } else if basin.contains(&(row_idx, col_idx)) {
                in_basin
            } else {
                normal
            };
            eprint!("{}", style.paint(format!("{}", height)))
        }
        eprintln!("");
    }
}

pub struct Part2;
impl Solution for Part2 {
    const DAY: u8 = 9;
    const PART: u8 = 1;

    fn solve(buf: &mut impl BufRead) -> String {
        let map = read_input(buf);
        let mut basins: Vec<Basin> = vec![];

        for (row_idx, row) in map.iter().enumerate() {
            for (col_idx, _height) in row.iter().enumerate() {
                if has_basin_at(&map, row_idx, col_idx) {
                    let mut basin = vec![];
                    map_basin(&map, row_idx, col_idx, &mut basin, &|_| {});
                    basins.push(basin);
                }
            }
        }

        basins.sort_by_key(Vec::len);
        // basins.iter().rev().for_each(|basin| {
        //     eprintln!("---");
        //     print_basin(&map, &basin);
        //     eprintln!("---");
        // });
        format!(
            "The top 3 basins' sizes multiplied together give: {}",
            basins.iter().rev().take(3).map(Vec::len).product::<usize>()
        )
    }
}

pub struct Progression;
impl Extra for Progression {
    const DAY: u8 = 9;
    const USE_SAMPLE: bool = false;

    fn run(buf: &mut impl BufRead) {
        let map = read_input(buf);

        for (row_idx, row) in map.iter().enumerate() {
            for (col_idx, _height) in row.iter().enumerate() {
                if has_basin_at(&map, row_idx, col_idx) {
                    let mut basin = vec![];
                    map_basin(&map, row_idx, col_idx, &mut basin, &|current_basin| {
                        // Clear the screen
                        print!("\x1B[2J\n");
                        print_basin(&map, current_basin);
                        sleep(Duration::from_millis(1000));
                    });
                }
            }
        }
    }
}
