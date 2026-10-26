struct Solution;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn count_islands(mut grid: Vec<Vec<i32>>, k: i32) -> i32 {
        let rows = grid.len();
        if rows == 0 {
            return 0;
        }
        let cols = grid[0].len();
        let mut count = 0;
        for row in 0..rows {
            for col in 0..cols {
                if grid[row][col] == 0 {
                    continue;
                }

                let mut stack = vec![(row, col)];
                let mut total = i64::from(grid[row][col]);
                grid[row][col] = 0;

                while let Some((r, c)) = stack.pop() {
                    for (nr, nc) in [
                        (r.wrapping_sub(1), c),
                        (r + 1, c),
                        (r, c.wrapping_sub(1)),
                        (r, c + 1),
                    ] {
                        if nr < rows && nc < cols && grid[nr][nc] != 0 {
                            total += i64::from(grid[nr][nc]);
                            grid[nr][nc] = 0;
                            stack.push((nr, nc));
                        }
                    }
                }

                if total % i64::from(k) == 0 {
                    count += 1;
                }
            }
        }

        count
    }

    fn average_of_subtree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let (_, _, matches) = Self::dfs(&root);

        matches
    }

    fn dfs(node: &Option<Rc<RefCell<TreeNode>>>) -> (i32, i32, i32) {
        let Some(node) = node else {
            return (0, 0, 0);
        };

        let node = node.borrow();
        let (left_sum, left_count, left_matches) = Self::dfs(&node.left);
        let (right_sum, right_count, right_matches) = Self::dfs(&node.right);

        let sum = node.val + left_sum + right_sum;
        let count = 1 + left_count + right_count;
        let matches = left_matches + right_matches + if node.val == sum / count { 1 } else { 0 };

        (sum, count, matches)
    }
}

#[cfg(test)]
mod tests {
    use crate::leetcode::solutions::count_islands_with_total_value_divisible_by_k::Solution;

    #[test]
    fn example1() {
        assert_eq!(
            Solution::count_islands(
                vec![
                    vec![0, 2, 1, 0, 0],
                    vec![0, 5, 0, 0, 5],
                    vec![0, 0, 1, 0, 0],
                    vec![0, 1, 4, 7, 0],
                    vec![0, 2, 0, 0, 8]
                ],
                5
            ),
            2
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            Solution::count_islands(
                vec![vec![3, 0, 3, 0], vec![0, 3, 0, 3], vec![3, 0, 3, 0]],
                3
            ),
            6
        );
    }

    #[test]
    fn large_island_total() {
        assert_eq!(
            Solution::count_islands(vec![vec![1_500_000_000, 1_500_000_000]], 3),
            1
        );
    }

    #[test]
    fn diagonal_cells_are_separate_islands() {
        assert_eq!(Solution::count_islands(vec![vec![2, 0], vec![0, 3]], 2), 1);
    }
}
