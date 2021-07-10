use crate::{problem::Problem, solution::Solution};

pub struct Basic;
impl Basic {
    pub fn new() -> Self {Self}
    pub fn solve(&self, problem: &Problem) -> Option<Solution> {
        let mut solution = Solution {
            vertices: problem.figure.vertices.clone()
        };
        let MAX_ITERATIONS: usize = 1_000_000_000;
        for i in 0..MAX_ITERATIONS {
            let result = solution.check(&problem);
            if result.is_valid() {
                return Some(solution);
            } else {
                if result.invalid_edges_intersecting.len() > 0 {
                    //fix an edge
                    continue;
                } else if result.invalid_vertices.len() > 0 {
                    //fix a vertex
                    continue;
                } else if result.invalid_edges_stretched.len() > 0 {
                    //fix a stretching problem
                    continue;
                }
            }
        }
        return None;
    }
}