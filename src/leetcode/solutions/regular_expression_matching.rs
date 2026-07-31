struct Solution;

impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        let s = s.as_bytes();
        let p = p.as_bytes();

        let mut pattern = vec![vec![false; p.len() + 1]; s.len() + 1];
        pattern[0][0] = true;

        for j in 2..=p.len() {
            if p[j - 1] == b'*' {
                pattern[0][j] = pattern[0][j - 2];
            }
        }

        for i in 1..=s.len() {
            for j in 1..=p.len() {
                if p[j - 1] == s[i - 1] || p[j - 1] == b'.' {
                    pattern[i][j] = pattern[i - 1][j - 1];
                } else if p[j - 1] == b'*' {
                    pattern[i][j] = pattern[i][j - 2];

                    if p[j - 2] == s[i - 1] || p[j - 2] == b'.' {
                        pattern[i][j] = pattern[i][j] || pattern[i - 1][j];
                    }
                }
            }
        }

        pattern[s.len()][p.len()]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example() {
        let s = "aa".to_string();
        let p = "a".to_string();
        assert_eq!(false, Solution::is_match(s, p));
    }

    #[test]
    fn example2() {
        let s = "aa".to_string();
        let p = "a*".to_string();
        assert_eq!(true, Solution::is_match(s, p));
    }

    #[test]
    fn example3() {
        let s = "ab".to_string();
        let p = ".*".to_string();
        assert_eq!(true, Solution::is_match(s, p));
    }
}
