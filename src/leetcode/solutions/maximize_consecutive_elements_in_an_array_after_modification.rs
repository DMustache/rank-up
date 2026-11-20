use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn max_selected_elements(mut nums: Vec<i32>) -> i32 {
        nums.sort_unstable();

        let mut dp = HashMap::new();
        let mut result = 0;

        for num in nums {
            let at_prev = dp.get(&(num - 1)).copied().unwrap_or(0);
            let at_num = dp.get(&num).copied().unwrap_or(0);

            {
                let inc = dp.entry(num + 1).or_default();
                *inc = (*inc).max(at_num + 1);
                result = result.max(*inc);
            }
            {
                let keep = dp.entry(num).or_default();
                *keep = (*keep).max(at_prev + 1);
                result = result.max(*keep);
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example1() {
        let nums: Vec<i32> = vec![2, 1, 5, 1, 1];
        assert_eq!(Solution::max_selected_elements(nums), 3);
    }

    #[test]
    fn example2() {
        let nums: Vec<i32> = vec![1, 4, 7, 10];
        assert_eq!(Solution::max_selected_elements(nums), 1);
    }
}
