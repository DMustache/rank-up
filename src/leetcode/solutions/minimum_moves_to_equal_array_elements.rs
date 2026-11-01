struct Solution;

impl Solution {
    pub fn min_moves(nums: Vec<i32>) -> i32 {
        let min = nums.iter().min().unwrap();

        nums.iter().map(|x| x - min).sum()
    }
}

#[cfg(test)]
mod tests {
    use crate::leetcode::solutions::minimum_moves_to_equal_array_elements::Solution;

    #[test]
    fn example1() {
        assert_eq!(Solution::min_moves(vec![1, 2, 3]), 3);
    }

    #[test]
    fn example2() {
        assert_eq!(Solution::min_moves(vec![1, 1, 1]), 0);
    }
}
