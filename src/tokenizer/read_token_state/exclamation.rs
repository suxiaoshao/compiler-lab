use crate::tokenizer::position::Position;
use crate::tokenizer::read_token_state::empty::Empty;
use crate::tokenizer::read_token_state::{ReadChar, ReadTokenState};
use crate::tokenizer::token::{Token, TokenType};

#[derive(Clone, Debug)]
pub(in crate::tokenizer) struct Exclamation {
    position: Position,
}
impl ReadChar for Exclamation {
    fn read_char(&self, c: char, position: &Position) -> (Option<Token>, ReadTokenState, bool) {
        let empty = ReadTokenState::Empty(Empty::new(position.clone()));
        match c {
            ' ' | '\n' | '\r' => (
                Some(Token::new(
                    "!".to_string(),
                    TokenType::Not,
                    &self.position,
                    &position,
                )),
                empty,
                true,
            ),
            '=' => (
                Some(Token::new(
                    "!=".to_string(),
                    TokenType::NotEqual,
                    &self.position,
                    &position,
                )),
                empty,
                true,
            ),
            _ => (
                Some(Token::new(
                    "!".to_string(),
                    TokenType::Not,
                    &self.position,
                    &position,
                )),
                empty,
                false,
            ),
        }
    }
}
impl Exclamation {
    pub(in crate::tokenizer::read_token_state) fn new(position: Position) -> Self {
        Self { position }
    }
}
