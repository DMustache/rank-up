struct Solution;

impl Solution {
    pub fn sort_permutation(nums: Vec<i32>) -> i32 {
        let mut is_sorted = true;
        let mut result = i32::MAX;
        for i in 0..nums.len() {
            if is_sorted {
                if nums[i] != i as i32 {
                    is_sorted = false;
                }
            }

            if nums[i] != i as i32 {
                result &= nums[i]
            }
        }

        if is_sorted {
            return 0;
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example1() {
        assert_eq!(Solution::sort_permutation(vec![0, 3, 2, 1]), 1);
    }

    #[test]
    fn example2() {
        assert_eq!(Solution::sort_permutation(vec![0, 1, 3, 2]), 2);
    }

    #[test]
    fn example3() {
        assert_eq!(Solution::sort_permutation(vec![3, 2, 1, 0]), 0);
    }
}
