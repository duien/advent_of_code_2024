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

use grid::*;
use nom::character::complete::{newline, one_of};
use nom::combinator::{all_consuming, map};
use nom::multi::many1;
use nom::sequence::terminated;
use nom::IResult;
use std::collections::HashSet;
use std::fs;

fn main() {
    do_the_thing(TEST_INPUT);
    do_the_thing(&file_input().as_str())
}

fn do_the_thing(input: &str) {
    let mut lab: Laboratory = parse_input(input).into();
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    while let Some((pos, guard)) = lab.guard() {
        visited.insert(pos);
        if let LabItem::Guard { facing: dir } = guard {
            if let Ok(dest) = shift(pos, dir) {
                let (r, c) = dest;
                let at_dest = lab.grid.get(r, c);
                match at_dest {
                    Some(LabItem::Nothing) => lab.grid.swap(pos, dest),
                    Some(LabItem::Guard { .. }) => unimplemented!(),
                    Some(LabItem::Obstacle) => {
                        lab.grid[pos] = LabItem::Guard {
                            facing: dir.to_clockwise(),
                        }
                    }
                    None => lab.grid[pos] = LabItem::Nothing,
                }
            }
        }
    }

    println!("VISITED: {}", visited.len());
}

// Given a point and a direction, move by 1 in that direction. If the
// destination would go off the north or west side of the grid (becoming
// negative) return Err variant instead. It does not check if it's gone
// too far south or east, since it doesn't know the grid size
fn shift((r, c): (usize, usize), dir: &Direction) -> Result<(usize, usize), &'static str> {
    let (r, c) = match dir {
        Direction::North => (r.checked_sub(1), Some(c)),
        Direction::South => (Some(r + 1), Some(c)),
        Direction::East => (Some(r), Some(c + 1)),
        Direction::West => (Some(r), c.checked_sub(1)),
    };

    match (r, c) {
        (Some(r), Some(c)) => Ok((r, c)),
        _ => Err("out of bounds"),
    }
}

#[derive(Debug)]
struct Laboratory {
    grid: Grid<LabItem>,
}

impl Laboratory {
    // Where is the guard in this lab?
    fn guard(&self) -> Option<((usize, usize), &LabItem)> {
        self.grid
            .indexed_iter()
            .find(|(_point, item)| item.is_guard())
    }
}

impl From<Grid<LabItem>> for Laboratory {
    // Create a lab from a grid of lab items
    fn from(grid: Grid<LabItem>) -> Self {
        Self { grid }
    }
}

#[derive(Debug)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    // What direction is to clockwise of this one?
    fn to_clockwise(&self) -> Self {
        match self {
            Self::North => Self::East,
            Self::South => Self::West,
            Self::East => Self::South,
            Self::West => Self::North,
        }
    }
}

#[derive(Debug)]
enum LabItem {
    Nothing,
    Obstacle,
    Guard { facing: Direction },
}

impl LabItem {
    // Is this lab item a guard?
    fn is_guard(&self) -> bool {
        match self {
            Self::Guard { .. } => true,
            _ => false,
        }
    }
}

impl From<char> for LabItem {
    // Turn a character from the input into the corresponding lab item
    fn from(c: char) -> Self {
        match c {
            '.' => Self::Nothing,
            '#' => Self::Obstacle,
            '^' => Self::Guard {
                facing: Direction::North,
            },
            _ => unimplemented!(),
        }
    }
}

fn file_input() -> String {
    let file_path = "../ruby/data/day_06.txt";
    fs::read_to_string(file_path).expect("unable to read file")
}

fn parse_input(input: &str) -> Grid<LabItem> {
    let result: IResult<&str, Grid<_>> = all_consuming(map(
        many1(terminated(
            many1(map(one_of(".#^"), LabItem::from)),
            newline,
        )),
        Grid::from,
    ))(input);
    result.expect("parse failure").1
}
