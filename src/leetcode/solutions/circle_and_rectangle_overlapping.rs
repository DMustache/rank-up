struct Solution;

impl Solution {
    pub fn check_overlap(
        radius: i32,
        x_center: i32,
        y_center: i32,
        x1: i32,
        y1: i32,
        x2: i32,
        y2: i32,
    ) -> bool {
        let closest_x = x_center.clamp(x1, x2);
        let closest_y = y_center.clamp(y1, y2);

        let dx = x_center - closest_x;
        let dy = y_center - closest_y;

        dx * dx + dy * dy <= radius * radius
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(Solution::check_overlap(1, 0, 0, 1, -1, 3, 1), true);
    }

    #[test]
    fn example2() {
        assert_eq!(Solution::check_overlap(1, 1, 1, 1, -3, 2, -1), false);
    }

    #[test]
    fn example3() {
        assert_eq!(Solution::check_overlap(1, 0, 0, -1, 0, 0, 1), true);
    }
}
