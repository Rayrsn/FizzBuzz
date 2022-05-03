package main

import (
	"fmt"
	"strconv"
)

func main() {
	var numcount string
	fmt.Print("How many numbers to display: ")
	fmt.Scanf("%s", &numcount)
	intnumcount, err := strconv.Atoi(numcount)
	if err != nil {
		fmt.Println("Error: ", err)
		return
	}
	for i := 1; i <= intnumcount; i++ {
		if i%3 == 0 && i%5 == 0 {
			fmt.Println("FizzBuzz")
		} else if i%3 == 0 {
			fmt.Println("Fizz")
		} else if i%5 == 0 {
			fmt.Println("Buzz")
		} else {
			fmt.Println(i)
		}
	}
}
