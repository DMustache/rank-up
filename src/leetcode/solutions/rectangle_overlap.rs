struct Solution;

struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

struct Rectangle {
    top_right: Point,
    bottom_left: Point,
}

impl TryFrom<Vec<i32>> for Rectangle {
    type Error = bool;

    fn try_from(value: Vec<i32>) -> Result<Self, Self::Error> {
        let rectangle = Self {
            top_right: Point::new(value[2], value[3]),
            bottom_left: Point::new(value[0], value[1]),
        };

        if rectangle.top_right.x < rectangle.bottom_left.x {
            return Err(false);
        }

        if rectangle.top_right.y < rectangle.bottom_left.y {
            return Err(false);
        }

        Ok(rectangle)
    }
}

impl Solution {
    pub fn is_rectangle_overlap(rec1: Vec<i32>, rec2: Vec<i32>) -> bool {
        let rec1 = match Rectangle::try_from(rec1) {
            Ok(rectangle) => rectangle,
            Err(_) => return false,
        };
        let rec2 = match Rectangle::try_from(rec2) {
            Ok(rectangle) => rectangle,
            Err(_) => return false,
        };

        let horizontal_overlap =
            rec1.bottom_left.x < rec2.top_right.x && rec2.bottom_left.x < rec1.top_right.x;
        let vertical_overlap =
            rec1.bottom_left.y < rec2.top_right.y && rec2.bottom_left.y < rec1.top_right.y;

        horizontal_overlap && vertical_overlap
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(
            Solution::is_rectangle_overlap(vec![0, 0, 2, 2], vec![1, 1, 3, 3]),
            true
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            Solution::is_rectangle_overlap(vec![0, 0, 1, 1], vec![1, 0, 2, 1]),
            false
        );
    }

    #[test]
    fn example3() {
        assert_eq!(
            Solution::is_rectangle_overlap(vec![0, 0, 1, 1], vec![2, 2, 3, 3]),
            false
        );
    }

    #[test]
    fn one_rectangle_contains_the_other() {
        assert!(Solution::is_rectangle_overlap(
            vec![0, 0, 4, 4],
            vec![1, 1, 2, 2]
        ));
    }

    #[test]
    fn rectangles_touching_at_an_edge_do_not_overlap() {
        assert!(!Solution::is_rectangle_overlap(
            vec![0, 0, 2, 2],
            vec![2, 0, 4, 2]
        ));
    }
}
