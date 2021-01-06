package main

import (
	"log"
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestPart1(t *testing.T) {
	instructions, _ := getData("test.txt")
	assert.Equal(t, 5, part1(instructions), "they should be equal")

	instructions, _ = getData("input.txt")
	assert.Equal(t, 1487, part1(instructions), "they should be equal")
}

func TestPart2(t *testing.T) {
	instructions, err := getData("test.txt")
	if err != nil {
		log.Fatal(err)
	}
	assert.Equal(t, 8, part2(instructions), "they should be equal")

	instructions, err = getData("input.txt")
	if err != nil {
		log.Fatal(err)
	}
	assert.Equal(t, 1607, part2(instructions), "they should be equal")
}
