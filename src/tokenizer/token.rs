use colored::Colorize;

use crate::tokenizer::position::Position;
use crate::tokenizer::token_type::TokenType;

#[derive(Clone, Debug)]
pub struct Token {
    pub(in crate::tokenizer) lex: String,
    pub(in crate) token_type: TokenType,
    pub(in crate::tokenizer) start: Position,
    pub(in crate::tokenizer) end: Position,
}

impl Token {
    /// 新建一个 token
    pub(in crate) fn new(
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
    pub fn show_string(&self) -> String {
        format!(
            "<{} {} {}>",
            format!("'{}'", self.lex).green(),
            format!("{:?}", self.token_type).yellow(),
            format!("{}-{}", self.start.to_string(), self.end.to_string()).cyan(),
        )
    }
}
