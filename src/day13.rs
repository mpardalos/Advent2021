use crate::Solution;
use itertools::Itertools;
use std::{cmp::max, fmt, io::BufRead};

#[derive(Debug, Clone, Copy)]
enum Direction {
    X,
    Y,
}

#[derive(Debug, Clone, Copy)]
struct Fold {
    direction: Direction,
    index: u32,
}

type Point = (u32, u32);

struct Origami {
    max_x: u32,
    max_y: u32,
    points: Vec<Point>,
    folds: Vec<Fold>,
}

impl Origami {
    fn do_fold(&mut self, fold: &Fold) {
        match fold.direction {
            Direction::X => {
                let original_max_x = self.max_x;
                self.max_x = 0;
                for (x, _) in self.points.iter_mut() {
                    if *x > fold.index {
                        *x = original_max_x - *x;
                        if *x > self.max_x {
                            self.max_x = *x;
                        }
                    }
                }
            }
            Direction::Y => {
                let original_max_y = self.max_y;
                self.max_y = 0;
                for (_, y) in self.points.iter_mut() {
                    if *y > fold.index {
                        *y = original_max_y - *y;
                        if *y > self.max_y {
                            self.max_y = *y;
                        }
                    }
                }
            }
        }
    }
}

impl fmt::Display for Origami {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for y in 0..=self.max_y {
            for x in 0..=self.max_x {
                if self.points.contains(&(x, y)) {
                    write!(f, "█")?;
                } else {
                    write!(f, " ")?;
                }
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

impl fmt::Debug for Origami {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}\n", self)?;
        write!(f, "max_x: {}\n", self.max_x)?;
        write!(f, "max_y: {}\n", self.max_y)?;
        for fold in &self.folds {
            write!(f, "{:?}\n", fold)?;
        }

        Ok(())
    }
}

fn read_input(buf: &mut impl BufRead) -> Origami {
    let mut origami = Origami {
        max_x: 0,
        max_y: 0,
        points: Vec::new(),
        folds: Vec::new(),
    };

    let mut lines = buf.lines().map(Result::unwrap);

    for line in lines.by_ref() {
        if let [x_str, y_str] = line.split(",").collect::<Vec<&str>>()[..] {
            let x = x_str.parse().expect("Invalid point line");
            let y = y_str.parse().expect("Invalid point line");

            origami.max_x = max(origami.max_x, x);
            origami.max_y = max(origami.max_y, y);

            origami.points.push((x, y));
        } else if line.is_empty() {
            break;
        } else {
            panic!("Invalid point line");
        }
    }

    for line in lines.by_ref() {
        if let [beginning, num_str] = line.split("=").collect::<Vec<&str>>()[..] {
            origami.folds.append(&mut vec![Fold {
                direction: match beginning.chars().last() {
                    Some('x') => Direction::X,
                    Some('y') => Direction::Y,
                    _ => panic!("Invalid fold line"),
                },
                index: num_str.parse().expect("Invalid point line"),
            }]);
        } else {
            panic!("Invalid fold line");
        }
    }

    origami
}

pub struct Part1;
impl Solution for Part1 {
    const DAY: u8 = 13;
    const PART: u8 = 1;

    fn solve(buf: &mut impl BufRead) -> String {
        let mut origami = read_input(buf);
        let fold = origami.folds[0].clone();

        origami.do_fold(&fold);
        let point_count = origami.points.into_iter().unique().count();

        format!("There are {} points after the first iteration", point_count)
    }
}

pub struct Part2;
impl Solution for Part2 {
    const DAY: u8 = 13;
    const PART: u8 = 2;

    fn solve(buf: &mut impl BufRead) -> String {
        let mut origami = read_input(buf);
        let folds = origami.folds.clone();

        folds.iter().for_each(|fold| {
            origami.do_fold(&fold);
        });

        format!("\n{}", origami)
    }
}
