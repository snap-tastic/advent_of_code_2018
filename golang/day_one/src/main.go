package main

import (
	"fmt"
	"io/ioutil"
	"log"
	"bufio"
	"strings"
	"strconv"
)

func day1_part1(contents string) int64 {
	scanner := bufio.NewScanner(strings.NewReader(contents))
	var counter_sum int64
	for scanner.Scan() {
		fmt.Println(scanner.Text())
		number, err := strconv.ParseInt(scanner.Text(), 10, 64)
		if err != nil {
			log.Fatal("Failed to convert string to int")
		}
		counter_sum += number
	}

	return counter_sum
}

func day1_part2(contents string) int64 {
	scanner := bufio.NewScanner(strings.NewReader(contents))
	frequency := make(map[int]int64)
	var counter_sum int64 = 0
	var sum_list []int64
	index := 0
	
	for scanner.Scan() {
		fmt.Println(scanner.Text())
		number, err := strconv.ParseInt(scanner.Text(), 10, 64)
		if err != nil {
			log.Fatal("Failed to convert string to int")
		}
		frequency[index] = number
		index++
	}

	index = 0
	for {
		counter_sum += frequency[index]
		for _, value := range sum_list {
			if counter_sum == value {
				return counter_sum
			}
		}
		sum_list = append(sum_list, counter_sum)

		if index == len(frequency) - 1{
			index = 0 
		} else {
			index++;
		}
	}
}

func main() {
	content, err := ioutil.ReadFile("../input")
	if err != nil {
		log.Fatal(err)
	}

	contents := string(content[:])
	counter_sum := day1_part1(contents)
	fmt.Println("Total sum: ", counter_sum)
	
	dupe := day1_part2(contents)
	fmt.Println("First dupe: ", dupe)
}
