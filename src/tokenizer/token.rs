use colored::Colorize;
use serde::Deserialize;

use crate::tokenizer::position::Position;

/// token 的 类型
#[derive(Clone, Copy, Debug, Eq, PartialEq, Deserialize)]
pub enum TokenType {
    #[serde(rename = "ε")]
    Epsilon, // 未知类型
    #[serde(rename = "int")]
    Int, // int
    #[serde(rename = "int_num")]
    IntNum, // 整数
    #[serde(rename = "real")]
    Real, // real
    #[serde(rename = "real_num")]
    RealNum, // 实数
    #[serde(rename = "bool")]
    Bool, // bool
    #[serde(rename = "true")]
    True, // true
    #[serde(rename = "false")]
    False, // false
    #[serde(rename = "if")]
    If, // if
    #[serde(rename = "else")]
    Else, // else
    #[serde(rename = "break")]
    Break, // break
    #[serde(rename = "id")]
    Id, //标识符
    #[serde(rename = "+")]
    Add, // +
    #[serde(rename = "-")]
    Sub, // -
    #[serde(rename = "*")]
    Mul, // *
    #[serde(rename = "/")]
    Div, // /
    #[serde(rename = "=")]
    Assign, // =
    #[serde(rename = "==")]
    Equal, // ==
    #[serde(rename = "!=")]
    NotEqual, // !=
    #[serde(rename = "&&")]
    And, // &&
    #[serde(rename = "||")]
    Or, // ||
    #[serde(rename = "!")]
    Not, // !
    #[serde(rename = ">")]
    Greater, // >
    #[serde(rename = ">=")]
    GreaterEqual, // >=
    #[serde(rename = "<")]
    Less, // <
    #[serde(rename = "<=")]
    LessEqual, // <=
    #[serde(rename = ";")]
    Semicolon, // ;
    #[serde(rename = "{")]
    LeftBlock, // {
    #[serde(rename = "}")]
    RightBlock, // }
    #[serde(rename = "[")]
    SqLeftBracket, // [
    #[serde(rename = "]")]
    SqRightBracket, // ]
    #[serde(rename = "(")]
    CirLeftBracket, // (
    #[serde(rename = ")")]
    CirRightBracket, // )
    #[serde(rename = "$")]
    Eof, // $
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
    pub fn display(&self) -> String {
        format!(
            "<{} {} {}>",
            format!("'{}'", self.lex).green(),
            format!("{:?}", self.token_type).yellow(),
            format!("{}-{}", self.start.to_string(), self.end.to_string()).cyan(),
        )
    }
}
