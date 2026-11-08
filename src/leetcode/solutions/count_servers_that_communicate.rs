struct Solution;

impl Solution {
    pub fn count_servers(grid: Vec<Vec<i32>>) -> i32 {
        let mut result = 0;

        let x_len = grid[0].len();
        let y_len = grid.len();

        for y in 0..y_len {
            for x in 0..x_len {
                if grid[y][x] == 0 {
                    continue;
                }

                if (0..x_len).any(|other_x| other_x != x && grid[y][other_x] == 1)
                    || (0..y_len).any(|other_y| other_y != y && grid[other_y][x] == 1)
                {
                    result += 1;
                }
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_1_no_communicating_servers() {
        assert_eq!(Solution::count_servers(vec![vec![1, 0], vec![0, 1]]), 0);
    }

    #[test]
    fn example_2_all_servers_communicate() {
        assert_eq!(Solution::count_servers(vec![vec![1, 0], vec![1, 1]]), 3);
    }

    #[test]
    fn example_3_one_isolated_server() {
        assert_eq!(
            Solution::count_servers(vec![
                vec![1, 1, 0, 0],
                vec![0, 0, 1, 0],
                vec![0, 0, 1, 0],
                vec![0, 0, 0, 1],
            ]),
            4
        );
    }

    #[test]
    fn servers_in_same_row_communicate_across_empty_cells() {
        assert_eq!(Solution::count_servers(vec![vec![1, 0, 0, 1]]), 2);
    }

    #[test]
    fn servers_in_same_column_communicate_across_empty_cells() {
        assert_eq!(
            Solution::count_servers(vec![vec![1], vec![0], vec![0], vec![1]]),
            2
        );
    }

    #[test]
    fn single_server_cannot_communicate() {
        assert_eq!(Solution::count_servers(vec![vec![1]]), 0);
    }
}
