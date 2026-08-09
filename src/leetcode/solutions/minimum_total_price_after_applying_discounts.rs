struct Solution;

impl Solution {
    pub fn min_price(mut prices: Vec<i32>, mut discounts: Vec<i32>) -> f64 {
        prices.sort_unstable();
        discounts.sort_unstable();

        let mut prices_pointer = prices.len().checked_sub(1);
        let mut discount_pointer = discounts.len().checked_sub(1);

        let mut result = 0.;
        let mut apply_discount =
            |price, discount| result += (price as f64 * (100. - discount as f64)) / 100.;

        while let (Some(price_index), Some(discount_index)) = (prices_pointer, discount_pointer) {
            apply_discount(prices[price_index], discounts[discount_index]);
            prices_pointer = price_index.checked_sub(1);
            discount_pointer = discount_index.checked_sub(1);
        }

        while let Some(price_index) = prices_pointer {
            result += prices[price_index] as f64;
            prices_pointer = price_index.checked_sub(1);
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example1() {
        assert_eq!(Solution::min_price(vec![10, 30, 21], vec![50, 60]), 32.5);
    }

    #[test]
    fn example2() {
        assert_eq!(Solution::min_price(vec![100, 70], vec![10, 40, 50]), 92.0);
    }

    #[test]
    fn example3() {
        assert_eq!(Solution::min_price(vec![7, 3, 9], vec![100, 100]), 3.0);
    }
}
