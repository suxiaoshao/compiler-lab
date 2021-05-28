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
    Mod,
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
    // pub(in crate::tokenizer) start: Position,
    // pub(in crate::tokenizer) end:Position
}

impl Token {
    /// 新建一个 token
    pub(in crate::tokenizer) fn new(lex: String, token_type: TokenType) -> Self {
        Self { lex, token_type }
    }
    pub fn display(&self) -> String {
        format!(
            "<{} {}>",
            format!("'{}'", self.lex).green(),
            format!("{:?}", self.token_type).yellow()
        )
    }
}
