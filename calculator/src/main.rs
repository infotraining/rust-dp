use crate::{
    builder::ExpressionBuilder, factory::{NumberToken, ScientificFactory, StandardFactory, TokenFactory}, token::{Function, Operator, Token}
};

mod factory;
mod token;
mod builder;

// struct Calulator<F: TokenFactory> {
//     factory: F,
//     expression: Vec<Token>
// }

// impl<F: TokenFactory> Calulator<F> {
//     fn new(factory: F) -> Self {
//         Self {
//             factory,
//             expression: Vec::new(),
//         }
//     }

//     fn parse(&mut self, input: &str) -> Result<(), String> {
//         for token in input.split_whitespace() {
//             // try operator first
//             if let Ok(op) = self.factory.create_operator(token) {
//                 self.expression.push(Token::Operator(op));
//                 continue;
//             }

//             // must be a number then
//             let number = self.factory.create_number(token)?;
//             self.expression.push(Token::Number(number));
//         }

//         Ok(())
//     }
// }

fn main() {
    let num_token = Token::number(42.0);
    let op_token = Token::operator(Operator::Add);
    let func_token = Token::function(Function::Sin);
    let var_token = Token::variable("x");

    let tokens = vec![num_token, op_token, func_token, var_token];

    println!("Tokens: {tokens:?}");

    // Demonstrate Factory from string
    match Token::from_str("3.14") {
        Ok(token) => println!("Parsed number: {:?}", token),
        Err(e) => println!("Error: {}", e),
    }

    // Demonstrate Abstract Factory
    let standard_factory = StandardFactory;
    let sci_factory = ScientificFactory;
    
    let standard_num = standard_factory.create_number("123").unwrap();
    let sci_num = sci_factory.create_number("1.23e-4").unwrap();
    
    println!("Standard number: {}", standard_num.format());
    println!("Scientific number: {}", sci_num.format());

    // Demonstrate Builder pattern
    let expr = ExpressionBuilder::new()
        .number(2.0)
        .operator(Operator::Add)
        .open_paren()
        .number(3.0)
        .operator(Operator::Multiply)
        .number(4.0)
        .close_paren().unwrap()  // close_paren returns Result<Self, String>
        .build().unwrap();
    
    println!("Built expression: {:?}", expr);
}
