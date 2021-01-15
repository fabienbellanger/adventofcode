package main

import (
	"log"
	"testing"

	"github.com/stretchr/testify/assert"
)

// func TestPart1(t *testing.T) {
// 	notes, _ := getData("test.txt")
// 	assert.Equal(t, 295, part1(notes), "they should be equal")

// 	notes, _ = getData("input.txt")
// 	assert.Equal(t, 2095, part1(notes), "they should be equal")
// }

func TestPart2(t *testing.T) {
	notes, err := getData("test_simple.txt")
	if err != nil {
		log.Fatal(err)
	}
	assert.Equal(t, 3417, part2(notes), "they should be equal")

	// notes, err = getData("test.txt")
	// if err != nil {
	// 	log.Fatal(err)
	// }
	// assert.Equal(t, 1068788, part2(notes), "they should be equal")

	// notes, err = getData("input.txt")
	// if err != nil {
	// 	log.Fatal(err)
	// }
	// assert.Equal(t, 1607, part2(notes), "they should be equal")
}
