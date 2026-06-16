#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Number(Number),
    Operator(Operator),
    Function(Function),
    Variable(String),
    OpenParen,
    CloseParen,
}

impl Token {
    pub fn number(value: f64) -> Self {
        Self::Number(Number {
            value,
            format: NumberFormat::Decimal,
        })
    }

    pub fn operator(op: Operator) -> Self {
        Self::Operator(op)
    }

    pub fn function(func: Function) -> Self {
        Self::Function(func)
    }

    pub fn variable(name: impl Into<String>) -> Self {
        Self::Variable(name.into())
    }

    pub fn from_str(s: &str) -> Result<Self, String> {
        if let Ok(num) = s.parse::<f64>() {
            return Ok(Self::number(num));
        }

        match s {
            "+" => Ok(Self::Operator(Operator::Add)),
            "-" => Ok(Self::Operator(Operator::Subtract)),
            "*" => Ok(Self::Operator(Operator::Multiply)),
            "/" => Ok(Self::Operator(Operator::Divide)),

            "sqrt" => Ok(Self::Function(Function::Sqrt)),
            "sin" => Ok(Self::Function(Function::Sin)),
            "cos" => Ok(Self::Function(Function::Cos)),
            "tan" => Ok(Self::Function(Function::Tan)),

            name if name.chars().all(|c| c.is_alphanumeric() || c == '_') => {
                Ok(Self::variable(name))
            }
            _ => Err(format!("Invalid token: {s}")),
        }
    }

    pub fn operator_from_str(s: &str) -> Result<Self, String> {
        let op = match s {
            "+" => Operator::Add,
            "-" => Operator::Subtract,
            "*" => Operator::Multiply,
            "/" => Operator::Divide,
            "**" | "^" => Operator::Power,
            "///" => Operator::Root,
            "!" => Operator::Factorial,
            _ => return Err(format!("Invalid operator: {s}")),
        };

        Ok(Self::Operator(op))
    }

    pub fn number_from_str(s: &str) -> Result<Self, String> {
        if s.contains('e') || s.contains('E') {
            Self::scientific_number(s)
        } else if s.contains('.') {
            Self::decimal_number(s)
        } else {
            s.parse::<f64>()
                .map(|v| {
                    Self::Number(Number {
                        value: v,
                        format: NumberFormat::Decimal,
                    })
                })
                .map_err(|_| format!("Invalid number: {s}"))
        }
    }

    fn scientific_number(s: &str) -> Result<Self, String> {
        s.parse::<f64>()
            .map(|v| {
                Self::Number(Number {
                    value: v,
                    format: NumberFormat::Scientific,
                })
            })
            .map_err(|_| format!("Invalid scientific number: {s}"))
    }

    fn decimal_number(s: &str) -> Result<Self, String> {
        s.parse::<f64>()
            .map(|v| {
                Self::Number(Number {
                    value: v,
                    format: NumberFormat::Decimal,
                })
            })
            .map_err(|_| format!("Invalid decimal number: {s}"))
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Number {
    pub value: f64,
    pub format: NumberFormat,
}

impl Number {
    pub fn new(value: f64) -> Self {
        Self {
            value, 
            format: NumberFormat::Decimal
        }
    }

    pub fn with_format(value: f64, format: NumberFormat) -> Self {
        Self { value, format }
    }

    pub fn format(&self) -> String {
        match self.format {
            NumberFormat::Decimal => format!("{}", self.value),
            NumberFormat::Scientific => format!("{:e}", self.value),
            NumberFormat::Engineering => {
                // Engineering notation adjusts exponent to be multiple of 3
                let exp = self.value.abs().log10().floor();
                let adj_exp = (exp - exp % 3.0).floor();
                let coeff = self.value / 10_f64.powf(adj_exp);
                format!("{}e{}", coeff, adj_exp)
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum NumberFormat {
    Decimal,
    Scientific,
    Engineering,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Power,
    Root,
    Factorial,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Function {
    Sqrt,
    Sin,
    Cos,
    Tan,
}

pub struct TokenParser {
    input: String,
    position: usize,
}

impl TokenParser {
    pub fn new(input: impl Into<String>) -> Self {
        Self {
            input: input.into(),
            position: 0,
        }
    }

    pub fn next_token(&mut self) -> Option<Result<Token, String>> {
        self.skip_whitespace();

        if self.position >= self.input.len() {
            return None;
        }

        let remaining_input = &self.input[self.position..];

        if let Some(len) = self.match_operator(remaining_input) {
            let op_str = &remaining_input[..len];
            self.position += len;
            return Some(Token::operator_from_str(op_str));
        }

        Some(Err("Incomplete token parsing logic".to_string()))
    }

    fn skip_whitespace(&mut self) {
        while self.position < self.input.len()
            && self.input[self.position..].starts_with(char::is_whitespace)
        {
            self.position += 1;
        }
    }

    fn match_operator(&self, input: &str) -> Option<usize> {
        let operators = ["+", "-", "*", "/", "**", "^", "///", "!"];

        operators
            .iter()
            .filter(|op| input.starts_with(*op))
            .map(|op| op.len())
            .next()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn numbers_from_str() {
        let num1 = Token::number_from_str("3.14").unwrap();
        let num2 = Token::number_from_str("2e10").unwrap();
        let num3 = Token::number_from_str("1.5e-5").unwrap();

        assert_eq!(
            num1,
            Token::Number(Number {
                value: 3.14,
                format: NumberFormat::Decimal
            })
        );
        assert_eq!(
            num2,
            Token::Number(Number {
                value: 2e10,
                format: NumberFormat::Scientific
            })
        );
        assert_eq!(
            num3,
            Token::Number(Number {
                value: 1.5e-5,
                format: NumberFormat::Scientific
            })
        );
    }

    #[test]
    fn number_formatting() {
        let num1 = Number {
            value: 3.14,
            format: NumberFormat::Decimal,
        };
        let num2 = Number {
            value: 2e10,
            format: NumberFormat::Scientific,
        };
        let num3 = Number {
            value: 1.5e-5,
            format: NumberFormat::Engineering,
        };

        assert_eq!(num1.format(), "3.14");
        assert_eq!(num2.format(), "2e10");
        assert_eq!(num3.format(), "0.015e-3");
    }

    #[test]
    fn tokenizing_from_str() {
        let tokens: Result<Vec<_>, _> = "2 + 3".split_whitespace().map(Token::from_str).collect();

        assert_eq!(
            tokens,
            Ok(vec![
                Token::number(2.0),
                Token::operator(Operator::Add),
                Token::number(3.0)
            ])
        );
    }

    #[test]
    fn parsing_tokens() {
        let mut parser = TokenParser::new("2 + 3");
        let token1 = parser.next_token().unwrap().unwrap();
        let token2 = parser.next_token().unwrap().unwrap();
        let token3 = parser.next_token().unwrap().unwrap();

        assert_eq!(token1, Token::number(2.0));
        assert_eq!(token2, Token::operator(Operator::Add));
        assert_eq!(token3, Token::number(3.0));
    }

    #[test]
    fn simple_expression() {
        let expr = vec![
            Token::number(2.0),
            Token::operator(Operator::Add),
            Token::number(3.0),
        ];

        println!("expr = {expr:?}");
    }
}
