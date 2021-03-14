/*
Problem {{.ProblemNum}}
https://projecteuler.net/problem={{.ProblemNum}}
==========

{{.ProblemText}}


Solution
========

[Explain your solution here]

*/

package main

import (
	"fmt"
)

const PROBLEM_NUM = {{.ProblemNum}}

func solve(verbose bool) int {
	return 0
}

func main() {
	fmt.Printf("Solving problem %d\n", PROBLEM_NUM)
	answer := solve(true)
	fmt.Printf("Obtained solution %d\n", answer)
}
