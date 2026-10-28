struct Solution;

impl Solution {
    pub fn min_xor(grid: Vec<Vec<i32>>) -> i32 {
        const XOR_VALUES: usize = 1024;
        let mut memo = vec![vec![[-1; XOR_VALUES]; grid[0].len()]; grid.len()];
        Self::dfs(&grid, 0, 0, grid[0][0] as usize, &mut memo) as i32
    }

    fn dfs(
        grid: &[Vec<i32>],
        row: usize,
        col: usize,
        xor: usize,
        memo: &mut [Vec<[i16; 1024]>],
    ) -> i16 {
        if row == grid.len() - 1 && col == grid[0].len() - 1 {
            return xor as i16;
        }
        if memo[row][col][xor] != -1 {
            return memo[row][col][xor];
        }

        let mut best = i16::MAX;
        if row + 1 < grid.len() {
            best = best.min(Self::dfs(
                grid,
                row + 1,
                col,
                xor ^ grid[row + 1][col] as usize,
                memo,
            ));
        }
        if col + 1 < grid[0].len() {
            best = best.min(Self::dfs(
                grid,
                row,
                col + 1,
                xor ^ grid[row][col + 1] as usize,
                memo,
            ));
        }

        memo[row][col][xor] = best;
        best
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn examples() {
        assert_eq!(Solution::min_xor(vec![vec![1, 2], vec![3, 4]]), 6);
        assert_eq!(Solution::min_xor(vec![vec![6, 7], vec![5, 8]]), 9);
        assert_eq!(Solution::min_xor(vec![vec![2, 7, 5]]), 0);
    }

    #[test]
    fn single_cell_and_single_column() {
        assert_eq!(Solution::min_xor(vec![vec![1023]]), 1023);
        assert_eq!(Solution::min_xor(vec![vec![2], vec![7], vec![5]]), 0);
    }

    #[test]
    fn larger_prefix_xor_can_give_smaller_result() {
        assert_eq!(Solution::min_xor(vec![vec![0, 1], vec![2, 3]]), 1);
    }

    #[test]
    fn longest_single_column() {
        assert_eq!(Solution::min_xor(vec![vec![1]; 1000]), 0);
    }
}
