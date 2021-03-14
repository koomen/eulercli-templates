# Julia solution templates

This directory contains a set of [Project Euler](https://projecteuler.net) solution templates written in [julia](https://julialang.org/). 

## Usage

### Generating a new julia solution program

You can use [eulercli](https://github.com/koomen/eulercli) to generate a new, ready-to-use solution program in julia.  From your `projecteuler` directory, run

```sh
eulercli generate 1 --language julia
```

If you're doing this for the first time, you should see a new `julia` directory in your project directory:

```sh
projecteuler/
    eulercli-templates/
    julia/
        initenv.jl
        README.md
        euler0001/
            solution.jl
```

### Completing your solution program

Once you've used `eulercli generate` to render your Julia templates for a new Project Euler problem, you're ready to solve the problem.  

Your job is to complete the definition of the `solve()` function in `solution.jl` such that it returns the correct answer to the appropriate Project Euler problem.


### Running your solution program

First, initialize your Julia environment:

```sh
cd julia
julia initenv.jl
```

Julia will create a new environment in `projecteuler/julia/projecteulerenv/`. **Note:** make sure to run `initenv.jl` in `projecteuler/julia`, *not* `projecteuler/eulercli-templates/julia`!

When this is done, you're ready to run your solution program:

```sh
cd euler0001
julia solution.jl
```

As you're working on your solution, you can use `eulercli check` to check your answers:

```sh
julia solution.jl | eulercli check
```

See the [eulercli documentation](https://github.com/koomen/eulercli) for more details.

### Benchmarking your solution

If it is run with the `-b` or `--benchmark` flag, `solution.jl` will use Julia's [BenchmarkTools](https://github.com/JuliaCI/BenchmarkTools.jl) package to measure the performance of your `solve()` function.

```sh
$ julia solution.jl -b
 Activating environment at `~/git/projecteuler/julia/projecteulerenv/Project.toml`
Benchmarking solution...
BenchmarkTools.Trial:
  memory estimate:  7.94 KiB
  allocs estimate:  1
  --------------
  minimum time:     17.675 ms (0.00% GC)
  median time:      18.695 ms (0.00% GC)
  mean time:        19.194 ms (0.00% GC)
  maximum time:     24.503 ms (0.00% GC)
  --------------
  samples:          261
  evals/sample:     1
```
