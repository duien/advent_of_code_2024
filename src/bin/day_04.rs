const TEST_INPUT: &str = "\
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

use std::fs;

fn file_input() -> String {
    let file_path = "data/day_04.txt";
    fs::read_to_string(file_path).expect("unable to read file")
}

fn main() {
    dbg!(find_xmas(TEST_INPUT));
    dbg!(find_xmas(file_input().as_str()));

    dbg!(find_mas_x(TEST_INPUT));
    dbg!(find_mas_x(file_input().as_str()));
}

fn find_mas_x(input: &str) -> usize {
    let char_grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let grid_height = char_grid.len();
    let grid_width = char_grid[0].len();
    let mut exes = 0;
    for (i, line) in char_grid.iter().enumerate() {
        if i == 0 || i == grid_height - 1 {
            continue;
        }
        for (j, chr) in line.iter().enumerate() {
            if j == 0 || j == grid_width - 1 {
                continue;
            }
            if *chr == 'A' {
                let diag_a = (char_grid[i - 1][j - 1], char_grid[i + 1][j + 1]);
                let diag_b = (char_grid[i - 1][j + 1], char_grid[i + 1][j - 1]);
                if diag_a == ('M', 'S') || diag_a == ('S', 'M') {
                    if diag_b == ('M', 'S') || diag_b == ('S', 'M') {
                        exes += 1
                    }
                }
            }
        }
    }
    exes
}

fn find_xmas(input: &str) -> usize {
    let char_grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let grid_height = char_grid.len();
    let grid_width = char_grid[0].len();
    let mut exes = 0;
    for (i, line) in char_grid.iter().enumerate() {
        for (j, chr) in line.iter().enumerate() {
            if *chr == 'X' {
                let can_go_north = i >= 3;
                let can_go_west = j >= 3;
                let can_go_south = i < grid_height - 3;
                let can_go_east = j < grid_width - 3;

                if can_go_north
                    && char_grid[i - 1][j] == 'M'
                    && char_grid[i - 2][j] == 'A'
                    && char_grid[i - 3][j] == 'S'
                {
                    exes += 1;
                }
                if can_go_south
                    && char_grid[i + 1][j] == 'M'
                    && char_grid[i + 2][j] == 'A'
                    && char_grid[i + 3][j] == 'S'
                {
                    exes += 1;
                }
                if can_go_west
                    && char_grid[i][j - 1] == 'M'
                    && char_grid[i][j - 2] == 'A'
                    && char_grid[i][j - 3] == 'S'
                {
                    exes += 1;
                }
                if can_go_east
                    && char_grid[i][j + 1] == 'M'
                    && char_grid[i][j + 2] == 'A'
                    && char_grid[i][j + 3] == 'S'
                {
                    exes += 1;
                }
                if can_go_north
                    && can_go_west
                    && char_grid[i - 1][j - 1] == 'M'
                    && char_grid[i - 2][j - 2] == 'A'
                    && char_grid[i - 3][j - 3] == 'S'
                {
                    exes += 1;
                }
                if can_go_north
                    && can_go_east
                    && char_grid[i - 1][j + 1] == 'M'
                    && char_grid[i - 2][j + 2] == 'A'
                    && char_grid[i - 3][j + 3] == 'S'
                {
                    exes += 1;
                }
                if can_go_south
                    && can_go_west
                    && char_grid[i + 1][j - 1] == 'M'
                    && char_grid[i + 2][j - 2] == 'A'
                    && char_grid[i + 3][j - 3] == 'S'
                {
                    exes += 1;
                }
                if can_go_south
                    && can_go_east
                    && char_grid[i + 1][j + 1] == 'M'
                    && char_grid[i + 2][j + 2] == 'A'
                    && char_grid[i + 3][j + 3] == 'S'
                {
                    exes += 1;
                }
            }
        }
    }

    exes
}
