use crate::lexer::token::keyword::Keyword;
use crate::lexer::token::Token;
use std::ops::Range;
use std::str::FromStr;
use std::vec::IntoIter;

pub mod token;

#[derive(Clone)]
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
        self.data.get(self.pos).map_or(false, |ch| func(ch))
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

pub struct Lexer {
    input: LexerInput,
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
        Some(match self.input.next()? {
            '(' => (Token::OpenParen, start..self.input.pos),
            ')' => (Token::CloseParen, start..self.input.pos),
            '{' => (Token::OpenBrace, start..self.input.pos),
            '}' => (Token::CloseBrace, start..self.input.pos),
            '[' => (Token::OpenBracket, start..self.input.pos),
            ']' => (Token::CloseBracket, start..self.input.pos),
            '+' => match self.input.peek_for('=') {
                true => (Token::AddAssign, start..self.input.pos),
                false => (Token::Add, start..self.input.pos),
            },
            '-' => match self.input.peek_for('=') {
                true => match self.input.peek(char::is_ascii_digit) {
                    true => (Token::SubAssign, start..self.input.pos),
                    false => match parse_number(&mut self.input) {
                        Token::UnsignedLiteral(n) => {
                            (Token::SignedLiteral(-(n as i64)), start..self.input.pos)
                        }
                        Token::FloatLiteral(n) => (Token::FloatLiteral(n), start..self.input.pos),
                        _ => unreachable!(),
                    },
                },
                false => (Token::SubAssign, start..self.input.pos),
            },
            '*' => match self.input.peek_for('=') {
                true => (Token::MulAssign, start..self.input.pos),
                false => match self.input.peek_for('*') {
                    true => match self.input.peek_for('=') {
                        true => (Token::PowAssign, start..self.input.pos),
                        false => (Token::Pow, start..self.input.pos),
                    },
                    false => (Token::Mul, start..self.input.pos),
                },
            },
            '/' => match self.input.peek_for('=') {
                true => (Token::DivAssign, start..self.input.pos),
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
                        false => (Token::Div, start..self.input.pos),
                    },
                },
            },
            '%' => match self.input.peek_for('=') {
                true => (Token::ModAssign, start..self.input.pos),
                false => (Token::Mod, start..self.input.pos),
            },
            '&' => match self.input.peek_for('=') {
                true => (Token::BitAndAssign, start..self.input.pos),
                false => match self.input.peek_for('&') {
                    true => (Token::And, start..self.input.pos),
                    false => (Token::BitAnd, start..self.input.pos),
                },
            },
            '|' => match self.input.peek_for('=') {
                true => (Token::BitOrAssign, start..self.input.pos),
                false => match self.input.peek_for('|') {
                    true => (Token::Or, start..self.input.pos),
                    false => (Token::BitOr, start..self.input.pos),
                },
            },
            '^' => match self.input.peek_for('=') {
                true => (Token::BitXorAssign, start..self.input.pos),
                false => (Token::BitXor, start..self.input.pos),
            },
            '!' => match self.input.peek_for('=') {
                true => (Token::Ne, start..self.input.pos),
                false => (Token::Not, start..self.input.pos),
            },
            '=' => match self.input.peek_for('=') {
                true => (Token::Eq, start..self.input.pos),
                false => (Token::Assign, start..self.input.pos),
            },
            '<' => match self.input.peek_for('=') {
                true => (Token::Le, start..self.input.pos),
                false => (Token::Lt, start..self.input.pos),
            },
            '>' => match self.input.peek_for('=') {
                true => (Token::Ge, start..self.input.pos),
                false => (Token::Gt, start..self.input.pos),
            },
            ',' => (Token::Comma, start..self.input.pos),
            ';' => (Token::Semicolon, start..self.input.pos),
            ':' => (Token::Colon, start..self.input.pos),
            '"' => {
                let mut string = String::new();
                while let Some(ch) = self.input.next() {
                    if ch == '"' {
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
                                    char::from_u32(u32::from_str_radix(&hex, 16).unwrap()).unwrap(),
                                );
                            }
                            _ => {
                                let end = self.input.pos;
                                error!(&mut self.input, start..end, "Invalid escape sequence")
                            }
                        }
                    }
                    string.push(ch);
                }
                (Token::StringLiteral(string), start..self.input.pos)
            }
            '\'' => {
                if let Some(c) = self.input.next() {
                    if self.input.peek_for('\'') {
                        (Token::CharLiteral(c), start..self.input.pos)
                    } else {
                        let end = self.input.pos;
                        error!(&mut self.input, start..end, "Invalid character literal")
                    }
                } else {
                    let end = self.input.pos;
                    error!(&mut self.input, start..end, "Unexpected end of input")
                }
            }
            c if c.is_ascii_digit() => (parse_number(&mut self.input), start..self.input.pos),
            c if c.is_alphabetic() => {
                let mut string = String::new();
                string.push(c);
                while self.input.peek(|&c| c.is_alphanumeric()) {
                    string.push(self.input.next().unwrap());
                }
                if let Ok(keyword) = Keyword::from_str(&string) {
                    (Token::Keyword(keyword), start..self.input.pos)
                } else {
                    (Token::Identifier(string), start..self.input.pos)
                }
            }
            c if c.is_whitespace() => return self.next(),
            _ => {
                let end = self.input.pos;
                error!(&mut self.input, start..end, "Unexpected character")
            }
        })
    }
}
