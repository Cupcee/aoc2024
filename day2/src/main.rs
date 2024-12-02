use shared::*;
use std::fs;

fn remove_at<T>(vec: &[T], index: usize) -> Vec<T>
where
    T: Clone,
{
    [&vec[..index], &vec[index + 1..]].concat()
}

fn check_conditions_asc(linenumbers: &[i32]) -> (bool, Option<usize>) {
    for (idx, w) in linenumbers.windows(2).enumerate() {
        let d = w[1] - w[0];
        // if NOT ASC then
        if d <= 0 {
            return (false, Some(idx));
        }
        let absdiff = d.abs();
        if absdiff < 1 || absdiff > 3 {
            return (false, Some(idx));
        }
    }

    (true, None)
}

fn line_is_safe(line: &str, with_tolerance: bool) -> Option<bool> {
    if line.is_empty() {
        return None;
    }
    let linenumbers = line
        .split_whitespace()
        .map(|num| num.parse::<i32>().unwrap())
        .collect::<Vec<_>>();
    let linenumbers_rev = linenumbers.clone().into_iter().rev().collect::<Vec<_>>();

    // Check original line
    let (success, v_idx) = check_conditions_asc(&linenumbers);
    let (success_rev, v_idx_rev) = check_conditions_asc(&linenumbers_rev);
    if success || success_rev {
        Some(true)
    } else if with_tolerance {
        let (v_idx, v_idx_rev) = (v_idx.unwrap(), v_idx_rev.unwrap());

        // first, test by removing the first index of the window
        let l1 = remove_at(&linenumbers, v_idx);
        let l2_rev = remove_at(&linenumbers_rev, v_idx_rev);
        let (success, _) = check_conditions_asc(&l1);
        let (success_rev, _) = check_conditions_asc(&l2_rev);

        // then, test by removing second index of the window
        let l1_offset = remove_at(&linenumbers, v_idx + 1);
        let l2_offset_rev = remove_at(&linenumbers_rev, v_idx_rev + 1);
        let (success_offset, _) = check_conditions_asc(&l1_offset);
        let (success_offset_rev, _) = check_conditions_asc(&l2_offset_rev);
        Some(success || success_rev || success_offset || success_offset_rev)
    } else {
        Some(false)
    }
}

fn problem1(input: String) {
    let sum = input
        .lines()
        .filter_map(|line| line_is_safe(line, false))
        .filter(|&is_safe| is_safe)
        .count();
    pretty_print_answer(sum);
}

fn problem2(input: String) {
    let sum = input
        .lines()
        .filter_map(|line| line_is_safe(line, true))
        .filter(|&is_safe| is_safe)
        .count();
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
