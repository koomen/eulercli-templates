package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestSolve(t *testing.T) {
	assert.Equal(t, 0, solve(false))
}

func BenchmarkSolve(b *testing.B) {
	for n := 0; n < b.N; n++ {
		solve(false)
	}
}
