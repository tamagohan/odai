package main

import "testing"

func TestFizzBuzz(t *testing.T) {
	var tests = []struct {
		input int
		want  string
	}{
		{1, "1"},
		{3, "Fizz"},
		{5, "Buzz"},
		{15, "FizzBuzz"},
	}

	for _, test := range tests {
		if got := fizzBuzz(test.input); got != test.want {
			t.Errorf("fizzBuzz(%v) = %q", test.input, got)
		}
	}
}
