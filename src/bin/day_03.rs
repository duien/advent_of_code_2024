const TEST_INPUT : &str = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
const TEST_2_INPUT: &str = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
use std::fs;
use regex::Regex;

fn file_input() -> String {
    let file_path = "../ruby/data/day_03.txt";
    fs::read_to_string(file_path)
        .expect("unable to read file")
}


fn main() {
    // let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    // // let mut results = vec![];
    // for (s, [m, n]) in re.captures_iter(TEST_INPUT).map(|c| c.extract()) {
    //     // results.push((path, lineno.parse::<u64>()?, line));
    //     println!("{:?} {m} {n}", s);
    // }

    // let nums : Vec<u32> = re.captures_iter(TEST_INPUT).map( |c| -> u32 {
    //     let (_, [m, n]) = c.extract();
    //     // println!("{m} {n}");
    //     // m.parse() * n.parse()
    //     let m : u32 = m.parse().expect("should be a number");
    //     let n : u32 = n.parse().expect("should be a number");
    //     dbg!(&m);
    //     dbg!(&n);
    //     m * n
    // }).collect();

    // dbg!(&nums);


    // let mut result = 0;
    // for (_, [m, n]) in re.captures_iter(TEST_INPUT).map(|c| c.extract()) {
    //     let m : u32 = m.parse().unwrap();
    //     let n : u32 = n.parse().unwrap();
    //     result += m * n
    // }
    println!("\n--- PART 1 ---");
    
    let result = sum_mults(TEST_INPUT);
    println!("sum: {result}");
    assert_eq!(result, 161);

    let result = sum_mults(file_input().as_str());
    println!("sum: {result}");

    println!("\n--- PART 2 ---");

    let result = sum_mults_conditionally(TEST_2_INPUT);
    println!("sum: {result}");
    assert_eq!(result, 48);

    let result = sum_mults_conditionally(file_input().as_str());
    println!("sum: {result}");

        
}

fn sum_mults(input : &str) -> u32 {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let mut result = 0;
    for (_s, [m, n]) in re.captures_iter(input).map(|c| c.extract()) {
        // dbg!(&_s);
        let m : u32 = m.parse().unwrap();
        let n : u32 = n.parse().unwrap();
        result += m * n
    }
    // println!("sum: {result}")
    result
}

fn sum_mults_conditionally(input : &str) -> u32 {
    // let re = Regex::new(r"(?:(do)\(\)|(don't)\(\)|(mul)\((\d+),(\d+)\))").unwrap();
    // let re = Regex::new(r"(do|don't|mul)\((?:(\d+),(\d))?\)").unwrap();
    let re = Regex::new(r"(do|don't|mul)\((\d*),?(\d*)\)").unwrap();
    dbg!(&re);
    let mut result = 0;

    let mut is_active = true;
    for (_, [f, m, n]) in re.captures_iter(input).map(|c| c.extract()) {
        
        match (f, m, n) {
            ("do", _, _) => {
                is_active = true;
                println!("DO {is_active}");
            },
            ("don't", _, _) => {
                is_active = false;
                println!("DONT {is_active}");
            },
            ("mul", m, n) => {
                println!("MUL {m} {n} {is_active}");
                if is_active {
                    let m : u32 = m.parse().unwrap();
                    let n : u32 = n.parse().unwrap();
                    result += m * n;
                }
            },
            _ => panic!()
        }
    }

    // for capture in re.captures_iter(input) {
    //     dbg!(&capture);
    //     // match c.extract() {
    //     //     (_s, [m, n]) => println!("{m} {n}")
    //     // }
    //     let (a, [b, c, d]) = capture.extract();
    //     println!("{a} {b} {c} {d}");
    //     let fun = capture.get(1).unwrap().as_str();
    //     dbg!(&fun);

    // }

    // for (_s, [m, n]) in re.captures_iter(input).map(|c| c.extract()) {
    //     dbg!(_s, m, n);
    // }
    result
}
