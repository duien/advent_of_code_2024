const TEST_INPUT: &str = "\
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
";

use std::fs;

// Add to report struct a dampened: Option<usize>
// ... maybe a more complicated Enum
// the report could be
// - safe without dampening
// - safe with specific dampening
// - not yet checked
// I want some of those differences for state as I'm checking and some
// for skipping computation

// safe is Option
// Some(true) means you've checked and it is
// Some(false) means you've checked and it's not
// None means you don't know and need to check

// but it means that _checking_ safe requires mutability (sort of)

// the dampening position and the direction are only needed during calculation

/*

totally OK
7 6 4 2 1
7 6          : -1 ok
  6 4        : -2 ok
    4 2      : -2 ok
      2 1    : -1 ok

unrecoverable (middle)
1 2 7 8 9
1 2          : +1 ok
  2 7        : +5 bad
------------
1   7        : +6 bad (skip left)
  2  8       : +6 bad (skip right)

unrecoverable (beginning)
1 6 7 8 9
1 6          : +5 bad
------------
  6          : not a pair
1   7        : + 6 bad

recoverable


recover then fail again


failure types:
- switched direction (but individual step sizes are ok)
- step size is bad (but direction is ok)
- those happening in an interlapping way but opposite??



=====
direction: [unknown, increasing, decreasing]
   (or Some(Direction))
dampened: [false, posistion] // could just be true/false

is_safe_pair_in_context
 (left, right), direction, dampening

 */

#[derive(Debug)]
struct Report {
    levels: Vec<i32>,
}

impl Report {
    fn is_safe(&self) -> bool {
        let diffs = self
            .levels
            .windows(2)
            .map(|w| w[1] - w[0])
            .collect::<Vec<_>>();

        let stable_increase = diffs.iter().all(|d| (1..=3).contains(d));
        let stable_decrease = diffs.iter().all(|d| (-3..=-1).contains(d));
        stable_increase || stable_decrease
    }

    fn could_be_safe(&self) -> bool {
        if self.is_safe() {
            return true;
        } else {
            self.levels.iter().enumerate().any(|(i, _v)| {
                let mut levels = self.levels.clone();
                levels.remove(i);
                Report { levels }.is_safe()
            })
        }
    }
}

fn file_input() -> String {
    let file_path = "data/day_02.txt";
    fs::read_to_string(file_path).expect("unable to read file")
}

fn main() {
    println!("{}", count_is_safe(TEST_INPUT));
    println!("{}", count_could_be_safe(TEST_INPUT));

    let why = file_input();
    let real_input = why.as_str();

    println!("{}", count_is_safe(real_input));
    println!("{}", count_could_be_safe(real_input));
}

fn count_is_safe(input: &str) -> usize {
    let reports = parse_input(input);
    reports.iter().filter(|r| r.is_safe()).count()
}

fn count_could_be_safe(input: &str) -> usize {
    let reports = parse_input(input);
    reports.iter().filter(|r| r.could_be_safe()).count()
}

fn parse_input(input: &str) -> Vec<Report> {
    input.lines().map(parse_line).collect()
}

fn parse_line(line: &str) -> Report {
    Report {
        levels: line
            .split_whitespace()
            .map(|n| n.parse().expect("not a number"))
            .collect(),
    }
}

#[cfg(test)]
mod safety_checking {
    use super::*;

    #[test]
    fn test_various_reports() {
        let (r1, r2, r3, r4, r5, r6) = (
            Report {
                levels: vec![7, 6, 4, 2, 1],
            },
            Report {
                levels: vec![1, 2, 7, 8, 9],
            },
            Report {
                levels: vec![9, 7, 6, 2, 1],
            },
            Report {
                levels: vec![1, 3, 2, 4, 5],
            },
            Report {
                levels: vec![8, 6, 4, 4, 1],
            },
            Report {
                levels: vec![1, 3, 6, 7, 9],
            },
        );

        assert_eq!(r1.is_safe(), true, "expected {:?} to be safe", r1);
        assert_eq!(r2.is_safe(), false, "expected {:?} NOT to be safe", r2);
        assert_eq!(r3.is_safe(), false, "expected {:?} NOT to be safe", r3);
        assert_eq!(r4.is_safe(), false, "expected {:?} NOT to be safe", r4);
        assert_eq!(r5.is_safe(), false, "expected {:?} NOT to be safe", r5);
        assert_eq!(r6.is_safe(), true, "expected {:?} to be safe", r6);

        assert_eq!(r1.could_be_safe(), true, "expected {:?} to become safe", r1);
        assert_eq!(
            r2.could_be_safe(),
            false,
            "expected {:?} NOT to become safe",
            r2
        );
        assert_eq!(
            r3.could_be_safe(),
            false,
            "expected {:?} NOT to become safe",
            r3
        );
        assert_eq!(r4.could_be_safe(), true, "expected {:?} to become safe", r4);
        assert_eq!(r5.could_be_safe(), true, "expected {:?} to become safe", r5);
        assert_eq!(r6.could_be_safe(), true, "expected {:?} to become safe", r6);
    }

    #[test]
    fn test_count_is_safe() {
        assert_eq!(count_is_safe(TEST_INPUT), 2);
    }

    #[test]
    fn test_count_could_be_safe() {
        assert_eq!(count_could_be_safe(TEST_INPUT), 4);
    }
}
