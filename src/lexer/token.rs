use crate::lexer::token::keyword::Keyword;

pub mod keyword;

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Identifier(String),
    Keyword(Keyword),
    SignedLiteral(i64),
    UnsignedLiteral(u64),
    FloatLiteral(f64),
    StringLiteral(String),
    CharLiteral(char),

    // Brackets
    /// (
    OpenParen,
    /// )
    CloseParen,
    /// {
    OpenBrace,
    /// }
    CloseBrace,
    /// [
    OpenBracket,
    /// ]
    CloseBracket,

    // Operators
    /// a + b
    Add,
    /// a - b
    Sub,
    /// a * b
    Mul,
    /// a / b
    Div,
    /// a % b
    Mod,
    /// a ** b
    Pow,
    /// a & b
    BitAnd,
    /// a | b
    BitOr,
    /// a ^ b
    BitXor,
    /// a << b
    Shl,
    /// a >> b
    Shr,
    /// a && b
    And,
    /// a || b
    Or,
    /// !a
    Not,
    /// a == b
    Eq,
    /// a != b
    Ne,
    /// a < b
    Lt,
    /// a <= b
    Le,
    /// a > b
    Gt,
    /// a >= b
    Ge,
    /// a = b
    Assign,
    /// a += b
    AddAssign,
    /// a -= b
    SubAssign,
    /// a *= b
    MulAssign,
    /// a /= b
    DivAssign,
    /// a %= b
    ModAssign,
    /// a **= b
    PowAssign,
    /// a &= b
    BitAndAssign,
    /// a |= b
    BitOrAssign,
    /// a ^= b
    BitXorAssign,
    /// a <<= b
    ShlAssign,
    /// a >>= b
    ShrAssign,
    /// ->
    Arrow,

    // Delimiters
    /// ,
    Comma,
    /// ;
    Semicolon,
    /// :
    Colon,
    /// .
    Dot,
}
impl Token {
    pub fn is_binary_op(&self) -> bool {
        matches!(
            self,
            Token::Add
                | Token::Sub
                | Token::Mul
                | Token::Div
                | Token::Mod
                | Token::Pow
                | Token::BitAnd
                | Token::BitOr
                | Token::BitXor
                | Token::Shl
                | Token::Shr
                | Token::And
                | Token::Or
                | Token::Not
                | Token::Eq
                | Token::Ne
                | Token::Lt
                | Token::Le
                | Token::Gt
                | Token::Ge
                | Token::Assign
                | Token::AddAssign
                | Token::SubAssign
                | Token::MulAssign
                | Token::DivAssign
                | Token::ModAssign
                | Token::PowAssign
                | Token::BitAndAssign
                | Token::BitOrAssign
                | Token::BitXorAssign
                | Token::ShlAssign
                | Token::ShrAssign
        )
    }
    pub fn is_unary_op(&self) -> bool {
        matches!(self, Token::Not | Token::Sub)
    }
    pub fn is_assign_op(&self) -> bool {
        matches!(
            self,
            Token::Assign
                | Token::AddAssign
                | Token::SubAssign
                | Token::MulAssign
                | Token::DivAssign
                | Token::ModAssign
                | Token::PowAssign
                | Token::BitAndAssign
                | Token::BitOrAssign
                | Token::BitXorAssign
                | Token::ShlAssign
                | Token::ShrAssign
        )
    }
}
