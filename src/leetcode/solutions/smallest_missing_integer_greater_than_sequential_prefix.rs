use std::collections::HashSet;

struct Solution;

impl Solution {
    pub fn missing_integer(nums: Vec<i32>) -> i32 {
        let nums_set: HashSet<i32> = nums.iter().copied().collect();

        let mut prefix_sum = nums[0];
        for i in 1..nums.len() {
            if nums[i] != nums[i - 1] + 1 {
                break;
            }

            prefix_sum += nums[i];
        }

        while nums_set.contains(&prefix_sum) {
            prefix_sum += 1;
        }

        prefix_sum
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let nums = vec![1, 2, 3, 2, 5];

        assert_eq!(6, Solution::missing_integer(nums));
    }

    #[test]
    fn example2() {
        let nums = vec![3, 4, 5, 1, 12, 14, 13];

        assert_eq!(15, Solution::missing_integer(nums));
    }
}
