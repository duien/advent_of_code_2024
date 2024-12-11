const TEST_INPUT: &str = "\
....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
";

use std::{collections::HashSet, fs};

fn main() {
    let lab = Laboratory::build_from_input(TEST_INPUT);
    dbg!(&lab.guard);
    let count = collect_visited_positions(lab).len();
    println!("Visited: {count}");
}

fn collect_visited_positions(lab: Laboratory) -> HashSet<Coord> {
    let mut visited: HashSet<Coord> = HashSet::new();
    if let Some(guard) = lab.guard {
        visited.insert(guard.location);
        // do more things
    }
    visited
}

#[derive(Debug)]
struct Laboratory {
    guard: Option<Guard>,
    grid: Vec<Vec<Location>>
}

impl Laboratory {
    fn advance(&mut self) {
        if self.guard.is_none() { return };
    //     match guard {
    //         None => return,
    //         Some(g) => {
    //             let next_pos: Option<Coord> = match g.direction {
    //                 North =>
    //             }
    //         }
    //     }
        todo!()
    }

    fn build_from_input(input: &str) -> Self {
        let mut guard : Option<Guard> = None;
        let mut grid : Vec<Vec<Location>> = vec![];

        for (r, line) in input.lines().enumerate() {
            let row = line.chars().enumerate().map(|(c, ch)| {
                match ch {
                    '.' => Location{ obstruction: false },
                    '#' => Location{obstruction: true},
                    '^' => {
                        guard = Some(Guard { facing: Direction::North, location: Coord(r,c) });
                        Location{obstruction: false}
                    },
                    _ => unimplemented!("bad character in input")
                }
            }).collect::<Vec<_>>();
            grid.push(row);
        }

        Self { guard, grid }
    }
}

#[derive(Debug)]
struct Location {
    obstruction: bool
}

#[derive(Debug)]
struct Guard {
    location: Coord,
    facing: Direction
}

impl Guard {
    fn facing_location(&self) -> Option<Coord> {
        todo!()
    }
}

#[derive(Debug, Eq, Hash, PartialEq)]
struct Coord(usize, usize);

#[derive(Debug)]
enum Direction {
    North, South, East, West
}

fn file_input() -> String {
    let file_path = "data/day_06.txt";
    fs::read_to_string(file_path).expect("unable to read file")
}
