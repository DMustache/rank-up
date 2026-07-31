struct Solution;

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        if strs.is_empty() {
            return String::new();
        }

        let mut result = String::new();
        let first = strs.first().unwrap().as_bytes();

        let mut pointer = 0;

        while pointer < first.len() {
            let prefix_char = first[pointer];

            for word in &strs {
                if word.len() <= pointer {
                    return result;
                }

                if word.as_bytes()[pointer] != prefix_char {
                    return result;
                }
            }

            result.push(prefix_char as char);
            pointer += 1;
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let case = ["dog".to_string(), "racecar".to_string(), "car".to_string()].to_vec();
        assert_eq!("".to_string(), Solution::longest_common_prefix(case));
    }

    #[test]
    fn example2() {
        let case = [
            "flower".to_string(),
            "flow".to_string(),
            "flight".to_string(),
        ]
        .to_vec();
        assert_eq!("fl".to_string(), Solution::longest_common_prefix(case));
    }
}
