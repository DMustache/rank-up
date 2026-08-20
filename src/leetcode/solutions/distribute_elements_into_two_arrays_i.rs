struct Solution;

impl Solution {
    pub fn result_array(nums: Vec<i32>) -> Vec<i32> {
        if nums.len() <= 2 {
            return nums;
        }

        let mut first = vec![nums[0]];
        let mut second = vec![nums[1]];

        for num in nums.into_iter().skip(2) {
            if first.last() > second.last() {
                first.push(num);
                continue;
            }
            second.push(num);
        }

        first.extend(second);
        first
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(Solution::result_array(vec![2, 1, 3]), vec![2, 3, 1]);
    }

    #[test]
    fn example_2() {
        assert_eq!(Solution::result_array(vec![5, 4, 3, 8]), vec![5, 3, 4, 8]);
    }
}
