struct Solution;

impl Solution {
    pub fn min_sum_square_diff(nums1: Vec<i32>, nums2: Vec<i32>, k1: i32, k2: i32) -> i64 {
        let differences: Vec<i64> = nums1
            .iter()
            .zip(&nums2)
            .map(|(&a, &b)| i64::from(a).abs_diff(i64::from(b)) as i64)
            .collect();
        let mut operations = i64::from(k1) + i64::from(k2);

        if differences.iter().sum::<i64>() <= operations {
            return 0;
        }

        let (mut left, mut right) = (0, *differences.iter().max().unwrap());

        while left < right {
            let mid = left + (right - left) / 2;
            let needed: i64 = differences.iter().map(|&value| (value - mid).max(0)).sum();
            if needed <= operations {
                right = mid;
            } else {
                left = mid + 1;
            }
        }

        let mut result = 0;
        for value in differences {
            let reduced = value.min(left);
            operations -= value - reduced;
            result += reduced * reduced;
        }

        result - operations * (2 * left - 1)
    }
}

#[cfg(test)]
mod tests {
    use crate::leetcode::solutions::minimum_sum_of_squared_difference::Solution;

    #[test]
    fn example1() {
        assert_eq!(
            Solution::min_sum_square_diff(vec![1, 2, 3, 4], vec![2, 10, 20, 19], 0, 0),
            579
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            Solution::min_sum_square_diff(vec![1, 4, 10, 12], vec![5, 8, 6, 9], 1, 1),
            43
        );
    }

    #[test]
    fn operations_cover_all_differences() {
        assert_eq!(
            Solution::min_sum_square_diff(vec![-5, 2], vec![1, -1], 4, 5),
            0
        );
    }

    #[test]
    fn remaining_operations_lower_multiple_equal_differences() {
        assert_eq!(
            Solution::min_sum_square_diff(vec![0, 0, 0], vec![5, 5, 5], 4, 0),
            41
        );
    }

    #[test]
    fn smaller_differences_stay_unchanged() {
        assert_eq!(
            Solution::min_sum_square_diff(vec![0, 0, 0], vec![1, 5, 5], 3, 0),
            26
        );
    }
}
