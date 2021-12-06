#![allow(dead_code)]

use clap::Parser;
use std::{
    fs::{self, File},
    io::BufReader,
    time::Instant,
};

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;

trait Solution {
    const DAY: u8;
    const PART: u8;

    fn solve(buf: &mut BufReader<File>) -> String;
}

trait Extra {
    const DAY: u8;
    const USE_SAMPLE: bool;

    fn run(buf: &mut BufReader<File>);
}

fn format_solution_with_file<S: Solution>(filepath: &String) -> String {
    let file = fs::File::open(filepath).expect("Could not read file");

    let before = Instant::now();
    let answer = S::solve(&mut BufReader::new(file));
    let after = Instant::now();

    let duration = after - before;
    let duration_display = if duration.as_millis() > 0 {
        format!("{:>3}ms", duration.as_millis())
    } else {
        format!("{:>3}us", duration.as_micros())
    };

    format!(
        "[{}][Day {:>2}][Part {:>2}]: {}",
        duration_display,
        S::DAY,
        S::PART,
        answer
    )
}

fn solution<S: Solution>() {
    println!(
        "{}",
        format_solution_with_file::<S>(&format!("inputs/{}", S::DAY))
    )
}

fn solution_with_sample<S: Solution>() {
    println!(
        "[SAMPLE] {}",
        format_solution_with_file::<S>(&format!("inputs/{}_sample", S::DAY))
    )
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
            solution::<day1::Part1>();
            solution::<day1::Part2>();

            solution::<day2::Part1>();
            solution::<day2::Part2>();

            solution::<day3::Part1>();
            solution::<day3::Part2>();

            solution::<day4::Part1>();
            solution::<day4::Part2>();

            solution::<day5::Part1<1024>>();
            solution::<day5::Part1<1024>>();
        }
        Some(e) => match e.as_str() {
            "vis4" => extra::<day4::Visualise>(),
            _ => eprintln!("Extra does not exist: {}", e),
        },
    }
}
