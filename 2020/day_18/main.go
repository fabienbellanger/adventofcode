package main

import (
	"io/ioutil"
	"log"
	"strings"
)

func main() {
	// fileContent, err := getFileContent("input.txt")
	// if err != nil {
	// 	log.Panicln(err)
	// }
	// data := getData(fileContent)
	data := getData([]byte("1 + (2 * 3) + (4 * (5 + 6))"))
	log.Printf("data=%v\n", data)

}

func part1() uint {
	return 0
}

func getFileContent(file string) ([]byte, error) {
	b, err := ioutil.ReadFile(file)
	if err != nil {
		return b, err
	}
	return b, nil
}

func getData(data []byte) []string {
	lines := strings.Split(string(data), "\n")

	for _, line := range lines {
		l := strings.ReplaceAll(line, " ", "")
		println(string(l[6]))
	}

	return lines
}
