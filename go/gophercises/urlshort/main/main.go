// Package main provides ...
package main

import (
	"fmt"
	"net/http"
	"shorturl/handler"
)

func main() {
	mux := defaultMux()

	pathToUrls := map[string]string{
		"/google":  "https://www.google.com",
		"/youtube": "https://www.youtube.com",
	}
	mapHandler := handler.MapHandler(pathToUrls, mux)

	// yaml := `
	// - path: /googlem
	//   url: https://www.google.com
	// - path: /youtube
	//   url: https://www.youtube.com
	// `
	json := `
	[
		{
			"path": "/google",
			"url": "https://www.google.com"
		},
		{
			"path": "/youtube",
			"url": "https://www.youtube.com"
		}
	]
	`
	jsonHandler, err := handler.JSONHandler([]byte(json), mapHandler)
	// yamlHandler, err := handler.YAMLHandler([]byte(yaml), mapHandler)
	if err != nil {
		panic(err)
	}
	// http.ListenAndServe(":8080", yamlHandler)
	http.ListenAndServe(":8080", jsonHandler)
	fmt.Println("Server started on :8080")
}

func defaultMux() *http.ServeMux {
	mux := http.NewServeMux()
	mux.HandleFunc("/", hello)
	return mux
}

func hello(w http.ResponseWriter, r *http.Request) {
	fmt.Println(w, "Hello Go Web Server!")
}
