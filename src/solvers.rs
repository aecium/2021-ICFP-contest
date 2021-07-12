use crate::solvers::solver_utils::RandRound;

use rand::prelude::*;

use crate::{problem::{Point, Problem}, solution::Solution};

mod solver_utils;

pub struct Basic;
impl Basic {
    pub fn new() -> Self {Self}
    pub fn solve(&self, problem: &Problem) -> Option<Solution> {
        let mut solution = Solution {
            vertices: problem.figure.vertices.clone()
        };
        const MAX_ITERATIONS: usize = 50_000;
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
                    //use the lazy way and teleport one of the points to the other
                    let edge_index = result.invalid_edges_intersecting[0].shape_edge_index;
                    let p1 = &solution.vertices[problem.figure.edges[edge_index][0]];
                    solution.vertices[problem.figure.edges[edge_index][1]] = p1.clone();
                } else if result.invalid_edges_stretched.len() > 0 {
                    let invalid_edge = result.invalid_edges_stretched[0].index;
                    //first, how far do we need to move these points?
                    //how far apart are they now?
                    let p1 = (solution.vertices[problem.figure.edges[invalid_edge][0]][0],solution.vertices[problem.figure.edges[invalid_edge][0]][1]);
                    let p2 = (solution.vertices[problem.figure.edges[invalid_edge][1]][0],solution.vertices[problem.figure.edges[invalid_edge][1]][1]);
                    let d_current = (((p2.0-p1.0).pow(2)+(p2.1-p1.1).pow(2)) as f64).sqrt();
                    //how far apart were they?
                    let op1 = &problem.figure.vertices[problem.figure.edges[invalid_edge][0]];
                    let op2 = &problem.figure.vertices[problem.figure.edges[invalid_edge][1]];
                    let vo = (op2[0]-op1[0],op2[1]-op1[1]);
                    let d_orig = ((vo.0.pow(2)+vo.1.pow(2)) as f64).sqrt();
                    let delta = if d_current == 0.0 {
                        //delta should be the original distance between the two points on the edges / 2
                        d_orig / 2.0
                    } else {
                        // figure out which of the two distances we need to achieve
                        let ef = problem.epsilon as f64/1_000_000.0;
                        let d_min = ((d_orig * d_orig) * ((-1.0 * ef) + 1.0)).sqrt();
                        let d_max = ((d_orig * d_orig) * (ef + 1.0)).sqrt();

                        if (d_max - d_current).abs() < (d_min - d_current).abs() {
                            //max is closer, calculate delta using that
                            (d_max - d_current) / 2.0
                        } else {
                            //max is closer, calculate delta using that
                            (d_min - d_current) / 2.0
                        }
                    };
                    //now we have a delta, time to move some points
                    let uv12 = if d_current == 0.0 {
                        //if the points are in the same place, their unit vector is undefined
                        // let's pick a random one, it has to work eventually, right?
                        let mut rng = rand::thread_rng();
                        let x: f64 = rng.gen();
                        let y: f64 = rng.gen();
                        let len = ((x * x) + (y * y)).sqrt();
                        let x_normalized = x/len;
                        let y_normalized = y/len;
                        (x_normalized,y_normalized)
                    } else {
                        (((p2.0-p1.0) as f64)/d_current,((p2.1-p1.1) as f64)/d_current)
                    };
                    let p2_x_new = p2.0 as f64+ (uv12.0 * delta);
                    let p2_y_new = p2.1 as f64+ (uv12.1 * delta);
                    let p1_x_new = p1.0 as f64+ (uv12.0 * delta * -1.0);
                    let p1_y_new = p1.1 as f64+ (uv12.1 * delta * -1.0);

                    let p2_new = vec![p2_x_new.rand_round() as i128,p2_y_new.rand_round() as i128];
                    let p1_new = vec![p1_x_new.rand_round() as i128,p1_y_new.rand_round() as i128];

                    solution.vertices[problem.figure.edges[invalid_edge][0]] = p1_new;
                    solution.vertices[problem.figure.edges[invalid_edge][1]] = p2_new;
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
                return (p_new.0.rand_round() as i128, p_new.1.rand_round() as i128);
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

pub struct Border;
impl Border {
    pub fn new() -> Self {Self}
    pub fn solve(&self, problem: &Problem) -> Option<Solution> {
        if problem.figure.vertices.len() > 10 || problem.hole.len() > 10 {
            //Too much work!
            return None
        } else {
            let options: Vec<(i128,i128)> = problem.hole.clone().iter().map(|i| (i[0],i[1])).collect();
            let solve_iterator = BorderIterator::new(problem.figure.vertices.len(), options.as_slice());
            for solution_verts in solve_iterator {
                //dbg!(&solution_verts);
                let solution_verticies = solution_verts.iter().map(|v| vec![v.0,v.1]).collect();
                let solution = Solution {
                    vertices: solution_verticies
                };
                if solution.check(&problem).is_valid() {
                    return Some(solution);
                }
            };
            return None;
        }
    }
}
#[derive(Debug)]
struct BorderIterator<'src> {
    len: usize,
    options: &'src[(i128,i128)],
    my_index : usize,
    child: Option<Box<BorderIterator<'src>>>
}


impl BorderIterator<'_> {
    fn new(len: usize,options: &'_[(i128,i128)]) -> BorderIterator {
        let child = if len == 0 {
            None
        } else {
            Some(Box::new(BorderIterator::new(len-1,options)))
        };
        BorderIterator {
            len,
            options,
            my_index: 0,
            child
        }
    }
}

impl Iterator for BorderIterator<'_> {
    type Item = Vec<(i128,i128)>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.len == 0 {
            None
        } else if self.len == 1 {
            if self.my_index == self.options.len() {
                return None
            }
            let mut result = Vec::new();
            result.push(self.options[self.my_index]);
            self.my_index += 1;
            Some(result)
        } else {
            let next = &self.child.as_mut().unwrap().next();
            match next.clone() {
                Some(mut child_vec) => {
                    child_vec.push(self.options[self.my_index]);
                    return Some(child_vec);
                },
                None => {
                    self.my_index += 1;
                    if self.my_index == self.options.len() {
                        return None;
                    } else {
                        self.child = Some(Box::new(BorderIterator::new(self.len-1,self.options)));
                        return self.next();
                    }
                }
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