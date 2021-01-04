package main

import (
	"bufio"
	"fmt"
	"io"
	"os"
)

func main() {
	program := scanProgram(os.Stdin)

	part1, _ := runProgram(program, -1)

	fmt.Println("Part 1:", part1)

	for i := 0; i < len(program); i++ {
		part2, success := runProgram(program, i)

		if success {
			fmt.Println("Part 2:", part2)
		}
	}
}

type instruction struct {
	string
	int
}

func scanProgram(r io.Reader) (program []instruction) {
	for scanner := bufio.NewScanner(r); scanner.Scan(); {
		program = append(program, parseInstruction(scanner.Text()))
	}
	return
}

func parseInstruction(s string) (i instruction) {
	fmt.Sscanf(s, "%s %d", &i.string, &i.int)
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

		op := program[i].string
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
			i += program[i].int
			continue
		case "acc":
			acc += program[i].int
		}
		i++
	}
	return acc, true
}
