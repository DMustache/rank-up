fn calc(expr: &str) -> f64 {
    Parser::new(expr).parse_expression()
}

struct Parser {
    chars: Vec<char>,
    position: usize,
}

impl Parser {
    fn new(expr: &str) -> Self {
        Self {
            chars: expr.chars().collect(),
            position: 0,
        }
    }

    fn parse_expression(&mut self) -> f64 {
        let mut result = self.parse_term();

        loop {
            self.skip_whitespace();

            match self.peek() {
                Some('+') => {
                    self.position += 1;
                    result += self.parse_term();
                }
                Some('-') => {
                    self.position += 1;
                    result -= self.parse_term();
                }
                _ => return result,
            }
        }
    }

    fn parse_term(&mut self) -> f64 {
        let mut result = self.parse_factor();

        loop {
            self.skip_whitespace();

            match self.peek() {
                Some('*') => {
                    self.position += 1;
                    result *= self.parse_factor();
                }
                Some('/') => {
                    self.position += 1;
                    result /= self.parse_factor();
                }
                _ => return result,
            }
        }
    }

    fn parse_factor(&mut self) -> f64 {
        self.skip_whitespace();

        match self.peek() {
            Some('+') => {
                self.position += 1;
                self.parse_factor()
            }
            Some('-') => {
                self.position += 1;
                -self.parse_factor()
            }
            Some('(') => {
                self.position += 1;
                let result = self.parse_expression();
                self.skip_whitespace();

                if self.peek() == Some(')') {
                    self.position += 1;
                }

                result
            }
            _ => self.parse_number(),
        }
    }

    fn parse_number(&mut self) -> f64 {
        self.skip_whitespace();

        let start = self.position;

        while matches!(self.peek(), Some('0'..='9' | '.')) {
            self.position += 1;
        }

        self.chars[start..self.position]
            .iter()
            .collect::<String>()
            .parse()
            .unwrap_or(0.0)
    }

    fn skip_whitespace(&mut self) {
        while self.peek() == Some(' ') {
            self.position += 1;
        }
    }

    fn peek(&self) -> Option<char> {
        self.chars.get(self.position).copied()
    }
}

#[cfg(test)]
mod tests {
    use super::calc;

    // Wrap custom message to reduce repitition
    macro_rules! assert_expr_eq {
        ($expr: expr, $expect: expr) => {
            assert_eq!(
                calc($expr),
                $expect,
                "\nexpected expression \"{}\" to equal \"{:?}\", but got \"{:?}\"",
                $expr,
                $expect,
                calc($expr),
            );
        };
    }

    #[test]
    fn single_values() {
        assert_expr_eq!("0", 0.0);
        assert_expr_eq!("1", 1.0);
        assert_expr_eq!("42", 42.0);
        assert_expr_eq!("350", 350.0);
    }

    #[test]
    fn basic_operations() {
        assert_expr_eq!("1 + 1", 2.0);
        assert_expr_eq!("1 - 1", 0.0);
        assert_expr_eq!("1 * 1", 1.0);
        assert_expr_eq!("1 / 1", 1.0);
        assert_expr_eq!("12 * 123", 1476.0);
    }

    #[test]
    fn whitespace_between_operators_and_operands() {
        assert_expr_eq!("1-1", 0.0);
        assert_expr_eq!("1 -1", 0.0);
        assert_expr_eq!("1- 1", 0.0);
        assert_expr_eq!("1* 1", 1.0);
    }

    #[test]
    fn unary_minuses() {
        assert_expr_eq!("1- -1", 2.0);
        assert_expr_eq!("1--1", 2.0);
        assert_expr_eq!("1 - -1", 2.0);
        assert_expr_eq!("-42", -42.0);
    }

    #[test]
    fn parentheses() {
        assert_expr_eq!("(1)", 1.0);
        assert_expr_eq!("((1))", 1.0);
        assert_expr_eq!("((80 - (19)))", 61.0);
    }

    #[test]
    fn multiple_operators() {
        assert_expr_eq!("12* 123/(-5 + 2)", -492.0);
        assert_expr_eq!("1 - -(-(-(-4)))", -3.0);
        assert_expr_eq!("2 /2+3 * 4.75- -6", 21.25);
        assert_expr_eq!("2 / (2 + 3) * 4.33 - -6", 7.732);
        assert_expr_eq!("(1 - 2) + -(-(-(-4)))", 3.0);
        assert_expr_eq!("((2.33 / (2.9+3.5)*4) - -6)", 7.45625);
    }
}
