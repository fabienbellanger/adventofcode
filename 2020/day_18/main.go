package main

import (
	"bytes"
	"io/ioutil"
	"log"
)

func main() {
	err := getData("input.txt")
	log.Printf("err=%v\n", err)

}

func part1() uint {
	return 0
}

func getData(file string) error {
	b, err := ioutil.ReadFile(file)
	if err != nil {
		return err
	}

	data := bytes.TrimSpace(b)
	lines := bytes.Split(data, []byte("\n"))
	log.Println(lines)

	return nil
}
