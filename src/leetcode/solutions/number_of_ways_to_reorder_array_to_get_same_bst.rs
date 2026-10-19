struct Solution;

const MOD: i64 = 1_000_000_007;

impl Solution {
    pub fn num_of_ways(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut combinations = vec![vec![0_i64; n + 1]; n + 1];
        for row in 0..=n {
            combinations[row][0] = 1;
            combinations[row][row] = 1;
            for column in 1..row {
                combinations[row][column] =
                    (combinations[row - 1][column - 1] + combinations[row - 1][column]) % MOD;
            }
        }

        ((Self::count_orders(&nums, &combinations) - 1 + MOD) % MOD) as i32
    }

    fn count_orders(nums: &[i32], combinations: &[Vec<i64>]) -> i64 {
        if nums.len() <= 1 {
            return 1;
        }

        let root = nums[0];
        let mut left = Vec::new();
        let mut right = Vec::new();
        for &value in &nums[1..] {
            if value < root {
                left.push(value);
            } else {
                right.push(value);
            }
        }

        let interleavings = combinations[nums.len() - 1][left.len()];
        interleavings * Self::count_orders(&left, combinations) % MOD
            * Self::count_orders(&right, combinations)
            % MOD
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        assert_eq!(Solution::num_of_ways(vec![2, 1, 3]), 1);
        assert_eq!(Solution::num_of_ways(vec![3, 4, 5, 1, 2]), 5);
        assert_eq!(Solution::num_of_ways(vec![1, 2, 3]), 0);
    }

    #[test]
    fn balanced_subtrees() {
        assert_eq!(Solution::num_of_ways(vec![4, 2, 1, 3, 6, 5, 7]), 79);
    }

    #[test]
    fn example() {
        assert_eq!(
            Solution::num_of_ways(vec![
                9, 4, 2, 1, 3, 6, 5, 7, 8, 14, 11, 10, 12, 13, 16, 15, 17, 18
            ]),
            216212978
        );
    }
}
