use shared::*;
use std::fs;

// problem 1
const START_CHAR1: char = 'X';
const KEYWORD1: &str = "XMAS";
const WORD_LEN1: usize = KEYWORD1.len();
const STEPS1: usize = WORD_LEN1 - 1;

// problem 2
const START_CHAR2: char = 'A';
const KEYWORD2: &str = "MAS";
const KEYWORD2_REV: &str = "SAM";

/// Check if the keyword is found starting at (row, col) in the given direction (d_row, d_col).
fn check_direction(
    row: usize,
    col: usize,
    d_row: isize,
    d_col: isize,
    nrows: usize,
    ncols: usize,
    mat: &Vec<Vec<char>>,
) -> usize {
    // Calculate the end position based on the steps required.
    let end_row = row as isize + d_row * (STEPS1 as isize);
    let end_col = col as isize + d_col * (STEPS1 as isize);

    // If the end position is outside the matrix boundaries, return 0 early.
    if end_row < 0 || end_col < 0 || end_row >= nrows as isize || end_col >= ncols as isize {
        return 0;
    }

    // Collect characters along this direction to form the candidate string.
    let seq: String = (0..WORD_LEN1)
        .map(|i| {
            let r = (row as isize + d_row * (i as isize)) as usize;
            let c = (col as isize + d_col * (i as isize)) as usize;
            mat[r][c]
        })
        .collect();

    if seq == KEYWORD1 {
        1
    } else {
        0
    }
}

fn get_char_in_direction(
    row: usize,
    col: usize,
    d_row_col: (isize, isize),
    nrows: usize,
    ncols: usize,
    mat: &Vec<Vec<char>>,
) -> Option<char> {
    // Calculate the end position based on the steps required.
    let end_row = row as isize + d_row_col.0;
    let end_col = col as isize + d_row_col.1;

    // If the end position is outside the matrix boundaries, return 0 early.
    if end_row < 0 || end_col < 0 || end_row >= nrows as isize || end_col >= ncols as isize {
        return None;
    }

    return Some(mat[end_row as usize][end_col as usize]);
}

/// Look around a given position in all eight directions and return how many times KEYWORD is found.
fn look_around1(
    row_idx: usize,
    col_idx: usize,
    nrows: usize,
    ncols: usize,
    mat: &Vec<Vec<char>>,
) -> usize {
    // Define all eight directions as (d_row, d_col).
    let directions: &[(isize, isize)] = &[
        (-1, 0),  // UP
        (1, 0),   // DOWN
        (0, -1),  // LEFT
        (0, 1),   // RIGHT
        (-1, -1), // UP-LEFT
        (-1, 1),  // UP-RIGHT
        (1, -1),  // DOWN-LEFT
        (1, 1),   // DOWN-RIGHT
    ];

    directions
        .iter()
        .map(|&(dr, dc)| check_direction(row_idx, col_idx, dr, dc, nrows, ncols, mat))
        .sum()
}

fn problem1(input: String) {
    let mat: Vec<Vec<_>> = input
        .lines()
        .into_iter()
        .map(|line| line.chars().collect())
        .collect();
    let (nrows, ncols) = (mat.len(), mat[0].len());
    let mut sum = 0;
    for i in 0..nrows {
        for j in 0..ncols {
            let c = mat[i][j];
            if c == START_CHAR1 {
                sum += look_around1(i, j, nrows, ncols, &mat);
            }
        }
    }
    pretty_print_answer(sum);
}

/// Check if the keyword is found starting at (row, col) in the given direction (d_row, d_col).
fn check_diagonals(
    row: usize,
    col: usize,
    nrows: usize,
    ncols: usize,
    mat: &Vec<Vec<char>>,
) -> usize {
    let directions: &[(isize, isize)] = &[
        (-1, -1), // UP-LEFT
        (-1, 1),  // UP-RIGHT
        (1, -1),  // DOWN-LEFT
        (1, 1),   // DOWN-RIGHT
    ];

    let upleft_char = get_char_in_direction(row, col, directions[0], nrows, ncols, mat);
    let upright_char = get_char_in_direction(row, col, directions[1], nrows, ncols, mat);
    let downleft_char = get_char_in_direction(row, col, directions[2], nrows, ncols, mat);
    let downright_char = get_char_in_direction(row, col, directions[3], nrows, ncols, mat);
    let diag1 = [
        upleft_char.unwrap_or('_'),
        'A',
        downright_char.unwrap_or('_'),
    ]
    .iter()
    .collect::<String>();
    let diag2 = [
        upright_char.unwrap_or('_'),
        'A',
        downleft_char.unwrap_or('_'),
    ]
    .iter()
    .collect::<String>();
    if (diag1 == KEYWORD2 || diag1 == KEYWORD2_REV) && (diag2 == KEYWORD2 || diag2 == KEYWORD2_REV)
    {
        1
    } else {
        0
    }
}

fn problem2(input: String) {
    let mat: Vec<Vec<_>> = input
        .lines()
        .into_iter()
        .map(|line| line.chars().collect())
        .collect();
    let (nrows, ncols) = (mat.len(), mat[0].len());
    let mut sum = 0;
    for i in 0..nrows {
        for j in 0..ncols {
            let c = mat[i][j];
            if c == START_CHAR2 {
                sum += check_diagonals(i, j, nrows, ncols, &mat);
            }
        }
    }
    pretty_print_answer(sum);
}

fn main() {
    let args = Args::argparse();
    let input = fs::read_to_string(args.input).unwrap();

    match args.problem {
        1 => problem1(input),
        2 => problem2(input),
        _ => panic!("Not implemented"),
    }
}
