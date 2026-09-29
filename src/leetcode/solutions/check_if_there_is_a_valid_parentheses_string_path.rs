struct Solution;

impl Solution {
    pub fn has_valid_path(grid: Vec<Vec<char>>) -> bool {
        let (rows_len, cols_len) = (grid.len(), grid.first().unwrap().len());

        if (rows_len + cols_len) % 2 == 0
            || grid[0][0] == ')'
            || grid[rows_len - 1][cols_len - 1] == '('
        {
            return false;
        }

        let mut memoization = vec![vec![vec![-1; rows_len + cols_len]; cols_len]; rows_len];
        Self::dfs(&grid, 0, 0, 1, &mut memoization)
    }

    fn dfs(
        grid: &Vec<Vec<char>>,
        row: usize,
        col: usize,
        balance: usize,
        memoization: &mut Vec<Vec<Vec<i8>>>,
    ) -> bool {
        let rows = grid.len();
        let cols = grid[0].len();
        if memoization[row][col][balance] != -1 {
            return memoization[row][col][balance] == 1;
        }

        if row == rows - 1 && col == cols - 1 {
            return balance == 0;
        }

        let visit = |next_row: usize, next_col: usize, memo: &mut Vec<Vec<Vec<i8>>>| {
            let next_balance: usize = if grid[next_row][next_col] == '(' {
                balance + 1
            } else if balance > 0 {
                balance - 1
            } else {
                return false;
            };

            Self::dfs(grid, next_row, next_col, next_balance, memo)
        };

        let found = (row + 1 < rows && visit(row + 1, col, memoization))
            || (col + 1 < cols && visit(row, col + 1, memoization));

        memoization[row][col][balance] = if found { 1 } else { 0 };
        found
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example1() {
        let grid = vec![
            vec!['(', '(', '('],
            vec![')', '(', ')'],
            vec!['(', '(', ')'],
            vec!['(', '(', ')'],
        ];

        assert!(Solution::has_valid_path(grid));
    }

    #[test]
    fn example2() {
        let grid = vec![vec![')', ')'], vec!['(', '(']];

        assert!(!Solution::has_valid_path(grid));
    }

    #[test]
    fn single_cell_cannot_form_valid_parentheses_string() {
        assert!(!Solution::has_valid_path(vec![vec!['(']]));
    }

    #[test]
    fn straight_path_can_form_valid_parentheses_string() {
        assert!(Solution::has_valid_path(vec![vec!['(', ')']]));
        assert!(Solution::has_valid_path(vec![vec!['('], vec![')']]));
    }

    #[test]
    fn path_must_never_have_negative_balance() {
        let grid = vec![vec!['(', ')'], vec![')', ')']];

        assert!(!Solution::has_valid_path(grid));
    }
}
