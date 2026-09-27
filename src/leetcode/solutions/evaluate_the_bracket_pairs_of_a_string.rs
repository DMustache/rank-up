use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn evaluate(s: String, knowledge: Vec<Vec<String>>) -> String {
        let knowledge: HashMap<String, String> = knowledge
            .into_iter()
            .map(|item| (item[0].clone(), item[1].clone()))
            .collect::<HashMap<String, String>>();

        let mut result = String::new();
        let mut chars = s.chars();

        while let Some(ch) = chars.next() {
            if ch != '(' {
                result.push(ch);
                continue;
            }

            let mut key = String::new();
            for ch in chars.by_ref() {
                if ch == ')' {
                    break;
                }
                key.push(ch);
            }

            result.push_str(knowledge.get(&key).map(String::as_str).unwrap_or("?"));
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn evaluates_known_keys() {
        assert_eq!(
            Solution::evaluate(
                "(name)is(age)yearsold".to_string(),
                vec![
                    vec!["name".to_string(), "bob".to_string()],
                    vec!["age".to_string(), "two".to_string()],
                ],
            ),
            "bobistwoyearsold"
        );
    }

    #[test]
    fn replaces_unknown_keys_with_question_marks() {
        assert_eq!(
            Solution::evaluate(
                "hi(name)".to_string(),
                vec![vec!["a".to_string(), "b".to_string()]],
            ),
            "hi?"
        );
    }

    #[test]
    fn evaluates_repeated_keys_without_touching_plain_text() {
        assert_eq!(
            Solution::evaluate(
                "(a)(a)(a)aaa".to_string(),
                vec![vec!["a".to_string(), "yes".to_string()]],
            ),
            "yesyesyesaaa"
        );
    }
}
