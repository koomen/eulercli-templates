# Go solution templates

This directory contains a set of [Project Euler](https://projecteuler.net) solution templates written in [go](https://golang.org/). 

## Usage

### Generating a new julia solution program

You can use [eulercli](https://github.com/koomen/eulercli) to generate a new, ready-to-use solution program in go.  From your `projecteuler` directory, run

```sh
eulercli generate 1 --language go
```

If you're doing this for the first time, you should see a new `go` directory in your project directory:

```sh
projecteuler/
    eulercli-templates/
    go/
        go.mod
        README.md
        euler0001/
            euler0001.go
            euler0001_test.go
```

### Completing your solution program

Once you've used `eulercli generate` to render your templates for a new Project Euler problem, you're ready to solve the problem.  

Your job is to complete the definition of the `solve()` function in `euler0001.go` such that it returns the correct answer to the appropriate Project Euler problem.


### Running your solution program

From within `projecteuler/go`, you can run your solution program using `go run`:

```sh
cd go
go run euler0001/euler0001.go
```

As you're working on your solution, you can use `eulercli check` to check your answers:

```sh
go run euler0001/euler0001.go | eulercli check
```

See the [eulercli documentation](https://github.com/koomen/eulercli) for more details.

### Benchmarking your solution

To benchmark your solution, use `go test` with the `-bench` flag:

```sh
$ cd euler0001
$ go test -bench=.
goos: darwin
goarch: amd64
pkg: example.com/projecteuler/euler0001
cpu: VirtualApple @ 2.50GHz
BenchmarkSolve-8   	1000000000	         0.3153 ns/op
PASS
ok  	example.com/projecteuler/euler0001	0.893s
```