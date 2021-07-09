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

pub type Point = Vec<i128>;
pub type Edge = Vec<usize>;