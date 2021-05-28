use crate::tokenizer::position::Position;
use crate::tokenizer::read_token_state::empty::Empty;
use crate::tokenizer::read_token_state::{ReadChar, ReadTokenState};
use crate::tokenizer::token::{Token, TokenType};

#[derive(Clone, Debug)]
pub(in crate::tokenizer) struct Less {
    position: Position,
}
impl ReadChar for Less {
    fn read_char(&self, c: char, position: &Position) -> (Option<Token>, ReadTokenState, bool) {
        let position = position.clone();
        match c {
            ' ' | '\n' | '\r' => (
                Some(Token::new(
                    "<".to_string(),
                    TokenType::Less,
                    &self.position,
                    &position,
                )),
                ReadTokenState::Empty(Empty::new(position)),
                true,
            ),
            '=' => (
                Some(Token::new(
                    "<=".to_string(),
                    TokenType::LessEqual,
                    &self.position,
                    &position,
                )),
                ReadTokenState::Empty(Empty::new(position)),
                true,
            ),
            _ => (
                Some(Token::new(
                    "<".to_string(),
                    TokenType::Less,
                    &self.position,
                    &position,
                )),
                ReadTokenState::Empty(Empty::new(position)),
                false,
            ),
        }
    }
}
impl Less {
    pub(in crate::tokenizer::read_token_state) fn new(position: Position) -> Self {
        Self { position }
    }
}
