package main

import (
	"bytes"
	"encoding/json"
	"flag"
	"fmt"
	"io/ioutil"
	"log"
	"os"
	"os/exec"
	"strconv"
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
var iteration_max int
var interactive = false
var verbose bool

func main() {

	var problem_id int

	flag.IntVar(&problem_id, "p", -1, "problem id to solve.")
	flag.BoolVar(&verbose, "v", false, "mora output!")
	flag.IntVar(&iteration_max, "i", 200, "set the number of iterations")

	flag.Parse()

	if problem_id == -1 {
		fmt.Println("Missing problem id! \nJust the number please!")
		os.Exit(1)
	}
	problem := strconv.Itoa(problem_id)

	if _, err := os.Stat("solutions/" + problem + ".json"); os.IsNotExist(err) {
		//nothing to do here :-)
	} else {
		check_solution(problem)
	}

	if verbose {
		fmt.Println("Loading problem: " + problem + ".json")
	}
	problem_file, _ := ioutil.ReadFile("../../problems/" + problem + ".json")

	this_problem := Problem{}

	_ = json.Unmarshal([]byte(problem_file), &this_problem)

	this_solution := Solution{}

	for i := 0; i < len(this_problem.Figure.Vertices); i++ {
		this_solution.Vertices = append(this_solution.Vertices, [][]int{{this_problem.Figure.Vertices[i][0], this_problem.Figure.Vertices[i][1]}}...)
	}

	iteration := 0

	// m    m   mm   mm   m mmmmmm        m       mmmm   mmmm  mmmmm
	// ##  ##   ##   #"m  # #             #      m"  "m m"  "m #   "#
	// # ## #  #  #  # #m # #mmmmm        #      #    # #    # #mmm#"
	// # "" #  #mm#  #  # # #             #      #    # #    # #
	// #    # #    # #   ## #mmmmm        #mmmmm  #mm#   #mm#  #
	for true {
		iteration++
		// Shift down 1
		for i := 0; i < len(this_problem.Figure.Vertices); i++ {
			this_solution.Vertices[i][1] = this_solution.Vertices[i][1] + 1
		}

		write_solotion_file(problem, this_solution)

		check_solution(problem)

		// Shift right
		for i := 0; i < len(this_problem.Figure.Vertices); i++ {
			this_solution.Vertices[i][0] = this_solution.Vertices[i][0] + 1
		}

		write_solotion_file(problem, this_solution)

		check_solution(problem)

		if iteration >= iteration_max {
			if verbose {
				fmt.Println("No solutions found in ", iteration, " trys for problem ", problem)
			}
			os.Exit(0)
		}
	}

}

func wait_for_key() {
	fmt.Println("Press the Enter Key for next iteration.")
	fmt.Println("Or Ctrl-c to quit.")
	fmt.Scanln()
}

func write_solotion_file(problem string, this_solution Solution) {
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

	fmt.Println(string(output))
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
		if verbose {
			fmt.Println("SOLVED!")
		}
		os.Exit(0)
	} else {
		if verbose {
			fmt.Println(this_testResults)
		}
	}

	if interactive {
		wait_for_key()
	}
}
