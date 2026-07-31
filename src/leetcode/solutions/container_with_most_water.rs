struct Solution;

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut result = 0;
        let mut left = 0;
        let mut right = height.len() - 1;

        while left < right {
            let width = (right - left) as i32;
            let water_height = height[left].min(height[right]);
            let volume = width * water_height;

            if result < volume {
                result = volume;
            }

            if height[left] < height[right] {
                left += 1;
            } else {
                right -= 1;
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let case = [1, 8, 6, 2, 5, 4, 8, 3, 7].to_vec();
        assert_eq!(49, Solution::max_area(case));
    }

    #[test]
    fn example2() {
        let case = [1, 1].to_vec();
        assert_eq!(1, Solution::max_area(case));
    }
}
