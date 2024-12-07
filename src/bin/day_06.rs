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

use nom::character::complete::{newline, one_of};
use nom::combinator::{all_consuming, map};
use nom::multi::many1;
use nom::sequence::terminated;
use nom::IResult;
use std::fmt;
use std::fs;

fn main() {
    let lab = parse_input(TEST_INPUT);

    dbg!(&lab);
    println!("{}", lab.grid)
}

fn file_input() -> String {
    let file_path = "../ruby/data/day_05.txt";
    fs::read_to_string(file_path).expect("unable to read file")
}


fn parse_input(input: &str) -> Laboratory {
    let result: IResult<&str, Laboratory> = all_consuming(map(
        map(
            many1(terminated(
                many1(map(one_of(".#^"), Location::from)),
                newline,
            )),
            Grid::from,
        ),
        Laboratory::from,
    ))(input);

    result.expect("parse failure").1
}


#[derive(Debug)]
struct Position(usize, usize);

#[derive(Debug)]
struct Grid<T>(Vec<Vec<T>>);

impl<T> From<Vec<Vec<T>>> for Grid<T> {
    fn from(vec: Vec<Vec<T>>) -> Self {
        Self(vec)
    }
}

#[derive(Debug)]
struct Laboratory {
    // guard: Option<Position>,
    grid: Grid<Location>,
}

impl From<Grid<Location>> for Laboratory {
    fn from(grid: Grid<Location>) -> Self {
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

#[derive(Debug)]
enum Location {
    Empty,
    Obstacle,
    Guard(Direction),
}

impl fmt::Display for Location {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Location::Empty => ".",
                Location::Obstacle => "#",
                Location::Guard(direction) => match direction {
                    Direction::North => "^",
                    Direction::South => "v",
                    Direction::East => ">",
                    Direction::West => "<",
                },
            }
        )
    }
}

impl fmt::Display for Grid<Location> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in &self.0 {
            for l in row {
                write!(f, "{}", l)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl From<char> for Location {
    fn from(c: char) -> Self {
        match c {
            '.' => Self::Empty,
            '#' => Self::Obstacle,
            '^' => Self::Guard(Direction::North),
            _ => unimplemented!(),
        }
    }
}
