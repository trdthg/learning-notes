package main

import (
	"encoding/json"
	"io"
)

type Story map[string]Chapter

func JsonStory(r io.Reader) (Story, error) {
	d := json.NewDecoder(r)
	var story Story
	if err := d.Decode(&story); err != nil {
		panic(err)
		return nil, err
	}
	return story, nil
}

type Chapter struct {
	Title      string `标题`
	Paragraphs []string
	Options    []Option
}

type Option struct {
	Text    string
	Chapter string
}
