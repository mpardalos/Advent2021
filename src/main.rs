use std::{
    fs,
    io::{BufRead, BufReader},
};

fn day1(buf: &mut dyn BufRead) {
    let errmsg = "Could not read number";
    let mut increases = 0;
    let mut lines = buf.lines().map(|l| l.unwrap());

    let first_line = lines.next().unwrap();

    let mut last: i32 = first_line.parse().unwrap();

    for line in lines {
        let this: i32 = line.parse().unwrap();
        if this > last {
            increases += 1;
        }
        last = this;
    }

    println!("The value increases {} times", increases);
}

fn main() {
    let file = fs::File::open("inputs/1").expect("Could not read file");

    day1(&mut BufReader::new(file));
}
