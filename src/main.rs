#![allow(dead_code)]

use clap::Parser;
use std::{
    fs,
    io::{BufRead, BufReader},
    time::{Duration, Instant},
};

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day9;
mod visualisation;

trait Solution {
    const DAY: u8;
    const PART: u8;

    fn solve(buf: &mut impl BufRead) -> String;
}

trait Extra {
    const DAY: u8;
    const USE_SAMPLE: bool;

    fn run(buf: &mut impl BufRead);
}

fn format_duration(duration: Duration) -> String {
    format!("{:>8.3}ms", duration.as_secs_f64() * 1000f64)
}

fn solution_with_file<S: Solution>(filepath: &String) -> Duration {
    let file = fs::File::open(filepath).expect("Could not read file");

    let before = Instant::now();
    let answer = S::solve(&mut BufReader::new(file));
    let after = Instant::now();

    let duration = after - before;

    println!(
        "[{}][Day {:>2}][Part {}]: {}",
        format_duration(duration),
        S::DAY,
        S::PART,
        answer
    );

    duration
}

fn solution<S: Solution>(use_sample: bool) -> Duration {
    if use_sample {
        solution_with_file::<S>(&format!("inputs/{}_sample", S::DAY))
    } else {
        solution_with_file::<S>(&format!("inputs/{}", S::DAY))
    }
}

fn extra<E: Extra>() {
    let filepath = if E::USE_SAMPLE {
        format!("inputs/{}_sample", E::DAY)
    } else {
        format!("inputs/{}", E::DAY)
    };

    let file = fs::File::open(filepath).expect("Could not read file");

    E::run(&mut BufReader::new(file));
}

#[derive(Parser)]
struct Opts {
    #[clap()]
    day: Option<i32>,

    #[clap(short, about = "Run an 'extra', e.g. a visualisation")]
    extra: bool,

    #[clap(short = 's', about = "Run an 'extra', e.g. a visualisation")]
    use_sample: bool,
}

fn main() {
    let opts = Opts::parse();

    match opts.day {
        None => {
            let mut clock: Duration = Duration::new(0, 0);

            clock += solution::<day1::Part1>(opts.use_sample);
            clock += solution::<day1::Part2>(opts.use_sample);

            clock += solution::<day2::Part1>(opts.use_sample);
            clock += solution::<day2::Part2>(opts.use_sample);

            clock += solution::<day3::Part1>(opts.use_sample);
            clock += solution::<day3::Part2>(opts.use_sample);

            clock += solution::<day4::Part1>(opts.use_sample);
            clock += solution::<day4::Part2>(opts.use_sample);

            clock += solution::<day5::Part1<1024>>(opts.use_sample);
            clock += solution::<day5::Part2<1024>>(opts.use_sample);

            clock += solution::<day6::Part1>(opts.use_sample);
            clock += solution::<day6::Part2>(opts.use_sample);

            clock += solution::<day7::Part1>(opts.use_sample);
            clock += solution::<day7::Part2>(opts.use_sample);

            clock += solution::<day9::Part1>(opts.use_sample);
            clock += solution::<day9::Part2>(opts.use_sample);

            println!("[{}]", format_duration(clock));
        }

        Some(1) => {
            solution::<day1::Part1>(opts.use_sample);
            solution::<day1::Part2>(opts.use_sample);
        }

        Some(2) => {
            solution::<day2::Part1>(opts.use_sample);
            solution::<day2::Part2>(opts.use_sample);
        }

        Some(3) => {
            solution::<day3::Part1>(opts.use_sample);
            solution::<day3::Part2>(opts.use_sample);
        }

        Some(4) => {
            if opts.extra {
                extra::<day4::Visualise>();
            } else {
                solution::<day4::Part1>(opts.use_sample);
                solution::<day4::Part2>(opts.use_sample);
            }
        }

        Some(5) => {
            solution::<day5::Part1<1024>>(opts.use_sample);
            solution::<day5::Part2<1024>>(opts.use_sample);
        }

        Some(6) => {
            solution::<day6::Part1>(opts.use_sample);
            solution::<day6::Part2>(opts.use_sample);
        }

        Some(7) => {
            if opts.extra {
                extra::<day7::Visualise>();
            } else {
                solution::<day7::Part1>(opts.use_sample);
                solution::<day7::Part2>(opts.use_sample);
            }
        }

        Some(9) => {
            if opts.extra {
                extra::<day9::Progression>();
            } else {
                solution::<day9::Part1>(opts.use_sample);
                solution::<day9::Part2>(opts.use_sample);
            }
        }

        Some(n) => {
            println!("I have no solution for day {}", n);
        }
    };
}
