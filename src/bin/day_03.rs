#![allow(unused_imports)]
#![allow(dead_code)]

const TEST_INPUT : &str = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
const TEST_2_INPUT: &str = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

use std::fs;
use regex::Regex;

use nom::{
    IResult,
    Finish,
    sequence::delimited,
    sequence::preceded,
    multi::many1,
    multi::many0,
    multi::separated_list1,
    sequence::terminated,
    sequence::tuple,
    sequence::separated_pair,
    character::complete::char,
    character::complete::one_of,
    character::complete::digit1,
    bytes::complete::is_not,
    bytes::complete::tag,
    bytes::complete::take,
    bytes::complete::take_until,
    combinator::map_res,
    branch::alt
};


fn file_input() -> String {
    let file_path = "../ruby/data/day_03.txt";
    fs::read_to_string(file_path)
        .expect("unable to read file")
}


fn main() {
    // let bare = "(2,4)";
    // let func = "mul(2,4)";

    // // dbg!(garbage(bare));
    // // dbg!(garbage(func));
    // dbg!(garbage(TEST_INPUT));
    // // let result = parens(valid);
    // dbg!(parens(bare));
    // dbg!(parens(func));
    // dbg!(parens(TEST_INPUT));
    // dbg!(parens("(anything)"));
    // dbg!(parens("(5)"));

    // // dbg!(maybe(TEST_INPUT));

    // // dbg!(many_parens(bare));
    // // dbg!(many_parens("(2,4)(5,5)(anything)"));

    // // dbg!(many0(maybe(TEST_INPUT)));

    // let (input, _) = take_until("mul")(TEST_INPUT).unwrap();
    // dbg!(input);

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
    // stops on a mul-ish but bad expression
    dbg!(many_after_garbage(TEST_INPUT));

    
}

fn many_after_garbage(input: &str) -> IResult<&str, Vec<(i32, i32)>> {
    many1(after_garbage)(input)
}

fn garbage(input: &str) -> IResult<&str, &str> {
    take_until("mul(")(input)
}

fn after_garbage(input: &str) -> IResult<&str, (i32, i32)> {
    // preceded(garbage, mul_args)(input)
    preceded(garbage, mul_args_combo)(input)
}

// fn mul_or_garbage(input: &str) -> IResult<&str, &str> {
//     alt(mul_args, tag("mul"))(input)
// }

fn parens(input: &str) -> IResult<&str, (i32, i32)> {
    delimited(
        char('('),
        // is_not(")"),
        // separated_list1(tag(","), digit1),
        two_numbers,
        char(')')
    )(input)
}

// fn better_two_numbers(input: &str) -> IResult<&str, (i32, i32)> {
//     map_res(
//         two_numbers,
//         my_parse
//     )
// }
fn two_numbers(input: &str) -> IResult<&str, (i32, i32)> {
    separated_pair(parsed_num, char(','), parsed_num)(input)
}

fn parsed_num(input: &str) -> IResult<&str, i32> {
    map_res(digit1, my_parse)(input)
}

fn my_parse(input: &str) -> Result<i32, std::num::ParseIntError> {
    i32::from_str_radix(input, 10)
}
fn mul_args(input: &str) -> IResult<&str, (i32, i32)> {
    // let (input, _) = tag("mul")(input)?;
    // parens(input)
    // // map_res(
    // //     parens(input),
    // //     my_parse
    // // )
    preceded(tag("mul"), parens)(input)
}

fn mul_args_combo(input: &str) -> IResult<&str, (i32, i32)> {
    // preceded(tag("mul"), parens)(input)
    alt((mul_args, preceded(tag("mul"), after_garbage)))(input)
}

// fn many_parens(input: &str) -> IResult<&str, Vec<&str>> {
//     many0(parens)(input)
// }


// fn maybe(input: &str) -> IResult<&str, &str> {
//     let (input, _) = garbage(TEST_INPUT)?;
//     let (input, _) = tag("mul")(input)?;
//     let (rest, content) = parens(input)?;

//     Ok((rest, content))
// }
