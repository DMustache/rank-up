struct Solution;

impl Solution {
    pub fn create_grid(m: i32, n: i32) -> Vec<String> {
        let mut grid = vec![vec!['#'; n as usize]; m as usize];
        let mut row_pos = 0;
        grid[0][0] = '.';
        (1..n as usize).for_each(|col| {
            grid[row_pos][col] = '.';
            if row_pos + 1 < m as usize {
                row_pos += 1;
                grid[row_pos][col] = '.';
            }
        });

        while row_pos + 1 < m as usize {
            row_pos += 1;
            grid[row_pos][n as usize - 1] = '.';
        }

        grid.into_iter()
            .map(|row| row.into_iter().collect())
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    fn assert_exactly_one_path(m: i32, n: i32) {
        let grid = Solution::create_grid(m, n);
        assert_eq!(grid.len(), m as usize);
        assert!(grid.iter().all(|row| row.len() == n as usize));

        let mut paths = vec![vec![0_u8; n as usize]; m as usize];
        for row in 0..m as usize {
            for col in 0..n as usize {
                match grid[row].as_bytes()[col] {
                    b'#' => continue,
                    b'.' => {}
                    cell => panic!("unexpected grid cell: {cell}"),
                }

                paths[row][col] = if row == 0 && col == 0 {
                    1
                } else {
                    (row.checked_sub(1).map_or(0, |above| paths[above][col])
                        + col.checked_sub(1).map_or(0, |left| paths[row][left]))
                    .min(2)
                };
            }
        }

        assert_eq!(paths[m as usize - 1][n as usize - 1], 1, "grid: {grid:?}");
    }

    #[test]
    fn example_1() {
        assert_exactly_one_path(2, 3);
        assert_eq!(Solution::create_grid(2, 3), ["..#", "#.."]);
    }

    #[test]
    fn example_2() {
        assert_exactly_one_path(3, 3);
        assert_eq!(Solution::create_grid(3, 3), ["..#", "#..", "##."]);
    }

    #[test]
    fn example_3() {
        assert_exactly_one_path(1, 4);
        assert_eq!(Solution::create_grid(1, 4), ["...."]);
    }
}
