// Package urlshort provides ...
package handler

import (
	"encoding/json"
	"fmt"
	"net/http"

	"gopkg.in/yaml.v2"
)

func MapHandler(pathToUrls map[string]string, fallback http.Handler) http.HandlerFunc {
	return func(w http.ResponseWriter, r *http.Request) {
		path := r.URL.Path
		if dest, ok := pathToUrls[path]; ok {
			http.Redirect(w, r, dest, http.StatusFound)
			return
		}
		fallback.ServeHTTP(w, r)
	}
}

func YAMLHandler(yamlBytes []byte, fallback http.Handler) (http.HandlerFunc, error) {
	pathUrls, err := parseYaml(yamlBytes)
	if err != nil {
		return nil, err
	}
	pathToUrls := buildMap(pathUrls)
	return MapHandler(pathToUrls, fallback), nil
}

func JSONHandler(jsonBytes []byte, fallback http.Handler) (http.HandlerFunc, error) {
	pathUrls, err := parseJson(jsonBytes)
	if err != nil {
		return nil, err
	}
	pathToUrls := buildMap(pathUrls)
	return MapHandler(pathToUrls, fallback), nil
}

func buildMap(pathUrls []pathUrl) map[string]string {
	pathToUrls := make(map[string]string)
	for _, pu := range pathUrls {
		pathToUrls[pu.Path] = pu.URL
	}
	return pathToUrls
}

func parseYaml(yamlBytes []byte) ([]pathUrl, error) {
	var pathUrls []pathUrl
	err := yaml.Unmarshal(yamlBytes, &pathUrls)
	fmt.Println(2343, err)
	if err != nil {
		return nil, err
	}
	return pathUrls, nil
}

func parseJson(jsonBytes []byte) ([]pathUrl, error) {
	var pathUrls []pathUrl
	err := json.Unmarshal(jsonBytes, &pathUrls)
	fmt.Println(2343, err)
	if err != nil {
		return nil, err
	}
	return pathUrls, nil
}

type pathUrl struct {
	Path string `yaml: "path"`
	URL  string `yaml: "url"`
}
