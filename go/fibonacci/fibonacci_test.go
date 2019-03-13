package main

import (
	"reflect"
	"testing"
)

func TestGenUtil(t *testing.T) {
	var tests = []struct {
		input uint32
		want  []uint32
	}{
		{1, []uint32{1}},
		{2, []uint32{1, 2}},
		{3, []uint32{1, 2, 3}},
		{4, []uint32{1, 2, 3}},
		{5, []uint32{1, 2, 3, 5}},
	}

	for _, test := range tests {
		if got := genUntil(test.input); !reflect.DeepEqual(got, test.want) {
			t.Errorf("genUntil(%v) = %q", test.input, got)
		}
	}
}

func TestSumEvenFibonacci(t *testing.T) {
	var tests = []struct {
		input uint32
		want  uint64
	}{
		{1, 0},
		{2, 2},
		{3, 2},
		{5, 2},
		{8, 10},
		{8, 10},
		{4000000, 4613732},
	}

	for _, test := range tests {
		if got := sumEvenFibonacci(test.input); got != test.want {
			t.Errorf("sumEvenFibonacci(%v) = %q", test.input, got)
		}
	}
}
