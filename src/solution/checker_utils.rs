use crate::problem::Point;
#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum RotationDirection {
    Clockwise,
    CounterClockwise
}
pub fn vector_from_points(p1:(i128,i128), p2:(i128,i128)) -> (i128,i128) {
    let x = p2.0-p1.0;
    let y = p2.1-p1.1;
    return (x,y);
}
pub fn cross_product(v1: (i128,i128), v2: (i128, i128)) -> i128 {
    return (v1.0*v2.1) - (v2.0*v1.1)
}

pub fn determine_rotation(polygon: &Vec<Point>) -> RotationDirection {
    let mut total = 0;
    for i in 0..polygon.len() {
        let p1= (polygon[i][0], polygon[i][1]);
        let p2 = (polygon[((i+1)%polygon.len())][0], polygon[((i+1)%polygon.len())][1]);
        total += (p1.0*p2.1)-(p2.0*p1.1)
    }
    if total > 0 {
        RotationDirection::Clockwise
    } else {
        RotationDirection::CounterClockwise
    }
}
pub fn is_point_on_shape(p: &Point, shape: &Vec<Point>) -> bool {
    for i in 0..shape.len() {
        let p1= (shape[i][0], shape[i][1]);
        let p2 = (shape[((i+1)%shape.len())][0], shape[((i+1)%shape.len())][1]);
        if ((p[0]*(p1.1-p2.1))+(p1.0*(p2.1-p[1]))+(p2.0*(p[1]-p1.1))) == 0 {
            //the points are colinear!
            if ((p[0] >= p1.0 && p[0] <= p2.0) ||
                (p[0] >= p2.0 && p[0] <= p1.0)) && 
               ((p[1] >= p1.0 && p[1] <= p2.1) ||
                (p[1] >= p2.0 && p[1] <= p1.1)) {
                return true;
            }
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    pub fn test_rotation_check_cw() {
        let triangle = vec![
            vec![0,0],
            vec![2,0],
            vec![1,1]
        ];
        assert_eq!(determine_rotation(&triangle), RotationDirection::Clockwise);
    }
    #[test]
    pub fn test_rotation_check_ccw() {
        let triangle = vec![
            vec![0,0],
            vec![1,1],
            vec![2,0]
        ];
        assert_eq!(determine_rotation(&triangle), RotationDirection::CounterClockwise);
    }
    #[test]
    pub fn test_rotation_check_concave_cw() {
        let triangle = vec![
            vec![0,0],
            vec![4,0],
            vec![3,3],
            vec![2,1],
            vec![1,3]
        ];
        assert_eq!(determine_rotation(&triangle), RotationDirection::Clockwise);
    }
    #[test]
    pub fn test_rotation_check_concave_ccw() {
        let mountain = vec![
            vec![0,0],
            vec![1,3],
            vec![2,1],
            vec![3,3],
            vec![4,0]
        ];
        assert_eq!(determine_rotation(&mountain), RotationDirection::CounterClockwise);
    }
    #[test]
    pub fn test_rotation_check_concave_ccw_shifted() {
        let mountain = vec![
            vec![2,1],
            vec![3,3],
            vec![4,0],
            vec![0,0],
            vec![1,3]
        ];
        assert_eq!(determine_rotation(&mountain), RotationDirection::CounterClockwise);
    }
    #[test]
    pub fn test_rotation_check_concave_cw_shifted() {
        let mountain = vec![
            vec![2,1],
            vec![1,3],
            vec![0,0],
            vec![4,0],
            vec![3,3]
        ];
        assert_eq!(determine_rotation(&mountain), RotationDirection::Clockwise);
    }
}