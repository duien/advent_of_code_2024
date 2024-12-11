#![allow(unused_imports)]
#![allow(dead_code)]

const TEST_INPUT: &str = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
const TEST_2_INPUT: &str =
    "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

use regex::Regex;
use std::fs;

use nom::{
    branch::alt, bytes::complete::is_not, bytes::complete::tag, bytes::complete::take,
    bytes::complete::take_until, character::complete::char, character::complete::digit1,
    character::complete::one_of, combinator::map_res, multi::many0, multi::many1,
    multi::separated_list1, sequence::delimited, sequence::preceded, sequence::separated_pair,
    sequence::terminated, sequence::tuple, Finish, IResult,
};

fn file_input() -> String {
    let file_path = "data/day_03.txt";
    fs::read_to_string(file_path).expect("unable to read file")
}

fn main() {
    dbg!(mul_args("mul(a,b)"));
    dbg!(mul_args("mul(5,6)"));
    dbg!(mul_args("mul(65)"));
    dbg!(mul_args("mul(65,)"));
    dbg!(mul_args("mul(65,187)"));
    dbg!(mul_args("mul(65,foo)"));
    dbg!(mul_args("mul(65,8)extra"));
    dbg!(mul_args("mul[65,8]extra"));

    // leading garbage
    dbg!(garbage(TEST_INPUT));

    // leading garbage then a good mul expression
    dbg!(after_garbage(TEST_INPUT));

    // leading garbage then a good mul expression, repeatedly
    dbg!(many_after_garbage(TEST_INPUT));

    let (_, result): (&str, Vec<(i32, i32)>) =
        many_after_garbage(TEST_INPUT).expect("parse failure");
    dbg!(&result);

    let result: Vec<i32> = result.into_iter().map(|(m, n)| m * n).collect();
    dbg!(&result);

    let sum: i32 = result.into_iter().fold(0, |acc, x| acc + x);
    dbg!(&sum);

    dbg!(parse_and_sum(TEST_INPUT));
    dbg!(parse_and_sum(file_input().as_str()));
}

fn parse_and_sum(input: &str) -> i32 {
    let (_, result): (&str, Vec<(i32, i32)>) = many_after_garbage(input).expect("parse failure");
    result
        .into_iter()
        .map(|(m, n)| m * n)
        .fold(0, |acc, x| acc + x)
}

// as many valid mul expressions as we can get
fn many_after_garbage(input: &str) -> IResult<&str, Vec<(i32, i32)>> {
    many1(after_garbage)(input)
}

// any old garbage until we see 'mul'
fn garbage(input: &str) -> IResult<&str, &str> {
    take_until("mul(")(input)
}

// garbage and then a valid mul expression
fn after_garbage(input: &str) -> IResult<&str, (i32, i32)> {
    preceded(garbage, mul_args_combo)(input)
}

// parens containing two numbers
fn parens(input: &str) -> IResult<&str, (i32, i32)> {
    delimited(char('('), two_numbers, char(')'))(input)
}

// two numbers separated by a comma
fn two_numbers(input: &str) -> IResult<&str, (i32, i32)> {
    separated_pair(parsed_num, char(','), parsed_num)(input)
}

// digits turned into an actual int
fn parsed_num(input: &str) -> IResult<&str, i32> {
    map_res(digit1, my_parse)(input)
}

// an int from a str
fn my_parse(input: &str) -> Result<i32, std::num::ParseIntError> {
    i32::from_str_radix(input, 10)
}

// 'mul' followed by its args
fn mul_args(input: &str) -> IResult<&str, (i32, i32)> {
    preceded(tag("mul"), parens)(input)
}

// either a valid mul expression or toss that and grab more garbage
// until the next mul expression
fn mul_args_combo(input: &str) -> IResult<&str, (i32, i32)> {
    alt((mul_args, preceded(tag("mul"), after_garbage)))(input)
}
