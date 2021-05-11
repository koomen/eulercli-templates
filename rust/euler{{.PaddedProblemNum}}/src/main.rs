/**
Project Euler - Problem {{.ProblemNum}}
https://projecteuler.net/problem={{.ProblemNum}}
==========

{{.ProblemText}}

Solution
========

[Explain your solution here]

*/

use std::env;
use std::time::Instant;


/// Solve problem {{.ProblemNum}}
pub fn solve(_verbose: bool) -> usize {
    0
}

fn main() {
    println!("Solving problem {{.ProblemNum}}");

    let args: Vec<String> = env::args().collect();
    let verbose = args.contains(&String::from("-v")) || args.contains(&String::from("--verbose"));

    if verbose {
        println!("Verbose logging enabled");
    } else {
        println!("Use -v or --verbose to enable verbose logging");
    }

    let start = Instant::now();
    let solution = solve(verbose);
    let duration = start.elapsed();

    println!("Computed answer {} in {:?}", solution, duration);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn solve_test() {
        assert_eq!(solve(false), 0);
    }
}