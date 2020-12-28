package main

import (
	"bytes"
	"errors"
	"io/ioutil"
	"log"
	"strconv"
)

type password struct {
	low     int
	hight   int
	pattern []byte
	pwd     []byte
}

func main() {
	result1, err := part1()
	if err != nil {
		log.Fatalln(err)
	}
	log.Printf("Part1 result: %d\n", result1)

	result2, err := part2()
	if err != nil {
		log.Fatalln(err)
	}
	log.Printf("Part2 result: %d\n", result2)
}

func part1() (int, error) {
	passwords, err := getPasswords()
	if err != nil {
		return 0, err
	}

	valid := 0
	for _, p := range passwords {
		patternCount := 0
		for _, v := range p.pwd {
			if v == p.pattern[0] {
				patternCount++
			}
		}

		if patternCount >= p.low && patternCount <= p.hight {
			valid++
		}
	}

	return valid, nil
}

func part2() (int, error) {
	passwords, err := getPasswords()
	if err != nil {
		return 0, err
	}

	valid := 0
	for _, p := range passwords {
		if (p.pwd[p.low-1] == p.pattern[0]) != (p.pwd[p.hight-1] == p.pattern[0]) {
			valid++
		}
	}

	return valid, nil
}

func getPasswords() ([]password, error) {
	passwords := make([]password, 0)

	data, err := getData()
	if err != nil {
		return passwords, err
	}

	passwords, err = parseData(data)
	if err != nil {
		return passwords, err
	}

	return passwords, nil
}

func parseData(data [][]byte) ([]password, error) {
	// 1-3 a: abcde

	passwords := make([]password, 0)

	for _, line := range data {
		parts := bytes.Split(line, []byte(" "))
		if len(parts) != 3 {
			return passwords, errors.New("Bad data")
		}

		lh := bytes.Split(parts[0], []byte("-"))
		if len(lh) != 2 {
			return passwords, errors.New("Bad data")
		}
		low, err := strconv.Atoi(string(lh[0]))
		if err != nil {
			return passwords, errors.New("Bad data")
		}
		hight, err := strconv.Atoi(string(lh[1]))
		if err != nil {
			return passwords, errors.New("Bad data")
		}

		pattern := bytes.Trim(parts[1], ":")
		passwords = append(passwords, password{
			low:     low,
			hight:   hight,
			pattern: pattern,
			pwd:     parts[2],
		})
	}

	return passwords, nil
}

func getData() ([][]byte, error) {
	lines := make([][]byte, 0)

	b, err := ioutil.ReadFile("input.txt")
	if err != nil {
		return lines, err
	}

	data := bytes.TrimSpace(b)
	for _, line := range bytes.Split(data, []byte("\n")) {
		lines = append(lines, line)
	}
	return lines, nil
}
