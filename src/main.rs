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
        "[{}][Day {:>2}][Part {:>2}]: {}",
        format_duration(duration),
        S::DAY,
        S::PART,
        answer
    );

    duration
}

fn solution<S: Solution>() -> Duration {
    solution_with_file::<S>(&format!("inputs/{}", S::DAY))
}

fn solution_with_sample<S: Solution>() -> Duration {
    solution_with_file::<S>(&format!("inputs/{}_sample", S::DAY))
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
    #[clap(short, long, about = "Run an 'extra', e.g. a visualisation")]
    extra: Option<String>,
}

fn main() {
    let opts = Opts::parse();

    match opts.extra {
        None => {
            let mut clock: Duration = Duration::new(0, 0);

            clock += solution::<day1::Part1>();
            clock += solution::<day1::Part2>();

            clock += solution::<day2::Part1>();
            clock += solution::<day2::Part2>();

            clock += solution::<day3::Part1>();
            clock += solution::<day3::Part2>();

            clock += solution::<day4::Part1>();
            clock += solution::<day4::Part2>();

            clock += solution::<day5::Part1<1024>>();
            clock += solution::<day5::Part2<1024>>();

            clock += solution::<day6::Part1>();
            clock += solution::<day6::Part2>();

            clock += solution::<day7::Part1>();
            clock += solution::<day7::Part2>();

            println!("[{}]", format_duration(clock));
        }
        Some(e) => match e.as_str() {
            "vis4" => extra::<day4::Visualise>(),
            _ => eprintln!("Extra does not exist: {}", e),
        },
    }
}
