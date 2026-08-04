struct Solution;

impl Solution {
    pub fn find_missing_elements(nums: Vec<i32>) -> Vec<i32> {
        let mut nums = nums;
        nums.sort_unstable();
        nums.windows(2)
            .flat_map(|pair| (pair[0] + 1)..pair[1])
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let nums = vec![1, 4, 2, 5];

        assert_eq!(vec![3], Solution::find_missing_elements(nums));
    }

    #[test]
    fn example2() {
        let nums = vec![7, 8, 6, 9];

        assert_eq!(Vec::<i32>::new(), Solution::find_missing_elements(nums));
    }

    #[test]
    fn example3() {
        let nums = vec![5, 1];

        assert_eq!(vec![2, 3, 4], Solution::find_missing_elements(nums));
    }
}
