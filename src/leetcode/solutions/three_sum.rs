struct Solution;

impl Solution {
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result = Vec::new();

        nums.sort_unstable();

        for i in 0..nums.len() {
            if i > 0 && nums[i] == nums[i - 1] {
                continue;
            }

            let mut left = i + 1;
            let mut right = nums.len() - 1;

            while left < right {
                let total = nums[i] + nums[left] + nums[right];

                if total == 0 {
                    result.push(vec![nums[i], nums[left], nums[right]]);
                    left += 1;
                    right -= 1;

                    while left < right && nums[left] == nums[left - 1] {
                        left += 1;
                    }

                    while left < right && nums[right] == nums[right + 1] {
                        right -= 1;
                    }
                } else if total < 0 {
                    left += 1;
                } else {
                    right -= 1;
                }
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let case = vec![-1, 0, 1, 2, -1, -4];

        assert_eq!(
            vec![vec![-1, -1, 2], vec![-1, 0, 1]],
            Solution::three_sum(case)
        );
    }

    #[test]
    fn example2() {
        let case = vec![0, 1, 1];
        let result: Vec<Vec<i32>> = vec![];

        assert_eq!(result, Solution::three_sum(case));
    }

    #[test]
    fn example3() {
        let case = vec![0, 0, 0];

        assert_eq!(vec![vec![0, 0, 0]], Solution::three_sum(case));
    }
}
