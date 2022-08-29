use crate::lexer::token::keyword::Keyword;
use crate::lexer::token::Token;
use std::ops::Range;
use std::str::FromStr;

pub mod token;

pub struct LexerInput {
    data: Vec<char>,
    pos: usize,
}
impl LexerInput {
    fn next(&mut self) -> Option<char> {
        self.pos += 1;
        self.data.get(self.pos - 1).cloned()
    }
    fn peek_for(&mut self, c: char) -> bool {
        self.data.get(self.pos).map_or(false, |&ch| {
            if ch == c {
                self.pos += 1;
                true
            } else {
                false
            }
        })
    }
    fn peek_for_str(&mut self, s: &str) -> bool {
        let mut matches = true;
        for (i, char) in s.chars().enumerate() {
            if self.data[self.pos + i] != char {
                matches = false;
                break;
            }
        }
        if matches {
            self.pos += s.len();
        }
        matches
    }
    fn peek<F>(&mut self, func: F) -> bool
    where
        F: Fn(&char) -> bool,
    {
        self.data.get(self.pos).map_or(false, func)
    }
}
impl From<String> for LexerInput {
    fn from(s: String) -> Self {
        (*s).into()
    }
}
impl From<&str> for LexerInput {
    fn from(s: &str) -> Self {
        LexerInput {
            data: s.chars().collect::<Vec<_>>(),
            pos: 0,
        }
    }
}
impl Iterator for LexerInput {
    type Item = char;
    fn next(&mut self) -> Option<Self::Item> {
        self.next()
    }
}
impl Clone for LexerInput {
    fn clone(&self) -> Self {
        LexerInput {
            data: self.data.clone(),
            pos: 0,
        }
    }
}

fn parse_number(input: &mut LexerInput) -> Token {
    let mut number = String::new();
    while let Some(ch) = input.next() {
        if ch.is_ascii_digit() {
            number.push(ch);
        } else {
            break;
        }
    }
    if number.contains('.') {
        Token::FloatLiteral(f64::from_str(&number).unwrap())
    } else {
        Token::UnsignedLiteral(u64::from_str(&number).unwrap())
    }
}

#[derive(Clone)]
pub struct Lexer {
    pub input: LexerInput,
}
impl Lexer {
    pub fn new(input: impl Into<LexerInput>) -> Self {
        Self {
            input: input.into(),
        }
    }
}
impl Iterator for Lexer {
    type Item = (Token, Range<usize>);

