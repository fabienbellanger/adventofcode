package main

import (
	"bytes"
	"fmt"
	"io/ioutil"
	"log"
)

func main() {
	instructions, err := getData("input.txt")
	if err != nil {
		log.Fatalln(err)
	}

	part1 := part1(instructions)
	fmt.Println("Part 1 result:", part1)

	part2 := part2(instructions)
	fmt.Println("Part 2 result:", part2)
}

func part1(program []instruction) int {
	result, _ := runProgram(program, -1)

	return result
}

func part2(program []instruction) int {
	result := 0
	for i := 0; i < len(program); i++ {
		acc, success := runProgram(program, i)

		if success {
			result = acc
			break
		}
	}

	return result
}

type instruction struct {
	operation string
	value     int
}

func getData(file string) ([]instruction, error) {
	instructions := make([]instruction, 0)

	b, err := ioutil.ReadFile(file)
	if err != nil {
		return instructions, err
	}

	data := bytes.TrimSpace(b)
	for _, line := range bytes.Split(data, []byte("\n")) {
		instructions = append(instructions, parseInstruction(string(line)))
	}
	return instructions, nil
}

func parseInstruction(s string) (i instruction) {
	fmt.Sscanf(s, "%s %d", &i.operation, &i.value)
	return
}

func runProgram(program []instruction, flip int) (acc int, success bool) {
	i := 0
	visited := map[int]bool{}

	for i < len(program) {
		if visited[i] {
			return acc, false
		}
		visited[i] = true

		op := program[i].operation
		if i == flip {
			switch op {
			case "jmp":
				op = "nop"
			case "nop":
				op = "jmp"
			}
		}

		switch op {
		case "jmp":
			i += program[i].value
			continue
		case "acc":
			acc += program[i].value
		}
		i++
	}
	return acc, true
}
