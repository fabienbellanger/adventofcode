package main

import (
	"bytes"
	"fmt"
	"io/ioutil"
	"log"
	"sort"
	"strconv"
)

type notes struct {
	depart int
	bus    []int
}

func (n notes) String() string {
	return fmt.Sprintf("depart=%d, bus=%v\n", n.depart, n.bus)
}

func (n notes) geaterBus() int {
	max := 0
	for _, b := range n.bus {
		if b > max {
			max = b
		}
	}
	return max
}

func main() {
	data, err := getData("input.txt")
	if err != nil {
		log.Fatalln(err)
	}

	part1 := part1(data)
	fmt.Println("Part 1 result:", part1)

	part2 := part2(data)
	fmt.Println("Part 2 result:", part2)
}

func part1(data notes) int {
	greaterBus := data.geaterBus()
	found := false
	bus := 0

	var i int
	for i = data.depart; i < data.depart+greaterBus && !found; i++ {
		for _, b := range data.bus {
			if i%b == 0 {
				found = true
				bus = b
			}
		}
	}

	if !found {
		log.Panicf("Depart not found")
	}
	return (i - 1 - data.depart) * bus
}

func part2(data notes) int {
	return 0
}

func getData(file string) (notes, error) {
	var n notes
	b, err := ioutil.ReadFile(file)
	if err != nil {
		return n, err
	}

	data := bytes.TrimSpace(b)
	lines := bytes.Split(data, []byte("\n"))
	if len(lines) != 2 {
		return n, nil
	}

	n.depart, _ = strconv.Atoi(string(lines[0]))
	for _, b := range bytes.Split(lines[1], []byte(",")) {
		i, err := strconv.Atoi(string(b))
		if err != nil {
			continue
		}
		n.bus = append(n.bus, i)
	}

	// Tri des bus
	sort.Ints(n.bus)

	return n, nil
}
