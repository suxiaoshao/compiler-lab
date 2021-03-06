use crate::tokenizer::position::Position;
use crate::tokenizer::read_token_state::empty::Empty;
use crate::tokenizer::read_token_state::{ReadChar, ReadTokenState};
use crate::tokenizer::token::Token;
use crate::tokenizer::token_type::TokenType;

#[derive(Clone, Debug)]
pub(in crate::tokenizer) struct Equal {
    position: Position,
}
impl ReadChar for Equal {
    fn read_char(
        &self,
        c: char,
        position: &Position,
        pre_position: &Position,
    ) -> (Option<Token>, ReadTokenState, bool) {
        let empty = ReadTokenState::Empty(Empty::new(position.clone()));
        match c {
            ' ' | '\n' | '\r' => (
                Some(Token::new(
                    "=".to_string(),
                    TokenType::Assign,
                    &self.position,
                    pre_position,
                )),
                empty,
                true,
            ),
            '=' => (
                Some(Token::new(
                    "==".to_string(),
                    TokenType::Equal,
                    &self.position,
                    &position,
                )),
                empty,
                true,
            ),
            _ => (
                Some(Token::new(
                    "=".to_string(),
                    TokenType::Assign,
                    &self.position,
                    pre_position,
                )),
                empty,
                false,
            ),
        }
    }
}
impl Equal {
    pub(in crate::tokenizer::read_token_state) fn new(position: Position) -> Self {
        Self { position }
    }
}
