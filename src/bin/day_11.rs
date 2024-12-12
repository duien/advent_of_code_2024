const DAY: u8 = 11;
const TEST_INPUT: &str = "\
125 17
";

use std::{fs, collections::HashMap};

fn main() {
    let stones = iterate_stones(TEST_INPUT);
    let count = stones.len();
    println!("naive  (test): {}", count);
    assert_eq!(count, 55312, "failed on test data");

    let stones = iterate_stones(file_input().as_str());
    let count = stones.len();
    println!("naive  (real): {}", count);

    let count = count_iterated_stones(TEST_INPUT, 25);
    println!("better (test): {}", count);

    let count = count_iterated_stones(file_input().as_str(), 25);
    println!("better (real): {}", count);

    let count = count_iterated_stones(file_input().as_str(), 75);
    println!("better (pt 2): {}", count);
}

fn count_iterated_stones(input: &str, iterations: u8) -> u64 {
    let mut stones: HashMap<u64, u64> = HashMap::new();
    input
        .split_ascii_whitespace()
        .map(|n| n.parse().unwrap())
        .for_each(|n| {stones.insert(n, 1);} );
    for _i in 0..iterations {
        // dbg!(&stones);
        stones = blink_map(&stones);
    }
    // dbg!(&stones);
    stones.values().sum()
}

fn iterate_stones(input: &str) -> Vec<u64> {
    let mut stones: Vec<u64> = input
        .split_ascii_whitespace()
        .map(|n| n.parse().unwrap())
        .collect();

    for _i in 0..25 {
        // println!("{i}: {}", stones.len());
        stones = blink(&stones);
    }
    stones
}

fn blink_map(stones: &HashMap<u64, u64>) -> HashMap<u64, u64> {
    let mut new_stones: HashMap<u64, u64> = HashMap::new();
    for (stone, count) in stones.iter() {
        if *stone == 0 {
            new_stones.entry(1).and_modify(|c| *c += count).or_insert(*count);
        } else {
            let as_chars: Vec<char> = stone.to_string().chars().collect();
            if as_chars.len() % 2 == 0 {
                let middle = as_chars.len() / 2;
                let left = as_chars[0..middle].into_iter().collect::<String>().parse().unwrap();
                let right = as_chars[middle..].into_iter().collect::<String>().parse().unwrap();

                new_stones.entry(left).and_modify(|c| *c += count).or_insert(*count);
                new_stones.entry(right).and_modify(|c| *c += count).or_insert(*count);
            } else {
                new_stones.entry(*stone * 2024).and_modify(|c| *c += count).or_insert(*count);
            }
        }
    }
    new_stones
}

fn blink(stones: &[u64]) -> Vec<u64> {
    let mut new_stones: Vec<u64> = vec![];
    for stone in stones {
        if *stone == 0 {
            new_stones.push(1);
        } else {
            let as_chars: Vec<char> = stone.to_string().chars().collect();
            if as_chars.len() % 2 == 0 {
                let middle = as_chars.len() / 2;
                new_stones.push(as_chars[0..middle].into_iter().collect::<String>().parse().unwrap());
                new_stones.push(as_chars[middle..].into_iter().collect::<String>().parse().unwrap());
            } else {
                new_stones.push(stone * 2024);
            }
        }
    }
    new_stones
}

fn file_input() -> String {
    let file_path = format!("data/day_{:02}.txt", DAY);
    fs::read_to_string(file_path).expect("unable to read file")
}
