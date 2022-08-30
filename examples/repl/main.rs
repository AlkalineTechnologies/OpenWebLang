use std::io::Write;
use OpenWebLang::lexer::Lexer;
use OpenWebLang::parser::{Expression, ParserInput, Statement};

fn main() {
    println!("OpenWebLang REPL");
    loop {
        print!(">> ");
        std::io::stdout().flush().unwrap();
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let lexer = Lexer::new(input);
        let mut lexer_input = lexer.input.clone();
        let mut parser_input = ParserInput::from(lexer);
        while let Some(statement) = Statement::parse(&mut parser_input, &mut lexer_input) {
            match statement {
                Statement::Expression(Expression::FunctionCall(name, args)) => {
                    if name == vec!["exit"] {
                        return;
                    } else {
                        println!(
                            "{:?}",
                            Statement::Expression(Expression::FunctionCall(name, args))
                        );
                    }
                }
                _ => {
                    println!("{:?}", statement);
                }
            }
        }
    }
}
