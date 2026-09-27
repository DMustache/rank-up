struct Solution;

impl Solution {
    pub fn reverse_parentheses(s: String) -> String {
        let mut result = String::with_capacity(s.len());

        let bracket_pairs: Vec<[usize; 2]> = {
            let mut stack = Vec::new();
            let mut completed_pairs: Vec<[usize; 2]> = Vec::new();
            for (index, byte) in s.bytes().enumerate() {
                match byte {
                    b'(' => {
                        stack.push([index, 0]);
                    }
                    b')' => {
                        let mut pair = stack.pop().unwrap();
                        pair[1] = index;
                        completed_pairs.push(pair);
                    }
                    _ => continue,
                }
            }
            completed_pairs
        };

        let mut partner = vec![0; s.len()];
        for [opening, closing] in bracket_pairs {
            partner[opening] = closing;
            partner[closing] = opening;
        }

        let mut index: isize = 0;
        let mut direction: isize = 1;

        while index >= 0 && (index as usize) < s.len() {
            match s.as_bytes()[index as usize] {
                b'(' | b')' => {
                    index = partner[index as usize] as isize;
                    direction = -direction;
                }

                value => result.push(value as char),
            }

            index += direction;
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn reverses_a_single_pair() {
        assert_eq!(Solution::reverse_parentheses("(abcd)".to_string()), "dcba");
    }

    #[test]
    fn reverses_from_the_innermost_pair_outwards() {
        assert_eq!(
            Solution::reverse_parentheses("(u(love)i)".to_string()),
            "iloveu"
        );
        assert_eq!(
            Solution::reverse_parentheses("(ed(et(oc))el)".to_string()),
            "leetcode"
        );
    }

    #[test]
    fn keeps_text_without_parentheses_unchanged() {
        assert_eq!(Solution::reverse_parentheses("abc".to_string()), "abc");
    }

    #[test]
    fn handles_text_before_and_after_parentheses() {
        assert_eq!(Solution::reverse_parentheses("a(bc)d".to_string()), "acbd");
    }

    #[test]
    fn handles_multiple_pairs() {
        assert_eq!(
            Solution::reverse_parentheses("(abc)(def)".to_string()),
            "cbafed"
        );
    }

    #[test]
    fn handles_nested_pairs_and_single_characters() {
        assert_eq!(
            Solution::reverse_parentheses("(a(bc)d)".to_string()),
            "dbca"
        );
        assert_eq!(Solution::reverse_parentheses("((ab))".to_string()), "ab");
        assert_eq!(Solution::reverse_parentheses("(a)".to_string()), "a");
    }
}
