use crate::lexer::token::keyword::Keyword;

pub mod keyword;

#[derive(Debug, PartialEq)]
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
