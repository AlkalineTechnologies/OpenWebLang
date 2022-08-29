use crate::lexer::token::keyword::Keyword;
use crate::lexer::token::Token;
use crate::lexer::LexerInput;
use std::ops::Range;

pub struct ParserInput {
    tokens: Vec<(Token, Range<usize>)>,
    pos: usize,
}
impl ParserInput {
    fn next(&mut self) -> Option<(Token, Range<usize>)> {
        self.pos += 1;
        self.tokens.get(self.pos - 1).cloned()
    }
    fn peek<F>(&mut self, f: F) -> bool
    where
        F: Fn(&Token) -> bool,
    {
        self.tokens.get(self.pos).map_or(false, |t| f(&t.0))
    }
    fn rewind(&mut self) {
        self.pos -= 1;
    }
}
impl<T> From<T> for ParserInput
where
    T: Iterator<Item = (Token, Range<usize>)>,
{
    fn from(tokens: T) -> Self {
        ParserInput {
            tokens: tokens.collect(),
            pos: 0,
        }
    }
}
impl Clone for ParserInput {
    fn clone(&self) -> Self {
        ParserInput {
            tokens: self.tokens.clone(),
            pos: 0,
        }
    }
}

#[derive(Debug)]
pub enum Statement {
    FunctionDecl(
        String,
        Option<Expression>,
        Vec<(String, Expression)>,
        Expression,
    ),
    VariableDecl(String, Option<Expression>),
    Assign(Expression, Token, Expression),
    Expression(Expression),
}
impl Statement {
    pub fn parse(input: &mut ParserInput, lexer_input: &mut LexerInput) -> Option<Statement> {
        if input.peek(|t| matches!(t, Token::Keyword(Keyword::Function))) {
            input.next();
            let ident = input.next().unwrap_or_else(|| {
                error!(
                    lexer_input.clone(),
                    input
                        .next()
                        .map_or(input.tokens.last().unwrap().1.clone(), |t| t.1),
                    "Expected identifier"
                )
            });
            let ident_str = match ident.0 {
                Token::Identifier(s) => s,
                _ => {
                    error!(lexer_input.clone(), ident.1, "Expected identifier");
                }
            };
            if !input.peek(|t| matches!(t, Token::OpenParen)) {
                error!(
                    lexer_input.clone(),
                    input
                        .next()
                        .map_or(input.tokens.last().unwrap().1.clone(), |t| t.1),
                    "Expected opening paren"
                );
            }
            let mut params = Vec::new();
            input.next();
            while let Some((Token::Identifier(name), _)) = input.next() {
                if !input.peek(|t| matches!(t, Token::Colon)) {
                    error!(
                        lexer_input.clone(),
                        input
                            .next()
                            .map_or(input.tokens.last().unwrap().1.clone(), |t| t.1),
                        "Expected colon"
                    );
                }
                input.next();
                let param_type = Expression::parse(input, lexer_input).unwrap_or_else(|| {
                    error!(
                        lexer_input.clone(),
                        input
                            .next()
                            .map_or(input.tokens.last().unwrap().1.clone(), |t| t.1),
                        "Expected expression"
                    );
                });
                params.push((name, param_type));
                if input.peek(|t| matches!(t, Token::CloseParen)) {
                    break;
                }
                if !input.peek(|t| matches!(t, Token::Comma)) {
                    error!(
                        lexer_input.clone(),
                        input
                            .next()
                            .map_or(input.tokens.last().unwrap().1.clone(), |t| t.1),
                        "Expected comma"
                    );
                } else {
                    input.next();
                }
            }
            if !input.peek(|t| matches!(t, Token::CloseParen)) {
                error!(
                    lexer_input.clone(),
                    input
                        .next()
                        .map_or(input.tokens.last().unwrap().1.clone(), |t| t.1),
                    "Expected closing paren"
                );
            } else {
                input.next();
            }
            let return_type = if input.peek(|t| matches!(t, Token::Arrow)) {
                input.next();
                Some(Expression::parse(input, lexer_input).unwrap_or_else(|| {
                    error!(
                        lexer_input.clone(),
                        input
                            .next()
                            .map_or(input.tokens.last().unwrap().1.clone(), |t| t.1),
                        "Expected expression"
                    );
                }))
            } else {
                None
            };
            let expr = Expression::parse(input, lexer_input).unwrap_or_else(|| {
                error!(
                    lexer_input.clone(),
                    input
                        .next()
                        .map_or(input.tokens.last().unwrap().1.clone(), |t| t.1),
                    "Expected expression"
                );
            });
            Some(Statement::FunctionDecl(
                ident_str,
                return_type,
                params,
                expr,
            ))
        } else {
            let val = Statement::variable_decl(input, lexer_input);
            if input.peek(|t| matches!(t, Token::Semicolon)) {
                input.next();
                val
            } else {
                error!(
                    lexer_input.clone(),
                    input
                        .next()
                        .map_or(input.tokens.last().unwrap().1.clone(), |t| t.1),
                    "Expected semicolon"
                );
            }
        }
    }
    pub fn variable_decl(
        input: &mut ParserInput,
        lexer_input: &mut LexerInput,
    ) -> Option<Statement> {
        if input.peek(|t| matches!(t, Token::Keyword(Keyword::Let))) {
            input.next();
            let ident = input.next().unwrap_or_else(|| {
                error!(
                    lexer_input.clone(),
                    input
                        .next()
                        .map_or(input.tokens.last().unwrap().1.clone(), |t| t.1),
                    "Expected identifier"
                )
            });
            let ident_str = match ident.0 {
                Token::Identifier(s) => s,
                _ => {
                    error!(lexer_input.clone(), ident.1, "Expected identifier");
                }
            };
            if input.peek(Token::is_assign_op) {
                input.next();
                let expr = Expression::parse(input, lexer_input)
                    .unwrap_or_else(|| error!(lexer_input.clone(), ident.1, "Expected expression"));
                Some(Statement::VariableDecl(ident_str, Some(expr)))
            } else {
                Some(Statement::VariableDecl(ident_str, None))
            }
        } else {
            Statement::assign(input, lexer_input)
        }
    }
    pub fn assign(input: &mut ParserInput, lexer_input: &mut LexerInput) -> Option<Statement> {
        let left = Expression::parse(input, lexer_input)?;
        if input.peek(Token::is_assign_op) {
            let op = input.next().unwrap().0;
            let right = Expression::parse(input, lexer_input)?;
            Some(Statement::Assign(left, op, right))
        } else {
            Some(Statement::Expression(left))
        }
    }
}

