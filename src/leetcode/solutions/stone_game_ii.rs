struct Solution;

impl Solution {
    pub fn stone_game_ii(piles: Vec<i32>) -> i32 {
        let pile_count = piles.len();
        if pile_count == 0 {
            return 0;
        }

        let mut suffix_sums = vec![0; pile_count + 1];
        for index in (0..pile_count).rev() {
            suffix_sums[index] = suffix_sums[index + 1] + piles[index];
        }

        let mut best = vec![vec![0; pile_count + 1]; pile_count + 1];
        for index in (0..pile_count).rev() {
            for max_take in 1..=pile_count {
                if index + 2 * max_take >= pile_count {
                    best[index][max_take] = suffix_sums[index];
                    continue;
                }

                for take in 1..=2 * max_take {
                    best[index][max_take] = best[index][max_take]
                        .max(suffix_sums[index] - best[index + take][max_take.max(take)]);
                }
            }
        }

        best[0][1]
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example1() {
        assert_eq!(Solution::stone_game_ii(vec![2, 7, 9, 4, 4]), 10);
    }

    #[test]
    fn example2() {
        assert_eq!(Solution::stone_game_ii(vec![1, 2, 3, 4, 5, 100]), 104);
    }

    #[test]
    fn single_pile() {
        assert_eq!(Solution::stone_game_ii(vec![7]), 7);
    }

    #[test]
    fn taking_all_piles_is_optimal() {
        assert_eq!(Solution::stone_game_ii(vec![1, 2]), 3);
    }
}
