struct Solution;

impl Solution {
    pub fn stone_game_iii(stone_value: Vec<i32>) -> String {
        let stone_length = stone_value.len();

        let (mut item1, mut item2, mut item3) = (0, 0, 0);

        for i in (0..stone_length).rev() {
            let mut taken = 0;
            let mut best = i32::MIN;
            for count in 1..=3 {
                if i + count > stone_length {
                    break;
                }

                taken += stone_value[i + count - 1];

                let opponent_result = match count {
                    1 => item1,
                    2 => item2,
                    3 => item3,
                    _ => unreachable!(),
                };

                best = best.max(taken - opponent_result);
            }

            item3 = item2;
            item2 = item1;
            item1 = best;
        }

        match item1.cmp(&0) {
            std::cmp::Ordering::Less => "Bob",
            std::cmp::Ordering::Equal => "Tie",
            std::cmp::Ordering::Greater => "Alice",
        }
        .to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let case = vec![1, 2, 3, 7];

        assert_eq!("Bob".to_string(), Solution::stone_game_iii(case));
    }

    #[test]
    fn example2() {
        let case = vec![1, 2, 3, -9];

        assert_eq!("Alice".to_string(), Solution::stone_game_iii(case));
    }

    #[test]
    fn example3() {
        let case = vec![1, 2, 3, 6];

        assert_eq!("Tie".to_string(), Solution::stone_game_iii(case));
    }
}
