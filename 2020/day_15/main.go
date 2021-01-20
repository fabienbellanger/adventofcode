package main

import (
	"fmt"
	"strconv"
	"strings"
)

func main() {
	input := "18,8,0,5,4,1,20"
	data := inputToArray(input)
	fmt.Printf("data=%v\n", data)

	part1 := part1()
	fmt.Println("Part 1 result:", part1)

	part2 := part2()
	fmt.Println("Part 2 result:", part2)
}

func part1() string {
	return ""
}

func part2() string {
	return ""
}

func inputToArray(input string) []int {
	values := strings.Split(input, ",")
	nums := make([]int, len(values), len(values))

	for i, v := range values {
		newV, _ := strconv.Atoi(v)
		nums[i] = newV
	}

	return nums
}
