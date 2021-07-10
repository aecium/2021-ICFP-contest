package main

import (
	"encoding/json"
	"fmt"
	"io/ioutil"
)

type Problem struct {
	Hole    [][]int `json:"hole"`
	Epsilon int     `json:"epsilon"`
	Figure  struct {
		Edges    [][]int `json:"edges"`
		Vertices [][]int `json:"vertices"`
	} `json:"figure"`
}

type Epsilon struct {
	OriganalLength []int
	NewLength      []int
}

type Solution struct {
	Vertices [][]int `json:"vertices"`
}

var xOffset = 7
var yOffset = 5

func main() {

	problem_file, _ := ioutil.ReadFile("../../problems/2.json")

	this_problem := Problem{}
	this_epsilon := Epsilon{}

	_ = json.Unmarshal([]byte(problem_file), &this_problem)

	sliceEdges := this_problem.Figure.Edges
	fmt.Println(sliceEdges)

	for 0 < len(sliceEdges) {
		fmt.Printf("checking for matching edge %d,%d\n", sliceEdges[0][0], sliceEdges[0][1])
		foundmatch := false
	matchFound:
		for j := 1; j < len(sliceEdges)-1; j++ {
			fmt.Printf("Checking: %d,%d - %d,%d\n", sliceEdges[0][0], sliceEdges[0][1], sliceEdges[j][0], sliceEdges[j][1])
			if sliceEdges[j][0] == sliceEdges[0][0] || sliceEdges[j][0] == sliceEdges[0][1] {
				fmt.Printf("Found a match!: %d,%d - %d,%d ", sliceEdges[0][0], sliceEdges[0][1], sliceEdges[j][0], sliceEdges[j][1])
				fmt.Printf("at vertex: %d\n", sliceEdges[j][0])
				//sliceEdges = append(sliceEdges[:j], sliceEdges[j+1:]...)
				sliceEdges = append(sliceEdges[:0], sliceEdges[1:]...)
				fmt.Println(sliceEdges)
				foundmatch = true
				break matchFound
			} else if sliceEdges[j][1] == sliceEdges[0][0] || sliceEdges[j][1] == sliceEdges[0][1] {
				fmt.Printf("Found a match!: %d,%d - %d,%d ", sliceEdges[0][0], sliceEdges[0][1], sliceEdges[j][0], sliceEdges[j][1])
				fmt.Printf("at vertex: %d\n", sliceEdges[j][1])
				//fmt.println(mirror(sliceEdges[0], sliceEdges[j], sliceEdges[j][1]))
				//sliceEdges = append(sliceEdges[:j], sliceEdges[j+1:]...)
				sliceEdges = append(sliceEdges[:0], sliceEdges[1:]...)
				fmt.Println(sliceEdges)
				foundmatch = true
				break matchFound
			}
		}
		if !foundmatch {
			fmt.Println("no match found")
			sliceEdges = append(sliceEdges[:0], sliceEdges[1:]...)
		}

	}

	fmt.Println(sliceEdges)

	return

	this_solution := Solution{}

	fmt.Println(this_epsilon)

	output, _ := json.Marshal(this_solution)
	fmt.Println(string(output))

}

func mirror(edge1 []int, edge2 []int, vertex int) string {

	return ""
}
