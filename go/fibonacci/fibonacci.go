package main

import "fmt"

func main() {
	sumEvenFibonacci(4000000)
}

func sumEvenFibonacci(_max int) {
	x := genUntil(3)
	y := filterEvenNumber(x)
	fmt.Println(y)
}

func filterEvenNumber(nums []int) []int {
	var result []int
	for _, n := range nums {
		if n%2 == 0 {
			result = append(result, n)
		}
	}
	return result
}

func genUntil(_max int) []int {
	return []int{1, 2, 3}
}
