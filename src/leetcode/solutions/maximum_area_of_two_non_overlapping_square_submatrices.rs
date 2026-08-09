struct Solution;
impl Solution {
    pub fn max_area(mat: Vec<Vec<i32>>) -> i32 {
        if mat.is_empty() || mat[0].is_empty() {
            return 0;
        }

        let rows = mat.len();
        let columns = mat[0].len();
        let prefix_sums = Self::build_prefix_sums(&mat, rows, columns);
        let mut smallest_possible_side = 0;
        let mut largest_possible_side = rows.min(columns);

        while smallest_possible_side < largest_possible_side {
            let side = (smallest_possible_side + largest_possible_side + 1) / 2;

            if Self::can_place_two_squares(&prefix_sums, rows, columns, side) {
                smallest_possible_side = side;
            } else {
                largest_possible_side = side - 1;
            }
        }

        (smallest_possible_side * smallest_possible_side) as i32
    }

    fn build_prefix_sums(mat: &[Vec<i32>], rows: usize, columns: usize) -> Vec<Vec<i32>> {
        let prefix_sums = vec![vec![0; columns + 1]; rows + 1];

        mat.iter()
            .enumerate()
            .fold(prefix_sums, |mut prefix_sums, (row, values)| {
                values.iter().enumerate().for_each(|(column, &value)| {
                    let above = prefix_sums[row][column + 1];
                    let left = prefix_sums[row + 1][column];
                    let diagonal = prefix_sums[row][column];

                    prefix_sums[row + 1][column + 1] = value + above + left - diagonal;
                });
                prefix_sums
            })
    }

    fn can_place_two_squares(
        prefix_sums: &[Vec<i32>],
        rows: usize,
        columns: usize,
        side: usize,
    ) -> bool {
        let required_sum = (side * side) as i32;
        let mut minimum_row = rows;
        let mut maximum_row = 0;
        let mut minimum_column = columns;
        let mut maximum_column = 0;
        let mut found_square = false;

        for row in 0..=rows - side {
            for column in 0..=columns - side {
                let square_sum = prefix_sums[row + side][column + side]
                    - prefix_sums[row][column + side]
                    - prefix_sums[row + side][column]
                    + prefix_sums[row][column];

                if square_sum == required_sum {
                    found_square = true;
                    minimum_row = minimum_row.min(row);
                    maximum_row = maximum_row.max(row);
                    minimum_column = minimum_column.min(column);
                    maximum_column = maximum_column.max(column);
                }
            }
        }

        found_square
            && (maximum_row - minimum_row >= side || maximum_column - minimum_column >= side)
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example1() {
        assert_eq!(
            Solution::max_area(vec![vec![1, 1, 1, 0], vec![1, 1, 1, 1], vec![0, 0, 1, 1],]),
            4
        );
    }

    #[test]
    fn example2() {
        assert_eq!(Solution::max_area(vec![vec![0, 1], vec![1, 0]]), 1);
    }

    #[test]
    fn example3() {
        assert_eq!(Solution::max_area(vec![vec![0, 0], vec![0, 1]]), 0);
    }
}
