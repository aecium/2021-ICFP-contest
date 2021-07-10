use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "bw")]
pub enum Opt {
    /// Check a given problem and solution pair
    Check {
        problem_file: String,
        solution_file: String
    },
}