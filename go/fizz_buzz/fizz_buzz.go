package main

import (
	"fmt"
	"strconv"
)

func main() {
	putsFizzBuzzUntil(32)
}

func putsFizzBuzzUntil(max int) {
	for i := 1; i <= max; i++ {
		fmt.Println(fizzBuzz(i))
	}
}

func fizzBuzz(i int) string {
	if i%15 == 0 {
		return "FizzBuzz"
	} else if i%3 == 0 {
		return "Fizz"
	} else if i%5 == 0 {
		return "Buzz"
	}
	return strconv.Itoa(i)
}
