const DAY: u8 = 14;
const TEST_INPUT: &str = "\
p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3
";

use nom::{
    bytes::complete::tag,
    character::complete::{char, line_ending, multispace1},
    combinator::map,
    multi::many1,
    number::complete::float,
    sequence::{preceded, separated_pair, terminated},
    IResult,
};
use std::{fs, io};

fn main() {
    let score = build_and_iterate(TEST_INPUT, 11, 7);
    println!("TEST SCORE: {score}");
    assert_eq!(score, 12);
    
    let score = build_and_iterate(file_input().as_str(), 101, 103);
    println!("REAL SCORE: {score}");

    let mut movers = build_movers(file_input().as_str(), 101, 103);

    let mut step : u32 = 0;
    for _ in 0..=step { movers.iter_mut().for_each(|mut m| m.step()); }
    let mut buf = String::new();
    while true {
        movers.iter_mut().for_each(|mut m| m.step());
        step += 1;
        // where do 61 and 24 come from? Who knows! But the visualizations
        // at either of those multiples looked oddly clumpy, and it turns
        // out the answer is at both (although it's one more than my step
        // number for whatever off-by-one reason)
        if step.rem_euclid(101) == 61 && step.rem_euclid(103) == 24 {
            println!("STEP {step}");
            visualize(&movers);
            io::stdin().read_line(&mut buf);
        }
    }
}

fn visualize(movers: &[RoboMover]) {
    let max_x = movers[0].max_x;
    let max_y = movers[0].max_y;

    for y in 0..max_y {
        for x in 0..max_x {
            let n = movers.into_iter().filter(|m| m.robot.position.x == x && m.robot.position.y == y).count();
            match n {
                0 => print!("."),
                x => print!("{x}")
            }
        }
        println!("");
    }
}

fn build_movers(input: &str, max_x: usize, max_y: usize) -> Vec<RoboMover> {
    let robots = parse_input(input);
    robots.into_iter().map(|robot| RoboMover{robot, max_x, max_y}).collect()
}

fn build_and_iterate(input: &str, max_x: usize, max_y: usize) -> i32 {
    let movers = build_movers(input, max_x, max_y);
    let mut final_bots = vec![];
    for mut mover in movers {
        for _ in 0..100 {
            mover.step();
        }
        final_bots.push(mover.robot);
    }
    let mut quad_counts = (0, 0, 0, 0);
    let mid_x = max_x / 2;
    let mid_y = max_y / 2;
    let small_x = 0..mid_x;
    let small_y = 0..mid_y;
    let large_x = mid_x + 1..max_x;
    let large_y = mid_y + 1..max_y;

    dbg!(&small_x);
    dbg!(&small_y);
    dbg!(&large_x);
    dbg!(&large_y);
    
    for robot in &final_bots {
        match &robot.position {
            Point{x, y} if small_x.contains(x) && small_y.contains(y) => quad_counts.0 += 1,
            Point{x, y} if small_x.contains(x) && large_y.contains(y) => quad_counts.1 += 1,
            Point{x, y} if large_x.contains(x) && small_y.contains(y) => quad_counts.2 += 1,
            Point{x, y} if large_x.contains(x) && large_y.contains(y) => quad_counts.3 += 1,
            _ => ()
        }
    }
    dbg!(quad_counts);
    quad_counts.0 * quad_counts.1 * quad_counts.2 * quad_counts.3
}

fn file_input() -> String {
    let file_path = format!("data/day_{:02}.txt", DAY);
    fs::read_to_string(file_path).expect("unable to read file")
}

fn parse_input(input: &str) -> Vec<Robot> {
    many1(map(
        terminated(
            separated_pair(parse_point, multispace1, parse_velocity),
            line_ending,
        ),
        |(position, velocity)| Robot { position, velocity },
    ))(input)
    .expect("failed to parse")
    .1
}

fn parse_number_pair(input: &str) -> IResult<&str, (i32, i32)> {
    separated_pair(
        map(float, |f| f as i32),
        char(','),
        map(float, |f| f as i32),
    )(input)
}

fn parse_point(input: &str) -> IResult<&str, Point> {
    map(preceded(tag("p="), parse_number_pair), |(x, y)| Point {
        x: x as usize,
        y: y as usize,
    })(input)
}

fn parse_velocity(input: &str) -> IResult<&str, Vector> {
    map(preceded(tag("v="), parse_number_pair), |(x, y)| Vector {
        x,
        y,
    })(input)
}

#[derive(Debug)]
struct Point {
    x: usize,
    y: usize,
}
#[derive(Debug)]
struct Vector {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Robot {
    position: Point,
    velocity: Vector,
}

#[derive(Debug)]
struct RoboMover {
    robot: Robot,
    max_x: usize,
    max_y: usize
}

impl RoboMover {
    fn step(&mut self) {
        let new_x = (self.robot.position.x as i32 + self.robot.velocity.x).rem_euclid(self.max_x as i32);
        let new_y = (self.robot.position.y as i32 + self.robot.velocity.y).rem_euclid(self.max_y as i32);

        self.robot.position = Point{ x: new_x as usize, y: new_y as usize };
    }
}
