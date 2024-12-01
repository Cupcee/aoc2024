use shared::*;
use std::fs;

fn parse_line(line: &str) -> Option<(i32, i32)> {
    let linenumbers: Vec<&str> = line.split(" ").filter(|item| *item != "").collect();
    if linenumbers.len() == 2 {
        let (item1, item2) = (linenumbers[0], linenumbers[1]);
        let item1_parsed = item1.parse::<i32>().unwrap();
        let item2_parsed = item2.parse::<i32>().unwrap();
        Some((item1_parsed, item2_parsed))
    } else {
        None
    }
}

fn sim_score_for_number(number: i32, col2_counter: &Counter<&i32>) -> i32 {
    number * (col2_counter.get(&&number) as i32)
}

fn problem1(input: String) {
    let (mut col1, mut col2): (Vec<i32>, Vec<i32>) = input
        .split("\n")
        .filter_map(|line| parse_line(line))
        .unzip();
    col1.sort_unstable();
    col2.sort_unstable();
    let pairs: Vec<(i32, i32)> = col1.into_iter().zip(col2).collect();
    // dbg!(&pairs);
    let sum = pairs
        .into_iter()
        .map(|(a, b)| (a - b).abs())
        .reduce(|row1, row2| row1 + row2)
        .unwrap();
    pretty_print_answer(sum);
}

fn problem2(input: String) {
    let (mut col1, col2): (Vec<i32>, Vec<i32>) = input
        .split("\n")
        .filter_map(|line| parse_line(line))
        .unzip();
    col1.sort_unstable();

    let mut col2_counter = Counter::new();
    col2.iter().for_each(|v| col2_counter.add(v));
    let scores: Vec<i32> = col1
        .into_iter()
        .map(|v| sim_score_for_number(v, &col2_counter))
        .collect();

    let sum = scores.into_iter().sum::<i32>();
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
