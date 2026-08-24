struct Solution;

impl Solution {
    pub fn stone_game_viii(stones: Vec<i32>) -> i32 {
        let mut prefix_sums = stones;

        for index in 1..prefix_sums.len() {
            prefix_sums[index] += prefix_sums[index - 1];
        }

        let mut best_difference = prefix_sums[prefix_sums.len() - 1];

        for index in (1..prefix_sums.len() - 1).rev() {
            best_difference = best_difference.max(prefix_sums[index] - best_difference);
        }

        best_difference
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let stones = vec![-1, 2, -3, 4, -5];

        assert_eq!(5, Solution::stone_game_viii(stones));
    }

    #[test]
    fn example_2() {
        let stones = vec![7, -6, 5, 10, 5, -2, -6];

        assert_eq!(13, Solution::stone_game_viii(stones));
    }

    #[test]
    fn example_3() {
        let stones = vec![-10, -12];

        assert_eq!(-22, Solution::stone_game_viii(stones));
    }
}
