use serde::Serialize;
use crate::{problem::{Problem, Point}, solution::checker_utils::{RotationDirection, determine_rotation, is_point_on_shape}};

mod checker_utils;

#[derive(Serialize, Debug)]
pub struct Solution {
    pub vertices: Vec<Point>
}
#[derive(Debug)]
pub struct CheckResult {
    is_valid: bool,
    invalid_vertices: Vec<Vec<i128>>
}
impl CheckResult {
    pub fn is_valid(&self) -> bool {
        return self.is_valid;
    }
}

impl Solution {
    pub fn check(&self, problem: &Problem) -> CheckResult {
        let mut result = CheckResult {is_valid: true, invalid_vertices: Vec::new()};
        //first, are all the vertices inside or on the shape.
        let rotation_direction = determine_rotation(&problem.hole);
        for p in &self.vertices {
            if !Self::is_point_inside_shape(&p, &problem.hole, rotation_direction) {
                result.is_valid = false;
                result.invalid_vertices.push(p.clone());
            }
        }
        //TODO second, does the solution satisfy the elasticity constraint
        //TODO third, do any lines intersect with the hole boundaries
        result
    }

    fn is_point_inside_shape(p: &Vec<i128>, shape: &Vec<Vec<i128>>, rotation_direction: RotationDirection) -> bool{
        dbg!(&p);
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
            dbg!(p1);
            dbg!(p2);
            dbg!(m);
            // interesting line
            if (p1.1 <= y && p2.1 >= y) || (p1.1 >= y && p2.1 <= y) {
                dbg!("This line segment crosses the y");
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
                    dbg!("incrementing crossings");
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
            epsilon: 200000
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
}