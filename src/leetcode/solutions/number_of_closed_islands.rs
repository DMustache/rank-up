struct Solution;

impl Solution {
    pub fn closed_island(mut grid: Vec<Vec<i32>>) -> i32 {
        let rows = grid.len();
        let cols = grid[0].len();
        let mut closed = 0;

        for row in 0..rows {
            for col in 0..cols {
                if grid[row][col] != 0 {
                    continue;
                }

                let mut stack = vec![(row, col)];
                let mut touches_border = false;
                grid[row][col] = 1;

                while let Some((r, c)) = stack.pop() {
                    if r == 0 || r == rows - 1 || c == 0 || c == cols - 1 {
                        touches_border = true;
                    }

                    for (nr, nc) in [
                        (r.wrapping_sub(1), c),
                        (r + 1, c),
                        (r, c.wrapping_sub(1)),
                        (r, c + 1),
                    ] {
                        if nr < rows && nc < cols && grid[nr][nc] == 0 {
                            grid[nr][nc] = 1;
                            stack.push((nr, nc));
                        }
                    }
                }

                if !touches_border {
                    closed += 1;
                }
            }
        }

        closed
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1() {
        let grid = vec![
            vec![1, 1, 1, 1, 1, 1, 1, 0],
            vec![1, 0, 0, 0, 0, 1, 1, 0],
            vec![1, 0, 1, 0, 1, 1, 1, 0],
            vec![1, 0, 0, 0, 0, 1, 0, 1],
            vec![1, 1, 1, 1, 1, 1, 1, 0],
        ];
        assert_eq!(Solution::closed_island(grid), 2);
    }

    #[test]
    fn example_2() {
        let grid = vec![
            vec![0, 0, 1, 0, 0],
            vec![0, 1, 0, 1, 0],
            vec![0, 1, 1, 1, 0],
        ];
        assert_eq!(Solution::closed_island(grid), 1);
    }

    #[test]
    fn example_3() {
        let grid = vec![
            vec![1, 1, 1, 1, 1, 1, 1],
            vec![1, 0, 0, 0, 0, 0, 1],
            vec![1, 0, 1, 1, 1, 0, 1],
            vec![1, 0, 1, 0, 1, 0, 1],
            vec![1, 0, 1, 1, 1, 0, 1],
            vec![1, 0, 0, 0, 0, 0, 1],
            vec![1, 1, 1, 1, 1, 1, 1],
        ];
        assert_eq!(Solution::closed_island(grid), 2);
    }
}
