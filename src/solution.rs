use std::usize;

use serde::{Serialize, Deserialize};
use crate::{problem::{Problem, Point}, solution::checker_utils::{RotationDirection, determine_rotation, is_point_on_shape}};

use self::checker_utils::{cross_product, vector_from_points};

mod checker_utils;

#[derive(Serialize, Deserialize, Debug)]
pub struct Solution {
    pub vertices: Vec<Point>
}
#[derive(Serialize, Debug)]
pub struct CheckResult {
    is_valid: bool,
    pub invalid_vertices: Vec<usize>,
    pub invalid_edges_stretched: Vec<StretchedEdge>,
    pub invalid_edges_intersecting: Vec<IntersectingEdge>
}
impl CheckResult {
    pub fn is_valid(&self) -> bool {
        return self.is_valid;
    }
}
#[derive(Debug, Serialize)]
pub struct StretchedEdge {
    pub index: usize,
    pub error: f64
}
#[derive(Debug, Serialize)]
pub struct IntersectingEdge {
    pub shape_edge_index: usize,
    pub hole_edge_start: usize
}
impl Solution {
    pub fn check(&self, problem: &Problem) -> CheckResult {
        let mut result = CheckResult {
            is_valid: true,
            invalid_vertices: Vec::new(),
            invalid_edges_stretched: Vec::new(),
            invalid_edges_intersecting: Vec::new()
        };
        //first, are all the vertices inside or on the shape.
        let rotation_direction = determine_rotation(&problem.hole);
        for i in  0..self.vertices.len() {
            let p = &self.vertices[i];
            if !Self::is_point_inside_shape(&p, &problem.hole, rotation_direction) {
                result.is_valid = false;
                result.invalid_vertices.push(i);
            }
        }
        //second, does the solution satisfy the elasticity constraint
        for i in 0..problem.figure.edges.len() {
            let edge = &problem.figure.edges[i];
            let p1 = &problem.figure.vertices[edge[0]];
            let p2 = &problem.figure.vertices[edge[1]];
            let p1_prime = &self.vertices[edge[0]];
            let p2_prime = &self.vertices[edge[1]];
            let numerator = ((p2_prime[0]-p1_prime[0]).pow(2)+(p2_prime[1]-p1_prime[1]).pow(2)) as f64;
            let denominator = ((p2[0]-p1[0]).pow(2)+(p2[1]-p1[1]).pow(2)) as f64;
            let error = ((numerator/denominator) - 1.0) * 1_000_000.0;
            if error.abs() > problem.epsilon as f64 {
                result.is_valid = false;
                result.invalid_edges_stretched.push(StretchedEdge {
                    index: i,
                    error: error
                });
            }
        }
        //third, do any lines intersect with the hole boundaries
        for edge_index in 0..problem.figure.edges.len() {
            for hole_edge in 0..problem.hole.len() {
                let solution_edge = &problem.figure.edges[edge_index];
                let ps1 = (self.vertices[solution_edge[0]][0],self.vertices[solution_edge[0]][1]);
                let ps2 = (self.vertices[solution_edge[1]][0],self.vertices[solution_edge[1]][1]);
                let pe1= (problem.hole[hole_edge][0], problem.hole[hole_edge][1]);
                let pe2 = (problem.hole[((hole_edge+1)%problem.hole.len())][0], problem.hole[((hole_edge+1)%problem.hole.len())][1]);
                let d1 = cross_product(vector_from_points(pe1, ps1), vector_from_points(pe1, pe2));
                let d2 = cross_product(vector_from_points(pe1, ps2), vector_from_points(pe1, pe2));
                let d3 = cross_product(vector_from_points(ps1, pe1), vector_from_points(ps1, ps2));
                let d4 = cross_product(vector_from_points(ps1, pe2), vector_from_points(ps1, ps2));
                if ((d1<0 && d2>0)||(d1>0 && d2<0)) && ((d3>0 && d4<0) || (d3<0 && d4>0)) {
                    result.is_valid = false;
                    result.invalid_edges_intersecting.push(IntersectingEdge {
                        shape_edge_index: edge_index,
                        hole_edge_start: hole_edge,
                    });
                }
            }
        }
        result
    }

