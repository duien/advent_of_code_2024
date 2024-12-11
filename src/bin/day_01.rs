const TEST_INPUT: &str = "3   4
4   3
2   5
1   3
3   9
3   3";

use std::collections::HashMap;
use std::fs;

fn main() {
    println!("\n--- TEST ---");
    let (left, right) = parse_input(TEST_INPUT);
    println!(
        "DIFFERENCE: {}",
        calculate_difference(left.clone(), right.clone())
    );
    println!("SIMILARITY: {}", calculate_similarity(left, right));

    let file_path = "data/day_01.txt";
    let input = fs::read_to_string(file_path).expect("unable to read file");

    println!("\n--- REAL ---");
    let (left, right) = parse_input(input.as_str());
    println!(
        "DIFFERENCE: {}",
        calculate_difference(left.clone(), right.clone())
    );
    println!("SIMILARITY: {}", calculate_similarity(left, right));
}

fn parse_input(input: &str) -> (Vec<i32>, Vec<i32>) {
    input.lines().map(parse_line).unzip()
}

fn parse_line(line: &str) -> (i32, i32) {
    let mut split = line.split_whitespace();
    let a = split.next().unwrap().parse().expect("not a number");
    let b = split.next().unwrap().parse().expect("not a number");
    (a, b)
}

fn calculate_difference(left: Vec<i32>, right: Vec<i32>) -> i32 {
    let mut left = left.clone();
    left.sort_unstable();

    let mut right = right.clone();
    right.sort_unstable();

    left.into_iter()
        .zip(right)
        .map(|(a, b)| (a - b).abs())
        .sum()
}

fn calculate_similarity(left: Vec<i32>, right: Vec<i32>) -> i32 {
    let mut right_tally = HashMap::new();
    for n in right {
        right_tally
            .entry(n)
            .and_modify(|counter| *counter += 1)
            .or_insert(1);
    }

    left.into_iter()
        .map(|n| n * right_tally.get(&n).unwrap_or(&0))
        .sum()
}
