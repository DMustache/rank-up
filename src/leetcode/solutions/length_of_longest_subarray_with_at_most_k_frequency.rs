struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn max_subarray_length(nums: Vec<i32>, k: i32) -> i32 {
        let mut counter = HashMap::new();
        let mut left = 0;
        let mut max_len = 0;

        for right in 0..nums.len() {
            let right_num = nums[right];
            *counter.entry(right_num).or_insert(0) += 1;

            while counter[&right_num] > k {
                let left_num = nums[left];
                *counter.get_mut(&left_num).unwrap() -= 1;
                left += 1;
            }

            max_len = max_len.max(right - left + 1);
        }

        max_len as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let nums = vec![1, 2, 3, 1, 2, 3, 1, 2];
        let k = 2;

        assert_eq!(Solution::max_subarray_length(nums, k), 6);
    }

    #[test]
    fn example_2() {
        let nums = vec![1, 2, 1, 2, 1, 2, 1, 2];
        let k = 1;

        assert_eq!(Solution::max_subarray_length(nums, k), 2);
    }

    #[test]
    fn example_3() {
        let nums = vec![5, 5, 5, 5, 5, 5, 5];
        let k = 4;

        assert_eq!(Solution::max_subarray_length(nums, k), 4);
    }
}
