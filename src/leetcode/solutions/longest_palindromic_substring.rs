struct Solution;

impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let s = s.chars().collect::<Vec<_>>();

        if s.is_empty() {
            return String::new();
        }

        let mut longest_palindrome = s[0].to_string();

        for i in 0..s.len() {
            let mut substring = s[i].to_string();
            let mut p = 1;

            while i >= p && i + p < s.len() && s[i - p] == s[i + p] {
                substring.insert(0, s[i - p]);
                let reversed = substring.chars().rev().skip(1).collect::<String>();
                let palindrome = format!("{substring}{reversed}");

                if palindrome.len() > longest_palindrome.len() {
                    longest_palindrome = palindrome;
                }

                p += 1;
            }

            if i + 1 < s.len() && s[i] == s[i + 1] {
                let mut substring = s[i].to_string();
                let mut p = 1;

                loop {
                    let reversed = substring.chars().rev().collect::<String>();
                    let palindrome = format!("{substring}{reversed}");

                    if palindrome.len() > longest_palindrome.len() {
                        longest_palindrome = palindrome;
                    }

                    if i < p || i + p + 1 >= s.len() || s[i - p] != s[i + p + 1] {
                        break;
                    }

                    substring.insert(0, s[i - p]);
                    p += 1;
                }
            }
        }

        longest_palindrome
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_longest_odd_palindrome() {
        let case = "babad".to_string();

        let valids = ["bab".to_string(), "aba".to_string()];
        assert!(valids.contains(&Solution::longest_palindrome(case)));
    }

    #[test]
    fn returns_longest_even_palindrome() {
        let case = "cbbd".to_string();

        assert_eq!("bb", Solution::longest_palindrome(case));
    }

    #[test]
    fn no_new_symbols() {
        let case = "aba".to_string();

        assert_eq!("aba", Solution::longest_palindrome(case))
    }
}
