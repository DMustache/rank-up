struct Solution;

impl Solution {
    pub fn unique_paths(grid: Vec<Vec<i32>>) -> i32 {
        const MOD: i64 = 1_000_000_007;

        let rows = grid.len();
        let cols = grid[0].len();
        let mut down = vec![0_i64; cols];
        down[0] = 1;

        for row in 0..rows {
            let mut right = 0;
            for col in 0..cols {
                let from_above = down[col];
                let from_left = right;

                if row == rows - 1 && col == cols - 1 {
                    return ((from_above + from_left) % MOD) as i32;
                }

                if grid[row][col] == 1 {
                    right = from_above;
                    down[col] = from_left;
                } else {
                    let ways = (from_above + from_left) % MOD;
                    right = ways;
                    down[col] = ways;
                }
            }
        }

        unreachable!()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn examples() {
        assert_eq!(
            Solution::unique_paths(vec![vec![0, 1, 0], vec![0, 0, 1], vec![1, 0, 0]]),
            5
        );
        assert_eq!(Solution::unique_paths(vec![vec![0, 0], vec![0, 0]]), 2);
        assert_eq!(
            Solution::unique_paths(vec![vec![0, 1, 1], vec![1, 1, 0]]),
            1
        );
    }

    #[test]
    fn reflection_off_the_grid_is_not_counted() {
        assert_eq!(
            Solution::unique_paths(vec![vec![0, 0, 0], vec![0, 1, 0]]),
            2
        );
    }

    #[test]
    fn count_is_reduced_modulo() {
        assert_eq!(Solution::unique_paths(vec![vec![0; 20]; 20]), 345_263_555);
    }
}
