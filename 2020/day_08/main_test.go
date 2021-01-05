package main

import (
	"log"
	"testing"
)

func TestPart1(t *testing.T) {
	instructions, err := getData("test.txt")
	if err != nil {
		log.Fatal(err)
	}
	got := part1(instructions)
	excepted := 5
	if got != excepted {
		t.Errorf("part1: got= %d; expected=%d", got, excepted)
	}

	instructions, err = getData("input.txt")
	if err != nil {
		log.Fatal(err)
	}
	got = part1(instructions)
	excepted = 1487
	if got != excepted {
		t.Errorf("part1: got= %d; expected=%d", got, excepted)
	}
}

func TestPart2(t *testing.T) {
	instructions, err := getData("test.txt")
	if err != nil {
		log.Fatal(err)
	}
	got := part2(instructions)
	excepted := 8
	if got != excepted {
		t.Errorf("part2: got= %d; expected=%d", got, excepted)
	}

	instructions, err = getData("input.txt")
	if err != nil {
		log.Fatal(err)
	}
	got = part2(instructions)
	excepted = 1607
	if got != excepted {
		t.Errorf("part2: got= %d; expected=%d", got, excepted)
	}
}
