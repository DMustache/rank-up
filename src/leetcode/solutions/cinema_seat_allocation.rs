use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn max_number_of_families(n: i32, reserved_seats: Vec<Vec<i32>>) -> i32 {
        let mut total = n * 2;
        let mut row_masks = HashMap::<i32, i32>::new();

        for seat in reserved_seats.iter() {
            let row = seat[0];
            let collumn = seat[1];
            if (2..=9).contains(&collumn) {
                *row_masks.entry(row).or_default() |= 1 << (collumn - 2);
            }
        }
        for (_, &mask) in row_masks.iter() {
            let left_group = mask & 0b00001111;
            let right_group = mask & 0b11110000;
            let middle_group = mask & 0b00111100;

            let mut groups_available = 0;
            
            if left_group == 0 && right_group == 0 {
                groups_available = 2;
            } else if left_group == 0 || right_group == 0 || middle_group == 0 {
                groups_available = 1;
            }

            total -= 2 - groups_available;
        }
        total
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let n = 3;
        let reserved_seats = vec![
            vec![1, 2],
            vec![1, 3],
            vec![1, 8],
            vec![2, 6],
            vec![3, 1],
            vec![3, 10],
        ];
        assert_eq!(Solution::max_number_of_families(n, reserved_seats), 4);
    }

    #[test]
    fn test_example_2() {
        let n = 2;
        let reserved_seats = vec![vec![2, 1], vec![1, 8], vec![2, 6]];
        assert_eq!(Solution::max_number_of_families(n, reserved_seats), 2);
    }

    #[test]
    fn test_example_3() {
        let n = 4;
        let reserved_seats = vec![vec![4, 3], vec![1, 4], vec![4, 6], vec![1, 7]];
        assert_eq!(Solution::max_number_of_families(n, reserved_seats), 4);
    }
}
