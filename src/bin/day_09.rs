const DAY: u8 = 9;
const TEST_INPUT: &str = "2333133121414131402
";

use std::fs;

fn main() {
    println!("\n-- PART 1 --");
    let result = check_fragment(TEST_INPUT);
    println!("TEST: {}", result);
    assert_eq!(result, 1928, "failed on test input");
    println!("REAL: {}", check_fragment(file_input().as_str()));

    // println!("\n-- PART 2 --");
    // let result = count_harmonic_antinodes(TEST_INPUT);
    // println!("TEST: {}", result);
    // assert_eq!(result, 34, "failed on test input");
    // println!("REAL: {}", count_harmonic_antinodes(file_input().as_str()));
}

fn check_fragment(input: &str) -> u64 {
    let disk = map_to_blocks(input);
    let result = fragmentalize(disk);
    let check = checksum(&result);
    check
}

fn checksum(disk: &[Option<u32>]) -> u64 {
    disk.into_iter().enumerate().map(|(i, n)| match n {
        Some(n) => stupid_mult(i, *n),
        None => 0
    }).sum()
}

fn stupid_mult(a: usize, b: u32) -> u64 {
    let a: u64 = a.try_into().unwrap();
    let b: u64 = b.into();
    a * b
}

fn fragmentalize(mut disk: Vec<Option<u32>>) -> Vec<Option<u32>> {
    let mut j = disk.len() - 1;
    for i in 0..disk.len() {
        match disk[i] {
            Some(..) => (),
            None => {
                while j > i {
                    match disk[j] {
                        None => (),
                        Some(..) => {
                            disk.swap(i, j);
                            break;
                        }
                    }
                    j -= 1;
                }
            }
        }
    }
    disk
}

fn map_to_blocks(input: &str) -> Vec<Option<u32>> {
    let digits = input.chars()
        .filter_map(|c| c.to_digit(10)).collect::<Vec<_>>();
    // dbg!(&digits);
    digits.chunks(2).enumerate().map(|(i, pair)| {
        let i : u32 = i.try_into().unwrap();
        let file = vec![Some(i); pair[0].try_into().unwrap()];
        // there must be a better way...
        if let Some(_) = pair.get(1) {
            let gap = vec![None; pair[1].try_into().unwrap()];
            vec![file, gap]
        } else {
            vec![file]
        }
    }).flatten().flatten().collect::<Vec<_>>()
}

fn file_input() -> String {
    let file_path = format!("../ruby/data/day_{:02}.txt", DAY);
    fs::read_to_string(file_path).expect("unable to read file")
}

fn inspect_disk(disk: &[Option<u32>]) {
    for block in disk {
        match block {
            Some(n) => print!("{n}"),
            None => print!(".")
        }
    }
    println!();
}
