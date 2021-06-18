use crate::tokenizer::token::Token;
#[derive(Debug)]
pub enum ParserType {
    Shift,
    Reduce(usize),
}
#[derive(Debug)]
pub struct ParserItem {
    pub token: Token,
    pub action: ParserType,
}

impl ParserItem {
    pub(in crate::parser) fn new(token: &Token, action: ParserType) -> Self {
        Self {
            token: token.clone(),
            action,
        }
    }
}
