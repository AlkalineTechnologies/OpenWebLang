#[macro_use]
pub mod error;
pub mod lexer;

#[cfg(test)]
mod test {
    use crate::lexer::token::keyword::Keyword;
    use crate::lexer::token::Token;
    use crate::lexer::Lexer;
    use std::time::Instant;

    #[test]
    fn lexer() {
        let lexer = Lexer::new("fn main() { return true; }");
        let start = Instant::now();
        let tokens = lexer.collect::<Vec<_>>();
        println!("Done in {:?}", start.elapsed());
        println!("{:?}", tokens);
    }
}
