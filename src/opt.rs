use structopt::clap::arg_enum;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "bw")]
pub enum Opt {
    /// Check a given problem and solution pair
    Check {
        problem_file: String,
        solution_file: String
    },
    Solve {
        problem_file: String,
        solver: Solver
    },
    AutoSolve {
        problem_dir: String,
        solution_dir: String,
        solver: Solver
    }
}

arg_enum! {
    #[derive(Debug)]
    pub enum Solver {
        Basic,
        Cat,
        SteppyCat,
        Border,
    }
}