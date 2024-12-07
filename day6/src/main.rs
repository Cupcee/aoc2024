use shared::*;
use std::fs;
use std::ops::{Index, IndexMut};
use std::process::exit;

pub struct Grid {
    grid: Vec<Vec<char>>,
    rows: usize,
    cols: usize,
    iter: usize,
    guard_distinct_pos: usize,
    guard_pos: (usize, usize),
}

impl Grid {
    fn new(grid: Vec<Vec<char>>) -> Self {
        let rows = grid.len();
        let cols = if rows > 0 { grid[0].len() } else { 0 };
        Grid {
            grid,
            rows,
            cols,
            iter: 0,
            guard_distinct_pos: 0,
            guard_pos: (0, 0),
        }
    }

    fn initialize_guard_pos(&mut self) {
        for i in 0..self.rows {
            for j in 0..self.cols {
                let cell = self[(i, j)];
                if vec!['^', '>', 'v', '<'].contains(&cell) {
                    self.guard_pos = (i, j);
                    return;
                }
            }
        }
        panic!("Guard is not found in grid!");
    }

    fn in_bounds(&self, index: (usize, usize)) -> bool {
        let (row, col) = index;
        row < self.rows && col < self.cols
    }

    fn print_grid(&self) {
        println!(
            "Grid at iteration {} with {} distinct guard positions:",
            self.iter, self.guard_distinct_pos
        );
        self.grid
            .iter()
            .for_each(|row| println!("{}", row.iter().collect::<String>()));
    }
}

impl Index<(usize, usize)> for Grid {
    type Output = char;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.grid[index.0][index.1]
    }
}

impl IndexMut<(usize, usize)> for Grid {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        &mut self.grid[index.0][index.1]
    }
}

fn direction_vector(direction: char) -> (isize, isize) {
    match direction {
        '^' => (-1, 0),
        '>' => (0, 1),
        'V' => (1, 0),
        '<' => (0, -1),
        _ => panic!("Invalid direction character: {}", direction),
    }
}

fn rotate_direction(direction: char) -> char {
    match direction {
        '^' => '>',
        '>' => 'V',
        'V' => '<',
        '<' => '^',
        _ => panic!("Invalid direction to rotate: {}", direction),
    }
}

fn try_move_guard(mat: &mut Grid, direction: char) {
    let (dr, dc) = direction_vector(direction);
    let old_pos = mat.guard_pos;

    // Compute the new position
    let new_r = old_pos.0 as isize + dr;
    let new_c = old_pos.1 as isize + dc;

    if new_r < 0 || new_c < 0 {
        // Out of bounds, guard leaves grid
        end_simulation(mat, old_pos);
    }

    let new_pos = (new_r as usize, new_c as usize);

    if mat.in_bounds(new_pos) {
        let next_cell = mat[new_pos];
        match next_cell {
            '.' => {
                // Move guard, and new distinct position
                mat[old_pos] = 'X';
                mat[new_pos] = direction;
                mat.guard_distinct_pos += 1;
                mat.guard_pos = new_pos;
            }
            'X' => {
                // Move guard, but previously visited position
                mat[old_pos] = 'X';
                mat[new_pos] = direction;
                mat.guard_pos = new_pos;
            }
            '#' => {
                // Rotate direction
                mat[old_pos] = rotate_direction(direction);
            }
            other => {
                panic!("Unexpected cell at try_move_guard {}", other);
            }
        }
    } else {
        // Out of bounds
        end_simulation(mat, old_pos);
    }
}

fn end_simulation(mat: &mut Grid, old_pos: (usize, usize)) {
    mat[old_pos] = 'X';
    mat.guard_distinct_pos += 1;
    mat.iter += 1;
    mat.print_grid();
    exit(0);
}

fn update_grid(mat: &mut Grid) {
    let guard_char = mat[mat.guard_pos];
    match guard_char {
        '^' => try_move_guard(mat, '^'),
        '>' => try_move_guard(mat, '>'),
        'V' => try_move_guard(mat, 'V'),
        '<' => try_move_guard(mat, '<'),
        other => panic!("Unexpected cell at update_grid {}", other),
    }
    mat.iter += 1;
    mat.print_grid();
}

fn parse_grid(input: &str) -> Grid {
    let lines: Vec<&str> = input.lines().collect();
    if lines.is_empty() {
        panic!("Input is empty, cannot construct a grid.");
    }

    // let line_length = lines[0].len();
    // if !lines.iter().all(|&line| line.len() == line_length) {
    //     panic!("Not all lines have equal length.");
    // }

    let grid: Vec<Vec<char>> = lines
        .iter()
        .map(|&line| line.chars().collect::<Vec<char>>())
        .collect();

    Grid::new(grid)
}

fn problem1(input: String, debug: bool) {
    let mut input_grid = parse_grid(&input);
    input_grid.initialize_guard_pos();
    input_grid.print_grid();
    loop {
        update_grid(&mut input_grid);
    }
}

fn main() {
    let args = Args::argparse();
    let input = fs::read_to_string(args.input).unwrap();

    match args.problem {
        1 => problem1(input, args.debug),
        _ => panic!("Not implemented"),
    }
}
