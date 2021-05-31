use serde::{Deserialize, Serialize};

use crate::tokenizer::token_type::TokenType::{
    Add, And, Assign, Bool, Break, CirLeftBracket, CirRightBracket, Div, Else, Eof, Epsilon, Equal,
    False, Greater, GreaterEqual, Id, If, Int, IntNum, LeftBlock, Less, LessEqual, Mul, Not,
    NotEqual, Or, Real, RealNum, RightBlock, Semicolon, SqLeftBracket, SqRightBracket, Sub, True,
    While,
};

/// token 的 类型
#[derive(Clone, Copy, Debug, Eq, PartialEq, Deserialize, Serialize, PartialOrd, Ord)]
pub enum TokenType {
    #[serde(rename = "ε")]
    Epsilon = 0, // 未知类型
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
    #[serde(rename = "while")]
    While,
    #[serde(rename = "$")]
    Eof, // $
}

impl TokenType {
    pub(crate) fn show_string(&self) -> String {
        let show_string = serde_json::to_string(self).unwrap();
        show_string[1..show_string.len() - 1].to_string()
    }
    pub(crate) fn get_all_vec() -> Vec<TokenType> {
        vec![
            Epsilon,
            Int,
            IntNum,
            Real,
            RealNum,
            Bool,
            True,
            False,
            If,
            Else,
            Break,
            Id,
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
            Semicolon,
            LeftBlock,
            RightBlock,
            SqLeftBracket,
            SqRightBracket,
            CirLeftBracket,
            CirRightBracket,
            While,
            Eof,
        ]
    }
}
