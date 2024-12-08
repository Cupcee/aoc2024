use num::integer::gcd;
use shared::*;
use std::collections::{HashMap, HashSet};
use std::fs;

fn input_to_map(grid: &Vec<Vec<char>>) -> HashMap<char, Vec<(isize, isize)>> {
    let mut ant_to_coords: HashMap<char, Vec<(isize, isize)>> = HashMap::new();
    grid.iter().enumerate().for_each(|(i, line)| {
        line.iter().enumerate().for_each(|(j, char)| match char {
            '.' => {}
            other => ant_to_coords
                .entry(*other)
                .or_insert_with(Vec::new)
                .push((i as isize, j as isize)),
        });
    });
    ant_to_coords
}

fn problem1(input: String) {
    let grid = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    let (nrows, ncols) = (grid.len(), grid[0].len());
    let map = input_to_map(&grid);
    let mut antinodes_set: HashSet<(isize, isize)> = HashSet::new();

    // Iterate over all pairs of antennas with the same frequency
    for (frequency, positions) in map.iter() {
        let n = positions.len();
        for i in 0..n {
            let (x1, y1) = positions[i];
            for j in i + 1..n {
                let (x2, y2) = positions[j];

                // Calculate dx and dy as the direction vector from antenna1 to antenna2
                let dx = x2 - x1;
                let dy = y2 - y1;

                // Calculate the two antinodes
                // Extend the line segment in both directions by twice the distance
                let antinode1 = (x1 - dx, y1 - dy);
                let antinode2 = (x2 + dx, y2 + dy);

                // Check if antinodes are within bounds and add them to the set
                if 0 <= antinode1.0
                    && antinode1.0 < nrows as isize
                    && 0 <= antinode1.1
                    && antinode1.1 < ncols as isize
                {
                    antinodes_set.insert(antinode1);
                }
                if 0 <= antinode2.0
                    && antinode2.0 < nrows as isize
                    && 0 <= antinode2.1
                    && antinode2.1 < ncols as isize
                {
                    antinodes_set.insert(antinode2);
                }
            }
        }
    }

    // Print the count of unique antinodes
    pretty_print_answer(antinodes_set.len());
}

fn problem2(input: String) {
    let grid = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    let (nrows, ncols) = (grid.len(), grid[0].len());
    let map = input_to_map(&grid);
    let mut antinodes_set: HashSet<(isize, isize)> = HashSet::new();

    // Iterate over all antenna frequencies
    for positions in map.values() {
        let n = positions.len();

        // Include all antenna positions themselves as antinodes
        positions.iter().for_each(|&pos| {
            antinodes_set.insert(pos);
        });

        for i in 0..n {
            let (x1, y1) = positions[i];
            for j in i + 1..n {
                let (x2, y2) = positions[j];

                // Calculate the direction vector between the two antennas
                let dx = x2 - x1;
                let dy = y2 - y1;

                // Use the greatest common divisor to normalize the direction.
                // Division by the gcd shared between the two direction vectors ensures
                // the direction vector becomes the smallest step increment to add to some
                // point p on the line to traverse to next point on the line
                let gcd = gcd(dx.abs(), dy.abs());
                let step_x = dx / gcd;
                let step_y = dy / gcd;

                // Extend the line in both directions to cover all collinear points
                // Forward direction

                // below works, because ALL collinear points along the line spanned by
                // the two points (x1, y1), (x2, y2) can be written as:
                // (xn, yn) = (x1 + k * dx, y1 + k * dy)
                // or in other direction:
                // (xn, yn) = (x2 + k * dx, y2 + k * dy)
                let mut x = x2 + step_x;
                let mut y = y2 + step_y;

                // "k" above is the iteration of the loop below
                while 0 <= x && x < nrows as isize && 0 <= y && y < ncols as isize {
                    antinodes_set.insert((x, y));
                    x += step_x;
                    y += step_y;
                }

                // Backward direction
                let mut x = x1 - step_x;
                let mut y = y1 - step_y;
                while 0 <= x && x < nrows as isize && 0 <= y && y < ncols as isize {
                    antinodes_set.insert((x, y));
                    x -= step_x;
                    y -= step_y;
                }
            }
        }
    }

    // Print the count of unique antinodes
    pretty_print_answer(antinodes_set.len());
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
