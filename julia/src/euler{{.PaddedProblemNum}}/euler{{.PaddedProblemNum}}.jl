"""
Problem {{.ProblemNum}}
https://projecteuler.net/problem={{.ProblemNum}}
==========

{{.ProblemText}}


Solution
========

[Explain your solution here]

"""


# Print usage instructions
println("""
$(PROGRAM_FILE) - Solve Project Euler problem {{.ProblemNum}}
Usage: $(PROGRAM_FILE) [--profile|-p] [--benchmark|p]
Options:
    --benchmark,-b      Benchmark your solution
    --profile,-p        Profile your solution
""")


# Activate the projecteuler package and add/import useful packages
import Pkg
Pkg.activate("../../projecteulerenv")

using BenchmarkTools
using Profile
using Test

# Solution code

"""
`solve()`

Solve the problem
"""
function solve(verbose::Bool=false)
    if verbose
        print("Computing solution...\n")
    end
    return 0
end

# Boilerplate timing code

if "--benchmark" in ARGS || "-b" in ARGS
    println("Benchmarking solution...")
    display(@benchmark solve())
    println()
elseif "--profile" in ARGS || "-p" in ARGS
    println("Profiling solution...")
    @profile solve()
    Profile.print(combine=true, sortedby=:count, mincount=5, maxdepth=8)
else
    println("Solving project euler problem {{.ProblemNum}}...")
    solution = solve(true)
    println("Obtained solution $(solution)")
end