use std::{fs::File, io::Read, thread, time};

use crate::{problem::{Point, Problem}, solution::Solution};

fn get_solution_for_problem_file(problem_file: &String) -> Option<Solution> {
    let solution_file = problem_file.replace("problem", "solution");
    let mut solution_text = String::new();

    match File::open(solution_file) {
        Err(_) => None,
        Ok(mut f) => match f.read_to_string(&mut solution_text) {
            Err(_) => None,
            Ok(_) => match serde_json::from_str(&solution_text) {
                Err(_) => None,
                Ok(solution) => Some(solution)
            }
        },
    }
}

pub struct Cat;
impl Cat {
    pub fn new() -> Self {Self}
    pub fn solve(&self, problem: &Problem, problem_file: &String) -> Option<Solution> {
        let solution = match get_solution_for_problem_file(problem_file) {
            Some(s) => s,
            None => Solution { vertices: problem.figure.vertices.clone() }
        };

        let json = serde_json::to_string(&solution).expect("Couldn't serialize solution.");

        println!("{}", json);
        return Some(solution);
    }

}

pub struct SteppyCat;
impl SteppyCat {
    pub fn new() -> Self {Self}
    pub fn solve(&self, problem: &Problem, problem_file: &String) -> Option<Solution> {
        let solution = match get_solution_for_problem_file(problem_file) {
            Some(s) => s,
            None => Solution { vertices: problem.figure.vertices.clone() }
        };

        let figure = problem.figure.vertices.clone();

        let mut pose = figure.clone();

        let goal = solution.vertices.clone();

        let steps = 10;
        for i in 0..steps+1 {
            // Slow it down there, bud.  Make it look like we're working.
            thread::sleep(time::Duration::from_secs(1));
            let ff = (steps)-i;
            let gf = i;

            for p in 0..pose.len() {
                pose[p][0] = (figure[p][0]*ff + goal[p][0]*gf)/steps;
                pose[p][1] = (figure[p][1]*ff + goal[p][1]*gf)/steps;
            }

            let json = serde_json::to_string(&Solution { vertices: pose.clone()})
                .expect("Couldn't serialize solution.");
            println!("{}", json);
        }

        return Some(solution);
    }

}

