package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestPart1(t *testing.T) {
	assert.Equal(t, 436, calculate(inputToArray("0,3,6"), 2020), "they should be equal")
	assert.Equal(t, 1, calculate(inputToArray("1,3,2"), 2020), "they should be equal")
	assert.Equal(t, 10, calculate(inputToArray("2,1,3"), 2020), "they should be equal")
	assert.Equal(t, 27, calculate(inputToArray("1,2,3"), 2020), "they should be equal")
	assert.Equal(t, 78, calculate(inputToArray("2,3,1"), 2020), "they should be equal")
	assert.Equal(t, 438, calculate(inputToArray("3,2,1"), 2020), "they should be equal")
	assert.Equal(t, 1836, calculate(inputToArray("3,1,2"), 2020), "they should be equal")
	assert.Equal(t, 253, calculate(inputToArray("18,8,0,5,4,1,20"), 2020), "they should be equal")
}

func TestPart2(t *testing.T) {
	assert.Equal(t, 175594, calculate(inputToArray("0,3,6"), 30000000), "they should be equal")
	assert.Equal(t, 13710, calculate(inputToArray("18,8,0,5,4,1,20"), 30000000), "they should be equal")
}
