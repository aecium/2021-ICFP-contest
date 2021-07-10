package main

import (
	"encoding/json"
	"fmt"
	"io/ioutil"
	"math"
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

var shrinkage float32 = 1
var xOffset = 7
var yOffset = 5

func main() {
	fmt.Printf("Shrinking by: %f\n", shrinkage)
	problem_file, _ := ioutil.ReadFile("../../problems/2.json")

	this_problem := Problem{}
	this_epsilon := Epsilon{}

	_ = json.Unmarshal([]byte(problem_file), &this_problem)

	maxRatio := float64(this_problem.Epsilon) / 1000000

	this_solution := Solution{}

	for i := 0; i < len(this_problem.Figure.Vertices); i++ {
		this_solution.Vertices = append(this_solution.Vertices, [][]int{{int(float32(this_problem.Figure.Vertices[i][0])-shrinkage) +
			+xOffset, int(float32(this_problem.Figure.Vertices[i][1])-shrinkage) + yOffset}}...)
	}

	for i := 0; i < len(this_problem.Figure.Edges); i++ {
		oldX1 := this_problem.Figure.Vertices[this_problem.Figure.Edges[i][0]][0]
		oldX2 := this_problem.Figure.Vertices[this_problem.Figure.Edges[i][1]][0]
		oldY1 := this_problem.Figure.Vertices[this_problem.Figure.Edges[i][0]][1]
		oldY2 := this_problem.Figure.Vertices[this_problem.Figure.Edges[i][1]][1]
		xProduct := (oldX1 - oldX2) * (oldX1 - oldX2)
		yProduct := (oldY1 - oldY2) * (oldY1 - oldY2)
		oldDistance := xProduct + yProduct
		this_epsilon.OriganalLength = append(this_epsilon.OriganalLength, []int{oldDistance}...)

		newX1 := this_solution.Vertices[this_problem.Figure.Edges[i][0]][0]
		newX2 := this_solution.Vertices[this_problem.Figure.Edges[i][1]][0]
		newY1 := this_solution.Vertices[this_problem.Figure.Edges[i][0]][1]
		newY2 := this_solution.Vertices[this_problem.Figure.Edges[i][1]][1]
		xProduct = (newX1 - newX2) * (newX1 - newX2)
		yProduct = (newY1 - newY2) * (newY1 - newY2)
		newDistance := xProduct + yProduct
		this_epsilon.NewLength = append(this_epsilon.NewLength, []int{newDistance}...)

		if math.Abs(float64(newDistance)/float64(oldDistance)-1) > maxRatio {
			fmt.Println("too much strech or shrink")
		}

	}

	fmt.Println(this_epsilon)

	output, _ := json.Marshal(this_solution)
	fmt.Println(string(output))

}