#[derive(Debug)]
pub enum Expression {
    Binary(Box<Expression>, Token, Box<Expression>),
    Unary(Token, Box<Expression>),
    Block(Vec<Statement>),
    FunctionCall(Vec<String>, Vec<Expression>),
    Path(Vec<String>),
    StringLiteral(String),
    SignedLiteral(i64),
    UnsignedLiteral(u64),
    FloatLiteral(f64),
}
impl Expression {
    pub fn parse(input: &mut ParserInput, lexer_input: &mut LexerInput) -> Option<Expression> {
        let left = Expression::unary(input, lexer_input)?;
        if input.peek(Token::is_binary_op) {
            let op = input.next().unwrap().0;
            let right = Expression::parse(input, lexer_input).unwrap_or_else(|| {
                error!(
                    lexer_input.clone(),
                    input
                        .next()
                        .map_or(input.tokens.last().unwrap().1.clone(), |t| t.1),
                    "Expected expression"
                );
            });
            Some(Expression::Binary(Box::new(left), op, Box::new(right)))
        } else {
            Some(left)
        }
    }
    pub fn unary(input: &mut ParserInput, lexer_input: &mut LexerInput) -> Option<Expression> {
        if input.peek(Token::is_unary_op) {
            Some(Expression::Unary(
                input.next().unwrap().0,
                Box::new(Expression::unary(input, lexer_input)?),
            ))
        } else {
            Expression::block(input, lexer_input)
        }
    }
    pub fn block(input: &mut ParserInput, lexer_input: &mut LexerInput) -> Option<Expression> {
        if input.peek(|t| matches!(t, Token::OpenBrace)) {
            input.next();
            let mut expressions = Vec::new();
            while let Some(expr) = Statement::parse(input, lexer_input) {
                expressions.push(expr);
                if input.peek(|t| matches!(t, Token::CloseBrace)) {
                    break;
                }
            }
            if !input.peek(|t| matches!(t, Token::CloseBrace)) {
                error!(
                    lexer_input.clone(),
                    input
                        .next()
                        .map_or(input.tokens.last().unwrap().1.clone(), |t| t.1),
                    "Expected '}}'"
                );
            } else {
                input.next();
            }
            Some(Expression::Block(expressions))
        } else {
            Expression::function_call(input, lexer_input)
        }
    }
    pub fn function_call(
        input: &mut ParserInput,
        lexer_input: &mut LexerInput,
    ) -> Option<Expression> {
        let expr = Expression::path(input, lexer_input)?;
        match expr {
            Expression::Path(path) => {
                if input.peek(|t| matches!(t, Token::OpenParen)) {
                    let mut args = Vec::new();
                    input.next();
                    while let Some(arg) = Expression::parse(input, lexer_input) {
                        args.push(arg);
                        if input.peek(|t| matches!(t, Token::CloseParen)) {
                            break;
                        }
                        if !input.peek(|t| matches!(t, Token::Comma)) {
                            error!(
                                lexer_input.clone(),
                                input
                                    .next()
                                    .map_or(input.tokens.last().unwrap().1.clone(), |t| t.1),
                                "Expected comma"
                            );
                        } else {
                            input.next();
                        }
                    }
                    if !input.peek(|t| matches!(t, Token::CloseParen)) {
                        error!(
                            lexer_input.clone(),
                            input
                                .next()
                                .map_or(input.tokens.last().unwrap().1.clone(), |t| t.1),
                            "Expected closing paren"
                        );
                    } else {
                        input.next();
                    }
                    Some(Expression::FunctionCall(path, args))
                } else {
                    Some(Expression::Path(path))
                }
            }
            _ => Some(expr),
        }
    }
    pub fn path(input: &mut ParserInput, lexer_input: &mut LexerInput) -> Option<Expression> {
        let mut nodes = Vec::new();
        let mut had_dot = false;
        if input.peek(|t| matches!(t, Token::Identifier(_))) {
            nodes.push(match input.next().unwrap().0 {
                Token::Identifier(s) => s,
                _ => unreachable!(),
            });
        } else if let Some(primary) = Expression::primary(input, lexer_input) {
            return Some(primary);
        } else {
            return None;
        }
        while let Some(t) = input.next() {
            match t.0 {
                Token::Dot => {
                    if had_dot {
                        error!(lexer_input.clone(), t.1, "Unexpected dot");
                    } else {
                        had_dot = true;
                    }
                }
                Token::Identifier(s) if had_dot => {
                    had_dot = false;
                    nodes.push(s);
                }
                _ => {
                    input.rewind();
                    return Some(Expression::Path(nodes));
                }
            }
        }
        unreachable!()
    }
    pub fn primary(input: &mut ParserInput, _: &mut LexerInput) -> Option<Expression> {
        if input.peek(|t| {
            matches!(
                t,
                Token::StringLiteral(_)
                    | Token::SignedLiteral(_)
                    | Token::UnsignedLiteral(_)
                    | Token::FloatLiteral(_)
            )
        }) {
            match input.next().unwrap().0 {
                Token::StringLiteral(s) => Some(Expression::StringLiteral(s)),
                Token::SignedLiteral(s) => Some(Expression::SignedLiteral(s)),
                Token::UnsignedLiteral(s) => Some(Expression::UnsignedLiteral(s)),
                Token::FloatLiteral(s) => Some(Expression::FloatLiteral(s)),
                _ => unreachable!(),
            }
        } else {
            None
        }
    }
}
