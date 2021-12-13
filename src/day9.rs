use std::io::BufRead;

use ansi_term::{Colour, Style};
use sdl2::{pixels::Color, rect::Rect, render::Canvas, video::Window};

use crate::{visualisation::WindowApp, Extra, Solution, util::neighbours};

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
    print_cb: &mut impl FnMut(&Basin),
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
                    map_basin(&map, row_idx, col_idx, &mut basin, &mut |_| {});
                    basins.push(basin);
                }
            }
        }

        basins.sort_by_key(Vec::len);
        format!(
            "The top 3 basins' sizes multiplied together give: {}",
            basins.iter().rev().take(3).map(Vec::len).product::<usize>()
        )
    }
}

pub struct Progression {
    map: HeightMap,
    basin_views: Vec<Basin>,
    next_basin_view: usize,

    background_drawn: bool,
}

impl Extra for Progression {
    const DAY: u8 = 9;
    const USE_SAMPLE: bool = false;

    fn run(buf: &mut impl BufRead) {
        Self::new(read_input(buf)).run_window();
    }
}

impl Progression {
    fn new(map: HeightMap) -> Self {
        let mut basin_views: Vec<Basin> = Vec::new();

        for (row_idx, row) in map.iter().enumerate() {
            for (col_idx, _height) in row.iter().enumerate() {
                if has_basin_at(&map, row_idx, col_idx) {
                    let mut basin = vec![];
                    map_basin(&map, row_idx, col_idx, &mut basin, &mut |basin_view| {
                        basin_views.push(basin_view.clone())
                    });
                }
            }
        }

        Self {
            map,
            basin_views,
            next_basin_view: 0,
            background_drawn: false,
        }
    }
}

impl WindowApp for Progression {
    const WINDOW_NAME: &'static str = "Day 9 - Basins";
    const WINDOW_WIDTH: u32 = 1100;
    const WINDOW_HEIGHT: u32 = 1100;
    const WINDOW_FPS: Option<u32> = Some(1000);

    fn reset(&mut self) {
        self.background_drawn = false;
        self.next_basin_view = 0;
    }

    fn draw_frame(&mut self, canvas: &mut Canvas<Window>) -> Result<(), String> {
        if !self.background_drawn {
            canvas.set_draw_color(Color::RGB(0x11, 0x11, 0x11));
            canvas.clear();

            canvas.set_draw_color(Color::RGB(0x88, 0x88, 0x88));
            for (row_idx, row) in self.map.iter().enumerate() {
                for (col_idx, height) in row.iter().enumerate() {
                    if *height >= 9 {
                        canvas.fill_rect(Rect::new(
                            10 * row_idx as i32,
                            10 * col_idx as i32,
                            10,
                            10,
                        ))?;
                    }
                }
            }

            self.background_drawn = true;
        }

        if let Some(basin) = self.basin_views.get(self.next_basin_view) {
            self.next_basin_view += 1;
            canvas.set_draw_color(Color::BLUE);
            canvas.fill_rects(
                &basin
                    .iter()
                    .map(|(row, col)| Rect::new(10 * *row as i32, 10 * *col as i32, 10, 10))
                    .collect::<Vec<Rect>>()[..],
            )?;
            canvas.present();
        }
        Ok(())
    }
}
