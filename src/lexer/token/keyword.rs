use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Keyword {
    Break,
    Continue,
    Else,
    False,
    Let,
    Function,
    For,
    If,
    Loop,
    Match,
    Return,
    True,
    While,
}
impl FromStr for Keyword {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "break" => Ok(Keyword::Break),
            "continue" => Ok(Keyword::Continue),
            "else" => Ok(Keyword::Else),
            "false" => Ok(Keyword::False),
            "let" => Ok(Keyword::Let),
            "function" => Ok(Keyword::Function),
            "for" => Ok(Keyword::For),
            "if" => Ok(Keyword::If),
            "loop" => Ok(Keyword::Loop),
            "match" => Ok(Keyword::Match),
            "return" => Ok(Keyword::Return),
            "true" => Ok(Keyword::True),
            "while" => Ok(Keyword::While),
            _ => Err(()),
        }
    }
}
