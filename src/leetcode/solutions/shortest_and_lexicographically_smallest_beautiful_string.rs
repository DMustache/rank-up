struct Solution;

impl Solution {
    pub fn shortest_beautiful_substring(s: String, k: i32) -> String {
        let bytes = s.as_bytes();
        let mut left = 0;
        let mut right = 0;
        let mut ones = 0;
        let mut best = "";

        while right < bytes.len() {
            if bytes[right] == b'1' {
                ones += 1;
            }

            while left <= right && bytes[left] == b'0' {
                left += 1;
            }

            if ones == k {
                let candiate = &s[left..=right];

                if best.is_empty() || candiate.len() < best.len() || (candiate.len() == best.len() && candiate < best)
                {
                    best = candiate;
                }

                left += 1;
                ones -= 1;

                while left <= right && bytes[left] == b'0' {
                    left += 1;
                }
            }
            right += 1;
        }

        best.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let s = "100011001".to_string();
        let k = 3;

        assert_eq!("11001", Solution::shortest_beautiful_substring(s, k));
    }

    #[test]
    fn example_2() {
        let s = "1011".to_string();
        let k = 2;

        assert_eq!("11", Solution::shortest_beautiful_substring(s, k));
    }

    #[test]
    fn example_3() {
        let s = "000".to_string();
        let k = 1;

        assert_eq!("", Solution::shortest_beautiful_substring(s, k));
    }
}
