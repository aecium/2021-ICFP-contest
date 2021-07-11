package main

import (
	"encoding/json"
	"fmt"
	"io/ioutil"
	"os/exec"
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

var rotation = 1
var xOffset = -1

var yOffset = 1

func main() {
	problem_file, _ := ioutil.ReadFile("../../problems/2.json")

	this_problem := Problem{}
	this_epsilon := Epsilon{}

	_ = json.Unmarshal([]byte(problem_file), &this_problem)

	this_solution := Solution{}

	for i := 0; i < len(this_problem.Figure.Vertices); i++ {
		this_solution.Vertices = append(this_solution.Vertices, [][]int{{this_problem.Figure.Vertices[i][0] +
			+xOffset, this_problem.Figure.Vertices[i][1] + yOffset}}...)
	}

	fmt.Println(this_epsilon)

	output, _ := json.Marshal(this_solution)
	fmt.Println(string(output))

	callRust := exec.Command("/mnt/e/workspace/2021-ICFP-contest/target/debug/icfp_2021")

	err := callRust.Run()

	if err != nil {
		fmt.Println(err)
	}
}
