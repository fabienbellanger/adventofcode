package main

import (
	"bytes"
	"fmt"
	"io/ioutil"
	"log"
	"strconv"
)

type notes struct {
	depart int
	bus    map[int]int
}

func newNotes() notes {
	return notes{
		depart: 0,
		bus:    make(map[int]int),
	}
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

func (n notes) firstBus() int {
	min := len(n.bus)
	for i := range n.bus {
		if i < min {
			min = i
		}
	}
	return min
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

func part2(data notes) (t int) {
	firstIndex := data.firstBus()
	inc := data.bus[firstIndex]

	for i, bus := range data.bus {
		if i != firstIndex {
			for (t+i)%bus != 0 {
				t += inc
			}
			inc *= bus
		}
	}

	return
}

func getData(file string) (notes, error) {
	n := newNotes()
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
	for index, b := range bytes.Split(lines[1], []byte(",")) {
		i, err := strconv.Atoi(string(b))
		if err != nil {
			continue
		}
		n.bus[index] = i
	}

	return n, nil
}
