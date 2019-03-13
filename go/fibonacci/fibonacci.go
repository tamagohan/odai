package main

import "fmt"

func main() {
	fmt.Println(sumEvenFibonacci(4000000))
}

func sumEvenFibonacci(max uint32) uint64 {
	var sum uint64
	fiboSeq := genUntil(max)
	for _, n := range fiboSeq {
		if n%2 == 0 {
			sum = sum + uint64(n)
		}
	}
	return sum
}

func genUntil(max uint32) []uint32 {
	if max < 2 {
		return []uint32{1}
	} else if max == 2 {
		return []uint32{1, 2}
	}
	return genUntilImp(max, []uint32{1, 2}, 1, 2)
}

func genUntilImp(max uint32, nums []uint32, n1 uint32, n2 uint32) []uint32 {
	var n3 = n1 + n2
	if n3 <= max {
		nums = append(nums, n3)
		return genUntilImp(max, nums, n2, n3)
	}
	return nums
}
