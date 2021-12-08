use std::cmp::min;
use std::io::BufRead;

use minifb::{Key, KeyRepeat, Window};
use raqote::{DrawOptions, DrawTarget, PathBuilder, SolidSource, Source};

use crate::visualisation::run_window;
use crate::{Extra, Solution};

fn read_input(buf: &mut impl BufRead) -> Vec<i32> {
    let line = buf.lines().next().unwrap().unwrap();
    line.split(",")
        .map(str::parse)
        .map(Result::unwrap)
        .collect()
}

impl Part1 {
    fn cost(p1: i32, p2: i32) -> i32 {
        (p2 - p1).abs()
    }

    fn align_spot(start_locations: &Vec<i32>) -> i32 {
        let mut start_locations = start_locations.clone();
        start_locations.sort();
        start_locations[start_locations.len() / 2]
    }
}

pub struct Part1;
impl Solution for Part1 {
    const DAY: u8 = 7;
    const PART: u8 = 1;

    fn solve(buf: &mut impl BufRead) -> String {
        let nums = read_input(buf);
        let align_spot = Self::align_spot(&nums);
        let fuel: i32 = nums.iter().map(|n| Self::cost(align_spot, *n)).sum();
        format!(
            "The crabs will need {} fuel to align at {}",
            fuel, align_spot
        )
    }
}

pub struct Part2;

impl Part2 {
    fn cost(p1: i32, p2: i32) -> i32 {
        // (1 .. (p1-p2).abs()).sum()
        let max = (p1 - p2).abs();
        max * (max + 1) / 2
    }

    fn align_spot(start_positions: &Vec<i32>) -> i32 {
        let mut start_positions = start_positions.clone();
        start_positions.sort();
        (1..*start_positions.iter().max().unwrap())
            .min_by_key(|target| {
                start_positions
                    .iter()
                    .map(|crab| Self::cost(*crab, *target))
                    .sum::<i32>()
            })
            .unwrap()
    }
}

impl Solution for Part2 {
    const DAY: u8 = 7;
    const PART: u8 = 2;

    fn solve(buf: &mut impl BufRead) -> String {
        let start_positions = read_input(buf);

        let align_spot = Self::align_spot(&start_positions);
        let cost = start_positions
            .iter()
            .map(|crab| Self::cost(*crab, align_spot))
            .sum::<i32>();

        format!(
            "The crabs will need {} fuel to align at {}",
            cost, align_spot
        )
    }
}

//---- Extra Visualisation --------------------------------

pub struct Visualise;
impl Extra for Visualise {
    const DAY: u8 = 7;
    const USE_SAMPLE: bool = false;

    fn run(buf: &mut impl BufRead) {
        let mut visualisation = VisualisationView::new(read_input(buf));
        run_window("Day 7", 800, 800, &mut |window, dt| {
            visualisation.update(window, dt)
        });
    }
}

struct VisualisationView {
    positions: Vec<i32>,
    start_positions: Vec<i32>,
    target_position: i32,
    done: bool,

    crab_height: f32,
    crab_spacing: f32,
    step_size: i32
}

impl VisualisationView {
    fn new(positions: Vec<i32>) -> Self {
        // let font = SystemSource::new()
        //     .select_best_match(&[FamilyName::SansSerif], &Properties::new())
        //     .unwrap()
        //     .load()
        //     .unwrap();

        Self {
            target_position: Part1::align_spot(&positions),
            start_positions: positions.clone(),
            positions,
            done: false,
            crab_height: 5.,
            crab_spacing: 1.,
            step_size: 1
        }
    }

    fn update(&mut self, window: &Window, dt: &mut DrawTarget) -> bool {
        if self.done {
            return false;
        }

        window.get_keys_pressed(KeyRepeat::No).map(|keys| {
            for t in keys {
                match t {
                    Key::R => self.positions = self.start_positions.clone(),
                    _ => (),
                }
            }
        });

        dt.clear(SolidSource::from_unpremultiplied_argb(0, 0, 0, 0));
        // Draw the crabs
        let mut y = 10.;
        let mut pb = PathBuilder::new();
        for crab_x in &self.positions {
            pb.rect(*crab_x as f32, y, 10., 10.);
            y += 15.;
        }
        let path = pb.finish();
        dt.fill(
            &path,
            &Source::Solid(SolidSource::from_unpremultiplied_argb(0xff, 0xff, 0, 0)),
            &DrawOptions::new(),
        );

        for crab_x in self.positions.iter_mut() {
            if *crab_x != self.target_position {
                if *crab_x < self.target_position {
                    *crab_x += min(self.step_size, self.target_position - *crab_x);
                } else if *crab_x > self.target_position {
                    *crab_x -= min(self.step_size, *crab_x - self.target_position);
                }
            }
        }

        true
    }
}
