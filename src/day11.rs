use std::io::BufRead;

use ansi_term::{Colour, Style};
use sdl2::{pixels::Color, rect::Rect, render::Canvas, video::Window};

use crate::{util::neighbour_indices_with_diag, visualisation::WindowApp, Extra, Solution};

// Negative value means the octopus has already flashed
type Grid = Vec<Vec<i8>>;

fn read_input(buf: &mut impl BufRead) -> Grid {
    buf.lines()
        .map(Result::unwrap)
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as i8)
                .collect()
        })
        .collect()
}

fn print_grid(grid: &Grid) {
    let normal: Style = Style::new().dimmed();
    let zero: Style = Style::new().bold();
    let way_over_nine: Style = Style::new().fg(Colour::Red);
    let negative: Style = Style::new().fg(Colour::Red);

    for row in grid {
        for c in row {
            let style = if *c == 0 {
                zero
            } else if *c > 9 {
                way_over_nine
            } else if *c < 0 {
                negative
            } else {
                normal
            };

            print!("{}", style.paint(format!("{}", *c)));
        }
        println!();
    }
}

fn step_grid(grid: &mut Grid) -> u32 {
    let width = grid.len();
    let height = grid[0].len();
    let mut flashes = 0;

    for row in grid.iter_mut() {
        for energy in row.iter_mut() {
            *energy += 1;
        }
    }

    loop {
        let mut had_a_flash = false;
        for row in 0..grid.len() {
            for col in 0..grid[row].len() {
                if grid[row][col] > 9 {
                    flashes += 1;
                    grid[row][col] = -1;

                    had_a_flash = true;

                    for (nrow, ncol) in neighbour_indices_with_diag(row, col, width, height) {
                        if grid[nrow][ncol] >= 0 {
                            // If it hasn't flashed already
                            grid[nrow][ncol] += 1
                        }
                    }
                }
            }
        }
        if !had_a_flash {
            break;
        }
    }

    for row in grid.iter_mut() {
        for energy in row.iter_mut() {
            if *energy < 0 {
                *energy = 0;
            }
        }
    }

    flashes
}

pub struct Part1;
impl Solution for Part1 {
    const DAY: u8 = 11;
    const PART: u8 = 1;

    fn solve(buf: &mut impl BufRead) -> String {
        const STEPS: u32 = 100;

        let mut grid: Grid = read_input(buf);
        let mut flashes = 0;

        for _step in 1..=STEPS {
            flashes += step_grid(&mut grid);
        }

        format!("There have been a total of {} flashes", flashes)
    }
}

pub struct Part2;
impl Solution for Part2 {
    const DAY: u8 = 11;
    const PART: u8 = 2;

    fn solve(buf: &mut impl BufRead) -> String {
        let mut grid: Grid = read_input(buf);
        let octopus_count = (grid.len() * grid[0].len()) as u32;

        let mut step = 1;
        loop {
            let flashes = step_grid(&mut grid);

            if flashes == octopus_count {
                break;
            }

            step += 1;
        }

        format!(
            "The first step when all octopuses flash together is {}",
            step
        )
    }
}

pub struct Blinkenlights {
    initial_grid: Grid,
    grid: Grid,
    background_drawn: bool,
}

impl Extra for Blinkenlights {
    const DAY: u8 = 11;
    const USE_SAMPLE: bool = false;

    fn run(buf: &mut impl BufRead) {
        Self::new(read_input(buf)).run_window();
    }
}

impl Blinkenlights {
    fn new(grid: Grid) -> Self {
        Self {
            initial_grid: grid.clone(),
            grid,
            background_drawn: false,
        }
    }
}

impl WindowApp for Blinkenlights {
    const WINDOW_NAME: &'static str = "Day 11 - Blinkenlights";
    const WINDOW_WIDTH: u32 = 800;
    const WINDOW_HEIGHT: u32 = 800;
    const WINDOW_FPS: Option<u32> = Some(30);

    fn reset(&mut self) {
        self.grid = self.initial_grid.clone();
    }

    fn draw_frame(&mut self, canvas: &mut Canvas<Window>) -> Result<(), String> {
        let size: u32 = 80;

        canvas.set_blend_mode(sdl2::render::BlendMode::Blend);

        canvas.set_draw_color(Color::RGB(0x00, 0x00, 0x33));
        canvas.clear();

        let _flash_count = step_grid(&mut self.grid);

        for (row_idx, row) in self.grid.iter().enumerate() {
            for (col_idx, energy) in row.iter().enumerate() {
                let color = Color::RGBA(
                    0xFF,
                    0x00,
                    0xFF,
                    if *energy == 0 { 0xFF } else { 6 * (*energy as u8) },
                );
                canvas.set_draw_color(color);
                let rect = Rect::new(
                    size as i32 * col_idx as i32,
                    size as i32 * row_idx as i32,
                    size,
                    size,
                );

                canvas.fill_rect(rect)?;
            }
        }

        Ok(())
    }
}
