#[macro_use]
pub mod error;
pub mod lexer;
pub mod parser;

#[cfg(test)]
mod test {
    use crate::lexer::{Lexer, LexerInput};
    use crate::parser::{Expression, ParserInput, Statement};
    use std::time::Instant;

    #[test]
    fn lexer() {
        let lexer = Lexer::new(include_str!("example.owl"));
        let start = Instant::now();
        let tokens = lexer.collect::<Vec<_>>();
        println!("Done in {:?}", start.elapsed());
        println!("{:?}", tokens);
    }

    #[test]
    fn parser() {
        let lexer = Lexer::new(include_str!("example.owl"));
        let mut lexer_input = lexer.input.clone();
        let mut parser_input = ParserInput::from(lexer);
        while let Some(statement) = Statement::parse(&mut parser_input, &mut lexer_input) {
            println!("{:?}", statement);
        }
    }
}
