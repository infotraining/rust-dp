use crate::token::{Function, Number, NumberFormat, Operator};

pub trait NumberToken {
    fn value(&self) -> f64;
    fn format(&self) -> String;
}

pub trait OperatorToken {
    fn precedence(&self) -> u8;
    fn symbol(&self) -> &'static str;
}

pub trait TokenFactory {
    type Number: NumberToken;
    type Operator: OperatorToken;

    fn create_number(&self, s: &str) -> Result<Self::Number, String>;
    fn create_operator(&self, s: &str) -> Result<Self::Operator, String>;
}

pub struct StandardNumber(f64);

impl NumberToken for StandardNumber {
    fn value(&self) -> f64 {
        self.0
    }

    fn format(&self) -> String {
        format!("{}", self.0)
    }
}

pub struct StandardOperator(pub Operator);

impl OperatorToken for StandardOperator {
    fn precedence(&self) -> u8 {
        match self.0 {
            Operator::Add | Operator::Subtract => 1,
            Operator::Multiply | Operator::Divide => 2,
            Operator::Power => 3,
            Operator::Root | Operator::Factorial => 4,
        }
    }

    fn symbol(&self) -> &'static str {
        match self.0 {
            Operator::Add => "+",
            Operator::Subtract => "-",
            Operator::Multiply => "*",
            Operator::Divide => "/",
            Operator::Power => "^",
            Operator::Factorial => "!",
            Operator::Root => "///"
        }
    }
}

pub struct StandardFactory;

impl TokenFactory for StandardFactory {
    type Number = StandardNumber;
    type Operator = StandardOperator;

    fn create_number(&self, s: &str) -> Result<Self::Number, String> {
        let value = s
            .parse::<f64>()
            .map_err(|_| format!("Invalid number: {s}"))?;
        Ok(StandardNumber(value))
    }

    fn create_operator(&self, s: &str) -> Result<Self::Operator, String> {
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

        Ok(StandardOperator(op))
    }
}

pub struct ScientificNumber(pub Number);

impl NumberToken for ScientificNumber {
    fn value(&self) -> f64 {
        self.0.value
    }

    fn format(&self) -> String {
        // Scientific calculator prefers scientific notation by default
        match self.0.format {
            NumberFormat::Decimal => format!("{:e}", self.0.value),
            _ => self.0.format(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ScientificOperator {
    Basic(Operator),
    Function(Function),
}

impl OperatorToken for ScientificOperator {
    fn precedence(&self) -> u8 {
        match self {
            ScientificOperator::Basic(op) => match op {
                Operator::Add | Operator::Subtract => 1,
                Operator::Multiply | Operator::Divide => 2,
                Operator::Power => 3,
                Operator::Factorial | Operator::Root => 4
            },
            ScientificOperator::Function(_) => 4,
        }
    }

    fn symbol(&self) -> &'static str {
        match self {
            ScientificOperator::Basic(op) => match op {
                Operator::Add => "+",
                Operator::Subtract => "-",
                Operator::Multiply => "*",
                Operator::Divide => "/",
                Operator::Power => "^",
                Operator::Factorial => "!",
                Operator::Root => "///"
            },
            ScientificOperator::Function(func) => match func {
                Function::Sin => "sin",
                Function::Cos => "cos",
                Function::Tan => "tan",
                Function::Sqrt => "sqrt",
            },
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ScientificFactory;

impl TokenFactory for ScientificFactory {
    type Number = ScientificNumber;
    type Operator = ScientificOperator;

    fn create_number(&self, s: &str) -> Result<Self::Number, String> {
        // Handle both scientific and standard notation
        match s.parse::<f64>() {
            Ok(value) => {
                let format = if s.contains('e') || s.contains('E') {
                    NumberFormat::Scientific
                } else {
                    NumberFormat::Decimal
                };
                Ok(ScientificNumber(Number::with_format(value, format)))
            }
            Err(_) => Err(format!("Invalid number: {}", s)),
        }
    }

    fn create_operator(&self, s: &str) -> Result<Self::Operator, String> {
        // Scientific calculator supports functions
        match s {
            "+" => Ok(ScientificOperator::Basic(Operator::Add)),
            "-" => Ok(ScientificOperator::Basic(Operator::Subtract)),
            "*" => Ok(ScientificOperator::Basic(Operator::Multiply)),
            "/" => Ok(ScientificOperator::Basic(Operator::Divide)),
            "^" => Ok(ScientificOperator::Basic(Operator::Power)),
            "sin" => Ok(ScientificOperator::Function(Function::Sin)),
            "cos" => Ok(ScientificOperator::Function(Function::Cos)),
            "tan" => Ok(ScientificOperator::Function(Function::Tan)),
            "sqrt" => Ok(ScientificOperator::Function(Function::Sqrt)),
            _ => Err(format!("Invalid operator: {}", s)),
        }
    }
}
