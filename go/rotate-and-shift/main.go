package main

import (
	"bytes"
	"encoding/json"
	"fmt"
	"io/ioutil"
	"log"
	"os"
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

type TestResults struct {
	IsValid                  bool          `json:"is_valid"`
	InvalidVertices          []int         `json:"invalid_vertices"`
	InvalidEdgesStretched    []interface{} `json:"invalid_edges_stretched"`
	InvalidEdgesIntersecting []int         `json:"invalid_edges_intersecting"`
}

var rotation = 1
var xOffset = 0
var yOffset = 1
var iteration_max = 50
var interactive = false

func main() {

	if len(os.Args) < 2 {
		fmt.Println("Missing problem id! \nJust the number please!")
		os.Exit(1)
	}
	problem := os.Args[1]

	if _, err := os.Stat("solutions/" + problem + ".json"); os.IsNotExist(err) {
		// path/to/whatever does not exist
	} else {
		check_solution(problem)
	}

	fmt.Println("Loading problem: " + problem + ".json")
	problem_file, _ := ioutil.ReadFile("../../problems/" + problem + ".json")

	this_problem := Problem{}

	_ = json.Unmarshal([]byte(problem_file), &this_problem)

	this_solution := Solution{}

	for i := 0; i < len(this_problem.Figure.Vertices); i++ {
		this_solution.Vertices = append(this_solution.Vertices, [][]int{{this_problem.Figure.Vertices[i][0], this_problem.Figure.Vertices[i][1]}}...)
	}

	iteration := 0

	for true {

		for i := 0; i < len(this_problem.Figure.Vertices); i++ {
			this_solution.Vertices[i][0] = this_solution.Vertices[i][0] + xOffset
			this_solution.Vertices[i][1] = this_solution.Vertices[i][1] + yOffset
		}

		output, _ := json.Marshal(this_solution)
		fileSolution, err := os.Create("solutions/" + problem + ".json")
		if err != nil {
			log.Fatal(err)
		}

		_, err = fileSolution.WriteString(string(output))
		if err != nil {
			log.Fatal(err)
		}
		fileSolution.Close()

		fmt.Println("\n", string(output), "\n")

		if iteration >= iteration_max {
			fmt.Println("No solutions found in ", iteration, " trys for problem ", problem)
			os.Exit(0)
		}

		check_solution(problem)
		iteration++

	}

}

func wait_for_key() {
	fmt.Println("Press the Enter Key for next iteration.")
	fmt.Println("Or Ctrl-c to quit.")
	fmt.Scanln()
}

func check_solution(problem string) {
	cmd := exec.Command("../../target/debug/icfp_2021", "check", "../../problems/"+problem+".json", "solutions/"+problem+".json")
	var outb, errb bytes.Buffer
	cmd.Stdout = &outb
	cmd.Stderr = &errb
	err := cmd.Run()
	if err != nil {
		log.Fatal(err)
	}

	this_testResults := TestResults{}

	_ = json.Unmarshal([]byte(outb.Bytes()), &this_testResults)

	if this_testResults.IsValid {
		fmt.Println("SOLVED!")
		os.Exit(0)
	} else {
		fmt.Println(this_testResults)
	}

	if interactive {
		wait_for_key()
	}
}
