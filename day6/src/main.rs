use indicatif::ProgressIterator;
use shared::*;
use std::fs;
use std::ops::{Index, IndexMut};
use std::process::exit;

const MAX_ITERS: usize = 100000;

#[derive(Clone, Debug)]
pub struct Grid {
    grid: Vec<Vec<char>>,
    rows: usize,
    cols: usize,
    iter: usize,
    guard_distinct_pos: usize,
    guard_pos: (usize, usize),
    previously_rotated: bool,
}

impl Grid {
    fn new(grid: Vec<Vec<char>>) -> Self {
        let rows = grid.len();
        let cols = if rows > 0 { grid[0].len() } else { 0 };
        let mut grid = Grid {
            grid,
            rows,
            cols,
            iter: 0,
            guard_distinct_pos: 0,
            guard_pos: (0, 0),
            previously_rotated: false,
        };
        grid.initialize_guard_pos();
        grid
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
        'v' => (1, 0),
        '<' => (0, -1),
        _ => panic!("Invalid direction character: {}", direction),
    }
}

fn rotate_direction(direction: char) -> char {
    match direction {
        '^' => '>',
        '>' => 'v',
        'v' => '<',
        '<' => '^',
        _ => panic!("Invalid direction to rotate: {}", direction),
    }
}

fn old_position_mark(direction: char, previously_rotated: bool) -> char {
    if previously_rotated {
        return '+';
    }
    match direction {
        '^' => '|',
        '>' => '-',
        'v' => '|',
        '<' => '-',
        _ => panic!("Invalid direction character: {}", direction),
    }
}

fn try_move_guard(mat: &mut Grid, direction: char, debug: bool) -> bool {
    let (dr, dc) = direction_vector(direction);
    let old_pos = mat.guard_pos;

    // Compute the new position
    let new_r = old_pos.0 as isize + dr;
    let new_c = old_pos.1 as isize + dc;

    if new_r < 0 || new_c < 0 {
        // Out of bounds, guard leaves grid
        end_simulation(mat, old_pos, direction, debug);
        return true;
    }

    let new_pos = (new_r as usize, new_c as usize);

    if mat.in_bounds(new_pos) {
        let next_cell = mat[new_pos];
        match next_cell {
            '.' => {
                // Move guard, and new distinct position
                mat[old_pos] = old_position_mark(direction, mat.previously_rotated);
                mat[new_pos] = direction;
                mat.guard_distinct_pos += 1;
                mat.guard_pos = new_pos;
                mat.previously_rotated = false;
            }
            's' | '-' | '|' | '+' => {
                // Move guard, but previously visited position
                mat[old_pos] = old_position_mark(direction, mat.previously_rotated);
                mat[new_pos] = direction;
                mat.guard_pos = new_pos;
                mat.previously_rotated = false;
            }
            '#' | 'O' => {
                // Rotate direction
                let new_direction = rotate_direction(direction);
                mat[old_pos] = new_direction;
                mat.previously_rotated = true;
            }
            other => {
                panic!("Unexpected cell at try_move_guard {}", other);
            }
        }
        false
    } else {
        // Out of bounds
        end_simulation(mat, old_pos, direction, debug);
        true
    }
}

fn end_simulation(mat: &mut Grid, old_pos: (usize, usize), direction: char, debug: bool) {
    mat[old_pos] = old_position_mark(direction, mat.previously_rotated);
    mat.guard_distinct_pos += 1;
    mat.iter += 1;
    if debug {
        mat.print_grid();
    }
}

fn update_grid(mat: &mut Grid, debug: bool) -> bool {
    let guard_char = mat[mat.guard_pos];
    let should_break = match guard_char {
        '^' => try_move_guard(mat, '^', debug),
        '>' => try_move_guard(mat, '>', debug),
        'v' => try_move_guard(mat, 'v', debug),
        '<' => try_move_guard(mat, '<', debug),
        other => panic!("Unexpected cell at update_grid {}", other),
    };
    mat.iter += 1;
    if debug {
        mat.print_grid();
    }
    should_break
}

fn parse_grid(input: &str) -> Grid {
    let lines: Vec<&str> = input.lines().collect();
    if lines.is_empty() {
        panic!("Input is empty, cannot construct a grid.");
    }

    let grid: Vec<Vec<char>> = lines
        .iter()
        .map(|&line| line.chars().collect::<Vec<char>>())
        .collect();

    Grid::new(grid)
}

/// Compute how many iters it takes for "guard" to leave the grid
fn problem1(input: String, debug: bool) {
    let mut input_grid = parse_grid(&input);
    if debug {
        input_grid.print_grid();
    }
    // break after MAX_ITERS at latest, in case we have a never ending cycle
    while input_grid.iter < MAX_ITERS {
        let should_break = update_grid(&mut input_grid, debug);
        if should_break {
            break;
        };
    }
    pretty_print_answer(input_grid.guard_distinct_pos);
}

/// Naively add in obstacles and detect which positions create a cycle
fn problem2(input: String, debug: bool) {
    let input_grid = parse_grid(&input);
    let mut obstruction_count = 0;
    for i in (0..input_grid.rows).progress() {
        for j in 0..input_grid.cols {
            let mut obstructed = true;
            let mut current_grid = input_grid.clone();
            let cell_at = current_grid[(i, j)];
            if cell_at != '.' {
                continue;
            } else {
                current_grid[(i, j)] = 'O';
            }
            if debug {
                current_grid.print_grid();
            }
            // break after MAX_ITERS at latest, in case we have a never ending cycle
            'inner: while current_grid.iter < MAX_ITERS {
                let should_break = update_grid(&mut current_grid, debug);
                if should_break {
                    obstructed = false;
                    break 'inner;
                };
            }
            if obstructed {
                obstruction_count += 1;
            }
        }
    }
    pretty_print_answer(obstruction_count);
}

fn main() {
    let args = Args::argparse();
    let input = fs::read_to_string(args.input).unwrap();

    match args.problem {
        1 => problem1(input, args.debug),
        2 => problem2(input, args.debug),
        _ => panic!("Not implemented"),
    }
}
