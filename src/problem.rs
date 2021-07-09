use std::u128;

pub struct Problem {
    hole: Vec<Point>,
    figure: Figure,
    epsilon: u128
}

pub struct Figure {
    edges: Vec<usize>,
    verticies: Vec<Point>
}

pub struct Point {
    x: u128,
    y: u128, 
}

pub struct Solution {
    verticies: Vec<Point>
}