use std::{fs::File, io::Read, thread, time};

use crate::{problem::{Point, Problem}, solution::Solution};
use crate::{solution::checker_utils::{RotationDirection, determine_rotation, is_point_inside_shape}};

fn get_edges_with_point(point_id: usize, edges: &Vec<Vec<usize>>) -> Vec<usize> {
    let mut matches = Vec::new();

    let mut i = 0;
    for edge in edges {
        if edge[0] == point_id || edge[1] == point_id {
            matches.push(i);
        }
        i = i+1;
    }

    return matches;
}

fn reflect(point: Point, p0: Point, p1: Point) -> Point {
    let px = point[0] as f64;
    let py = point[1] as f64;
    let p0x = p0[0] as f64;
    let p0y = p0[1] as f64;
    let p1x = p1[0] as f64;
    let p1y = p1[1] as f64;
    let dx = p1x - p0x;
    let dy = p1y - p0y;
    let a = (dx * dx - dy * dy) / (dx * dx + dy * dy);
    let b = 2.0 * dx * dy / (dx * dx + dy * dy);
    let x = (a * (px - p0x) + b * (py - p0y) + p0x) as isize; 
    let y = (b * (px - p0x) - a * (py - p0y) + p0y) as isize;

    return vec![x, y];
}

pub fn print(pose: Vec<Vec<isize>>){
    thread::sleep(time::Duration::from_millis(100));
    let json = serde_json::to_string(&Solution { vertices: pose}).expect("Couldn't serialize solution.");
    println!("{}", json);
}

pub struct Flect;
impl Flect {
    pub fn new() -> Self {Self}
    pub fn solve(&self, problem: &Problem, problem_file: &String) -> Option<Solution> {
        let figure = problem.figure.vertices.clone();
        let edges = problem.figure.edges.clone();
        let hole = problem.hole.clone();

        let mut pose = figure.clone();

        let solution = Solution{vertices: pose.clone()};
        let result = solution.check(&problem);

        //let json = serde_json::to_string(&result).expect("Couldn't serialize result.");
        print(pose.clone());

        for _ in 0..10 {
        for e in 0..edges.len() {
            let edge = &edges[e];
            for point_id in edge {
                let point_id = point_id.to_owned();
                let point = pose[point_id].clone();
                if !is_point_inside_shape(&point, &hole) {
                    let friends = get_edges_with_point(point_id, &edges);
                    //println!("{:?} {:?}", point, friends);
                    if friends.len() == 2 {
                        let r = reflect(point, pose[friends[0]].clone(), pose[friends[1]].clone());
                        pose[point_id][0] = r[0];
                        pose[point_id][1] = r[1];
                        print(pose.clone());
                    }
                }
            }
        }

    }
        let goal = solution.vertices.clone();

        print(pose.clone());

        return Some(solution);
    }

}

