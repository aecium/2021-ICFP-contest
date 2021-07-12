use std::{fs::File, io::{Read, Write}};

use problem::Problem;

use solution::Solution;
use structopt::StructOpt;

mod problem;
mod solution;
mod opt;
use opt::Opt;


mod solvers;
use solvers::{Basic,Border};

mod cats;
use cats::{ Cat, SteppyCat };

mod flect;
use flect::{ Flect };

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
                    let result = Basic::new().solve(&p);
                    match result {
                        Some(solution) => {
                            println!("Solution Found!\n {} ", serde_json::to_string(&solution).unwrap());
                        },
                        None => {
                            println!("Giving up...");
                        }
                    }
                },
                &opt::Solver::Flect => {
                    Flect::new().solve(&p, problem_file);
                },
                &opt::Solver::Cat => {
                    Cat::new().solve(&p, problem_file);
                },
                &opt::Solver::SteppyCat => {
                    SteppyCat::new().solve(&p, problem_file);
                }
                &opt::Solver::Border => {
                    let result = Border::new().solve(&p);
                    match result {
                        Some(solution) => {
                            println!("Solution Found!\n {} ", serde_json::to_string(&solution).unwrap());
                        },
                        None => {
                            println!("Giving up...");
                        }
                    }
                },
            }
        },
        Opt::AutoSolve {problem_dir, solution_dir, solver}=> {
            let mut problem_text = String::new();
            for i in 1..=132 {
                problem_text.truncate(0);
                File::open(format!("{}{}{}", problem_dir,i,".json")).unwrap()
                .read_to_string(&mut problem_text).unwrap();
                let p = serde_json::from_str(&problem_text).unwrap();
                match solver {
                    &opt::Solver::Basic => {
                        let solution_path = format!("{}{}{}",solution_dir, i, ".json");
                        if !std::path::Path::new(&solution_path).exists() {
                            let result = Basic::new().solve(&p);
                            match result {
                                Some(solution) => {
                                    println!("Solution Found! for problem {}", i);
                                    File::create(solution_path).unwrap().write_all(serde_json::to_string(&solution).unwrap().as_bytes()).unwrap();
                                },
                                None => {
                                    println!("Giving up on problem {} ...", i);
                                }
                            }
                        }
                    },
                    &opt::Solver::Flect => {
                        println!("Flect doesn't autosolve (yet).");
                        break;
                    },
                    &opt::Solver::Cat => {
                        println!("Cats can't autosolve.");
                        break;
                    },
                    &opt::Solver::SteppyCat => {
                        println!("Cats can't autosolve.");
                        break;
                    }
                    &opt::Solver::Border => {
                        let solution_path = format!("{}{}{}",solution_dir, i, ".json");
                        if !std::path::Path::new(&solution_path).exists() {
                            let result = Border::new().solve(&p);
                            match result {
                                Some(solution) => {
                                    println!("Solution Found! for problem {}", i);
                                    File::create(solution_path).unwrap().write_all(serde_json::to_string(&solution).unwrap().as_bytes()).unwrap();
                                },
                                None => {
                                    println!("Giving up on problem {} ...", i);
                                }
                            }
                        }
                    },
                }
            }
        }
    }
}