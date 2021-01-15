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
	n, err := getData("test_simple.txt")
	if err != nil {
		log.Fatal(err)
	}

	assert.Equal(t, 3417, part2(n), "they should be equal")
	n = notes{
		depart: 0,
		bus: map[int]int{
			0: 67,
			1: 7,
			3: 59,
			4: 61,
		},
	}
	assert.Equal(t, 1261476, part2(n), "they should be equal")

	n = notes{
		depart: 0,
		bus: map[int]int{
			0: 1789,
			1: 37,
			2: 47,
			3: 1889,
		},
	}
	assert.Equal(t, 1202161486, part2(n), "they should be equal")

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
