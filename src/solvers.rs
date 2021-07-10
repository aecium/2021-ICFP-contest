use core::num;
use std::thread;

use rand::prelude::*;

use crate::{problem::{Point, Problem}, solution::Solution};

pub struct Basic;
impl Basic {
    pub fn new() -> Self {Self}
    pub fn solve(&self, problem: &Problem) -> Option<Solution> {
        let mut solution = Solution {
            vertices: problem.figure.vertices.clone()
        };
        let MAX_ITERATIONS: usize = 1_000_000_000;
        for _i in 0..MAX_ITERATIONS {
            let result = solution.check(&problem);
            if result.is_valid() {
                return Some(solution);
            } else {
                if result.invalid_vertices.len() > 0 {
                    //fix a vertex
                    let p = result.invalid_vertices.choose(&mut rand::thread_rng()).unwrap();
                    //find the closest edge
                    let new_loc = Self::find_new_vertex_location(&solution.vertices[*p], &problem.hole);
                    solution.vertices[*p] = vec![new_loc.0, new_loc.1];
                } else if result.invalid_edges_intersecting.len() > 0 {
                    //fix an edge
                    break;
                } else if result.invalid_edges_stretched.len() > 0 {
                    break;
                    //fix a stretching problem
                }
            }
        }
        return None;
    }
    fn find_new_vertex_location(p_vec: &Vec<i128>, hole: &Vec<Point>) -> (i128, i128) {
        let p = (p_vec[0],p_vec[1]);
        let mut closest_edge = None;
        for i in 0..hole.len() {
            let p1= (hole[i][0], hole[i][1]);
            let p2 = (hole[((i+1)%hole.len())][0], hole[((i+1)%hole.len())][1]);

            let numerator = (((p2.0-p1.0)*(p1.1-p.1))-((p1.0-p.0)*(p2.1-p1.1))).abs() as f64;
            let denominator = (((p2.0-p1.0).pow(2)+(p2.1-p1.1).pow(2)) as f64).sqrt();
            let h = numerator/denominator;

            match &mut closest_edge {
                Some(((ref mut p1x,ref mut p1y),(ref mut p2x,ref mut p2y),ref mut h_old)) => {
                    if h < *h_old {
                        *p1x = p1.0;
                        *p1y = p1.1;
                        *p2x = p2.0;
                        *p2y = p2.1;
                        *h_old = h;
                    }
                }
                none => {
                    *none = Some(((p1.0,p1.1),(p2.0,p2.1),h));
                },
            }
        }
        //closest edge is now the edge we want to project against
        let p1 = closest_edge.unwrap().0;
        let p2 = closest_edge.unwrap().1;
        let v1 = (p.0-p1.0,p.1-p1.1);
        let v2 = (p2.0-p1.0,p2.1-p1.1);
        let v3 = (p.0-p2.0,p.1-p2.1);
        let b = (((p2.0-p1.0).pow(2)+(p2.1-p1.1).pow(2))as f64).sqrt();
        let len_along = (((v1.0*v2.0)+(v1.1*v2.1)) as f64)/b;
        let p_new = ((p1.0 as f64 +((len_along/b)* ((p2.0-p1.0) as f64))),(p1.1 as f64 +((len_along/b)* ((p2.1-p1.1) as f64))));

        //we have a projected point, lets just check if it's between the two on our edge, and if not, default it to one of the corners
        if ((p_new.0 <= p1.0 as f64 && p_new.0 >= p2.0 as f64) ||
            (p_new.0 <= p2.0 as f64 && p_new.0 >= p1.0 as f64)) &&
           ((p_new.1 <= p1.1 as f64 && p_new.1 >= p2.1 as f64) ||
            (p_new.1 <= p2.1 as f64 && p_new.1 >= p1.1 as f64)) {
                //it's between, just round and return
                return (p_new.0.round() as i128, p_new.1.round() as i128);
                
            } else {
                //uh oh, we need to saturate to one of the points.
                let l_p1 = ((v1.0.pow(2)+v1.1.pow(2))as f64).sqrt();
                let l_p2 = ((v3.0.pow(2)+v3.1.pow(2))as f64).sqrt();
                if l_p1 < l_p2 {
                    //p1 is closer, let's use that
                    return p1;
                } else {
                    //p2 is closer, let's use that
                    return p2;
                }
            }

    }
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    pub fn test_closest_edge() {
        let p = vec![2,1];
        let hole = vec![
            vec![0,0],
            vec![0,4],
            vec![4,4],
            vec![4,0]
        ];
        let result = Basic::find_new_vertex_location(&p, &hole);
        assert!(result.0 == 2);
        assert!(result.1 == 0);
    }
}