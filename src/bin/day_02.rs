const TEST_INPUT :&str = "\
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
";

use std::fs;

#[derive(Debug)]
struct Report {
    levels: Vec<i32>
}

impl Report {
    fn is_safe(&self) -> bool {
        let diffs = self.levels.windows(2)
            .map(|w| w[1] - w[0])
            .collect::<Vec<_>>();
        
        let stable_increase = diffs.iter().all(|d| ( 1..= 3).contains(d));
        let stable_decrease = diffs.iter().all(|d| (-3..=-1).contains(d));
        stable_increase || stable_decrease
    }
}

fn file_input() -> String {
    let file_path = "../ruby/data/day_02.txt";
    fs::read_to_string(file_path)
        .expect("unable to read file")
}

fn main() {
    let reports = parse_input(TEST_INPUT);
    println!("{:?}", reports);
    let safe_count = reports.iter()
        .filter(|r| r.is_safe())
        .count();
    println!("{safe_count}");

    let real_reports = parse_input(file_input().as_str());
    let real_safe_count = real_reports.iter()
        .filter(|r| r.is_safe())
        .count();
    println!("{real_safe_count}");
}



fn parse_input(input: &str) -> Vec<Report> {
    input.lines()
        .map(parse_line)
        .collect()
}

fn parse_line(line: &str) -> Report {
    Report {
        levels: line
            .split_whitespace()
            .map(|n| n.parse().expect("not a number"))
            .collect()
    }
}
