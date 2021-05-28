use crate::tokenizer::position::Position;
use crate::tokenizer::read_token_state::empty::Empty;
use crate::tokenizer::read_token_state::{ReadChar, ReadTokenState};
use crate::tokenizer::token::{Token, TokenType};

#[derive(Clone)]
pub(in crate::tokenizer) struct More {
    position: Position,
}
impl ReadChar for More {
    fn read_char(&self, c: char, position: &Position) -> (Option<Token>, ReadTokenState, bool) {
        let empty = ReadTokenState::Empty(Empty::new(position.clone()));
        match c {
            ' ' | '\n' | '\r' => (
                Some(Token::new(">".to_string(), TokenType::Greater)),
                empty,
                true,
            ),
            '=' => (
                Some(Token::new(">=".to_string(), TokenType::GreaterEqual)),
                empty,
                true,
            ),
            _ => (
                Some(Token::new(">".to_string(), TokenType::Greater)),
                empty,
                false,
            ),
        }
    }
}
impl More {
    pub(in crate::tokenizer::read_token_state) fn new(position: Position) -> Self {
        Self { position }
    }
}
