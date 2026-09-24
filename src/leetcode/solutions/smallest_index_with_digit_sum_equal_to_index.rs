struct Solution;

impl Solution {
    pub fn smallest_index(nums: Vec<i32>) -> i32 {
        for (index, &num) in nums.iter().enumerate() {
            let mut value = num;
            let mut digit_sum = 0;

            while value > 0 {
                digit_sum += value % 10;
                value /= 10;
            }

            if digit_sum == index as i32 {
                return index as i32;
            }
        }

        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(Solution::smallest_index(vec![1, 3, 2]), 2);
    }

    #[test]
    fn example2_returns_the_smallest_matching_index() {
        assert_eq!(Solution::smallest_index(vec![1, 10, 11]), 1);
    }

    #[test]
    fn example3_returns_negative_one_when_there_is_no_match() {
        assert_eq!(Solution::smallest_index(vec![1, 2, 3]), -1);
    }

    #[test]
    fn zero_matches_index_zero() {
        assert_eq!(Solution::smallest_index(vec![0]), 0);
    }

    #[test]
    fn zero_digits_are_included_in_the_sum() {
        assert_eq!(Solution::smallest_index(vec![1000, 0, 20]), 2);
    }
}
