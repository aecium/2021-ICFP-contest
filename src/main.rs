use std::{fs::File, io::Read};

use problem::Problem;

use solution::Solution;
use structopt::StructOpt;

mod problem;
mod solution;
mod opt;
use opt::Opt;

mod solvers;
use solvers::Basic;

mod cats;
use cats::{ Cat, SteppyCat };

fn main() {
    let matches = Opt::from_args();
    match &matches {
        Opt::Check { problem_file, solution_file } => {
            let mut problem_text = String::new();
            File::open(problem_file).unwrap().read_to_string(&mut problem_text).unwrap();
            let mut solution_text = String::new();
            File::open(solution_file).unwrap().read_to_string(&mut solution_text).unwrap();

            let p : Problem = serde_json::from_str(&problem_text).unwrap();
            let s : Solution = serde_json::from_str(&solution_text).unwrap();
            let result = s.check(&p);
            println!("{}", serde_json::to_string(&result).unwrap());
        },
        Opt::Solve {problem_file, solver} => {
            let mut problem_text = String::new();
            File::open(problem_file).unwrap().read_to_string(&mut problem_text).unwrap();
            let p : Problem = serde_json::from_str(&problem_text).unwrap();
            
            match solver {
                &opt::Solver::Basic => {
                    Basic::new().solve(&p);
                },
                &opt::Solver::Cat => {
                    Cat::new().solve(&p, problem_file);
                }
                &opt::Solver::SteppyCat => {
                    SteppyCat::new().solve(&p, problem_file);
                }
            }
        }
    }
}