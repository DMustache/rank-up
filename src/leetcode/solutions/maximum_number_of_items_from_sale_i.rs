struct Solution;

impl Solution {
    pub fn maximum_sale_items(items: Vec<Vec<i32>>, budget: i32) -> i32 {
        let free_items: std::collections::HashMap<usize, Vec<usize>> = items
            .iter()
            .enumerate()
            .map(|(i, item)| {
                let free = items
                    .iter()
                    .enumerate()
                    .filter_map(|(j, other)| (i != j && other[0] % item[0] == 0).then_some(j))
                    .collect();
                (i, free)
            })
            .collect();

        let budget = budget as usize;
        let cheapest = items.iter().map(|item| item[1]).min().unwrap();
        let mut dp = vec![0; budget + 1];

        for (i, item) in items.iter().enumerate() {
            let price = item[1] as usize;
            let first_copy = 1 + free_items[&i].len() as i32;
            for spent in (price..=budget).rev() {
                dp[spent] = dp[spent].max(dp[spent - price] + first_copy);
            }
        }

        dp.into_iter()
            .enumerate()
            .map(|(spent, count)| count + (budget - spent) as i32 / cheapest)
            .max()
            .unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn example() {
        let items: Vec<Vec<i32>> = vec![vec![6, 2], vec![2, 6], vec![3, 4]];
        let budget: i32 = 9;

        assert_eq!(Solution::maximum_sale_items(items, budget), 4);
    }

    #[test]
    fn example2() {
        let items: Vec<Vec<i32>> =
            vec![vec![2, 4], vec![3, 2], vec![4, 1], vec![6, 4], vec![12, 4]];
        let budget: i32 = 8;

        assert_eq!(Solution::maximum_sale_items(items, budget), 10);
    }

    #[test]
    fn duplicate_factors_are_separate_items() {
        assert_eq!(
            Solution::maximum_sale_items(vec![vec![2, 2], vec![2, 3]], 5),
            4
        );
    }

    #[test]
    fn extra_copies_of_cheapest_item() {
        assert_eq!(
            Solution::maximum_sale_items(vec![vec![2, 2], vec![4, 5]], 6),
            4
        );
    }
}
