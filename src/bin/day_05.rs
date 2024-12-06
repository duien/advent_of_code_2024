const TEST_INPUT : &str = "\
47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47
";

use std::fs;
use nom::{
    IResult,
    sequence::{
        separated_pair,
        terminated
    },
    character::complete::{
        char,
        digit1,
        multispace0,
        line_ending
    },
    combinator::map_res,
    combinator::map,
    multi::{
        many1,
        separated_list1,
    }
};

fn file_input() -> String {
    let file_path = "../ruby/data/day_05.txt";
    fs::read_to_string(file_path)
        .expect("unable to read file")
}

#[derive(Debug)]
struct OrderingRule(i32, i32);

impl OrderingRule {
    fn is_satisfied_by(&self, update: &Update) -> bool {
        // println!("  {:?}", self);
        let pos_m = update.pages.iter().position(|&x| x == self.0);
        let pos_n = update.pages.iter().position(|&x| x == self.1);
        
        match (pos_m, pos_n) {
            (None, None) => true,
            (None, _) => true,
            (_, None) => true,
            (Some(m), Some(n)) => m < n
        }
    }
}

#[derive(Debug)]
struct Update {
    pages: Vec<i32>
}

impl Update {
    fn satisfies_all(&self, rules: &Vec<OrderingRule>) -> bool {
        rules.iter().all(|r| r.is_satisfied_by(self))
    }

    fn middle_page(&self) -> i32 {
        self.pages[self.pages.len() / 2]
    }
}

fn main() {
    let (rules, updates) = parse_input(TEST_INPUT);
    let page_sum = sum_valid_pages(rules, updates);
    println!("ANSWER IS: {page_sum}");

    let (rules, updates) = parse_input(file_input().as_str());
    let page_sum = sum_valid_pages(rules, updates);
    println!("ANSWER IS: {page_sum}");

    
}

fn sum_valid_pages(rules : Vec<OrderingRule>, updates: Vec<Update>) -> i32 {
    updates.iter()
        .filter(|u| u.satisfies_all(&rules))
        .map(|u| u.middle_page())
        .fold(0, |acc, x| acc + x)
}

fn parse_input(input : &str) -> (Vec<OrderingRule>, Vec<Update>) {
    let (_, (rules, updates)) = separated_pair(parse_ordering_rules, multispace0, parse_updates)(input)
        .expect("unable to parse input");
    (rules, updates)
}

fn parse_ordering_rules(input: &str) -> IResult<&str, Vec<OrderingRule>> {
    many1(terminated(ordering_rule, line_ending))(input)
}

fn parse_updates(input: &str) -> IResult<&str, Vec<Update>> {
    many1(terminated(update_list, line_ending))(input)
}

fn update_list_from_digits(pages: Vec<i32>) -> Update {
    Update{ pages }
}

fn update_list(input: &str) -> IResult<&str, Update> {
    map(separated_list1(char(','), parsed_int), update_list_from_digits)(input)
}

fn ordering_rule_from_digits((m, n) : (i32, i32)) -> OrderingRule {
    OrderingRule(m, n)
}

fn ordering_rule(input: &str) -> IResult<&str, OrderingRule> {
    map(digit_pair, ordering_rule_from_digits)(input)
}

fn digit_pair(input: &str) -> IResult<&str, (i32, i32)> {
    separated_pair(parsed_int, char('|'), parsed_int)(input)
}

fn parsed_int(input: &str) -> IResult<&str, i32> {
    map_res(digit1, parse_int)(input)
}
fn parse_int(input: &str) -> Result<i32, std::num::ParseIntError> {
    i32::from_str_radix(input, 10)
}
