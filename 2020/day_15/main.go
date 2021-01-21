package main

import (
	"fmt"
	"strconv"
	"strings"
)

func main() {
	data := inputToArray("18,8,0,5,4,1,20")

	fmt.Println("Part 1 result:", calculate(data, 2020))
	fmt.Println("Part 2 result:", calculate(data, 30000000))
}

func calculate(data []int, year int) int {
	used := make(map[int]int)
	for i, v := range data {
		if i < len(data)-1 {
			used[v] = i
		}
	}
	value := data[len(data)-1]
	for i := len(data) - 1; i < year-1; i++ {
		lastIndex, ok := used[value]
		newValue := 0
		if ok {
			newValue = i - lastIndex
		}
		used[value] = i
		value = newValue
	}

	return value
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
