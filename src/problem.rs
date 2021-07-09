use std::u128;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct Problem {
    pub hole: Vec<Point>,
    pub figure: Figure,
    pub epsilon: u128
}

#[derive(Deserialize, Debug)]
pub struct Figure {
    pub edges: Vec<Edge>,
    pub vertices: Vec<Point>
}

type Point = Vec<i128>;
type Edge = Vec<usize>;

#[derive(Serialize, Debug)]
pub struct Solution {
    pub vertices: Vec<Point>
}

impl Solution {
    pub fn check(&self, problem: &Problem) -> bool {
        //first, are all the vertices inside or on the shape.
        for p in &self.vertices {
            dbg!(&p);
            let x = p[0];
            let y = p[1];
            let mut wn = 0;
            for i in 0..problem.hole.len() {
                //does the current edge cross the y value of the point in question                
                let p1= (problem.hole[i][0], problem.hole[i][1]);
                let p2 = (problem.hole[((i+1)%problem.hole.len())][0], problem.hole[((i+1)%problem.hole.len())][1]);
                let m: f64 = ((p2.1-p1.1) as f64)/((p2.0-p1.0) as f64);
                dbg!(p1);
                dbg!(p2);
                dbg!(m);
                // upward crossing
                if p1.1 < y && p2.1 >= y {
                    if (y - p1.1) as f64/m + p1.0 as f64 >= x as f64 {
                        dbg!("winding number increased!");
                        wn+=1;
                    }
                }
                //downward crossing
                else if p1.1 > y && p2.1 <= y {
                    if (y - p1.1) as f64/m + p1.0 as f64 >= x as f64 {
                        dbg!("winding number decreased!");
                        wn-=1;
                    }
                }
                // not used for winding number calculation
                else { continue; }
            }
            if wn == 0 {return false;}
        }
        //second, does the solution satisfy the elasticity constraint
        //third, do any lines intersect with the 
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
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
        assert!(s1.check(&p));
        //assert!(s2.check(&p));
        assert!(s3.check(&p));
        assert!(s4.check(&p));
    }
}