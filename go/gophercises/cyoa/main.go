package main

import (
	"flag"
	"fmt"
	"os"
)

func main() {
	filename := flag.String("filename", "gopher.json", "the Json file with the croa story")
	flag.Parse()

	f, err := os.Open(*filename)
	if err != nil {
		panic(err)
	}

	story, err := JsonStory(f)
	if err != nil {
		panic(err)
	}

	fmt.Printf("%+v%q\n", story, 1)
	fmt.Printf("%d\n", -1)

}
