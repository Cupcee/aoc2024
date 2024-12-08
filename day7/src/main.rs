use colored::*;
use indicatif::ParallelProgressIterator;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use shared::*;
use std::fs;
use std::sync::atomic::{AtomicI64, Ordering};
use std::sync::{Arc, Mutex};

#[derive(Debug)]
struct Equation {
    test_value: i64,
    numbers: Vec<i64>,
}

fn parse_equation(line: &str) -> Option<Equation> {
    // Split the line by colon
    let parts: Vec<&str> = line.split(':').collect();
    if parts.len() != 2 {
        return None;
    }

    // Parse the test value
    let test_value = parts[0].trim().parse::<i64>().ok()?;

    // Parse the numbers
    let numbers: Vec<i64> = parts[1]
        .trim()
        .split_whitespace()
        .filter_map(|num_str| num_str.trim().parse::<i64>().ok())
        .collect();

    Some(Equation {
        test_value,
        numbers,
    })
}

fn generate_operator_permutations(n: usize, operators: &[String]) -> Vec<Vec<String>> {
    if n < 2 {
        return vec![];
    }

    let k = operators.len();
    let total_combinations = k.pow((n - 1) as u32);
    let mut permutations = Vec::with_capacity(total_combinations);

    for i in 0..total_combinations {
        let mut ops = Vec::with_capacity(n - 1);
        let mut index = i;
        for _ in 0..(n - 1) {
            ops.push(operators[index % k].clone());
            index /= k;
        }
        permutations.push(ops);
    }

    permutations
}

fn evaluate_expression(numbers: &Vec<i64>, operators: &Vec<String>) -> i64 {
    let mut total = numbers[0]; // first number in the sequence is used as the initial accumulator value
    for i in 0..operators.len() {
        let num = numbers[i + 1];
        let concatenated = (total.to_string() + &num.to_string())
            .parse::<i64>()
            .unwrap();
        match operators[i].as_str() {
            "+" => total += num,
            "*" => total *= num,
            "||" => total = concatenated,
            _ => panic!("Unsupported operator"),
        }
    }
    total
}

fn visualize_expression(numbers: &Vec<i64>, operators: &Vec<String>, result: i64) -> String {
    let mut expression = String::new();
    expression.push_str(&numbers[0].to_string());

    for (op, num) in operators.iter().zip(numbers.iter().skip(1)) {
        let op_colored = match op.as_str() {
            "+" => "+".green(),
            "*" => "*".red(),
            "||" => "||".yellow(),
            _ => op.to_string().normal(),
        };
        expression.push_str(&format!(" {} {}", op_colored, num));
    }

    expression.push_str(&format!(" = {}", result));
    expression
}

fn process_equations(equations: &Vec<Equation>, operators: &[String]) -> (i64, Vec<String>) {
    let valid_test_values_sum = AtomicI64::new(0);
    let valid_expressions = Arc::new(Mutex::new(Vec::new()));

    equations.par_iter().progress().for_each(|eq| {
        let num_count = eq.numbers.len();

        // Handle cases with only one number
        if num_count == 1 {
            if eq.numbers[0] == eq.test_value {
                // valid_test_values_sum += eq.test_value;
                valid_test_values_sum.fetch_add(eq.test_value, Ordering::SeqCst);
                valid_expressions
                    .lock()
                    .unwrap()
                    .push(format!("{} = {}", eq.numbers[0], eq.test_value));
            }
        } else {
            let operator_permutations = generate_operator_permutations(num_count, operators);

            let mut is_valid = false;

            for ops in operator_permutations {
                let result = evaluate_expression(&eq.numbers, &ops);
                if result == eq.test_value {
                    is_valid = true;
                    let expr = visualize_expression(&eq.numbers, &ops, result);
                    valid_expressions
                        .lock()
                        .unwrap()
                        .push(format!("{} = {}", expr, eq.test_value));
                }
            }

            if is_valid {
                // valid_test_values_sum += eq.test_value;
                valid_test_values_sum.fetch_add(eq.test_value, Ordering::SeqCst);
            }
        }
    });
    let valid_expressions = valid_expressions.lock().unwrap().to_vec();
    (
        valid_test_values_sum.load(Ordering::SeqCst),
        valid_expressions,
    )
}

fn problem1(input: String) {
    let operators = ["*".to_string(), "+".to_string()];
    // Parse all equations
    let mut equations = Vec::new();
    for line in input.lines() {
        if let Some(eq) = parse_equation(line) {
            equations.push(eq);
        } else {
            println!("Failed to parse line: {}", line);
        }
    }

    // Process equations
    let (sum, expressions) = process_equations(&equations, &operators);

    // Print valid expressions
    println!("Valid Expressions:");
    for expr in expressions {
        println!("- {}", expr);
    }

    // Print the sum of valid test values
    println!("\nTotal Sum of Valid Test Values: {}", sum);
    pretty_print_answer(sum);
}

fn problem2(input: String) {
    let operators = ["*".to_string(), "+".to_string(), "||".to_string()];
    // Parse all equations
    let mut equations = Vec::new();
    for line in input.lines() {
        if let Some(eq) = parse_equation(line) {
            equations.push(eq);
        } else {
            println!("Failed to parse line: {}", line);
        }
    }

    // Process equations
    let (sum, expressions) = process_equations(&equations, &operators);

    // Print valid expressions
    println!("Valid Expressions:");
    for expr in expressions {
        println!("- {}", expr);
    }

    // Print the sum of valid test values
    println!("\nTotal Sum of Valid Test Values: {}", sum);
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
