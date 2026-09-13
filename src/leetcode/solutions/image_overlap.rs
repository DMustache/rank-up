struct Solution;

impl Solution {
    pub fn largest_overlap(img1: Vec<Vec<i32>>, img2: Vec<Vec<i32>>) -> i32 {
        use std::collections::HashMap;

        let mut shifts = HashMap::new();

        for (r1, row1) in img1.iter().enumerate() {
            for (c1, &value1) in row1.iter().enumerate() {
                if value1 == 0 {
                    continue;
                }

                for (r2, row2) in img2.iter().enumerate() {
                    for (c2, &value2) in row2.iter().enumerate() {
                        if value2 == 0 {
                            continue;
                        }

                        let shift = (r2 as i32 - r1 as i32, c2 as i32 - c1 as i32);

                        *shifts.entry(shift).or_insert(0) += 1;
                    }
                }
            }
        }

        shifts.into_values().max().unwrap_or(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            Solution::largest_overlap(
                vec![vec![1, 1, 0], vec![0, 1, 0], vec![0, 1, 0]],
                vec![vec![0, 0, 0], vec![0, 1, 1], vec![0, 0, 1]],
            ),
            3
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(Solution::largest_overlap(vec![vec![1]], vec![vec![1]]), 1);
    }

    #[test]
    fn example_3() {
        assert_eq!(Solution::largest_overlap(vec![vec![0]], vec![vec![0]]), 0);
    }
}
