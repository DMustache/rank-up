struct Solution;

impl Solution {
    pub fn result_array(nums: Vec<i32>, k: i32) -> Vec<i64> {
        let k = k as usize;
        let mut result = vec![0; k];
        let mut dp = vec![0; k];

        for num in nums {
            let value = (num as usize) % k;
            let mut next = vec![0; k];
            next[value] += 1;

            for (old_rem, _) in dp.iter().enumerate().take(k) {
                let new_rem = (old_rem * value) % k;
                next[new_rem] += dp[old_rem];
            }

            for rem in 0..k {
                result[rem] += next[rem];
            }

            dp = next;
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(
            Solution::result_array(vec![1, 2, 3, 4, 5], 3),
            vec![9, 2, 4]
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            Solution::result_array(vec![1, 2, 4, 8, 16, 32], 4),
            vec![18, 1, 2, 0]
        );
    }

    #[test]
    fn example3() {
        assert_eq!(Solution::result_array(vec![1, 1, 2, 1, 1], 2), vec![9, 6]);
    }
}
