struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn four_sum_count(
        nums1: Vec<i32>,
        nums2: Vec<i32>,
        nums3: Vec<i32>,
        nums4: Vec<i32>,
    ) -> i32 {
        let mut sums = HashMap::new();

        for first in &nums1 {
            for second in &nums2 {
                *sums.entry(first + second).or_insert(0) += 1;
            }
        }

        let mut count = 0;

        for third in &nums3 {
            for fourth in &nums4 {
                count += sums.get(&-(third + fourth)).unwrap_or(&0);
            }
        }

        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let nums1 = vec![1, 2];
        let nums2 = vec![-2, -1];
        let nums3 = vec![-1, 2];
        let nums4 = vec![0, 2];

        assert_eq!(2, Solution::four_sum_count(nums1, nums2, nums3, nums4));
    }

    #[test]
    fn example2() {
        let nums1 = vec![0];
        let nums2 = vec![0];
        let nums3 = vec![0];
        let nums4 = vec![0];

        assert_eq!(1, Solution::four_sum_count(nums1, nums2, nums3, nums4));
    }
}
