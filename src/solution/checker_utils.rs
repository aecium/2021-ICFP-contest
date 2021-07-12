use crate::problem::Point;
#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum RotationDirection {
    Clockwise,
    CounterClockwise
}
pub fn vector_from_points(p1:(isize,isize), p2:(isize,isize)) -> (isize,isize) {
    let x = p2.0-p1.0;
    let y = p2.1-p1.1;
    return (x,y);
}
pub fn cross_product(v1: (isize,isize), v2: (isize, isize)) -> isize {
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

               ((p[1] >= p1.1 && p[1] <= p2.1) ||
                (p[1] >= p2.1 && p[1] <= p1.1)) {
                return true;
            }
        }
    }
    false
}

pub fn is_point_inside_shape(p: &Vec<isize>, shape: &Vec<Vec<isize>>) -> bool{
    //if p is on the border, it's inside, no need to continue further
    if is_point_on_shape(&p,shape) {
        return true;
    }
    let x = p[0];
    let y = p[1];
    let mut crossings = 0;
    let mut hole = shape.clone();
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
                let found_point = (ray_x as isize, y);
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