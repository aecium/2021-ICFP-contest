use serde::Serialize;
use crate::{problem::{Problem, Point}, solution::checker_utils::{RotationDirection, determine_rotation, is_point_on_shape}};

mod checker_utils;

#[derive(Serialize, Debug)]
pub struct Solution {
    pub vertices: Vec<Point>
}
#[derive(Debug)]
pub struct CheckResult {
    is_valid: bool
}
impl CheckResult {
    pub fn is_valid(&self) -> bool {
        return self.is_valid;
    }
}

impl Solution {
    pub fn check(&self, problem: &Problem) -> CheckResult {
        //first, are all the vertices inside or on the shape.
        let rotation_direction = determine_rotation(&problem.hole);
        for p in &self.vertices {
            dbg!(&p);
            //if p is on the border, it's inside, no need to continue further
            if is_point_on_shape(&p,&problem.hole) {
                continue;
            }
            let x = p[0];
            let y = p[1];
            let mut crossings = 0;
            let mut hole = problem.hole.clone();
            if rotation_direction == RotationDirection::Clockwise {
                hole.reverse();
            }
            // hold the last x, this is only for a specific special case where the point in question is exactly horizontal with a point in the hole
            let mut last_x = -1.0;
            for i in 0..hole.len() {
                //does the current edge cross the y value of the point in question
                let p1= (hole[i][0], hole[i][1]);
                let p2 = (hole[((i+1)%hole.len())][0], hole[((i+1)%hole.len())][1]);
                let m: f64 = ((p2.1-p1.1) as f64)/((p2.0-p1.0) as f64);
                if m == 0.0 {
                    //horizontal lines aren't helpful, they only matter if the point is on the line, which is already checked
                    continue;
                }
                dbg!(p1);
                dbg!(p2);
                dbg!(m);
                // interesting line
                if (p1.1 <= y && p2.1 >= y) || (p1.1 >= y && p2.1 <= y) {
                    dbg!("This line segment crosses the y");
                    //find the x value of the intersection with this line and the horizontal ray from the point
                    let ray_x = (((y - p1.1) as f64)/(m as f64)) + p1.0 as f64;
                    if ray_x < x as f64 && ray_x != last_x{
                        last_x = ray_x;
                        dbg!("incrementing crossings");
                        crossings += 1;
                    }
                }
                else { continue; }
            }
            if (crossings % 2) == 0 {
                return CheckResult {is_valid: false};
            }
        }
        //TODO second, does the solution satisfy the elasticity constraint
        //TODO third, do any lines intersect with the hole boundaries
        CheckResult{ is_valid: true}
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
            epsilon: 2
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
        //TODO uncomment these to check colinear and co-point solutions
        //assert!(s1.check(&p).is_valid());
        //assert!(s2.check(&p).is_valid());
        //assert!(s3.check(&p).is_valid());
        //assert!(s4.check(&p).is_valid());
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
}