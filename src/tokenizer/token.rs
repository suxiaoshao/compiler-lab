use crate::tokenizer::position::Position;
use colored::Colorize;

/// token 的 类型
#[derive(Clone, Copy, Debug)]
pub enum TokenType {
    Unknown,
    // 未知类型
    Int,
    IntNum,
    Real,
    // 实数
    RealNum,
    Bool,
    True,
    False,
    If,
    Else,
    For,
    Return,
    Break,
    Id,
    //标识符
    Add,
    Sub,
    Mul,
    Div,
    Assign,
    Equal,
    NotEqual,
    And,
    Or,
    Not,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    Comma,
    // ,
    Semicolon,
    // ;
    LeftBlock,
    RightBlock,
    SqLeftBracket,
    // [
    SqRightBracket,
    // ]
    CirLeftBracket,
    // (
    CirRightBracket, // )
}

#[derive(Clone, Debug)]
pub struct Token {
    pub(in crate::tokenizer) lex: String,
    pub(in crate::tokenizer) token_type: TokenType,
    pub(in crate::tokenizer) start: Position,
    pub(in crate::tokenizer) end: Position,
}

impl Token {
    /// 新建一个 token
    pub(in crate::tokenizer) fn new(
        lex: String,
        token_type: TokenType,
        start: &Position,
        end: &Position,
    ) -> Self {
        Self {
            lex,
            token_type,
            start: start.clone(),
            end: end.clone(),
        }
    }
    pub(in crate::tokenizer) fn check_position(&mut self) {
        if self.end.x - self.start.x == self.lex.len() as u64 {
            self.end.back();
        }
    }
    pub fn display(&self) -> String {
        format!(
            "<{} {} {}>",
            format!("'{}'", self.lex).green(),
            format!("{:?}", self.token_type).yellow(),
            format!("{}-{}", self.start.to_string(), self.end.to_string()).cyan(),
        )
    }
}