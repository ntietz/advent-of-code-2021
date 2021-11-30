use aoc::runner;

use std::env;

fn main() {
    let args: Vec<_> = env::args().collect();

    if args.len() < 2 {
        panic!("Error: must provide at least one day");
    }

    let days: Vec<u32> = args[1..].iter().map(|arg| arg.parse().expect("Must provide integers")).collect();
    days.iter().for_each(|&day| runner::run(day));
}
