package main

import (
	"encoding/csv"
	"flag"
	"fmt"
	"os"
	"strings"
	"time"
)

func main() {
	csvFilename := flag.String("csv", "problem.csv", "a csv file in the format of 'question,answer'")
	timeLimit := flag.Int("linit", 3, "the time limit for each question in seconds")
	flag.Parse()

	file, err := os.Open(*csvFilename)
	if err != nil {
		exit(fmt.Sprintf("Failed to open CSV file: %s\n", *csvFilename))
	}
	r := csv.NewReader(file)
	lines, err := r.ReadAll()
	if err != nil {
		exit(fmt.Sprintf("Failed to parse the provided CSV file. "))
	}
	problems := parseLines(lines)
	timer := time.NewTimer(time.Duration(*timeLimit) * time.Second)
	correct := 0
problemLoop:
	for i, p := range problems {
		fmt.Printf("Problem: #%d: %s = \n", i+1, p.q)
		answerChannel := make(chan string)
		go func() {
			var answer string
			fmt.Scanf("%s\n", &answer)
			answerChannel <- answer
		}()
		select {
		case <-timer.C:
			fmt.Printf("Your scored %d out of %d.\n", correct, len(problems))
			break problemLoop
		case answer := <-answerChannel:
			if answer == p.a {
				correct += 1
				fmt.Printf("Correct!\n")
			}
		}
	}
	fmt.Printf("Your scored %d out of %d.\n", correct, len(problems))
}

type problem struct {
	q string
	a string
}

func parseLines(lines [][]string) []problem {
	ret := make([]problem, len(lines))
	for i, line := range lines {
		ret[i] = problem{
			q: line[0],
			a: strings.TrimSpace(line[1]),
		}
	}
	return ret
}

func exit(msg string) {
	fmt.Println(msg)
	os.Exit(1)
}
