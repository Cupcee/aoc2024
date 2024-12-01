use clap::Parser;
use std::collections::HashMap;
use std::fmt::Debug;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[arg(short, long)]
    pub input: String,

    #[arg(short, long)]
    pub problem: i32,
}

impl Args {
    pub fn argparse() -> Args {
        Args::parse()
    }
}

pub struct Counter<T> {
    counts: HashMap<T, usize>,
}

impl<T> Counter<T>
where
    T: std::hash::Hash + Eq + Clone,
{
    pub fn new() -> Self {
        Counter {
            counts: HashMap::new(),
        }
    }

    pub fn add(&mut self, item: T) {
        *self.counts.entry(item).or_insert(0) += 1;
    }

    pub fn remove(&mut self, item: &T) {
        if let Some(count) = self.counts.get_mut(item) {
            if *count > 1 {
                *count -= 1;
            } else {
                self.counts.remove(item);
            }
        }
    }

    pub fn get(&self, item: &T) -> usize {
        *self.counts.get(item).unwrap_or(&0)
    }
}

pub fn pretty_print_answer<T: Debug>(answer: T) {
    println!("=====================");
    println!("Answer: {:?}", answer);
    println!("=====================");
}
