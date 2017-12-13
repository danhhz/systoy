// Copyright 2017 Daniel Harrison. All Rights Reserved.

package benches

import (
	"fmt"
	"math/rand"
	"testing"
)

//go:noinline
func add(a, b int) int {
	return a + b
}

type adder interface {
	add(a, b int) int
}

type adderImpl struct{}

//go:noinline
func (adderImpl) add(a, b int) int {
	return a + b
}

// BenchmarkFuncCall compares various ways of making virtual function calls
// (with non-virtual baselines).
func BenchmarkFuncCall(b *testing.B) {

	// Overhead with no function call.
	b.Run("None", func(b *testing.B) {
		var total int
		b.ResetTimer()
		for i := 0; i < b.N; i++ {
			total += i
		}
		b.StopTimer()
		if total == rand.Int() {
			fmt.Println(total)
		}
	})

	// Non-inlined non-virtual function call.
	b.Run("Bare", func(b *testing.B) {
		var total int
		b.ResetTimer()
		for i := 0; i < b.N; i++ {
			total = add(i, total)
		}
		b.StopTimer()
		if total == rand.Int() {
			fmt.Println(total)
		}
	})

	// Non-inlined function call.
	b.Run("Virtual", func(b *testing.B) {
		func(a adder) {
			var total int
			b.ResetTimer()
			for i := 0; i < b.N; i++ {
				total = a.add(i, total)
			}
			b.StopTimer()
			if total == rand.Int() {
				fmt.Println(total)
			}
		}(adderImpl(struct{}{}))
	})

	// Try to pay the cost of the virtual function lookup only once.
	//
	// TODO(dan): Why isn't this faster than "Virtual"?
	b.Run("Pointer", func(b *testing.B) {
		func(add func(int, int) int) {
			var total int
			b.ResetTimer()
			for i := 0; i < b.N; i++ {
				total = add(i, total)
			}
			b.StopTimer()
			if total == rand.Int() {
				fmt.Println(total)
			}
		}(adderImpl(struct{}{}).add)
	})
}
