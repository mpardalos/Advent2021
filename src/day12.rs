use std::{
    collections::{HashMap, HashSet},
    io::BufRead,
};

use crate::Solution;

type Node = String;
type Map = HashMap<String, Vec<Node>>;
type Path<'a> = Vec<&'a str>;

fn read_input(buf: &mut impl BufRead, map: &mut Map) {
    buf.lines()
        .map(Result::unwrap)
        .map(|line| {
            if let [from, to] = line.split("-").collect::<Vec<_>>()[..] {
                (from.to_string(), to.to_string())
            } else {
                panic!("Invalid line: {}", line);
            }
        })
        .for_each(|(from, to)| {
            map.entry(from.clone())
                .or_insert(Vec::new())
                .push(to.clone());
            map.entry(to).or_insert(Vec::new()).push(from);
        });
}

fn find_end<'a, 'b>(
    map: &'a Map,
    start: &'a str,
    small_visited: &mut HashSet<&'a str>,
) -> Vec<Path<'a>> {
    if small_visited.contains(start) {
        return vec![];
    }

    if start == "end" {
        return vec![vec!["end"]];
    }

    if start.chars().next().unwrap().is_lowercase() {
        small_visited.insert(start);
    }

    let mut paths: Vec<Path> = map[start]
        .iter()
        .flat_map(|next| find_end(map, next.as_str(), small_visited))
        .collect();

    for path in paths.iter_mut() {
        path.insert(0, start)
    }

    small_visited.remove(start);

    paths
}

fn find_end_double_visit<'a>(
    map: &'a Map,
    start: &'a str,
    small_visited: &mut HashSet<&'a str>,
    double_visit_done: bool,
    start_allowed: bool,
) -> Vec<Path<'a>> {
    if !start_allowed && start == "start" {
        return vec![];
    } else if start == "end" {
        return vec![vec!["end"]];
    } else if small_visited.contains(start) && double_visit_done {
        return vec![];
    }

    let second_visit = if start.chars().next().unwrap().is_lowercase() {
        !small_visited.insert(start) && start != "start"
    } else {
        false
    };

    let mut paths: Vec<Path> = map[start]
        .iter()
        .flat_map(|next| {
            find_end_double_visit(
                map,
                next.as_str(),
                small_visited,
                double_visit_done || second_visit,
                false,
            )
        })
        .collect();

    for path in paths.iter_mut() {
        path.insert(0, start)
    }

    if !second_visit {
        small_visited.remove(start);
    }

    paths
}

pub struct Part1;
impl Solution for Part1 {
    const DAY: u8 = 12;
    const PART: u8 = 1;

    fn solve(buf: &mut impl BufRead) -> String {
        let mut map: Map = HashMap::new();
        read_input(buf, &mut map);
        let paths = find_end(&map, "start", &mut HashSet::new());

        format!("There are {} valid paths", paths.len())
    }
}

pub struct Part2;
impl Solution for Part2 {
    const DAY: u8 = 12;
    const PART: u8 = 2;

    fn solve(buf: &mut impl BufRead) -> String {
        let mut map: Map = HashMap::new();
        read_input(buf, &mut map);
        let paths = find_end_double_visit(&map, "start", &mut HashSet::new(), false, true);

        format!("There are {} valid paths", paths.len())
    }
}
