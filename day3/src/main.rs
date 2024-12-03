use regex::Regex;
use shared::*;
use std::fs;

fn filter_string(input: &str, re_enable: &Regex, skip: &mut bool) -> String {
    let mut result = String::new();
    let mut last_end = 0;

    let matches: Vec<_> = re_enable.find_iter(input).collect();
    if matches.is_empty() {
        return String::from(input);
    }

    for mat in matches {
        let matched_text = mat.as_str();
        let start = mat.start();
        let end = mat.end();

        if *skip {
            if matched_text == "do()" {
                *skip = false;
                last_end = end;
            } else {
                last_end = end;
            }
        } else {
            result.push_str(&input[last_end..start]);

            if matched_text == "don't()" {
                *skip = true;
                last_end = end;
            } else {
                last_end = end;
            }
        }
    }

    if !*skip && last_end < input.len() {
        result.push_str(&input[last_end..]);
    }

    result
}

fn parse_line1(line: &str, re_line: &Regex, re_mul: &Regex) -> i32 {
    re_line.captures_iter(line).fold(0, |acc, capture| {
        let (full, _): (&str, [&str; 0]) = capture.extract();
        if let Some(caps) = re_mul.captures(full) {
            let left = caps
                .get(1)
                .map(|c| c.as_str().parse::<i32>().unwrap())
                .unwrap();
            let right = caps
                .get(2)
                .map(|c| c.as_str().parse::<i32>().unwrap())
                .unwrap();
            return left * right + acc;
        }
        acc
    })
}

fn problem1(input: String) {
    let re_line = Regex::new(r"mul\(-?\d+,-?\d+\)").unwrap();
    let re_mul = Regex::new(r"mul\((-?\d+),(-?\d+)\)").unwrap();
    let sum = input
        .lines()
        .fold(0, |acc, line| acc + parse_line1(line, &re_line, &re_mul));
    pretty_print_answer(sum);
}

fn problem2(input: String) {
    let re_line = Regex::new(r"mul\(-?\d+,-?\d+\)").unwrap();
    let re_mul = Regex::new(r"mul\((-?\d+),(-?\d+)\)").unwrap();
    let re_enable = Regex::new(r"don't\(\)|do\(\)").unwrap();
    let mut skip = false; // Initialize skip for the entire input to keep its state over lines

    let filtered_input = filter_string(&input, &re_enable, &mut skip);
    let sum = re_line
        .captures_iter(&filtered_input)
        .fold(0, |acc, capture| {
            let (full, _): (&str, [&str; 0]) = capture.extract();
            if let Some(caps) = re_mul.captures(full) {
                let left = caps.get(1).unwrap().as_str().parse::<i32>().unwrap();
                let right = caps.get(2).unwrap().as_str().parse::<i32>().unwrap();
                return left * right + acc;
            }
            acc
        });
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
