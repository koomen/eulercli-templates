use log::debug;
use std::{cmp::max, env, time::Instant};
/**
Project Euler - Problem {{.ProblemNum}}
https://projecteuler.net/problem={{.ProblemNum}}
==========

{{.ProblemText}}

Solution
========

[Explain your solution here]

*/

const BENCHMARK_ITERATIONS: u32 = 1;

/// Solve problem {{.ProblemNum}}
pub fn solve() -> usize {
    0
}

fn main() {
    println!("Solving problem {{.ProblemNum}}");

    // check `env::args()` for a verbose flag
    let log_level = match env::args().find(|s| s == "-v" || s == "--verbose") {
        Some(_) => log::LevelFilter::Debug,
        None => log::LevelFilter::Error,
    };

    // Initialize a logger with default level set according to the verbose flag
    env_logger::builder().filter_level(log_level).init();

    // This will only display in verbose mode
    debug!("Verbose output enabled");

    // Run `solve` `BENCHMARK_ITERATIONS` times, and compute the duration per iteration
    let start = Instant::now();
    let mut solution = 0;
    for _ in 0..BENCHMARK_ITERATIONS {
        solution = solve();
    }
    let duration = start.elapsed() / max(BENCHMARK_ITERATIONS, 1);

    println!("Computed answer {solution} in {duration:?}");
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn solve_test() {
        assert_eq!(solve(), 0);
    }
}