    fn is_point_inside_shape(p: &Vec<i128>, shape: &Vec<Vec<i128>>, rotation_direction: RotationDirection) -> bool{
        //if p is on the border, it's inside, no need to continue further
        if is_point_on_shape(&p,shape) {
            return true;
        }
        let x = p[0];
        let y = p[1];
        let mut crossings = 0;
        let mut hole = shape.clone();
        if rotation_direction == RotationDirection::Clockwise {
            hole.reverse();
        }
        for i in 0..hole.len() {
            //does the current edge cross the y value of the point in question
            let p1= (hole[i][0], hole[i][1]);
            let p2 = (hole[((i+1)%hole.len())][0], hole[((i+1)%hole.len())][1]);
            let m: f64 = ((p2.1-p1.1) as f64)/((p2.0-p1.0) as f64);
            if m == 0.0 {
                //horizontal lines aren't helpful, they only matter if the point is on the line, which is already checked
                continue;
            }
            // interesting line
            if (p1.1 <= y && p2.1 >= y) || (p1.1 >= y && p2.1 <= y) {
                //find the x value of the intersection with this line and the horizontal ray from the point
                let ray_x = (((y - p1.1) as f64)/(m as f64)) + p1.0 as f64;
                if ray_x < x as f64{
                    //deal with edge cases regarding lines at exactly the vertical height of a hole vertex
                    let found_point = (ray_x as i128, y);
                    if (found_point.0 == p1.0 && found_point.1 == p1.1) ||
                       (found_point.0 == p2.0 && found_point.1 == p2.1) {
                           if p1.1 > y || p2.1 > y {
                               continue;
                           }
                        }
                    crossings += 1;
                }
            }
            else { continue; }
        }
    return (crossings % 2) == 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::problem::Figure;
    #[test]
    pub fn test_inside_shape() {
        let p = Problem{
            hole: vec![
                vec![0,0],
                vec![4,0],
                vec![4,4],
                vec![0,4]
            ],
            figure: Figure{
                edges: vec![
                    vec![0,1]
                ],
                vertices: vec![
                    vec![1,1],
                    vec![3,3]
                ]
            },
            epsilon: 20_000_000
        };
        let s1 = Solution {
            vertices: vec![
                vec![1,1],
                vec![0,0],
            ]
        };
        let s2 = Solution {
            vertices: vec![
                vec![1,1],
                vec![4,0],
            ]
        };
        let s3 = Solution {
            vertices: vec![
                vec![1,1],
                vec![4,4],
            ]
        };
        let s4 = Solution {
            vertices: vec![
                vec![1,1],
                vec![0,4],
            ]
        };
        let s5 = Solution {
            vertices: vec![
                vec![1,1],
                vec![3,3],
            ]
        };
        assert!(s1.check(&p).is_valid());
        assert!(s2.check(&p).is_valid());
        assert!(s3.check(&p).is_valid());
        assert!(s4.check(&p).is_valid());
        assert!(s5.check(&p).is_valid());
    }
    #[test]
    pub fn test_inside_shape_tricky_in() {
        let p = Problem{
            hole: vec![
                vec![5,1],
                vec![7,3],
                vec![6,5],
                vec![5,3],
                vec![4,3],
                vec![3,4],
                vec![2,2],
            ],
            figure: Figure{
                edges: vec![
                    vec![0,1]
                ],
                vertices: vec![
                    vec![5,2],
                    vec![6,3]
                ]
            },
            epsilon: 2
        };
        let s = Solution {
            vertices: vec![
                vec![5,2],
                vec![6,3]
            ]
        };

        assert!(s.check(&p).is_valid());
    }
    #[test]
    pub fn test_inside_shape_tricky_out() {
        let p = Problem{
            hole: vec![
                vec![5,1],
                vec![7,3],
                vec![6,5],
                vec![5,3],
                vec![4,3],
                vec![3,4],
                vec![2,2],
            ],
            figure: Figure{
                edges: vec![
                    vec![0,1]
                ],
                vertices: vec![
                    vec![5,2],
                    vec![6,3],
                    vec![8,3]
                ]
            },
            epsilon: 2
        };
        let s = Solution {
            vertices: vec![
                vec![5,2],
                vec![6,3],
                vec![8,3]
            ]
        };
        assert!(!s.check(&p).is_valid());
    }
    #[test]
    pub fn test_inside_shape_tricky_pt2_in() {
        let p = Problem{
            hole: vec![
                vec![2,0],
                vec![3,0],
                vec![4,1],
                vec![3,3],
                vec![2,1],
                vec![0,2],
                vec![1,1],
                vec![0,1],
            ],
            figure: Figure{
                edges: vec![
                    vec![0,1]
                ],
                vertices: vec![
                    vec![3,1],
                    vec![3,2]
                ]
            },
            epsilon: 2
        };
        let s = Solution {
            vertices: vec![
                vec![3,1],
                vec![3,2]
            ]
        };
        assert!(s.check(&p).is_valid());
    }
    #[test]
    pub fn test_intersect() {
        let p = Problem{
            hole: vec![
                vec![0,0],
                vec![4,0],
                vec![4,4],
                vec![0,4]
            ],
            figure: Figure{
                edges: vec![
                    vec![0,1]
                ],
                vertices: vec![
                    vec![1,1],
                    vec![3,3]
                ]
            },
            epsilon: 20_000_000
        };
        let s1 = Solution {
            vertices: vec![
                vec![1,1],
                vec![5,5],
            ]
        };
        let result = s1.check(&p);
        dbg!(&result);
        assert!(!&result.is_valid());
    }
    #[test]
    pub fn test_found_epsilon_bug() {
        let p = Problem{
            hole: vec![
                //take from problem 3
                vec![50,70],vec![35,75],vec![35,65],vec![15,55],vec![30,45],vec![25,30],vec![30,30],vec![30,15],vec![45,25],vec![55,35],vec![55,15],vec![65,20],vec![80,5],vec![85,25],vec![90,25],vec![80,45],vec![95,45],vec![105,50],vec![100,65],vec![85,70],vec![90,85],vec![65,80],vec![60,85],vec![55,70],vec![50,110],vec![45,110]
            ],
            figure: Figure{
                edges: vec![
                    vec![0,1]
                ],
                vertices: vec![
                    vec![9,6],
                    vec![11,15]
                ]
            },
            epsilon: 180_000
        };
        let s1 = Solution {
            vertices: vec![
                vec![46,27],
                vec![45,40]
            ]
        };
        let result = s1.check(&p);
        dbg!(&result);
        assert!(!&result.is_valid());
    }
}