use std::cmp::max;

struct Solution;

impl Solution {
    pub fn min_operations(nums: Vec<i32>, x: i32) -> i32 {
        let target = nums.iter().sum::<i32>() - x;

        if target == 0 {
            return nums.len() as i32;
        }

        if target < 0 {
            return -1;
        }

        let mut current_sum = 0;
        let mut max_lenght = 0;
        let mut left = 0;

        for right in 0..nums.len() {
            current_sum += nums[right];

            while current_sum > target && left <= right {
                current_sum -= nums[left];
                left += 1;
            }

            if current_sum == target {
                max_lenght = max(max_lenght, right - left + 1);
            }
        }

        if max_lenght != 0 {
            return (nums.len() - max_lenght) as i32;
        }
        -1
    }
}

#[cfg(test)]
mod test {
    use crate::leetcode::solutions::minimum_operations_to_reduce_x_to_zero::Solution;

    #[test]
    fn example1() {
        assert_eq!(Solution::min_operations(vec![1, 1, 4, 2, 3], 5), 2);
    }

    #[test]
    fn example2() {
        assert_eq!(Solution::min_operations(vec![5, 6, 7, 8, 9], 4), -1);
    }

    #[test]
    fn example3() {
        assert_eq!(Solution::min_operations(vec![3, 2, 20, 1, 1, 3], 10), 5);
    }
}