    fn next(&mut self) -> Option<Self::Item> {
        let start = self.input.pos;
        Some((
            match self.input.next()? {
                '(' => Token::OpenParen,
                ')' => Token::CloseParen,
                '{' => Token::OpenBrace,
                '}' => Token::CloseBrace,
                '[' => Token::OpenBracket,
                ']' => Token::CloseBracket,
                '+' => match self.input.peek_for('=') {
                    true => Token::AddAssign,
                    false => Token::Add,
                },
                '-' => match self.input.peek_for('=') {
                    true => Token::SubAssign,
                    false => match self.input.peek(char::is_ascii_digit) {
                        true => match parse_number(&mut self.input) {
                            Token::UnsignedLiteral(n) => Token::SignedLiteral(-(n as i64)),
                            Token::FloatLiteral(n) => Token::FloatLiteral(n),
                            _ => unreachable!(),
                        },
                        false => match self.input.peek_for('>') {
                            true => Token::Arrow,
                            false => Token::Sub,
                        },
                    },
                },
                '*' => match self.input.peek_for('=') {
                    true => Token::MulAssign,
                    false => match self.input.peek_for('*') {
                        true => match self.input.peek_for('=') {
                            true => Token::PowAssign,
                            false => Token::Pow,
                        },
                        false => Token::Mul,
                    },
                },
                '/' => match self.input.peek_for('=') {
                    true => Token::DivAssign,
                    false => match self.input.peek_for('/') {
                        true => {
                            while let Some(ch) = self.input.next() {
                                if ch == '\n' {
                                    break;
                                }
                            }
                            return self.next();
                        }
                        false => match self.input.peek_for('*') {
                            true => {
                                while let Some(ch) = self.input.next() {
                                    if ch == '*' && self.input.peek_for('/') {
                                        break;
                                    }
                                }
                                return self.next();
                            }
                            false => Token::Div,
                        },
                    },
                },
                '%' => match self.input.peek_for('=') {
                    true => Token::ModAssign,
                    false => Token::Mod,
                },
                '&' => match self.input.peek_for('=') {
                    true => Token::BitAndAssign,
                    false => match self.input.peek_for('&') {
                        true => Token::And,
                        false => Token::BitAnd,
                    },
                },
                '|' => match self.input.peek_for('=') {
                    true => Token::BitOrAssign,
                    false => match self.input.peek_for('|') {
                        true => Token::Or,
                        false => Token::BitOr,
                    },
                },
                '^' => match self.input.peek_for('=') {
                    true => Token::BitXorAssign,
                    false => Token::BitXor,
                },
                '!' => match self.input.peek_for('=') {
                    true => Token::Ne,
                    false => Token::Not,
                },
                '=' => match self.input.peek_for('=') {
                    true => Token::Eq,
                    false => Token::Assign,
                },
                '<' => match self.input.peek_for('=') {
                    true => Token::Le,
                    false => Token::Lt,
                },
                '>' => match self.input.peek_for('=') {
                    true => Token::Ge,
                    false => Token::Gt,
                },
                ',' => Token::Comma,
                ';' => Token::Semicolon,
                '.' => Token::Dot,
                ':' => Token::Colon,
                '"' => {
                    let mut string = String::new();
                    let mut ended = false;
                    while let Some(ch) = self.input.next() {
                        if ch == '"' {
                            ended = true;
                            break;
                        }
                        if ch == '\\' {
                            match self.input.next()? {
                                'n' => string.push('\n'),
                                'r' => string.push('\r'),
                                't' => string.push('\t'),
                                '\\' => string.push('\\'),
                                '"' => string.push('"'),
                                'u' => {
                                    let mut hex = String::new();
                                    for _ in 0..4 {
                                        hex.push(self.input.next()?);
                                    }
                                    string.push(
                                        char::from_u32(u32::from_str_radix(&hex, 16).unwrap())
                                            .unwrap(),
                                    );
                                }
                                _ => error!(
                                    self.input.clone(),
                                    start..self.input.pos,
                                    "Invalid escape sequence"
                                ),
                            }
                        }
                        string.push(ch);
                    }
                    if !ended {
                        error!(
                            self.input.clone(),
                            start..self.input.pos,
                            "Unterminated string"
                        );
                    }
                    Token::StringLiteral(string)
                }
                '\'' => {
                    if let Some(c) = self.input.next() {
                        if self.input.peek_for('\'') {
                            Token::CharLiteral(c)
                        } else {
                            error!(
                                self.input.clone(),
                                start..self.input.pos,
                                "Invalid character literal"
                            )
                        }
                    } else {
                        error!(
                            self.input.clone(),
                            start..self.input.pos,
                            "Unexpected end of input"
                        )
                    }
                }
                c if c.is_ascii_digit() => parse_number(&mut self.input),
                c if c.is_alphabetic() => {
                    let mut string = String::new();
                    string.push(c);
                    while self.input.peek(|&c| c.is_alphanumeric()) {
                        string.push(self.input.next().unwrap());
                    }
                    if let Ok(keyword) = Keyword::from_str(&string) {
                        Token::Keyword(keyword)
                    } else {
                        Token::Identifier(string)
                    }
                }
                c if c.is_whitespace() => return self.next(),
                _ => error!(
                    self.input.clone(),
                    start..self.input.pos,
                    "Unexpected character"
                ),
            },
            start..self.input.pos,
        ))
    }
}
