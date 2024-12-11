const DAY: u8 = 8;
const TEST_INPUT: &str = "\
............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............
";

use std::{collections::{HashMap, HashSet}, fs};
use std::ops::{Add, Sub};

fn main() {
    println!("\n-- PART 1 --");
    let result = count_antinodes(TEST_INPUT);
    println!("TEST: {}", result);
    assert_eq!(result, 14, "failed on test input");
    println!("REAL: {}", count_antinodes(file_input().as_str()));

    println!("\n-- PART 2 --");
    let result = count_harmonic_antinodes(TEST_INPUT);
    println!("TEST: {}", result);
    assert_eq!(result, 34, "failed on test input");
    println!("REAL: {}", count_harmonic_antinodes(file_input().as_str()));
}

fn count_harmonic_antinodes(input: &str) -> usize {
    let (antennas, bounds) = find_the_antennas(input);
    let mut antinodes: HashSet<Point> = HashSet::new();

    for frequency in antennas.keys() {
        let combos = combinations(&antennas[frequency]);
        for (a, b) in combos {
            let dist = b - a;

            let mut anti1 = b - &dist; // this is a
            while anti1.in_bounds(&bounds) {
                let new_anti = &anti1 - &dist;
                antinodes.insert(anti1);
                anti1 = new_anti;
            }

            let mut anti2 = a + &dist; // this is b
            while anti2.in_bounds(&bounds) {
                let new_anti = &anti2 + &dist;
                antinodes.insert(anti2);
                anti2 = new_anti;
            }
        }
    }

    antinodes.len()
}

fn count_antinodes(input: &str) -> usize {
    let (antennas, bounds) = find_the_antennas(input);
    let mut antinodes : HashSet<Point> = HashSet::new();

    for frequency in antennas.keys() {
        let combos = combinations(&antennas[frequency]);
        for (a, b) in combos {
            let dist = b - a;

            let anti1 = a - &dist;
            if anti1.in_bounds(&bounds) { antinodes.insert(anti1); }

            let anti2 = b + &dist;
            if anti2.in_bounds(&bounds) { antinodes.insert(anti2); }
        }
    }
    antinodes.len()
}

fn find_the_antennas(input: &str) -> (HashMap<char, Vec<Point>>, Point) {
    let mut antennas: HashMap<char, Vec<Point>> = HashMap::new();
    let mut max_r = 0;
    let mut max_c = 0;
    for (r, line) in input.lines().enumerate() {
        if r > max_r { max_r = r }
        for (c, place) in line.char_indices() {
            if c > max_c { max_c = c }
            match place {
                '.' => (),
                ch => {
                    let point = Point(r.try_into().unwrap(), c.try_into().unwrap());
                    if let Some(list) = antennas.get_mut(&ch) {
                        list.push(point);
                    } else {
                        antennas.insert(ch, vec![point]);
                    }
                }
            }
        }
    }
    (antennas, Point(max_r.try_into().unwrap(), max_c.try_into().unwrap()))
}

fn combinations<T>(slice: &[T]) -> Vec<(&T, &T)> {
    let mut combos = vec![];
    for (i, t1) in slice.into_iter().enumerate() {
        for t2 in slice[i+1..].into_iter() {
            combos.push((t1, t2));
        }
    }
    combos
}

fn file_input() -> String {
    let file_path = format!("data/day_{:02}.txt", DAY);
    fs::read_to_string(file_path).expect("unable to read file")
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Point(i32, i32);
impl Point {
    // Is point no smaller than 0,0 and no greater than bounds
    // (equal bounds is true)
    fn in_bounds(&self, bounds: &Self) -> bool {
        (0..=bounds.0).contains(&self.0) && (0..=bounds.1).contains(&self.1)
    }
}
impl Add for Point {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self(self.0 + other.0, self.1 + other.1)
    }
}
impl<'a, 'b> Add<&'a Point> for &'b Point {
    type Output = Point;
    fn add(self, other: &Point) -> Self::Output {
        Point(self.0 + other.0, self.1 + other.1)
    }
}
impl Sub for Point {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self(self.0 - other.0, self.1 - other.1)
    }
}
impl<'a, 'b> Sub<&'a Point> for &'b Point {
    type Output = Point;
    fn sub(self, other: &Point) -> Self::Output {
        Point(self.0 - other.0, self.1 - other.1)
    }
}
