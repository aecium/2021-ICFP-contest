use std::u128;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct Problem {
    hole: Vec<Point>,
    figure: Figure,
    epsilon: u128
}

#[derive(Deserialize, Debug)]
pub struct Figure {
    edges: Vec<Edge>,
    vertices: Vec<Point>
}

type Point = Vec<u128>;
type Edge = Vec<usize>;

#[derive(Serialize, Debug)]
pub struct Solution {
    vertices: Vec<Point>
}